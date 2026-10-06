use std::fmt::Write;

use anyhow::{Result, bail, ensure};
use regex::{Captures, Regex};

/// Give the constrained `SEQUENCE OF` newtypes the codec of
/// `crate::per::sequence_of!` in place of rasn's derived one.
pub fn fix_constrained_sequences(generated: &str) -> Result<String> {
    // rustfmt splits long attributes and SEQUENCE OF declarations over lines.
    let sequence = Regex::new(
        r#"(?ms)(    #\[derive\(AsnType, Debug, Clone, )Decode, Encode(, PartialEq, Eq, Hash\)\]
    #\[rasn\(\s*delegate,\s*size\("([0-9]+)\.\.=([0-9]+)"\)(?:,\s*identifier = "[^"]+")?,?\s*\)\]
    pub struct ([A-Za-z0-9_]+)\(\s*pub\s+SequenceOf<\s*([A-Za-z0-9_]+),?\s*>,?\s*\);)"#,
    )?;

    let mut replacements = 0usize;
    let generated = sequence
        .replace_all(generated, |captures: &Captures<'_>| {
            replacements += 1;
            let declaration = captures[0].replacen("Decode, Encode, ", "", 1);
            let (minimum, maximum, name) = (&captures[3], &captures[4], &captures[5]);
            // From 64K the length determinant is unconstrained.
            let long = maximum.parse::<u64>().is_ok_and(|maximum| maximum >= 65536);
            let unconstrained = if long { ", unconstrained" } else { "" };
            format!(
                "{declaration}\n    crate::per::sequence_of! {{ {name}, {minimum}, {maximum}{unconstrained} }}"
            )
        })
        .into_owned();

    ensure!(
        replacements > 0,
        "no constrained SEQUENCE OF declarations found"
    );
    let unpatched = Regex::new(
        r#"(?ms)#\[derive\([^\]]*\bDecode\b[^\]]*\)\]\s*#\[rasn\([^\]]*size\("[0-9]+\.\.=[0-9]+"\)[^\]]*\)\]\s*pub struct ([A-Za-z0-9_]+)[^;]*\bSequenceOf\b"#,
    )?;
    ensure!(
        !unpatched.is_match(&generated),
        "a constrained SEQUENCE OF declaration still uses the derived decoder"
    );
    Ok(generated)
}

/// Decode unknown extension additions before returning an extensible SEQUENCE.
///
/// rasn 0.28 leaves them unread when the type has no known additions. All
/// current NGAP SEQUENCE additions are empty; the protocol's named extension
/// containers remain ordinary root fields. Preserve their root field tokens
/// and constraints in the decoder macro, and fail if a future ASN.1 version
/// defines additions that need a different decoder.
pub fn fix_extensible_sequences(generated: &str) -> Result<String> {
    let sequence = Regex::new(
        r#"(?ms)(    #\[derive\(AsnType, Debug, Clone, )Decode, (Encode, PartialEq, Eq, Hash\)\]
    #\[rasn\(\s*automatic_tags(?:,\s*identifier = "([^"]+)")?,?\s*\)\]
    #\[non_exhaustive\]
    pub struct ([A-Za-z0-9_]+) \{
(.*?)^    \})"#,
    )?;
    let field = Regex::new(r"(?m)^        pub ([a-z0-9_]+):\s*")?;
    let mut replacements = 0usize;
    let mut failure = None;
    let generated = sequence
        .replace_all(generated, |captures: &Captures<'_>| {
            let name = &captures[4];
            let identifier = captures.get(3).map_or(name, |value| value.as_str());
            let fields = &captures[5];
            if fields.contains("extension_addition") {
                failure.get_or_insert_with(|| format!("defined SEQUENCE additions in {name}"));
                return captures[0].to_string();
            }
            let mut arguments = String::new();
            let mut rest = 0;
            for member in field.captures_iter(fields) {
                let whole = member.get(0).expect("field declaration");
                let start = whole.end();
                let mut depth = 0usize;
                let end = fields[start..].char_indices().find_map(|(offset, character)| {
                    match character {
                        '<' | '[' | '(' => depth += 1,
                        '>' | ']' | ')' => depth -= 1,
                        ',' if depth == 0 => return Some(start + offset),
                        _ => (),
                    }
                    None
                });
                let Some(end) = end else {
                    failure.get_or_insert_with(|| format!("unterminated field in {name}"));
                    return captures[0].to_string();
                };
                arguments.push_str(&fields[rest..whole.start()]);
                // Raw token groups preserve Option<T> for rasn's derives;
                // forwarding a macro `ty` would hide it behind a Type::Group.
                write!(arguments, "        {}: [{}],", &member[1], &fields[start..end])
                    .expect("write to String");
                rest = end + 1;
            }
            arguments.push_str(&fields[rest..]);
            if arguments.contains("pub ") {
                failure.get_or_insert_with(|| format!("unrecognized root field in {name}"));
                return captures[0].to_string();
            }
            replacements += 1;
            let declaration = captures[0].replacen("Decode, Encode, ", "Encode, ", 1);
            format!(
                "{declaration}\n    crate::per::decode_extensible_sequence! {{ {name}, \"{identifier}\" {{\n{arguments}    }} }}"
            )
        })
        .into_owned();
    if let Some(failure) = failure {
        bail!("extensible SEQUENCE: {failure}");
    }
    ensure!(replacements > 0, "no extensible SEQUENCE declarations found");
    let unpatched = Regex::new(
        r"(?ms)#\[derive\([^\]]*\bDecode\b[^\]]*\)\]\s*#\[rasn\([^\]]*\)\]\s*#\[non_exhaustive\]\s*pub struct",
    )?;
    ensure!(
        !unpatched.is_match(&generated),
        "an extensible SEQUENCE declaration still uses the derived decoder"
    );
    Ok(generated)
}

/// Work around rasn 0.28 APER decoding of size-constrained `UTF8String` values.
///
/// A constraint on a character string type that is not a known-multiplier
/// type is not PER-visible (X.691 §9.3.6): the value is an unconstrained
/// length in octets followed by its UTF-8 octets (§27.6, 07/2002 numbering).
/// rasn encodes it that way but applies the size constraint when decoding.
pub fn fix_utf8_strings(generated: &str) -> Result<String> {
    let string = Regex::new(
        r#"(?m)^(    #\[derive\(AsnType, Debug, Clone, )Decode, Encode(, PartialEq, Eq, Hash\)\]
    #\[rasn\(delegate, size\("[^"]+"(?:, extensible)?\)\)\]
    pub struct ([A-Za-z0-9_]+)\(pub Utf8String\);)$"#,
    )?;

    let mut replacements = 0usize;
    let generated = string
        .replace_all(generated, |captures: &Captures<'_>| {
            replacements += 1;
            let declaration = captures[0].replacen("Decode, Encode, ", "Encode, ", 1);
            let name = &captures[3];
            format!(
                r#"{declaration}
    impl Decode for {name} {{
        fn decode_with_tag_and_constraints<D: Decoder>(
            decoder: &mut D,
            tag: Tag,
            constraints: Constraints,
        ) -> Result<Self, D::Error> {{
            if decoder.codec() != rasn::Codec::Aper {{
                return Utf8String::decode_with_tag_and_constraints(decoder, tag, constraints)
                    .map(Self);
            }}
            decoder
                .decode_utf8_string(tag, Constraints::default())
                .map(Self)
        }}
    }}"#
            )
        })
        .into_owned();

    ensure!(
        replacements > 0 || !generated.contains("(pub Utf8String);"),
        "no size-constrained UTF8String declarations found"
    );
    Ok(generated)
}

/// Work around rasn 0.28 APER decoding of fixed-size `BIT STRING` values
/// longer than 16 bits.
///
/// Such a bit string is octet-aligned in APER, with no length determinant
/// (X.691 (07/2002) §15.10). rasn encodes it that way but decodes it from the
/// current bit offset. The generated decoders read the first octet as an
/// octet-aligned integer (0..255) and the remaining bits as a short bit string
/// that is then octet-aligned; every other encoding rule keeps rasn's codec.
/// This covers the newtypes over `FixedBitString` and the CHOICE alternatives
/// that are such bit strings, including those with an extensible size.
pub fn fix_fixed_bit_strings(generated: &str) -> Result<String> {
    let newtype = Regex::new(
        r#"(?m)^(    #\[derive\(AsnType, Debug, Clone, )Decode, Encode(, PartialEq, Eq, Hash\)\]
    #\[rasn\(delegate(?:, identifier = "[^"]+")?\)\]
    pub struct ([A-Za-z0-9_]+)\(pub FixedBitString<([0-9]+)usize>\);)$"#,
    )?;
    let mut replacements = 0usize;
    let generated = newtype
        .replace_all(generated, |captures: &Captures<'_>| {
            let length: usize = captures[4].parse().expect("bit string length");
            if length <= 16 {
                return captures[0].to_string();
            }
            replacements += 1;
            let declaration = captures[0].replacen("Decode, Encode, ", "Encode, ", 1);
            let name = &captures[3];
            let bits = aligned_bits(length, "            ");
            format!(
                r#"{declaration}
    impl Decode for {name} {{
        fn decode_with_tag_and_constraints<D: Decoder>(
            decoder: &mut D,
            tag: Tag,
            constraints: Constraints,
        ) -> Result<Self, D::Error> {{
            if decoder.codec() != rasn::Codec::Aper {{
                return FixedBitString::<{length}usize>::decode_with_tag_and_constraints(
                    decoder,
                    tag,
                    constraints,
                )
                .map(Self);
            }}
{bits}
            let mut value = FixedBitString::<{length}usize>::ZERO;
            value[..{length}].copy_from_bitslice(&bits);
            Ok(Self(value))
        }}
    }}"#
            )
        })
        .into_owned();
    ensure!(
        replacements > 0,
        "no fixed-size BIT STRING newtypes longer than 16 bits found"
    );

    let choice = Regex::new(
        r#"(?ms)^(    #\[derive\(AsnType, Debug, Clone, )Decode, Encode(, PartialEq, Eq, Hash\)\]
    #\[rasn\(choice, automatic_tags(?:, identifier = "[^"]+")?\)\]
(?:    #\[non_exhaustive\]
)?    pub enum ([A-Za-z0-9_]+) \{
(.*?)^    \}
)"#,
    )?;
    let variant = Regex::new(
        r#"(?m)^        (?:#\[rasn\(([^\n]*)\)\]\n        )?([A-Za-z0-9_]+)\(([A-Za-z0-9_]+)\),$"#,
    )?;
    // Extension alternatives are open types, which start octet-aligned: the
    // same decoders read them.
    let fixed_size = Regex::new(
        r#"^(?:extension_addition, )?size\("([0-9]+)"(, extensible)?\), identifier = "[^"]+"$"#,
    )?;
    let identifier_only = Regex::new(r#"^(?:extension_addition, )?identifier = "[^"]+"$"#)?;
    let variant_line = Regex::new(r"(?m)^        [A-Za-z0-9_]+\(")?;
    let mut failure = None;
    let mut choices = 0usize;
    let generated = choice
        .replace_all(&generated, |captures: &Captures<'_>| {
            let name = &captures[3];
            let alternatives: Vec<_> = variant.captures_iter(&captures[4]).collect();
            let length = |attributes: &str, variant_type: &str| {
                fixed_size
                    .captures(attributes)
                    .and_then(|size| Some((size[1].parse::<usize>().ok()?, size.get(2).is_some())))
                    .filter(|(length, _)| *length > 16 && variant_type == "BitString")
            };
            if !alternatives.iter().any(|alternative| {
                length(alternative.get(1).map_or("", |value| value.as_str()), &alternative[3])
                    .is_some()
            }) {
                return captures[0].to_string();
            }
            // Automatic tags number the alternatives, so every one must be read.
            if alternatives.len() != variant_line.find_iter(&captures[4]).count() {
                failure.get_or_insert_with(|| format!("unrecognized alternative in {name}"));
            }
            let mut decoders = String::new();
            for (index, alternative) in alternatives.iter().enumerate() {
                let attributes = alternative.get(1).map_or("", |value| value.as_str());
                let variant_name = &alternative[2];
                let variant_type = &alternative[3];
                if let Some((length, extensible)) = length(attributes, variant_type) {
                    let bits = aligned_bits(length, "                ");
                    // Outside an extensible root, a semi-constrained length
                    // precedes the bits (X.691 (07/2002) §15.6).
                    let (size, extension) = if extensible {
                        (
                            format!("{length}, extensible"),
                            format!(
                                r#"
                if decoder.decode_bool(Tag::BOOL)? {{
                    return decoder
                        .decode_bit_string(Tag::BIT_STRING, Constraints::default())
                        .map(Self::{variant_name});
                }}"#
                            ),
                        )
                    } else {
                        (length.to_string(), String::new())
                    };
                    write!(
                        decoders,
                        r#"
            if tag == Tag::new(Class::Context, {index}) {{
                const SIZE: Constraints = rasn::constraints!(rasn::size_constraint!({size}));
                if decoder.codec() != rasn::Codec::Aper {{
                    return BitString::decode_with_tag_and_constraints(decoder, tag, SIZE)
                        .map(Self::{variant_name});
                }}{extension}
{bits}
                return Ok(Self::{variant_name}(bits));
            }}"#
                    )
                    .expect("write to String");
                } else if attributes.is_empty() || identifier_only.is_match(attributes) {
                    write!(
                        decoders,
                        r#"
            if tag == Tag::new(Class::Context, {index}) {{
                return {variant_type}::decode_with_tag(decoder, tag).map(Self::{variant_name});
            }}"#
                    )
                    .expect("write to String");
                } else {
                    failure.get_or_insert_with(|| {
                        format!("unsupported alternative {name}::{variant_name}")
                    });
                }
            }
            choices += 1;
            let declaration = captures[0].replacen("Decode, Encode, ", "Encode, ", 1);
            format!(
                r#"{declaration}    impl rasn::types::DecodeChoice for {name} {{
        fn from_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {{{decoders}
            Err(rasn::de::Error::no_valid_choice("{name}", decoder.codec()))
        }}
    }}
    impl Decode for {name} {{
        fn decode_with_tag_and_constraints<D: Decoder>(
            decoder: &mut D,
            tag: Tag,
            _: Constraints,
        ) -> Result<Self, D::Error> {{
            decoder.decode_explicit_prefix(tag)
        }}
        fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {{
            decoder.decode_choice(Self::CONSTRAINTS)
        }}
    }}
"#
            )
        })
        .into_owned();
    if let Some(failure) = failure {
        bail!("fixed-size BIT STRING CHOICE: {failure}");
    }
    ensure!(
        choices > 0,
        "no CHOICE with a fixed-size BIT STRING longer than 16 bits found"
    );
    Ok(generated)
}

/// Work around rasn 0.28 APER encoding of inline `OCTET STRING` and
/// `BIT STRING` values whose size range is 256 or more with an upper bound
/// below 64K.
///
/// Their length determinant is one or two octet-aligned octets (X.691
/// (02/2021) §11.5.7.2, §11.5.7.3), which rasn writes unaligned. A named type
/// always starts on an octet boundary where it is used, but a SEQUENCE
/// component or CHOICE alternative need not, so those take the
/// `crate::sized` types, which encode the length as rasn encodes a
/// constrained integer: aligned. The generated `new` constructors take the
/// same types.
pub fn fix_long_inline_strings(generated: &str) -> Result<String> {
    let field = Regex::new(
        r#"(?m)^        #\[rasn\(size\("([0-9]+)\.\.=([0-9]+)"(, extensible)?\)(?:, (identifier = "[^"]+"))?\)\]
        (pub [a-z0-9_]+: |[A-Za-z0-9_]+\()(Option<)?(OctetString|BitString)(>?)([,)])"#,
    )?;
    let mut output = String::with_capacity(generated.len());
    let mut rest = 0usize;
    let mut replacements = 0usize;
    for captures in field.captures_iter(generated) {
        let whole = captures.get(0).expect("whole match");
        let lower: usize = captures[1].parse()?;
        let upper: usize = captures[2].parse()?;
        let extensible = captures.get(3).is_some();
        let sized = match (&captures[7], extensible) {
            _ if upper - lower < 255 || upper >= 65536 => continue,
            ("OctetString", false) => format!("crate::sized::SizedOctetString<{lower}, {upper}>"),
            ("BitString", _) => format!("crate::sized::SizedBitString<{lower}, {upper}, {extensible}>"),
            _ => bail!("no sized type for an extensible OCTET STRING in `{}`", whole.as_str()),
        };
        replacements += 1;
        let attribute = captures
            .get(4)
            .map(|identifier| format!("        #[rasn({})]\n", identifier.as_str()))
            .unwrap_or_default();
        let option = captures.get(6).map_or("", |option| option.as_str());
        let declaration = &captures[5];
        output.push_str(&generated[rest..whole.start()]);
        write!(
            output,
            "{attribute}        {declaration}{option}{sized}{}{}",
            &captures[8], &captures[9]
        )
        .expect("write to String");
        rest = whole.end();
        // The struct's own `new` follows it and takes the same type.
        if let Some(name) = declaration
            .strip_prefix("pub ")
            .and_then(|name| name.strip_suffix(": "))
        {
            let plain = format!("            {name}: {option}{}{},", &captures[7], &captures[8]);
            let constructor = generated[rest..]
                .find("        pub fn new(")
                .map(|offset| rest + offset)
                .ok_or_else(|| anyhow::anyhow!("no constructor after field `{name}`"))?;
            let parameter = generated[constructor..]
                .find(&plain)
                .map(|offset| constructor + offset)
                .ok_or_else(|| anyhow::anyhow!("no constructor parameter `{name}`"))?;
            output.push_str(&generated[rest..parameter]);
            write!(output, "            {name}: {option}{sized}{},", &captures[8])
                .expect("write to String");
            rest = parameter + plain.len();
        }
    }
    output.push_str(&generated[rest..]);
    ensure!(
        replacements > 0,
        "no inline OCTET STRING or BIT STRING with a two-octet length found"
    );
    Ok(output)
}

/// Statements that decode an octet-aligned bit string of `length` bits into
/// `bits`, a `BitString`.
fn aligned_bits(length: usize, indent: &str) -> String {
    let rest = length - 8;
    [
        "const OCTET: Constraints = rasn::constraints!(rasn::value_constraint!(0, 255));".to_string(),
        format!("const REST: Constraints = rasn::constraints!(rasn::size_constraint!({rest}));"),
        "let first = decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?;".to_string(),
        "let mut bits = BitString::from_element(first);".to_string(),
        "bits.extend_from_bitslice(&decoder.decode_bit_string(Tag::BIT_STRING, REST)?);".to_string(),
    ]
    .map(|line| format!("{indent}{line}"))
    .join("\n")
}
