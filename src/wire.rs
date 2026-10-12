//! The octets of a PDU and of a message that is a container of protocol IEs, read and
//! written without the generic codec.
//!
//! Every PDU is one of three alternatives with the same three components, and every
//! message but the private one is a SEQUENCE of one container of protocol IEs. In
//! aligned PER each of their components starts an octet, so both are read and written
//! octet by octet:
//!
//! ```text
//! PDU      0cc00000  procedureCode  kk000000  length  message
//! message  x0000000  count (2 octets)  then for each IE:
//!          id (2 octets)  kk000000  length  value
//! ```
//!
//! `cc` is the alternative of the PDU, `kk` a criticality, `x` the extension bit of
//! the message, and a length is the length determinant of an open type: one octet up
//! to 127, two octets up to 16383 (ITU-T X.691 (02/2021) §11.9.3.6-7).
//!
//! These functions read and write that layout and nothing else. What is not of it is
//! `None`, and the caller gives it to the codec of rasn, which reads it or says why it
//! does not: a PDU of an alternative that a later release adds, a message with an
//! extension addition, a length of 16384 or more, which is fragmented, a criticality
//! that is none of the three, and octets that end early or go on after the value. So
//! a value and an error are the same as without these functions.

use rasn::types::Any;

use crate::registry::{Criticality, ProtocolIEContainer, ProtocolIEField, ProtocolIEID};

/// The criticality that two bits are.
fn criticality(octet: u8) -> Option<Criticality> {
    match octet >> 6 {
        0 => Some(Criticality::reject),
        1 => Some(Criticality::ignore),
        2 => Some(Criticality::notify),
        _ => None,
    }
}

/// The octet that has a criticality in its first two bits.
fn criticality_octet(criticality: Criticality) -> u8 {
    match criticality {
        Criticality::reject => 0x00,
        Criticality::ignore => 0x40,
        Criticality::notify => 0x80,
    }
}

/// The octets of an open type at the start of `octets`, and what follows them: a
/// length determinant of one or two octets, then that many octets.
fn open_type(octets: &[u8]) -> Option<(&[u8], &[u8])> {
    let (length, rest) = match *octets {
        [length @ 0..=0x7f, ref rest @ ..] => (usize::from(length), rest),
        [high @ 0x80..=0xbf, low, ref rest @ ..] => {
            (usize::from(u16::from_be_bytes([high & 0x3f, low])), rest)
        }
        _ => return None,
    };
    (rest.len() >= length).then(|| rest.split_at(length))
}

/// Write the length determinant of an open type of `length` octets; `None` for a
/// length that is fragmented.
fn open_type_length(length: usize, octets: &mut Vec<u8>) -> Option<()> {
    match length {
        0..=0x7f => octets.push(length as u8),
        0x80..=0x3fff => octets.extend([0x80 | (length >> 8) as u8, length as u8]),
        _ => return None,
    }
    Some(())
}

/// The alternative, the procedure code, the criticality and the message of the PDU
/// that `octets` are, whole.
pub(crate) fn pdu(octets: &[u8]) -> Option<(u8, u8, Criticality, &[u8])> {
    let [alternative, code, criticality_bits, ref rest @ ..] = *octets else {
        return None;
    };
    // No extension bit, and one of the three alternatives.
    if alternative & 0x80 != 0 || alternative >> 5 > 2 {
        return None;
    }
    let (message, after) = open_type(rest)?;
    // An open type holds one octet at least, and a PDU ends with its message.
    if message.is_empty() || !after.is_empty() {
        return None;
    }
    Some((
        alternative >> 5,
        code,
        criticality(criticality_bits)?,
        message,
    ))
}

/// The octets of the PDU of an alternative, a procedure code, a criticality and a
/// message.
pub(crate) fn pdu_octets(
    alternative: u8,
    code: u8,
    criticality: Criticality,
    message: &[u8],
) -> Option<Vec<u8>> {
    if message.is_empty() {
        return None;
    }
    let mut octets = Vec::with_capacity(message.len() + 5);
    octets.extend([alternative << 5, code, criticality_octet(criticality)]);
    open_type_length(message.len(), &mut octets)?;
    octets.extend_from_slice(message);
    Some(octets)
}

/// The protocol IEs of the message that `octets` are, whole: a SEQUENCE of one
/// container of protocol IEs, without extension additions.
pub(crate) fn protocol_ies(octets: &[u8]) -> Option<ProtocolIEContainer> {
    let [preamble, high, low, ref rest @ ..] = *octets else {
        return None;
    };
    if preamble & 0x80 != 0 {
        return None;
    }
    let count = usize::from(u16::from_be_bytes([high, low]));
    // An IE is four octets at least: what a count claims is not reserved.
    let mut ies = Vec::with_capacity(count.min(rest.len() / 4));
    let mut rest = rest;
    for _ in 0..count {
        let [id_high, id_low, criticality_bits, ref after @ ..] = *rest else {
            return None;
        };
        let (value, after) = open_type(after)?;
        if value.is_empty() {
            return None;
        }
        ies.push(ProtocolIEField {
            id: ProtocolIEID(u16::from_be_bytes([id_high, id_low])),
            criticality: criticality(criticality_bits)?,
            value: Any::new(value.to_vec()),
        });
        rest = after;
    }
    rest.is_empty().then_some(ProtocolIEContainer(ies))
}

/// The octets of the message that has these protocol IEs.
pub(crate) fn message_octets(ies: &ProtocolIEContainer) -> Option<Vec<u8>> {
    let count = u16::try_from(ies.0.len()).ok()?;
    let values: usize = ies.0.iter().map(|ie| ie.value.as_bytes().len() + 5).sum();
    let mut octets = Vec::with_capacity(3 + values);
    octets.push(0);
    octets.extend(count.to_be_bytes());
    for ie in &ies.0 {
        let value = ie.value.as_bytes();
        if value.is_empty() {
            return None;
        }
        octets.extend(ie.id.0.to_be_bytes());
        octets.push(criticality_octet(ie.criticality));
        open_type_length(value.len(), &mut octets)?;
        octets.extend_from_slice(value);
    }
    Some(octets)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdu::AnyMessage;
    use crate::registry::Pdu;

    /// The octets of each message of the fixtures, then of each vector with content.
    fn fixtures() -> Vec<Vec<u8>> {
        let octets = |text: &str| -> Vec<u8> {
            let digits = text.as_bytes().chunks(2);
            digits
                .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
                .collect()
        };
        let messages = include_str!("../tests/fixtures/messages.tsv").lines();
        let lines = messages.chain(include_str!("../tests/fixtures/vectors.tsv").lines());
        let vectors = lines.filter(|line| !line.starts_with('#'));
        vectors
            .map(|line| octets(line.split('\t').nth(1).unwrap()))
            .collect()
    }

    /// Give `read` the `octets`, then what a peer may send in their place: the octets
    /// with one more, and for a place among them the octets up to it and the octets
    /// with each bit of it changed. The places are each of the first 32 octets, where
    /// the layout of a short message is, then one in 13, and each of the last 8. Octets
    /// that are more than 2048 are cut at their first 32 and their last 8 places, and
    /// none of their bits is changed: what they have more is the octets of one value.
    fn variants(octets: &[u8], mut read: impl FnMut(&[u8])) {
        let long = octets.len() > 2048;
        let chosen = |at: &usize| match *at {
            0..32 => true,
            at if at + 8 >= octets.len() => true,
            at => !long && at % 13 == 0,
        };
        read(octets);
        let mut other = [octets, &[0]].concat();
        read(&other);
        other.pop();
        for at in (0..octets.len()).filter(chosen) {
            read(&octets[..at]);
            for bit in (0..8).filter(|_| !long) {
                other[at] ^= 0x80 >> bit;
                read(&other);
                other[at] ^= 0x80 >> bit;
            }
        }
    }

    /// Whether a value is of the length that these functions read and write: one that
    /// is not in fragments.
    fn whole(value: &[u8]) -> bool {
        value.len() < 16384
    }

    /// What the codec of rasn reads as a PDU.
    fn pdu_by_rasn(octets: &[u8]) -> Option<Pdu> {
        crate::per::decode_complete(octets).ok()
    }

    /// What the codec of rasn reads as the protocol IEs of a message.
    fn ies_by_rasn(octets: &[u8]) -> Option<ProtocolIEContainer> {
        let message = crate::per::decode_complete::<AnyMessage>(octets).ok()?;
        Some(message.protocol_ies)
    }

    #[test]
    fn a_pdu_is_read_as_the_codec_of_rasn_reads_it() {
        let (mut read, mut left) = (0, 0);
        for fixture in fixtures() {
            // The octets of a PDU as a sender writes them are read here, but for a
            // message in fragments.
            let message = pdu_by_rasn(&fixture).unwrap();
            assert_eq!(pdu(&fixture).is_some(), whole(message.value().as_bytes()));
            variants(&fixture, |octets| {
                let Some((alternative, code, criticality, message)) = pdu(octets) else {
                    left += 1;
                    return;
                };
                let theirs = pdu_by_rasn(octets);
                let theirs = theirs.unwrap_or_else(|| panic!("rasn refuses {octets:02x?}"));
                assert_eq!(theirs.alternative(), alternative, "{octets:02x?}");
                assert_eq!(theirs.procedure_code(), code, "{octets:02x?}");
                assert_eq!(theirs.criticality(), criticality, "{octets:02x?}");
                assert_eq!(theirs.value().as_bytes(), message, "{octets:02x?}");
                read += 1;
            });
        }
        // Both ways are taken: what is not of the layout is left to rasn.
        assert!(
            read > 1000 && left > 1000,
            "{read} read, {left} left to rasn"
        );
    }

    #[test]
    fn the_protocol_ies_of_a_message_are_read_as_the_codec_of_rasn_reads_them() {
        let (mut read, mut left) = (0, 0);
        for fixture in fixtures() {
            let sent = pdu_by_rasn(&fixture).unwrap();
            // The private message is no container of protocol IEs.
            if sent.protocol_ies().is_err() {
                continue;
            }
            let message = sent.value().as_bytes();
            // The IEs of a message as a sender writes them are read here, but for a
            // value in fragments.
            let ies = ies_by_rasn(message).unwrap();
            let here = ies.0.iter().all(|ie| whole(ie.value.as_bytes()));
            assert_eq!(protocol_ies(message), here.then_some(ies));
            variants(message, |octets| match protocol_ies(octets) {
                Some(ies) => {
                    assert_eq!(Some(ies), ies_by_rasn(octets), "{octets:02x?}");
                    read += 1;
                }
                None => left += 1,
            });
        }
        assert!(
            read > 1000 && left > 1000,
            "{read} read, {left} left to rasn"
        );
    }

    #[test]
    fn a_pdu_and_a_message_are_written_as_the_codec_of_rasn_writes_them() {
        for fixture in fixtures() {
            let sent = pdu_by_rasn(&fixture).unwrap();
            let message = sent.value().as_bytes();
            let written = pdu_octets(
                sent.alternative(),
                sent.procedure_code(),
                sent.criticality(),
                message,
            );
            // A message and a value in fragments are left to rasn.
            assert_eq!(written, whole(message).then(|| fixture.clone()));
            if let Some(ies) = ies_by_rasn(message) {
                let written = ies.0.iter().all(|ie| whole(ie.value.as_bytes()));
                assert_eq!(message_octets(&ies), written.then(|| message.to_vec()));
            }
        }
        // A value of 16384 octets or more has a fragmented length, and an empty one
        // is not what an open type holds: both are left to rasn.
        let ie = |value: Vec<u8>| ProtocolIEField {
            id: ProtocolIEID(1),
            criticality: Criticality::reject,
            value: Any::new(value),
        };
        let longest = ProtocolIEContainer(vec![ie(vec![0; 16383])]);
        let by_rasn = rasn::aper::encode(&AnyMessage::of(longest.clone())).unwrap();
        assert_eq!(message_octets(&longest), Some(by_rasn));
        assert_eq!(
            message_octets(&ProtocolIEContainer(vec![ie(vec![0; 16384])])),
            None
        );
        assert_eq!(message_octets(&ProtocolIEContainer(vec![ie(vec![])])), None);
        assert_eq!(pdu_octets(0, 1, Criticality::reject, &[0; 16384]), None);
        assert_eq!(pdu_octets(0, 1, Criticality::reject, &[]), None);
    }
}
