//! APER codecs that the bindings of the oracle use in place of rasn 0.29's.

use rasn::prelude::*;

/// X.691 (02/2021) §19.8-19.9: the extension presence bitmap is followed
/// by one open type for each present addition. Small bitmaps are unaligned;
/// larger ones use an unconstrained length, including fragmentation
/// (§11.9.3.4). The additions are unknown to these generated SEQUENCE types.
pub fn skip_unknown_extensions<D: Decoder>(decoder: &mut D) -> Result<(), D::Error> {
    let present = if decoder.decode_bool(Tag::BOOL)? {
        decoder
            .decode_bit_string(Tag::BIT_STRING, &Constraints::default())?
            .count_ones()
    } else {
        const LENGTH: Constraints = rasn::constraints!(rasn::value_constraint!(0, 63));
        let count = decoder.decode_integer::<usize>(Tag::INTEGER, &LENGTH)? + 1;
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

pub(crate) use sequence;

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

pub(crate) use sequence_of;

#[cfg(test)]
mod tests {
    //! The same types twice, with the derived codec of rasn and with the codec of
    //! `sequence!`: each encoding rule writes the same for both, and each reads the
    //! same from the same octets.

    /// The types of `derived`, with the codec of `sequence!`.
    mod direct {
        use rasn::prelude::*;

        #[derive(AsnType, Debug, Clone, PartialEq, Eq)]
        #[rasn(automatic_tags)]
        #[non_exhaustive]
        pub struct Inner {
            pub flag: bool,
            #[rasn(value("0..=4095"))]
            pub number: Option<u16>,
        }
        crate::per::sequence! { Inner extensible {
            flag: [bool] {},
            #[rasn(value("0..=4095"))]
            number: [Option<u16>] {? [u16] !},
        } }

        #[derive(AsnType, Debug, Clone, PartialEq, Eq)]
        #[rasn(automatic_tags)]
        pub struct Closed {
            #[rasn(size("3"))]
            pub octets: OctetString,
            pub inner: Option<Inner>,
        }
        crate::per::sequence! { Closed closed {
            #[rasn(size("3"))]
            octets: [OctetString] {!},
            inner: [Option<Inner>] {? [Inner]},
        } }

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq)]
        #[rasn(choice, automatic_tags)]
        pub enum Either {
            Inner(Inner),
            Closed(Closed),
        }

        #[derive(AsnType, Debug, Clone, PartialEq, Eq)]
        #[rasn(automatic_tags)]
        #[non_exhaustive]
        pub struct Outer {
            #[rasn(identifier = "first-member")]
            pub first: Closed,
            #[rasn(size("0..=4"))]
            pub list: SequenceOf<Inner>,
            pub either: Option<Either>,
            #[rasn(size("1..=150"), identifier = "a-name")]
            pub name: Option<PrintableString>,
            pub last: u8,
        }
        crate::per::sequence! { Outer extensible {
            #[rasn(identifier = "first-member")]
            first: [Closed] {},
            #[rasn(size("0..=4"))]
            list: [SequenceOf<Inner>] {!},
            either: [Option<Either>] {? [Either]},
            #[rasn(size("1..=150"), identifier = "a-name")]
            name: [Option<PrintableString>] {? [PrintableString] !},
            last: [u8] {},
        } }
    }

    /// The types of `direct`, with the derived codec of rasn.
    mod derived {
        use rasn::prelude::*;

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq)]
        #[rasn(automatic_tags)]
        #[non_exhaustive]
        pub struct Inner {
            pub flag: bool,
            #[rasn(value("0..=4095"))]
            pub number: Option<u16>,
        }

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq)]
        #[rasn(automatic_tags)]
        pub struct Closed {
            #[rasn(size("3"))]
            pub octets: OctetString,
            pub inner: Option<Inner>,
        }

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq)]
        #[rasn(choice, automatic_tags)]
        pub enum Either {
            Inner(Inner),
            Closed(Closed),
        }

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq)]
        #[rasn(automatic_tags)]
        #[non_exhaustive]
        pub struct Outer {
            #[rasn(identifier = "first-member")]
            pub first: Closed,
            #[rasn(size("0..=4"))]
            pub list: SequenceOf<Inner>,
            pub either: Option<Either>,
            #[rasn(size("1..=150"), identifier = "a-name")]
            pub name: Option<PrintableString>,
            pub last: u8,
        }
    }

    /// Values of the types of a module: members that are there and that are not, at
    /// each depth, and a number and a size outside their constraints.
    macro_rules! samples {
        ($types:ident) => {{
            use rasn::prelude::*;
            use $types::{Closed, Either, Inner, Outer};
            let inner = |flag, number| Inner { flag, number };
            let closed = |octets: [u8; 3], inner| Closed {
                octets: OctetString::from(octets.to_vec()),
                inner,
            };
            let name = |name: &str| PrintableString::try_from(name).unwrap();
            vec![
                Outer {
                    first: closed([1, 2, 3], None),
                    list: vec![],
                    either: None,
                    name: None,
                    last: 0,
                },
                Outer {
                    first: closed([0xff, 0, 0x80], Some(inner(true, Some(4095)))),
                    list: vec![
                        inner(false, None),
                        inner(true, Some(0)),
                        inner(true, Some(7)),
                    ],
                    either: Some(Either::Inner(inner(true, Some(1)))),
                    name: Some(name("a name of a few letters")),
                    last: 255,
                },
                Outer {
                    first: closed([0, 0, 0], Some(inner(false, None))),
                    list: vec![inner(true, None); 4],
                    either: Some(Either::Closed(closed(
                        [9, 8, 7],
                        Some(inner(false, Some(256))),
                    ))),
                    name: Some(name(&"x".repeat(150))),
                    last: 1,
                },
                // A number outside its constraint, and more entries than a list has.
                Outer {
                    first: closed([1, 2, 3], Some(inner(true, Some(4096)))),
                    list: vec![],
                    either: None,
                    name: None,
                    last: 0,
                },
                Outer {
                    first: closed([1, 2, 3], None),
                    list: vec![inner(true, None); 5],
                    either: None,
                    name: None,
                    last: 0,
                },
            ]
        }};
    }

    #[test]
    fn a_sequence_is_written_as_the_derived_codec_writes_it() {
        let (direct, derived) = (samples!(direct), samples!(derived));
        let mut written = 0;
        for (direct, derived) in direct.iter().zip(&derived) {
            macro_rules! same {
                ($($codec:ident),*) => {$(
                    let ours = rasn::$codec::encode(direct).map_err(|error| error.to_string());
                    let theirs = rasn::$codec::encode(derived).map_err(|error| error.to_string());
                    assert_eq!(ours, theirs, "{} of {derived:?}", stringify!($codec));
                    written += usize::from(ours.is_ok());
                )*};
            }
            same!(aper, uper, ber, der, cer, oer, coer, jer, xer);
        }
        // Three of the values encode, and two are outside their constraints.
        assert!(written >= 3 * 9, "{written} encodings");
    }

    #[test]
    fn a_sequence_is_read_as_the_derived_codec_reads_it() {
        let values = samples!(derived);
        let mut read = 0;
        for value in &values {
            macro_rules! same {
                ($($codec:ident),*) => {$(
                    if let Ok(octets) = rasn::$codec::encode(value) {
                        // The octets, and each of their truncations.
                        for length in 0..=octets.len() {
                            let octets = &octets[..length];
                            let ours = rasn::$codec::decode::<direct::Outer>(octets);
                            let theirs = rasn::$codec::decode::<derived::Outer>(octets);
                            assert_eq!(
                                ours.as_ref().map(|value| format!("{value:?}")).ok(),
                                theirs.as_ref().map(|value| format!("{value:?}")).ok(),
                                "{} of {octets:02x?}",
                                stringify!($codec)
                            );
                            read += usize::from(ours.is_ok());
                        }
                    }
                )*};
            }
            same!(aper, uper, ber, der, cer, oer, coer, jer, xer);
        }
        assert!(read >= 3 * 9, "{read} values read");
    }

    /// A list of 1 to 65536 elements, with the codec of `sequence_of!`.
    #[derive(rasn::AsnType, Debug, PartialEq)]
    #[rasn(delegate)]
    struct Flags(Vec<bool>);
    crate::per::sequence_of! { Flags, 1, 65536, unconstrained }

    #[test]
    fn a_list_in_fragments_is_read_as_the_sequence_codec_of_rasn_reads_it() {
        // A count of each form: what is written for it, then the same cut at each of
        // its counts and with another octet in the place of each.
        let mut read = 0;
        for count in [1, 127, 128, 16383, 16384, 16385, 49153, 65536] {
            let flags: Vec<bool> = (0..count).map(|at| at % 3 == 0).collect();
            let written = rasn::aper::encode(&flags).unwrap();
            assert_eq!(rasn::aper::encode(&Flags(flags)).unwrap(), written);
            // The first count, and the count after a fragment of 16384 flags or more.
            let fragment = usize::from(written[0] & 0x3f) * 2048;
            let after = (written[0] >= 0xc0).then_some(1 + fragment);
            let counts = [0, 1].into_iter().chain(after).chain([written.len() - 1]);
            let mut inputs = vec![written.clone()];
            for at in counts {
                inputs.push(written[..at].to_vec());
                for other in [0x00, 0x7f, 0x80, 0xc0, 0xc1, 0xc5] {
                    let mut changed = written.clone();
                    changed[at] = other;
                    inputs.push(changed);
                }
            }
            for octets in inputs {
                let ours = rasn::aper::decode::<Flags>(&octets).ok();
                let theirs = rasn::aper::decode::<Vec<bool>>(&octets).ok();
                // The size of the list is checked here alone.
                let theirs = theirs.filter(|flags| (1..=65536).contains(&flags.len()));
                assert_eq!(ours.map(|flags| flags.0), theirs, "{} octets", octets.len());
                read += usize::from(theirs.is_some());
            }
        }
        assert!(read > 50, "{read} lists read");
    }

    #[test]
    fn the_additions_of_a_later_release_are_skipped_in_both_variants_of_per() {
        // `Inner` with its extension bit set, no number and its flag, then one addition:
        // a count of one in seven bits, the bit of its presence, and an open type of one
        // octet, which starts an octet in the aligned variant.
        let aligned = [0b1010_0000, 0b0010_0000, 0x01, 0xaa];
        let read = rasn::aper::decode::<direct::Inner>(&aligned).unwrap();
        assert_eq!(
            read,
            direct::Inner {
                flag: true,
                number: None
            }
        );
        let unaligned = [0b1010_0000, 0b0010_0000, 0b0011_0101, 0b0100_0000];
        let read = rasn::uper::decode::<direct::Inner>(&unaligned).unwrap();
        assert_eq!(
            read,
            direct::Inner {
                flag: true,
                number: None
            }
        );
    }
}
