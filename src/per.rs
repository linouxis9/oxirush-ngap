//! Work around rasn 0.28 leaving unknown SEQUENCE additions in its APER input.

use rasn::prelude::*;

/// X.691 (02/2021) §19.8-19.9: the extension presence bitmap is followed
/// by one open type for each present addition. Small bitmaps are unaligned;
/// larger ones use an unconstrained length, including fragmentation
/// (§11.9.3.4). The additions are unknown to these generated SEQUENCE types.
pub(crate) fn skip_unknown_extensions<D: Decoder>(decoder: &mut D) -> Result<(), D::Error> {
    let present = if decoder.decode_bool(Tag::BOOL)? {
        decoder
            .decode_bit_string(Tag::BIT_STRING, Constraints::default())?
            .count_ones()
    } else {
        const LENGTH: Constraints = rasn::constraints!(rasn::value_constraint!(0, 63));
        let count = decoder.decode_integer::<usize>(Tag::INTEGER, LENGTH)? + 1;
        let mut present = 0;
        for _ in 0..count {
            present += usize::from(decoder.decode_bool(Tag::BOOL)?);
        }
        present
    };
    for _ in 0..present {
        let _ = Any::decode(decoder)?;
    }
    Ok(())
}

/// Keep the public SEQUENCE's AsnType and Encode implementations. In APER,
/// read its extension bit, decode the identical nonextensible root, then
/// consume the unknown additions. Other codecs use the original structure.
macro_rules! decode_extensible_sequence {
    ($name:ident, $identifier:literal {
        $($(#[$($attribute:tt)*])* $field:ident: [$($field_type:tt)*],)*
    }) => {
        impl rasn::Decode for $name {
            fn decode_with_tag_and_constraints<D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::types::Tag,
                constraints: rasn::types::Constraints,
            ) -> Result<Self, D::Error> {
                #[derive(rasn::AsnType, rasn::Decode)]
                #[rasn(automatic_tags, identifier = $identifier)]
                struct Root {
                    $($(#[$($attribute)*])* $field: $($field_type)*,)*
                }
                #[derive(rasn::AsnType, rasn::Decode)]
                #[rasn(automatic_tags, identifier = $identifier)]
                #[non_exhaustive]
                struct Extensible {
                    $($(#[$($attribute)*])* $field: $($field_type)*,)*
                }
                if decoder.codec() != rasn::Codec::Aper {
                    let value = Extensible::decode_with_tag_and_constraints(decoder, tag, constraints)?;
                    return Ok(Self { $($field: value.$field,)* });
                }
                let extensions_present = decoder.decode_bool(rasn::types::Tag::BOOL)?;
                let value = Root::decode_with_tag_and_constraints(decoder, tag, constraints)?;
                if extensions_present {
                    $crate::per::skip_unknown_extensions(decoder)?;
                }
                Ok(Self { $($field: value.$field,)* })
            }
        }
    };
}

pub(crate) use decode_extensible_sequence;
