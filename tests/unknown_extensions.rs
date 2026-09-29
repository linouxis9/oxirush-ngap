use oxirush_ngap::ngap::{SNSSAI, SST, SliceSupportList};
use rasn::prelude::*;

/// The first SliceSupportItem has a future SEQUENCE extension addition.
/// Its open type must be skipped before decoding the following item
/// (X.691 (02/2021) §19.8-19.9).
#[test]
fn unknown_sequence_addition_preserves_the_following_item() {
    let wire = hex::decode("000180080801000010").unwrap();
    let value = rasn::aper::decode::<SliceSupportList>(&wire).unwrap();
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
        rasn::aper::encode(&value).unwrap(),
        hex::decode("000100080080").unwrap()
    );
}

#[derive(AsnType, Decode, Debug, PartialEq)]
#[rasn(automatic_tags)]
struct FollowingSNSSAI {
    value: SNSSAI,
    #[rasn(value("0..=255"))]
    following: u8,
}

/// Hand-built X.691 fields, independent of rasn's encoder. In particular,
/// normally small lengths <=64 encode n-1 in six unaligned bits; larger
/// bitmaps use an unconstrained length (§11.9.3.4), including fragments.
#[derive(Default)]
struct Wire(BitString);

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
        self.0.into_vec()
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
        value: SNSSAI::new(SST([1].into()), None, None),
        following: 0x7c,
    };
    for count in [1, 64, 65, 128, 16384, 65536] {
        let wire = extended_snssai(count, false);
        assert_eq!(
            rasn::aper::decode::<FollowingSNSSAI>(&wire).unwrap(),
            expected
        );
        assert!(rasn::aper::decode::<FollowingSNSSAI>(&wire[..wire.len() - 1]).is_err());
    }
}

#[test]
fn unknown_fragmented_open_types_preserve_following_fields_and_reject_truncation() {
    for count in [1, 64, 65, 128] {
        let wire = extended_snssai(count, true);
        assert_eq!(
            rasn::aper::decode::<FollowingSNSSAI>(&wire)
                .unwrap()
                .following,
            0x7c
        );
        for cut in 0..wire.len() {
            assert!(
                rasn::aper::decode::<FollowingSNSSAI>(&wire[..cut]).is_err(),
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
            assert!(rasn::aper::decode::<FollowingSNSSAI>(&wire[..cut]).is_err());
        }
    }
}

#[test]
fn root_fields_and_other_codecs_keep_the_original_sequence_layout() {
    let value = SNSSAI::new(SST([1].into()), Some([1, 2, 3].into()), None);
    macro_rules! round_trip {
        ($codec:ident) => {
            let wire = rasn::$codec::encode(&value).unwrap();
            assert_eq!(rasn::$codec::decode::<SNSSAI>(&wire).unwrap(), value);
        };
    }
    round_trip!(aper);
    round_trip!(uper);
    round_trip!(ber);
    round_trip!(der);
    round_trip!(oer);
    round_trip!(coer);
    round_trip!(jer);
    round_trip!(xer);
}
