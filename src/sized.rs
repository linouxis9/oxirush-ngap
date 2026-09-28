//! OCTET STRING and BIT STRING values whose size range makes rasn 0.28
//! misplace their length determinant in APER.
//!
//! A size with a range of 256 or more and an upper bound below 64K has a
//! length determinant of one or two octets, octet-aligned in APER (ITU-T
//! X.691 (02/2021) §11.5.7.2, §11.5.7.3, §11.9.3.3). rasn 0.28 encodes a
//! two-octet length without that alignment, which shows only where the
//! value does not start on an octet boundary. The generated bindings use
//! these types for such values inside a SEQUENCE or CHOICE. In APER they
//! encode the length as a constrained whole number, which rasn aligns, and
//! then the items, which the aligned length leaves octet-aligned (§16.11,
//! §17.8); every other encoding rule uses rasn's codec for the plain type
//! under the type's own size constraint.

use rasn::prelude::*;

/// An integer constraint that makes rasn write one octet-aligned octet.
const OCTET: Constraints = rasn::constraints!(rasn::value_constraint!(0, 255));

/// An `OCTET STRING (SIZE (LB..UB))` whose size range is 256 or more and
/// whose upper bound is below 64K.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SizedOctetString<const LB: usize, const UB: usize>(pub OctetString);

impl<const LB: usize, const UB: usize> SizedOctetString<LB, UB> {
    /// The length determinant as a constrained whole number.
    const LENGTH: Constraints = rasn::constraints!(rasn::value_constraint!(LB as i128, UB as i128));
    /// Refuses sizes that rasn already encodes correctly.
    const SIZE_NEEDS_ALIGNMENT: () = assert!(LB <= UB && UB - LB >= 255 && UB < 65536);
}

impl<const LB: usize, const UB: usize> AsnType for SizedOctetString<LB, UB> {
    const TAG: Tag = Tag::OCTET_STRING;
    const CONSTRAINTS: Constraints = rasn::constraints!(rasn::size_constraint!(LB, UB));
}

impl<const LB: usize, const UB: usize> Encode for SizedOctetString<LB, UB> {
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
        identifier: Identifier,
    ) -> Result<(), E::Error> {
        let () = Self::SIZE_NEEDS_ALIGNMENT;
        if encoder.codec() != rasn::Codec::Aper {
            return encoder
                .encode_octet_string(tag, Self::CONSTRAINTS, &self.0, identifier)
                .map(drop);
        }
        let _ =
            encoder.encode_integer(Tag::INTEGER, Self::LENGTH, &self.0.len(), Identifier::EMPTY)?;
        for octet in self.0.iter() {
            let _ = encoder.encode_integer(Tag::INTEGER, OCTET, octet, Identifier::EMPTY)?;
        }
        Ok(())
    }
}

impl<const LB: usize, const UB: usize> Decode for SizedOctetString<LB, UB> {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: Constraints,
    ) -> Result<Self, D::Error> {
        let () = Self::SIZE_NEEDS_ALIGNMENT;
        if decoder.codec() != rasn::Codec::Aper {
            return decoder
                .decode_octet_string(tag, Self::CONSTRAINTS)
                .map(Self);
        }
        let length = decoder.decode_integer::<usize>(Tag::INTEGER, Self::LENGTH)?;
        // The octets grow as they are read, so a length alone reserves nothing.
        let mut octets = Vec::new();
        for _ in 0..length {
            octets.push(decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?);
        }
        Ok(Self(octets.into()))
    }
}

impl<const LB: usize, const UB: usize> From<OctetString> for SizedOctetString<LB, UB> {
    fn from(octets: OctetString) -> Self {
        Self(octets)
    }
}

impl<const LB: usize, const UB: usize> From<Vec<u8>> for SizedOctetString<LB, UB> {
    fn from(octets: Vec<u8>) -> Self {
        Self(octets.into())
    }
}

impl<const LB: usize, const UB: usize> core::ops::Deref for SizedOctetString<LB, UB> {
    type Target = OctetString;

    fn deref(&self) -> &OctetString {
        &self.0
    }
}

/// A `BIT STRING (SIZE (LB..UB))`, or `(SIZE (LB..UB, ...))` when
/// `EXTENSIBLE`, whose size range is 256 or more and whose upper bound is
/// below 64K. A length outside an extensible root is unconstrained
/// (X.691 (02/2021) §16.6), which rasn encodes correctly.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SizedBitString<const LB: usize, const UB: usize, const EXTENSIBLE: bool>(pub BitString);

impl<const LB: usize, const UB: usize, const EXTENSIBLE: bool> SizedBitString<LB, UB, EXTENSIBLE> {
    /// The length determinant as a constrained whole number.
    const LENGTH: Constraints = rasn::constraints!(rasn::value_constraint!(LB as i128, UB as i128));
    /// Refuses sizes that rasn already encodes correctly.
    const SIZE_NEEDS_ALIGNMENT: () = assert!(LB <= UB && UB - LB >= 255 && UB < 65536);
}

impl<const LB: usize, const UB: usize, const EXTENSIBLE: bool> AsnType
    for SizedBitString<LB, UB, EXTENSIBLE>
{
    const TAG: Tag = Tag::BIT_STRING;
    const CONSTRAINTS: Constraints = if EXTENSIBLE {
        rasn::constraints!(rasn::size_constraint!(LB, UB, extensible))
    } else {
        rasn::constraints!(rasn::size_constraint!(LB, UB))
    };
}

impl<const LB: usize, const UB: usize, const EXTENSIBLE: bool> Encode
    for SizedBitString<LB, UB, EXTENSIBLE>
{
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
        identifier: Identifier,
    ) -> Result<(), E::Error> {
        let () = Self::SIZE_NEEDS_ALIGNMENT;
        if encoder.codec() != rasn::Codec::Aper {
            return encoder
                .encode_bit_string(tag, Self::CONSTRAINTS, &self.0, identifier)
                .map(drop);
        }
        let length = self.0.len();
        if EXTENSIBLE {
            let extended = !(LB..=UB).contains(&length);
            let _ = encoder.encode_bool(Tag::BOOL, extended, Identifier::EMPTY)?;
            if extended {
                return encoder
                    .encode_bit_string(tag, Constraints::default(), &self.0, identifier)
                    .map(drop);
            }
        }
        let _ = encoder.encode_integer(Tag::INTEGER, Self::LENGTH, &length, Identifier::EMPTY)?;
        let mut octets = self.0.chunks_exact(8);
        for octet in &mut octets {
            let octet = octet
                .iter()
                .fold(0u8, |octet, bit| octet << 1 | u8::from(*bit));
            let _ = encoder.encode_integer(Tag::INTEGER, OCTET, &octet, Identifier::EMPTY)?;
        }
        for bit in octets.remainder() {
            let _ = encoder.encode_bool(Tag::BOOL, *bit, Identifier::EMPTY)?;
        }
        Ok(())
    }
}

impl<const LB: usize, const UB: usize, const EXTENSIBLE: bool> Decode
    for SizedBitString<LB, UB, EXTENSIBLE>
{
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: Constraints,
    ) -> Result<Self, D::Error> {
        let () = Self::SIZE_NEEDS_ALIGNMENT;
        if decoder.codec() != rasn::Codec::Aper {
            return decoder.decode_bit_string(tag, Self::CONSTRAINTS).map(Self);
        }
        if EXTENSIBLE && decoder.decode_bool(Tag::BOOL)? {
            return decoder
                .decode_bit_string(tag, Constraints::default())
                .map(Self);
        }
        let length = decoder.decode_integer::<usize>(Tag::INTEGER, Self::LENGTH)?;
        // The bits grow as they are read, so a length alone reserves nothing.
        let mut bits = BitString::new();
        for _ in 0..length / 8 {
            let octet = decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?;
            bits.extend_from_bitslice(BitString::from_element(octet).as_bitslice());
        }
        for _ in 0..length % 8 {
            bits.push(decoder.decode_bool(Tag::BOOL)?);
        }
        Ok(Self(bits))
    }
}

impl<const LB: usize, const UB: usize, const EXTENSIBLE: bool> From<BitString>
    for SizedBitString<LB, UB, EXTENSIBLE>
{
    fn from(bits: BitString) -> Self {
        Self(bits)
    }
}

impl<const LB: usize, const UB: usize, const EXTENSIBLE: bool> core::ops::Deref
    for SizedBitString<LB, UB, EXTENSIBLE>
{
    type Target = BitString;

    fn deref(&self) -> &BitString {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    struct Octets {
        flag: bool,
        value: SizedOctetString<1, 1000>,
    }

    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    struct Bits {
        flag: bool,
        value: SizedBitString<1, 1024, true>,
    }

    fn bits(bytes: &[u8], length: usize) -> BitString {
        let mut bits = BitString::from_slice(bytes);
        bits.truncate(length);
        bits
    }

    /// The two-octet length and the octets both start on an octet boundary
    /// (X.691 (02/2021) §11.5.7.3, §17.8), as pycrate and asn1tools encode it.
    #[test]
    fn octets_have_an_aligned_two_octet_length() {
        let value = Octets {
            flag: true,
            value: vec![0x11, 0x22, 0x33].into(),
        };
        let wire = rasn::aper::encode(&value).unwrap();
        assert_eq!(wire, [0x80, 0x00, 0x02, 0x11, 0x22, 0x33]);
        assert_eq!(rasn::aper::decode::<Octets>(&wire).unwrap(), value);
    }

    /// The extension bit, the aligned two-octet length, then the bits
    /// (X.691 (02/2021) §16.6, §16.11).
    #[test]
    fn bits_have_an_aligned_two_octet_length() {
        for (value, wire) in [
            (bits(&[0xa0], 3), vec![0x80, 0x00, 0x02, 0xa0]),
            (
                bits(&[0xab, 0xcd, 0xe0], 20),
                vec![0x80, 0x00, 0x13, 0xab, 0xcd, 0xe0],
            ),
        ] {
            let value = Bits {
                flag: true,
                value: value.into(),
            };
            assert_eq!(rasn::aper::encode(&value).unwrap(), wire);
            assert_eq!(rasn::aper::decode::<Bits>(&wire).unwrap(), value);
        }
    }

    /// A length outside the extensible root is unconstrained (§16.6).
    #[test]
    fn bits_outside_the_root_have_an_unconstrained_length() {
        let value = Bits {
            flag: true,
            value: BitString::repeat(true, 1025).into(),
        };
        let wire = rasn::aper::encode(&value).unwrap();
        assert_eq!(&wire[..3], [0xc0, 0x84, 0x01]);
        assert_eq!(rasn::aper::decode::<Bits>(&wire).unwrap(), value);
    }
}
