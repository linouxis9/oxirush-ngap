//! The message of a PDU as its type, and how long it lives: as long as the PDU, with
//! `decode_value`, and as long as what the PDU was decoded from, with `message`.

use oxirush_ngap::ngap::*;
use oxirush_ngap::{Error, ErrorKind};

/// An INITIAL UE MESSAGE of two IEs: RAN-UE-NGAP-ID 7, and a NAS-PDU of three octets.
const INITIAL_UE_MESSAGE: &str = "000f401100000200550002000700264004037e0041";

/// The message of octets that the caller keeps: it borrows from them, and the PDU that
/// it was read through is gone when the function returns.
fn read(received: &[u8]) -> Result<InitialUEMessage<'_>, Error> {
    let pdu = NGAP_PDU::decode_borrowed(received)?;
    pdu.message()
}

#[test]
fn a_message_outlives_the_pdu_that_borrowed_its_octets() -> Result<(), Error> {
    let received = hex::decode(INITIAL_UE_MESSAGE).unwrap();
    let message = read(&received)?;
    assert_eq!(message.protocol_ies.0.len(), 2);
    // Nothing is copied: the value of each IE is a part of what was received.
    let within = received.as_ptr_range();
    for ie in &message.protocol_ies.0 {
        assert!(ie.value.is_borrowed());
        assert!(within.contains(&ie.value.as_slice().as_ptr()));
    }
    let nas = &message.protocol_ies.0[1].value;
    assert_eq!(nas.as_slice(), [0x03, 0x7e, 0x00, 0x41]);
    Ok(())
}

#[test]
fn the_message_of_a_pdu_that_owns_its_octets_owns_its_own() -> Result<(), Error> {
    let received = hex::decode(INITIAL_UE_MESSAGE).unwrap();
    let kept: InitialUEMessage<'static> = {
        let pdu = NGAP_PDU::decode(&received)?;
        // As decode_value and into_owned give it.
        let borrowed: InitialUEMessage = pdu.decode_value()?;
        let kept: InitialUEMessage<'static> = pdu.message()?;
        assert_eq!(kept, borrowed.into_owned());
        kept
    };
    drop(received);
    assert!(kept.protocol_ies.0.iter().all(|ie| !ie.value.is_borrowed()));
    assert_eq!(kept.protocol_ies.0[0].value.as_slice(), [0x00, 0x07]);
    Ok(())
}

#[test]
fn a_message_of_another_type_is_refused_as_decode_value_refuses_it() -> Result<(), Error> {
    let received = hex::decode(INITIAL_UE_MESSAGE).unwrap();
    for pdu in [
        NGAP_PDU::decode_borrowed(&received)?,
        NGAP_PDU::decode(&received)?,
    ] {
        let refused = pdu.message::<NGSetupRequest>().unwrap_err();
        assert_eq!(refused.kind(), ErrorKind::WrongMessage);
        assert_eq!(refused, pdu.decode_value::<NGSetupRequest>().unwrap_err());
    }
    Ok(())
}

/// A message of 16384 octets and more is in fragments (X.691 clause 11.9.3.8), which
/// are gathered: the PDU then owns the octets of its message however it was decoded.
#[test]
fn a_message_in_fragments_is_read_through_a_pdu_that_borrows() -> Result<(), Error> {
    // A NAS-PDU of 20000 octets in an INITIAL UE MESSAGE
    let nas = NASPDU(vec![0x7e; 20_000].into());
    let ie = ProtocolIEField::new(
        ProtocolIEID(38),
        Criticality::reject,
        oxirush_ngap::aper::encode(&nas)?.into(),
    );
    let mut pdu = NGAP_PDU::decode(&hex::decode(INITIAL_UE_MESSAGE).unwrap())?;
    pdu.set_protocol_ies(ProtocolIEContainer(vec![ie]))?;
    let received = pdu.encode()?;
    assert!(received.len() > 20_000);

    let message = read(&received)?;
    assert_eq!(message.protocol_ies.0.len(), 1);
    assert_eq!(message, pdu.decode_value::<InitialUEMessage>()?);
    Ok(())
}
