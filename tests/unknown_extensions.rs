use oxirush_ngap::aper::{AsnType, Decode, Error, Reader, decode, encode};
use oxirush_ngap::ngap::{SD, SNSSAI, SST, SliceSupportList, ULCPSecurityInformation};

/// The first SliceSupportItem has a future SEQUENCE extension addition.
/// Its open type must be skipped before decoding the following item
/// (X.691 (02/2021) §19.8-19.9).
#[test]
fn unknown_sequence_addition_preserves_the_following_item() {
    let wire = hex::decode("000180080801000010").unwrap();
    let value = decode::<SliceSupportList>(&wire).unwrap();
    assert_eq!(
        value
            .0
            .iter()
            .map(|item| item.s_nssai.s_st.0[0])
            .collect::<Vec<_>>(),
        [1, 2],
    );
    // Unknown additions are skipped, so encoding retains the understood root.
    assert_eq!(
        encode(&value).unwrap(),
        hex::decode("000100080080").unwrap()
    );
}

/// An S-NSSAI, then a number of one octet: what follows the additions of a value is
/// read where they end.
#[derive(Debug, PartialEq)]
struct FollowingSNSSAI<'a> {
    value: SNSSAI<'a>,
    following: u8,
}

impl AsnType for FollowingSNSSAI<'_> {
    const NAME: &'static str = "FollowingSNSSAI";
}

impl<'a> Decode<'a> for FollowingSNSSAI<'a> {
    fn decode(reader: &mut Reader<'a>) -> Result<Self, Error> {
        Ok(Self {
            value: SNSSAI::decode(reader)?,
            following: reader.unsigned(0, 255)? as u8,
        })
    }
}

/// Hand-built X.691 fields, independent of the writer of the crate. In particular,
/// normally small lengths <=64 encode n-1 in six unaligned bits; larger
/// bitmaps use an unconstrained length (X.691 (02/2021) §11.9.3.4),
/// including fragments.
#[derive(Default)]
struct Wire(Vec<bool>);

impl Wire {
    fn bits(&mut self, value: usize, count: usize) {
        for bit in (0..count).rev() {
            self.0.push(value & (1 << bit) != 0);
        }
    }

    fn align(&mut self) {
        while !self.0.len().is_multiple_of(8) {
            self.0.push(false);
        }
    }

    fn byte(&mut self, value: u8) {
        self.align();
        self.bits(value.into(), 8);
    }

    fn length(&mut self, value: usize) {
        if value < 128 {
            self.byte(value as u8);
        } else {
            self.byte(0x80 | (value >> 8) as u8);
            self.byte(value as u8);
        }
    }

    fn finish(mut self) -> Vec<u8> {
        self.align();
        let octet = |bits: &[bool]| {
            bits.iter()
                .fold(0, |octet, bit| octet << 1 | u8::from(*bit))
        };
        self.0.chunks(8).map(octet).collect()
    }
}

fn extended_snssai(count: usize, fragmented_open_type: bool) -> Vec<u8> {
    let mut wire = Wire::default();
    wire.bits(1, 1); // SNSSAI has extension additions.
    wire.bits(0, 2); // SD and iE-Extensions are absent.
    wire.bits(1, 8); // SST is a fixed one-octet string, unaligned.
    if count <= 64 {
        wire.bits(0, 1);
        wire.bits(count - 1, 6);
        for bit in 0..count {
            wire.bits(usize::from(bit == 0 || bit == count - 1), 1);
        }
    } else {
        wire.bits(1, 1);
        let mut offset = 0;
        while count - offset >= 16384 {
            let blocks = ((count - offset) / 16384).min(4);
            wire.byte(0xc0 | blocks as u8);
            for bit in offset..offset + blocks * 16384 {
                wire.bits(usize::from(bit == 0 || bit == count - 1), 1);
            }
            offset += blocks * 16384;
        }
        wire.length(count - offset);
        for bit in offset..count {
            wire.bits(usize::from(bit == 0 || bit == count - 1), 1);
        }
    }
    for _ in 0..if count == 1 { 1 } else { 2 } {
        if fragmented_open_type {
            wire.byte(0xc1);
            for _ in 0..16384 {
                wire.byte(0x5a);
            }
            wire.byte(0); // Exact-size fragments need a final zero length.
        } else {
            wire.byte(1);
            wire.byte(0x5a);
        }
    }
    wire.byte(0x7c); // The following field must remain intact.
    wire.finish()
}

#[test]
fn unknown_additions_handle_small_large_and_fragmented_bitmaps() {
    let expected = FollowingSNSSAI {
        value: SNSSAI::new(SST([1]), None, None),
        following: 0x7c,
    };
    for count in [1, 64, 65, 128, 16384, 65536] {
        let wire = extended_snssai(count, false);
        assert_eq!(decode::<FollowingSNSSAI>(&wire).unwrap(), expected);
        assert!(decode::<FollowingSNSSAI>(&wire[..wire.len() - 1]).is_err());
    }
}

#[test]
fn unknown_fragmented_open_types_preserve_following_fields_and_reject_truncation() {
    for count in [1, 64, 65, 128] {
        let wire = extended_snssai(count, true);
        assert_eq!(decode::<FollowingSNSSAI>(&wire).unwrap().following, 0x7c);
        for cut in 0..wire.len() {
            assert!(
                decode::<FollowingSNSSAI>(&wire[..cut]).is_err(),
                "accepted truncated addition: bitmap={count}, prefix={cut}",
            );
        }
    }
}

#[test]
fn unknown_additions_reject_truncated_small_open_types() {
    for count in [1, 64, 65, 128] {
        let wire = extended_snssai(count, false);
        for cut in 0..wire.len() {
            assert!(decode::<FollowingSNSSAI>(&wire[..cut]).is_err());
        }
    }
}

#[test]
fn root_fields_keep_the_original_sequence_layout() {
    // The extension bit, a bit for each of the two members that may be absent, the SST
    // in eight bits, then the SD, which is three octets and so starts an octet.
    let value = SNSSAI::new(SST([1]), Some(SD([1, 2, 3])), None);
    let wire = encode(&value).unwrap();
    assert_eq!(wire, [0b0100_0000, 0b0010_0000, 1, 2, 3]);
    assert_eq!(decode::<SNSSAI>(&wire).unwrap(), value);
}

/// What X.691 (02/2021) gives no encoding for is refused. The extension bit of a
/// SEQUENCE is one when an addition is present, and zero otherwise (§19.1), and the
/// length of the bitmap of the additions is one at least (§19.8, §11.9.3.1 a)).
///
/// A differential run against the codec of rasn found the first of these, which the
/// crate read as a value without additions.
#[test]
fn an_extension_bit_without_an_addition_that_is_present_is_refused() {
    let refused = |octets: &[u8]| {
        let error = decode::<ULCPSecurityInformation>(octets).unwrap_err();
        error.to_string()
    };
    let none = "the extension bit of a SEQUENCE is set, and it has no extension addition";
    // The extension bit, no iE-Extensions, the 16 bits of the MAC and the 5 of the
    // count, then the bitmap of the additions: a one bit for the form of more than 64
    // bits, and its length, which is zero.
    let error = refused(&[0x80, 0x02, 0x01, 0x00]);
    assert!(error.ends_with(none), "{error}");
    // A bitmap of one addition, which is not present.
    let error = refused(&[0x80, 0x02, 0x00, 0x00]);
    assert!(error.ends_with(none), "{error}");
    // One addition that is present, counted in the form of more than 64 bits, where 64
    // bits at most have their length, less one, in six bits (§11.9.3.4).
    let error = refused(&[0x80, 0x02, 0x01, 0x01, 0x80, 0x01, 0xaa]);
    assert!(error.ends_with("has its length in six bits"), "{error}");
    // One addition that is present, in an open type of no octet: a complete encoding
    // is one octet at least (§11.1.3.1).
    let error = refused(&[0x80, 0x02, 0x00, 0x02, 0x00]);
    assert!(
        error.contains("an extension addition has no octet"),
        "{error}"
    );
    // The same addition in one octet is skipped, and the value is read.
    let value = decode::<ULCPSecurityInformation>(&[0x80, 0x02, 0x00, 0x02, 0x01, 0x00]).unwrap();
    assert_eq!(encode(&value).unwrap(), [0x00, 0x02, 0x00]);
}
