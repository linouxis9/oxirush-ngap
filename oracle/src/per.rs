//! APER codecs that the bindings of the oracle use in place of rasn 0.29's.

use rasn::prelude::*;

/// X.691 (02/2021) §19.8-19.9: the extension presence bitmap is followed
/// by one open type for each present addition. Small bitmaps are unaligned;
/// larger ones use an unconstrained length, including fragmentation
/// (§11.9.3.4). The additions are unknown to these generated SEQUENCE types.
///
/// What the clauses give no encoding for is refused, where rasn 0.29 reads it: a bitmap
/// of no bit or of which no bit is set, as the extension bit is one when an addition is
/// present and zero otherwise (§19.1, and the note of §19.8); a bitmap of 64 bits at
/// most whose length is in the form of more; and an addition of no octet, as an open
/// type holds a complete encoding (§11.1.3.1).
pub fn skip_unknown_extensions<D: Decoder>(decoder: &mut D) -> Result<(), D::Error> {
    let refused = |why: &str, decoder: &D| {
        let codec = decoder.codec();
        Err(<D::Error as rasn::de::Error>::custom(why, codec))
    };
    let present = if decoder.decode_bool(Tag::BOOL)? {
        let bitmap = decoder.decode_bit_string(Tag::BIT_STRING, &Constraints::default())?;
        if (1..=64).contains(&bitmap.len()) {
            return refused(
                "a bitmap of 64 additions at most has its length in six bits",
                decoder,
            );
        }
        bitmap.count_ones()
    } else {
        const LENGTH: Constraints = rasn::constraints!(rasn::value_constraint!(0, 63));
        let count = decoder.decode_integer::<usize>(Tag::INTEGER, &LENGTH)? + 1;
        let mut present = 0;
        for _ in 0..count {
            present += usize::from(decoder.decode_bool(Tag::BOOL)?);
        }
        present
    };
    if present == 0 {
        return refused(
            "the extension bit is set, and no addition is present",
            decoder,
        );
    }
    for _ in 0..present {
        if Any::decode(decoder)?.as_bytes().is_empty() {
            return refused("an extension addition has no octet", decoder);
        }
    }
    Ok(())
}

/// Whether `E` is the encoder of the Packed Encoding Rules of rasn, which is one type
/// for the aligned and the unaligned variant. It is known where the code is compiled
/// for `E`, so that a branch on it is compiled for the encoders that take it and for no
/// other: the codec of a SEQUENCE is so compiled once for PER, and the derived one for
/// the other encoding rules alone.
///
/// Rust has no way to ask whether two types are the same one in a constant, so the
/// sizes say it. An encoder of rasn has the type of the encoder of a SEQUENCE of `R`
/// root components and `X` additions, and the one of PER holds a table of each: its
/// size changes with both, and `E` is taken for it when its encoders have the same
/// sizes for four pairs of them. The constants under the functions hold for the
/// version of rasn that the crate is built with, or the crate does not compile: the
/// size of the PER encoder changes with both numbers, and no other codec of rasn is
/// taken for it.
pub const fn is_per_encoder<'a, E: rasn::Encoder<'a>>() -> bool {
    use core::mem::size_of;
    use rasn::aper::Encoder as Per;
    size_of::<E::AnyEncoder<'static, 0, 0>>() == size_of::<Per<0, 0>>()
        && size_of::<E::AnyEncoder<'static, 1, 0>>() == size_of::<Per<1, 0>>()
        && size_of::<E::AnyEncoder<'static, 0, 1>>() == size_of::<Per<0, 1>>()
        && size_of::<E::AnyEncoder<'static, 5, 3>>() == size_of::<Per<5, 3>>()
}

/// Whether `D` is the decoder of the Packed Encoding Rules of rasn: see
/// [`is_per_encoder`].
pub const fn is_per_decoder<D: rasn::Decoder>() -> bool {
    use core::mem::size_of;
    use rasn::aper::Decoder as Per;
    size_of::<D::AnyDecoder<0, 0>>() == size_of::<Per<'static, 0, 0>>()
        && size_of::<D::AnyDecoder<1, 0>>() == size_of::<Per<'static, 1, 0>>()
        && size_of::<D::AnyDecoder<0, 1>>() == size_of::<Per<'static, 0, 1>>()
        && size_of::<D::AnyDecoder<5, 3>>() == size_of::<Per<'static, 5, 3>>()
}

const _: () = {
    use core::mem::size_of;
    use rasn::{aper, avn, ber, jer, oer, xer};
    assert!(size_of::<aper::Encoder<1, 0>>() != size_of::<aper::Encoder<0, 0>>());
    assert!(size_of::<aper::Encoder<0, 1>>() != size_of::<aper::Encoder<0, 0>>());
    assert!(size_of::<aper::Decoder<'static, 1, 0>>() != size_of::<aper::Decoder<'static, 0, 0>>());
    assert!(size_of::<aper::Decoder<'static, 0, 1>>() != size_of::<aper::Decoder<'static, 0, 0>>());
    assert!(is_per_encoder::<aper::Encoder<0, 0>>() && is_per_encoder::<aper::Encoder<7, 2>>());
    assert!(is_per_decoder::<aper::Decoder<'static, 0, 0>>());
    assert!(is_per_decoder::<aper::Decoder<'static, 7, 2>>());
    assert!(!is_per_encoder::<oer::Encoder<'static, 0, 0>>());
    assert!(!is_per_encoder::<ber::enc::Encoder>() && !is_per_encoder::<jer::enc::Encoder>());
    assert!(!is_per_encoder::<xer::enc::Encoder>() && !is_per_encoder::<avn::enc::Encoder>());
    assert!(!is_per_decoder::<oer::Decoder<'static, 0, 0>>());
    assert!(
        !is_per_decoder::<ber::de::Decoder<'static>>() && !is_per_decoder::<jer::de::Decoder>()
    );
    assert!(!is_per_decoder::<xer::de::Decoder>() && !is_per_decoder::<avn::de::Decoder>());
};

/// The codec of a SEQUENCE that has no extension addition and no default value.
///
/// In PER a SEQUENCE is its preamble, one bit for its extension marker when it has one
/// and one for each member that may be absent, then its members in their order (X.691
/// (02/2021) §19). Each of these is written and read here with the codec of the member,
/// in the encoder and the decoder that the SEQUENCE is given. The sequence codec of
/// rasn makes an encoder or a decoder of another type for each SEQUENCE, which has the
/// number of its members in its type: the codec of every member is then compiled once
/// more for each such number.
///
/// The additions of a later release are skipped where they are read, and none is
/// written. The other encoding rules, and the JSON forms, take the sequence codec of
/// rasn, called as the derived codec calls it: each member as the type that it is
/// declared with, under the context tag of its position. [`is_per_encoder`] says which
/// of the two is compiled for an encoder.
///
/// `$marker` is `extensible` for a SEQUENCE that has an extension marker, and `closed`
/// for one that has none. A member is `name: [Type] {}`. In the braces, `? [Inner]`
/// says that it may be absent, and what it is when it is there, and `!` that its
/// attributes constrain it: its constraint is then that of a type that has these
/// attributes alone.
#[macro_export]
macro_rules! sequence {
    ($name:ident $marker:ident {
        $($(#[$($attribute:tt)*])* $field:ident: [$($field_type:tt)*] {$($kind:tt)*},)*
    }) => {
        impl rasn::Decode for $name {
            // The position of a member is counted where the members are read, and not
            // read after the last one.
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn decode_with_tag_and_constraints<D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::types::Tag,
                _: &rasn::types::Constraints,
            ) -> Result<Self, D::Error> {
                if const { !$crate::per::is_per_decoder::<D>() } {
                    // The other encoding rules: the sequence codec of rasn with each
                    // member in its order, as the derived decoder calls it.
                    const COUNT: usize = <[&str]>::len(&[$(stringify!($field)),*]);
                    let initializer =
                        $crate::per::sequence!(@initializer $name $($field {$($kind)*})*);
                    return rasn::Decoder::decode_sequence::<COUNT, 0, Self, _, _>(
                        decoder,
                        tag,
                        initializer,
                        |decoder| {
                            let mut index = 0;
                            $(
                                let $field = $crate::per::sequence!(
                                    @tagged decoder $name $field index [$($field_type)*]
                                    [$(#[$($attribute)*])*] {$($kind)*}
                                );
                                index += 1;
                            )*
                            Ok(Self { $($field,)* })
                        },
                    );
                }
                let additions = $crate::per::sequence!(@marker decoder $marker);
                // The preamble: whether each member that may be absent is there, under
                // the name of the member.
                $($crate::per::sequence!(@present decoder $field {$($kind)*});)*
                $(let $field = $crate::per::sequence!(
                    @decode decoder $name $field [$($field_type)*]
                    [$(#[$($attribute)*])*] {$($kind)*}
                );)*
                if additions {
                    $crate::per::skip_unknown_extensions(decoder)?;
                }
                Ok(Self { $($field,)* })
            }
        }

        impl rasn::Encode for $name {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn encode_with_tag_and_constraints<'b, E: rasn::Encoder<'b>>(
                &self,
                encoder: &mut E,
                tag: rasn::types::Tag,
                _: &rasn::types::Constraints,
                identifier: rasn::types::Identifier,
            ) -> Result<(), E::Error> {
                if const { !$crate::per::is_per_encoder::<E>() } {
                    // The other encoding rules: the sequence codec of rasn with each
                    // member in its order, as the derived encoder calls it.
                    const COUNT: usize = <[&str]>::len(&[$(stringify!($field)),*]);
                    let fields = <Self as rasn::types::Constructed<COUNT, 0>>::FIELDS;
                    let written = rasn::Encoder::encode_sequence::<COUNT, 0, Self, _>(
                        encoder,
                        tag,
                        |encoder| {
                            let mut index = 0;
                            $(
                                $crate::per::sequence!(
                                    @tagged_write encoder (self.$field) (fields[index].name) index
                                    [$($field_type)*] [$(#[$($attribute)*])*] {$($kind)*}
                                );
                                index += 1;
                            )*
                            Ok(())
                        },
                        identifier,
                    );
                    return written.map(drop);
                }
                $crate::per::sequence!(@write_marker encoder $marker);
                $($crate::per::sequence!(@write_present encoder (self.$field) {$($kind)*});)*
                $($crate::per::sequence!(
                    @encode encoder (self.$field) [$($field_type)*]
                    [$(#[$($attribute)*])*] {$($kind)*}
                );)*
                Ok(())
            }
        }
    };

    // The extension marker of a SEQUENCE that has one: whether additions follow its
    // members. None is written.
    (@marker $decoder:ident extensible) => {
        rasn::Decoder::decode_bool($decoder, rasn::types::Tag::BOOL)?
    };
    (@marker $decoder:ident closed) => {
        false
    };
    (@write_marker $encoder:ident extensible) => {
        let _ = rasn::Encoder::encode_bool(
            $encoder,
            rasn::types::Tag::BOOL,
            false,
            rasn::types::Identifier::EMPTY,
        )?;
    };
    (@write_marker $encoder:ident closed) => {};

    // The bit of the preamble of a member that may be absent.
    (@present $decoder:ident $field:ident {? [$($inner:tt)*] $($constrained:tt)?}) => {
        let $field = rasn::Decoder::decode_bool($decoder, rasn::types::Tag::BOOL)?;
    };
    (@present $decoder:ident $field:ident {$($constrained:tt)?}) => {};
    (@write_present $encoder:ident ($value:expr) {? [$($inner:tt)*] $($constrained:tt)?}) => {
        let _ = rasn::Encoder::encode_bool(
            $encoder,
            rasn::types::Tag::BOOL,
            $value.is_some(),
            rasn::types::Identifier::EMPTY,
        )?;
    };
    (@write_present $encoder:ident ($value:expr) {$($constrained:tt)?}) => {};

    // A member: its value, with the constraint of its attributes when it has one, and
    // an error that names it. `$type` is the type of the value that is read or written,
    // and `$declared` the type that the member is declared with, which is an `Option`
    // of it for a member that may be absent.
    (@decode $decoder:ident $name:ident $field:ident [$($type:tt)*]
        [$($attributes:tt)*] {? [$($inner:tt)*] $($constrained:tt)?}) => {
        match $field {
            true => Some($crate::per::sequence!(
                @value $decoder $name $field [$($inner)*] [$($type)*] [$($attributes)*]
                {$($constrained)?}
            )),
            false => None,
        }
    };
    (@decode $decoder:ident $name:ident $field:ident [$($type:tt)*]
        [$($attributes:tt)*] {$($constrained:tt)?}) => {
        $crate::per::sequence!(
            @value $decoder $name $field [$($type)*] [$($type)*] [$($attributes)*]
            {$($constrained)?}
        )
    };
    (@value $decoder:ident $name:ident $field:ident [$($type:tt)*] [$($declared:tt)*]
        [$($attributes:tt)*] {}) => {
        <$($type)* as rasn::Decode>::decode($decoder).map_err(|error| {
            rasn::de::Error::field_error(
                concat!(stringify!($name), ".", stringify!($field)),
                error.into(),
                rasn::Decoder::codec(&*$decoder),
            )
        })?
    };
    (@value $decoder:ident $name:ident $field:ident [$($type:tt)*] [$($declared:tt)*]
        [$($attributes:tt)*] {!}) => {{
        const CONSTRAINTS: rasn::types::Constraints =
            $crate::per::sequence!(@constraint [$($declared)*] [$($attributes)*]);
        <$($type)* as rasn::Decode>::decode_with_constraints($decoder, &CONSTRAINTS).map_err(
            |error| {
                rasn::de::Error::field_error(
                    concat!(stringify!($name), ".", stringify!($field)),
                    error.into(),
                    rasn::Decoder::codec(&*$decoder),
                )
            },
        )?
    }};

    (@encode $encoder:ident ($value:expr) [$($type:tt)*] [$($attributes:tt)*]
        {? [$($inner:tt)*] $($constrained:tt)?}) => {
        if let Some(value) = &$value {
            $crate::per::sequence!(
                @write $encoder value [$($type)*] [$($attributes)*] {$($constrained)?}
            );
        }
    };
    (@encode $encoder:ident ($value:expr) [$($type:tt)*] [$($attributes:tt)*]
        {$($constrained:tt)?}) => {{
        let value = &$value;
        $crate::per::sequence!(
            @write $encoder value [$($type)*] [$($attributes)*] {$($constrained)?}
        );
    }};
    (@write $encoder:ident $value:ident [$($declared:tt)*] [$($attributes:tt)*] {}) => {
        rasn::Encode::encode($value, $encoder)?
    };
    (@write $encoder:ident $value:ident [$($declared:tt)*] [$($attributes:tt)*] {!}) => {{
        const CONSTRAINTS: rasn::types::Constraints =
            $crate::per::sequence!(@constraint [$($declared)*] [$($attributes)*]);
        rasn::Encode::encode_with_constraints($value, $encoder, &CONSTRAINTS)?
    }};

    // What the sequence codec of rasn makes of a SEQUENCE that has no content: one whose
    // members may all be absent has none of them.
    (@initializer $name:ident $($field:ident {? $($kind:tt)*})*) => {
        Some(|| $name { $($field: None,)* })
    };
    (@initializer $name:ident $($field:ident {$($kind:tt)*})*) => {
        None::<fn() -> $name>
    };

    // A member in the other encoding rules: the type that it is declared with, under the
    // context tag of its position, with the constraint of its attributes when it has one.
    (@tagged $decoder:ident $name:ident $field:ident $index:ident [$($declared:tt)*]
        [$($attributes:tt)*] {$(? [$($inner:tt)*])?}) => {
        <$($declared)* as rasn::Decode>::decode_with_tag(
            $decoder,
            rasn::types::Tag::new(rasn::types::Class::Context, $index),
        )
        .map_err(|error| {
            rasn::de::Error::field_error(
                concat!(stringify!($name), ".", stringify!($field)),
                error.into(),
                rasn::Decoder::codec(&*$decoder),
            )
        })?
    };
    (@tagged $decoder:ident $name:ident $field:ident $index:ident [$($declared:tt)*]
        [$($attributes:tt)*] {$(? [$($inner:tt)*])? !}) => {{
        const CONSTRAINTS: rasn::types::Constraints =
            $crate::per::sequence!(@constraint [$($declared)*] [$($attributes)*]);
        <$($declared)* as rasn::Decode>::decode_with_tag_and_constraints(
            $decoder,
            rasn::types::Tag::new(rasn::types::Class::Context, $index),
            &CONSTRAINTS,
        )
        .map_err(|error| {
            rasn::de::Error::field_error(
                concat!(stringify!($name), ".", stringify!($field)),
                error.into(),
                rasn::Decoder::codec(&*$decoder),
            )
        })?
    }};
    (@tagged_write $encoder:ident ($value:expr) ($identifier:expr) $index:ident
        [$($declared:tt)*] [$($attributes:tt)*] {$(? [$($inner:tt)*])?}) => {
        rasn::Encode::encode_with_tag_and_identifier(
            &$value,
            $encoder,
            rasn::types::Tag::new(rasn::types::Class::Context, $index as u32),
            rasn::types::Identifier(Some($identifier)),
        )?
    };
    (@tagged_write $encoder:ident ($value:expr) ($identifier:expr) $index:ident
        [$($declared:tt)*] [$($attributes:tt)*] {$(? [$($inner:tt)*])? !}) => {{
        const CONSTRAINTS: rasn::types::Constraints =
            $crate::per::sequence!(@constraint [$($declared)*] [$($attributes)*]);
        rasn::Encode::encode_with_tag_and_constraints(
            &$value,
            $encoder,
            rasn::types::Tag::new(rasn::types::Class::Context, $index as u32),
            &CONSTRAINTS,
            rasn::types::Identifier(Some($identifier)),
        )?
    }};

    // The constraint of a member: that of a type that has the attributes of the member
    // and is the type that it is declared with otherwise, as the derived codec has it.
    (@constraint [$($declared:tt)*] [$($attributes:tt)*]) => {{
        #[allow(dead_code)]
        #[derive(rasn::AsnType)]
        #[rasn(delegate)]
        $($attributes)*
        struct Constrained($($declared)*);
        <Constrained as rasn::AsnType>::CONSTRAINTS
    }};
}

pub use sequence;

/// The codec of a `SEQUENCE (SIZE (lower..upper)) OF` newtype.
///
/// rasn's derived encoder can disagree with its decoder about the alignment
/// of the length and of the elements. In APER the length is encoded as a
/// constrained whole number, through rasn's integer codec, and the elements
/// follow it. An `unconstrained` list has an upper bound of 64K or more, so
/// its length is an unconstrained length determinant (X.691 (02/2021)
/// §11.9.3.5-11.9.3.8): one octet up to 127, two up to 16383, and for more a
/// fragment of one to four times 16384 elements, each after its count, until
/// a count that is less than 16384. It is written by rasn's sequence codec
/// without the size, which is checked here, and read here: each octet of a
/// count starts an octet, and is read as a number of eight bits.
///
/// The decoded list grows with the elements read, so that a count alone
/// reserves no memory. rasn's sequence codec reserves for the count that it
/// reads, as far as the octets after it have bits: 3 MiB for a fragment count
/// and 8192 octets that are no element. Every other encoding rule keeps rasn's
/// sequence codec.
#[macro_export]
macro_rules! sequence_of {
    ($name:ident, $lower:literal, $upper:literal) => {
        impl rasn::Encode for $name {
            fn encode_with_tag_and_constraints<'b, E: rasn::Encoder<'b>>(
                &self,
                encoder: &mut E,
                tag: rasn::types::Tag,
                constraints: &rasn::types::Constraints,
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
                    &LENGTH,
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
                constraints: &rasn::types::Constraints,
            ) -> Result<Self, D::Error> {
                if decoder.codec() != rasn::Codec::Aper {
                    return decoder.decode_sequence_of(tag, constraints).map(Self);
                }
                const LENGTH: rasn::types::Constraints =
                    rasn::constraints!(rasn::value_constraint!($lower, $upper));
                let length = decoder.decode_integer::<usize>(rasn::types::Tag::INTEGER, &LENGTH)?;
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
                constraints: &rasn::types::Constraints,
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
                        &rasn::types::Constraints::default(),
                        identifier,
                    )
                    .map(drop)
            }
        }
        impl rasn::Decode for $name {
            fn decode_with_tag_and_constraints<D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::types::Tag,
                constraints: &rasn::types::Constraints,
            ) -> Result<Self, D::Error> {
                if decoder.codec() != rasn::Codec::Aper {
                    return decoder.decode_sequence_of(tag, constraints).map(Self);
                }
                const OCTET: rasn::types::Constraints =
                    rasn::constraints!(rasn::value_constraint!(0, 255));
                let mut values = Vec::new();
                loop {
                    let first = decoder.decode_integer::<u8>(rasn::types::Tag::INTEGER, &OCTET)?;
                    let (count, last) = match first {
                        0..=0x7f => (usize::from(first), true),
                        0x80..=0xbf => {
                            let second =
                                decoder.decode_integer::<u8>(rasn::types::Tag::INTEGER, &OCTET)?;
                            (usize::from(first & 0x3f) << 8 | usize::from(second), true)
                        }
                        0xc1..=0xc4 => (usize::from(first & 0x3f) * 16384, false),
                        _ => {
                            return Err(rasn::de::Error::custom(
                                "a fragment of a list has one to four times 16384 elements",
                                decoder.codec(),
                            ));
                        }
                    };
                    for _ in 0..count {
                        values.push(rasn::Decode::decode(decoder)?);
                    }
                    if last {
                        break;
                    }
                }
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

pub use sequence_of;
