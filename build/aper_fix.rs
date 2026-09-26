use anyhow::{Result, ensure};
use regex::{Captures, Regex};

/// Work around rasn 0.28 APER handling for constrained `SEQUENCE OF` values.
///
/// TS 38.413 uses constrained ranges for protocol IE, extension, and other list
/// containers. The derived encoder can disagree with its decoder about length
/// and element alignment. The generated implementations below use rasn's public
/// constrained-integer codec path for APER and retain the normal sequence codec
/// for every other encoding rule.
pub fn fix_constrained_sequences(generated: &str) -> Result<String> {
    let sequence = Regex::new(
        r#"(?ms)(    #\[derive\(AsnType, Debug, Clone, )Decode, Encode(, PartialEq, Eq, Hash\)\]
    #\[rasn\(delegate, size\("([0-9]+)\.\.=([0-9]+)"\)(?:, identifier = "[^"]+")?\)\]
    pub struct ([A-Za-z0-9_]+)\(\s*pub SequenceOf<([A-Za-z0-9_]+)>,?\s*\);)"#,
    )?;

    let mut replacements = 0usize;
    let generated = sequence
        .replace_all(generated, |captures: &Captures<'_>| {
            replacements += 1;
            let declaration = captures[0].replacen("Decode, Encode, ", "", 1);
            let minimum = &captures[3];
            let maximum = &captures[4];
            let name = &captures[5];
            let element = &captures[6];
            format!(
                r#"{declaration}
    impl Encode for {name} {{
        fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
            &self,
            encoder: &mut E,
            tag: Tag,
            constraints: Constraints,
            identifier: Identifier,
        ) -> Result<(), E::Error> {{
            if encoder.codec() != rasn::Codec::Aper {{
                return encoder
                    .encode_sequence_of(tag, &self.0, constraints, identifier)
                    .map(drop);
            }}
            const LENGTH_CONSTRAINTS: Constraints =
                rasn::constraints!(rasn::value_constraint!({minimum}, {maximum}));
            let _ = encoder.encode_integer(
                Tag::INTEGER,
                LENGTH_CONSTRAINTS,
                &self.0.len(),
                Identifier::EMPTY,
            )?;
            for value in &self.0 {{
                value.encode(encoder)?;
            }}
            Ok(())
        }}
    }}
    impl Decode for {name} {{
        fn decode_with_tag_and_constraints<D: Decoder>(
            decoder: &mut D,
            tag: Tag,
            constraints: Constraints,
        ) -> Result<Self, D::Error> {{
            if decoder.codec() != rasn::Codec::Aper {{
                return decoder
                    .decode_sequence_of::<{element}>(tag, constraints)
                    .map(Self);
            }}
            const LENGTH_CONSTRAINTS: Constraints =
                rasn::constraints!(rasn::value_constraint!({minimum}, {maximum}));
            let length = decoder.decode_integer::<usize>(Tag::INTEGER, LENGTH_CONSTRAINTS)?;
            let mut values = Vec::with_capacity(length);
            for _ in 0..length {{
                values.push({element}::decode(decoder)?);
            }}
            Ok(Self(values))
        }}
    }}"#
            )
        })
        .into_owned();

    ensure!(
        replacements > 0,
        "no constrained SEQUENCE OF declarations found"
    );
    Ok(generated)
}
