//! APER codecs that the generated bindings use in place of rasn 0.28's.

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
///
/// `Fields<false>` is that root and `Fields<true>` the SEQUENCE as declared.
/// They have one derived decoder and the public type's description of its
/// components, so the SEQUENCE is described once.
macro_rules! decode_extensible_sequence {
    ($name:ident {
        $($(#[$($attribute:tt)*])* $field:ident: [$($field_type:tt)*],)*
    }) => {
        impl rasn::Decode for $name {
            fn decode_with_tag_and_constraints<D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::types::Tag,
                constraints: rasn::types::Constraints,
            ) -> Result<Self, D::Error> {
                #[derive(rasn::Decode)]
                #[rasn(automatic_tags)]
                struct Fields<const EXTENSIBLE: bool> {
                    $($(#[$($attribute)*])* $field: $($field_type)*,)*
                }
                const COUNT: usize = <[&str]>::len(&[$(stringify!($field)),*]);
                impl<const EXTENSIBLE: bool> rasn::AsnType for Fields<EXTENSIBLE> {
                    const TAG: rasn::types::Tag = <$name as rasn::AsnType>::TAG;
                    const IDENTIFIER: rasn::types::Identifier =
                        <$name as rasn::AsnType>::IDENTIFIER;
                }
                impl<const EXTENSIBLE: bool> rasn::types::Constructed<COUNT, 0>
                    for Fields<EXTENSIBLE>
                {
                    const FIELDS: rasn::types::fields::Fields<COUNT> =
                        <$name as rasn::types::Constructed<COUNT, 0>>::FIELDS;
                    const IS_EXTENSIBLE: bool = EXTENSIBLE;
                }
                if decoder.codec() != rasn::Codec::Aper {
                    let value =
                        Fields::<true>::decode_with_tag_and_constraints(decoder, tag, constraints)?;
                    return Ok(Self { $($field: value.$field,)* });
                }
                let extensions_present = decoder.decode_bool(rasn::types::Tag::BOOL)?;
                let value =
                    Fields::<false>::decode_with_tag_and_constraints(decoder, tag, constraints)?;
                if extensions_present {
                    $crate::per::skip_unknown_extensions(decoder)?;
                }
                Ok(Self { $($field: value.$field,)* })
            }
        }
    };
}

pub(crate) use decode_extensible_sequence;

/// The codec of a `SEQUENCE (SIZE (lower..upper)) OF` newtype.
///
/// rasn's derived encoder can disagree with its decoder about the alignment
/// of the length and of the elements. In APER the length is encoded as a
/// constrained whole number, through rasn's integer codec, and the elements
/// follow it. An `unconstrained` list has an upper bound of 64K or more, so
/// its length is an unconstrained length determinant (X.691 §11.9.3.5):
/// rasn's sequence codec without the size, which is checked here. The
/// decoded list grows with the elements read, so that a count alone reserves
/// no memory. Every other encoding rule keeps rasn's sequence codec.
macro_rules! sequence_of {
    ($name:ident, $lower:literal, $upper:literal) => {
        impl rasn::Encode for $name {
            fn encode_with_tag_and_constraints<'b, E: rasn::Encoder<'b>>(
                &self,
                encoder: &mut E,
                tag: rasn::types::Tag,
                constraints: rasn::types::Constraints,
                identifier: rasn::types::Identifier,
            ) -> Result<(), E::Error> {
                if encoder.codec() != rasn::Codec::Aper {
                    return encoder
                        .encode_sequence_of(tag, &self.0, constraints, identifier)
                        .map(drop);
                }
                const LENGTH: rasn::types::Constraints =
                    rasn::constraints!(rasn::value_constraint!($lower, $upper));
                let _ = encoder.encode_integer(
                    rasn::types::Tag::INTEGER,
                    LENGTH,
                    &self.0.len(),
                    rasn::types::Identifier::EMPTY,
                )?;
                for value in &self.0 {
                    rasn::Encode::encode(value, encoder)?;
                }
                Ok(())
            }
        }
        impl rasn::Decode for $name {
            fn decode_with_tag_and_constraints<D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::types::Tag,
                constraints: rasn::types::Constraints,
            ) -> Result<Self, D::Error> {
                if decoder.codec() != rasn::Codec::Aper {
                    return decoder.decode_sequence_of(tag, constraints).map(Self);
                }
                const LENGTH: rasn::types::Constraints =
                    rasn::constraints!(rasn::value_constraint!($lower, $upper));
                let length = decoder.decode_integer::<usize>(rasn::types::Tag::INTEGER, LENGTH)?;
                let mut values = Vec::new();
                for _ in 0..length {
                    values.push(rasn::Decode::decode(decoder)?);
                }
                Ok(Self(values))
            }
        }
    };
    ($name:ident, $lower:literal, $upper:literal, unconstrained) => {
        impl rasn::Encode for $name {
            fn encode_with_tag_and_constraints<'b, E: rasn::Encoder<'b>>(
                &self,
                encoder: &mut E,
                tag: rasn::types::Tag,
                constraints: rasn::types::Constraints,
                identifier: rasn::types::Identifier,
            ) -> Result<(), E::Error> {
                if encoder.codec() != rasn::Codec::Aper {
                    return encoder
                        .encode_sequence_of(tag, &self.0, constraints, identifier)
                        .map(drop);
                }
                if !($lower..=$upper).contains(&self.0.len()) {
                    return Err(rasn::error::EncodeError::size_constraint_not_satisfied(
                        self.0.len(),
                        &rasn::types::constraints::Size::new(
                            rasn::types::constraints::Bounded::new($lower, $upper),
                        ),
                        encoder.codec(),
                    )
                    .into());
                }
                encoder
                    .encode_sequence_of(
                        tag,
                        &self.0,
                        rasn::types::Constraints::default(),
                        identifier,
                    )
                    .map(drop)
            }
        }
        impl rasn::Decode for $name {
            fn decode_with_tag_and_constraints<D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::types::Tag,
                constraints: rasn::types::Constraints,
            ) -> Result<Self, D::Error> {
                if decoder.codec() != rasn::Codec::Aper {
                    return decoder.decode_sequence_of(tag, constraints).map(Self);
                }
                let values =
                    decoder.decode_sequence_of(tag, rasn::types::Constraints::default())?;
                if !($lower..=$upper).contains(&values.len()) {
                    return Err(rasn::error::DecodeError::size_constraint_not_satisfied(
                        Some(values.len()),
                        concat!($lower, "..=", $upper).into(),
                        decoder.codec(),
                    )
                    .into());
                }
                Ok(Self(values))
            }
        }
    };
}

pub(crate) use sequence_of;
