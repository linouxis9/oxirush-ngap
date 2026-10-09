//! Decoded PDUs as JSON trees that can be edited and encoded again.
//!
//! [`inspect_pdu`] returns `procedure_code`, `direction`, `criticality` and `message`: the
//! message in the ASN.1 JSON encoding (JER). An IE whose identifier has one type has its
//! typed `value` (`extensionValue` in an extension container) in place of its octets. A
//! contained transfer, and the value of an IE declared `OCTET STRING (CONTAINING ...)`, is
//! its `decoded` value beside its `_raw_transfer`. Repeated IEs stay ordered array entries.
//!
//! The values of some well-known types are shown as they are usually written:
//!
//! - a PLMN identity as its MCC and MNC, `"208-93"`;
//! - a transport layer address as an IP address, `"10.0.0.1"` or `"2001:db8::1"`, or as
//!   the two with a comma when it holds both;
//! - an IMSI as its digits;
//! - an identifier or a counter, such as a TAC, a GTP-TEID, a TMSI, a cell identity, the
//!   parts of a GUAMI or an SST, as a number.
//!
//! [`encode_pdu`] takes them in this form, a number also as a `"0x…"` string, and as JER
//! writes them. It takes the name of an ENUMERATED value whatever its case, with `-`, `_`
//! and space taken as the same.
//!
//! The members that start with `_` say what was received:
//!
//! - `_raw_value`, `_raw_transfer` and `_raw_message`: the octets, in hexadecimal;
//! - `_original_id` and `_ie_name`: the identifier that the octets were decoded as, and the
//!   name that ASN.1 gives it. An entry whose `id` was changed is encoded as the type of
//!   its new identifier;
//! - `_decode_error`: why an IE's `value` is still its octets in hexadecimal, or why a
//!   transfer has no `decoded` member. The identifier is unknown or has several types, the
//!   octets do not decode, JER cannot represent the value, or the IE is nested deeper
//!   than 64 levels;
//! - `_edited`: [`set`], [`remove`] or [`insert`] changed something under the value, whose
//!   octets are no longer those beside it.
//!
//! [`encode_pdu`] encodes the tree. What was not edited keeps the octets received. To edit:
//!
//! - change a typed `value` or the `decoded` member of a transfer, which is encoded again.
//!   This is refused when the octets received do not decode and encode back to themselves,
//!   as with an extension addition that the typed value does not keep;
//! - to add an IE, write an entry with its `id`, its `criticality` and its `value`, and no
//!   `_raw_value`: the value is encoded as the type of the identifier, whatever JSON it is;
//! - to send given octets as an IE, write them in hexadecimal as `_raw_value` and leave
//!   `value` out. For a transfer, replace the member with its octets;
//! - to add a transfer, or the value of an IE that contains a type, write it as an object
//!   with its `decoded` value alone, which is encoded as the type contained;
//! - an IE with a `_decode_error` has its octets as `value`: change them there.
//!
//! Beside a `value`, `_raw_value` is what the value is compared with, as `_raw_message` is
//! for the message, so changing one alone sends nothing else. A member that the ASN.1 type
//! does not have is refused, and so is one beside the `decoded` value of a transfer.
//! Hexadecimal is taken in either case.
//!
//! # Paths
//!
//! [`paths`] gives each value of a tree with the path that selects it, [`select`] the
//! values at a path, and [`set`], [`remove`] and [`insert`] edit a tree at a path:
//!
//! ```text
//! /ngap/AMF-UE-NGAP-ID/criticality = "reject"
//! /ngap/AMF-UE-NGAP-ID/value = 1
//! /ngap/Cause/value/radioNetwork = "user-inactivity"
//! /procedure_code = 42
//! ```
//!
//! A path is a JSON pointer into the tree, where `/ngap` stands for the IEs of the
//! message. The segment after it selects among them:
//!
//! - the name of an IE, as ASN.1 has it after `id-` and in any case:
//!   `/ngap/RAN-UE-NGAP-ID`. An IE that is there twice is selected twice;
//! - a position, `/ngap/0`, which is how [`paths`] names an IE that has no name or is
//!   there twice;
//! - `@id=N`, the IEs with that identifier, whether it has a name or not, and `*`, each
//!   of them;
//! - `-`, the end of the list, where [`insert`] adds an IE.
//!
//! Under an IE are its `criticality`, its typed `value`, and its `octets`: what it was
//! received as, in hexadecimal. The IEs of a message that the tree contains, as the
//! `decoded` value of a transfer, are selected the same way in its place:
//!
//! ```text
//! /ngap/PDUSessionResourceSetupListSUReq/value/0/pDUSessionResourceSetupRequestTransfer/decoded/PDUSessionType/value = "ipv4"
//! ```
//!
//! Any other list selects by position, `*` and `@id=N`. An IE that the message does not
//! have selects nothing. A member that a value does not have is an error, whether its
//! type has none of that name or the value has it absent, so a member that is written
//! wrong is not taken for one that is not there. The name of an IE is taken in any case,
//! with `-`, `_` and space as the same, and the members of a value as ASN.1 spells them.
//!
//! [`paths`] lists the values of a tree without the members that start with `_`, except
//! `_decode_error`: the octets of an IE or of a transfer are selected as its `octets`,
//! and set there in place of its value. Those of a value that an edit changed are an
//! error to select: [`encode_pdu`] gives the PDU its octets. `null` takes an optional
//! member out; the value of an IE or of a transfer is not taken out, as its octets would
//! be sent in its place.
//!
//! ```
//! use oxirush_ngap::{build_ngap, inspect, ngap::*};
//! use serde_json::json;
//!
//! let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
//!     IGNORE, UEContextReleaseRequest,
//!     REJECT AMF_UE_NGAP_ID(1u64),
//!     REJECT RAN_UE_NGAP_ID(7u32),
//!     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
//! );
//! let mut tree = inspect::inspect_pdu(&pdu)?;
//! assert_eq!(inspect::select(&tree, "/ngap/RAN-UE-NGAP-ID/value")?, [&json!(7)]);
//!
//! inspect::set(&mut tree, "/ngap/Cause/value", json!({"nas": "normal-release"}))?;
//! inspect::remove(&mut tree, "/ngap/AMF-UE-NGAP-ID")?;
//! let name = json!({"id": "RANNodeName", "criticality": "ignore", "value": "gnb-1"});
//! inspect::insert(&mut tree, "/ngap/-", name)?;
//! let edited = inspect::inspect_pdu(&inspect::encode_pdu(&tree)?)?;
//! assert_eq!(inspect::select(&edited, "/ngap/*/id")?, [&json!(85), &json!(15), &json!(82)]);
//! # Ok::<(), String>(())
//! ```

use serde_json::{Value, json};

use crate::inspect_paths::EDITED;
pub use crate::inspect_paths::{insert, paths, remove, select, set};
use crate::inspect_registry as registry;
use crate::ngap::NGAP_PDU;

/// ASN.1-derived IE identifiers and names, including extension IEs.
pub fn ie_names() -> &'static [(u16, &'static str)] {
    registry::IE_NAMES
}

/// The messages, each as the `direction` and the `procedure_code` that a tree has for it
/// and the name that ASN.1 gives it.
pub fn message_names() -> impl Iterator<Item = (&'static str, u8, &'static str)> {
    let messages = registry::MESSAGES.iter();
    messages.map(|(direction, code, name, ..)| (*direction, *code, *name))
}

/// The letters and the digits of a name, in lower case: what a name is whatever its case
/// and its hyphens, underscores and spaces.
fn letters(name: &str) -> Vec<u8> {
    let letters = name.bytes().filter(u8::is_ascii_alphanumeric);
    letters.map(|letter| letter.to_ascii_lowercase()).collect()
}

/// The name that ASN.1 gives the message of a PDU, as `InitialContextSetupResponse`; `None`
/// for a procedure that the specification does not have in that direction.
pub fn message_name(pdu: &NGAP_PDU) -> Option<&'static str> {
    let (direction, code) = (pdu.direction(), pdu.procedure_code());
    message_names()
        .find(|(of, procedure, _)| (*of, *procedure) == (direction, code))
        .map(|(.., name)| name)
}

/// The `direction` and the `procedure_code` of the message that `name` names, whatever
/// its case and its hyphens, underscores and spaces.
pub fn message_named(name: &str) -> Option<(&'static str, u8)> {
    let written = letters(name);
    message_names()
        .find(|(.., known)| letters(known) == written)
        .map(|(direction, code, _)| (direction, code))
}

/// The IEs that the message `name` can have, as its ASN.1 object set lists them: the
/// identifier of each and whether its presence is mandatory, in the order of the set. The
/// name is the one that ASN.1 gives the message, taken as [`message_named`] takes it.
/// `None` for a name that is no message; a message without an object set of IEs, as a
/// private message, has none.
pub fn message_ies(name: &str) -> Option<&'static [(u16, bool)]> {
    let written = letters(name);
    let mut messages = registry::MESSAGES.iter();
    let message = messages.find(|(_, _, known, ..)| letters(known) == written);
    message.map(|(.., ies)| *ies)
}

/// The functions of the type of the message of a direction and a procedure code.
fn message_type(direction: &str, code: u8) -> Result<Typed, String> {
    let mut messages = registry::MESSAGES.iter();
    let message = messages.find(|(of, procedure, ..)| (*of, *procedure) == (direction, code));
    let protocol = registry::PROTOCOL;
    message
        .map(|(_, _, _, typed, _)| typed())
        .ok_or_else(|| format!("unknown {protocol} direction/procedure {direction}/{code}"))
}

fn decode_typed<T: rasn::Decode + rasn::Encode>(raw: &[u8]) -> Result<Value, String> {
    let value = crate::ngap::decode_open_type::<T>(&rasn::types::Any::new(raw.to_vec()))
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&rasn::jer::encode(&value).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}

pub(crate) trait RepairOpenTypes {
    fn repair_open_types(&mut self) -> Result<(), String>;
}

/// Implements [`RepairOpenTypes`] for a generated type, from the fields, or the CHOICE
/// alternatives, through which it reaches an open type.
macro_rules! open_types {
    ($type:path { $($open:ident),* } else { $($closed:ident),* }) => {
        impl $crate::inspect::RepairOpenTypes for $type {
            fn repair_open_types(&mut self) -> Result<(), String> {
                match self {
                    $(Self::$open(value) => {
                        $crate::inspect::RepairOpenTypes::repair_open_types(value)
                    })*
                    $(Self::$closed { .. } => Ok(()),)*
                }
            }
        }
    };
    ($type:path { $($field:tt),* }) => {
        impl $crate::inspect::RepairOpenTypes for $type {
            fn repair_open_types(&mut self) -> Result<(), String> {
                $($crate::inspect::RepairOpenTypes::repair_open_types(&mut self.$field)?;)*
                Ok(())
            }
        }
    };
}
pub(crate) use open_types;

impl RepairOpenTypes for rasn::types::Any {
    fn repair_open_types(&mut self) -> Result<(), String> {
        // JER Any retains JSON text. APER Any needs decoded open-type bytes.
        let json: Value = serde_json::from_slice(self.as_bytes()).map_err(|e| e.to_string())?;
        *self = Self::new(unhex(&json)?);
        Ok(())
    }
}
impl RepairOpenTypes for rasn::types::OctetString {
    fn repair_open_types(&mut self) -> Result<(), String> {
        Ok(())
    }
}
impl<T: RepairOpenTypes> RepairOpenTypes for Option<T> {
    fn repair_open_types(&mut self) -> Result<(), String> {
        if let Some(value) = self {
            value.repair_open_types()?;
        }
        Ok(())
    }
}
impl<T: RepairOpenTypes> RepairOpenTypes for Vec<T> {
    fn repair_open_types(&mut self) -> Result<(), String> {
        for value in self {
            value.repair_open_types()?;
        }
        Ok(())
    }
}
impl<T: RepairOpenTypes> RepairOpenTypes for Box<T> {
    fn repair_open_types(&mut self) -> Result<(), String> {
        self.as_mut().repair_open_types()
    }
}

/// The value of the type `T` that JER reads `value` as, and `value` with the names of
/// ENUMERATED values as the specification spells them.
fn decode_jer<T: rasn::Decode>(value: &Value) -> Result<(T, std::borrow::Cow<'_, Value>), String> {
    let mut value = std::borrow::Cow::Borrowed(value);
    let mut respellings = 0;
    loop {
        match rasn::jer::decode::<T>(&value.to_string()) {
            Ok(typed) => return Ok((typed, value)),
            Err(error) => match respelled(&value, &error).filter(|_| respellings < 64) {
                Some(respelled) => {
                    value = std::borrow::Cow::Owned(respelled);
                    respellings += 1;
                }
                None => return Err(error.to_string()),
            },
        }
    }
}

fn encode_typed<T: rasn::Decode + rasn::Encode + RepairOpenTypes>(
    value: &Value,
) -> Result<Vec<u8>, String> {
    let (mut typed, value) = decode_jer::<T>(value)?;
    typed.repair_open_types()?;
    let checked: Value =
        serde_json::from_str(&rasn::jer::encode(&typed).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if !written_as(&checked, &value) {
        return Err("the value has a member that its ASN.1 type does not have, or is not written as JER writes it".into());
    }
    Ok(crate::ngap::encode_open_type(&typed)
        .map_err(|e| e.to_string())?
        .as_bytes()
        .to_vec())
}

/// Whether `written` is the value that JER writes as `checked`: the same, with
/// hexadecimal in either case.
fn written_as(checked: &Value, written: &Value) -> bool {
    match (checked, written) {
        (Value::String(checked), Value::String(written)) => {
            checked == written
                || (checked.eq_ignore_ascii_case(written)
                    && checked.bytes().all(|digit| digit.is_ascii_hexdigit()))
        }
        (Value::Array(checked), Value::Array(written)) => {
            checked.len() == written.len()
                && checked.iter().zip(written).all(|(c, w)| written_as(c, w))
        }
        (Value::Object(checked), Value::Object(written)) => {
            checked.len() == written.len()
                && (checked.iter())
                    .all(|(name, c)| written.get(name).is_some_and(|w| written_as(c, w)))
        }
        _ => checked == written,
    }
}

/// The transfers of the registry: the member that holds each, and the type that its
/// octets contain. A member that has several types has its name alone.
macro_rules! transfers {
    ($($field:literal $($transfer:path)?;)*) => {
        pub(crate) const TRANSFER_FIELDS: &[&str] = &[$($field,)*];
        #[allow(clippy::match_single_binding)]
        pub(crate) fn transfer(field: &str) -> Result<Typed, String> {
            match field {
                $($($field => Ok(Typed::of::<$transfer>()),)?)*
                _ => Err(format!(
                    "{PROTOCOL} contained transfer {field} is unknown or has several types"
                )),
            }
        }
    };
}
pub(crate) use transfers;

/// The functions of a type of the registry.
pub(crate) struct Typed {
    pub(crate) decode: fn(&[u8]) -> Result<Value, String>,
    pub(crate) encode: fn(&Value) -> Result<Vec<u8>, String>,
}

impl Typed {
    pub(crate) fn of<T: rasn::Decode + rasn::Encode + RepairOpenTypes>() -> Self {
        Self {
            decode: decode_typed::<T>,
            encode: encode_typed::<T>,
        }
    }
}

fn hex(raw: &[u8]) -> String {
    const DIGITS: &[u8] = b"0123456789ABCDEF";
    raw.iter()
        .flat_map(|b| {
            [
                DIGITS[(b >> 4) as usize] as char,
                DIGITS[(b & 15) as usize] as char,
            ]
        })
        .collect()
}

fn unhex(value: &Value) -> Result<Vec<u8>, String> {
    let s = value.as_str().ok_or("open type must be a hex string")?;
    if !s.len().is_multiple_of(2) {
        return Err("odd hex length".into());
    }
    s.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let a = (pair[0] as char).to_digit(16).ok_or("invalid hex")?;
            let b = (pair[1] as char).to_digit(16).ok_or("invalid hex")?;
            Ok(((a << 4) | b) as u8)
        })
        .collect()
}

/// How a tree shows the values of a well-known type, and takes them back.
#[derive(Clone, Copy)]
pub(crate) enum Form {
    /// A PLMN identity as its MCC and MNC: `"208-93"`.
    Plmn,
    /// The digits of an identity in TBCD, as an IMSI has them. A protocol may have no
    /// type of this form.
    #[allow(dead_code)]
    Digits,
    /// An identifier or a counter of this many bits, as a number.
    Number(u32),
    /// A transport layer address: an IPv4 address, an IPv6 address, or both with a comma.
    Address,
}

impl Form {
    /// The readable form of a value as JER has it; `None` when it has none.
    fn read(self, jer: &Value) -> Option<Value> {
        let digits = |digits: &[u8]| {
            // A filler takes the place of a last digit that is not there.
            let digits = digits.strip_suffix(&[15]).unwrap_or(digits);
            let text = digits
                .iter()
                .map(|digit| char::from_digit((*digit).into(), 10));
            text.collect::<Option<String>>()
        };
        match self {
            Self::Plmn => {
                let [a, b, c] = <[u8; 3]>::try_from(unhex(jer).ok()?).ok()?;
                let text = digits(&[a & 15, a >> 4, b & 15, c & 15, c >> 4, b >> 4])?;
                (text.len() >= 5).then(|| json!(format!("{}-{}", &text[..3], &text[3..])))
            }
            Self::Digits => {
                let octets = unhex(jer).ok()?;
                let nibbles: Vec<u8> = octets.iter().flat_map(|o| [o & 15, o >> 4]).collect();
                digits(&nibbles).map(Value::from)
            }
            Self::Number(bits) => {
                let (text, octets) = (jer.as_str()?, bits.div_ceil(8));
                let number = u64::from_str_radix(text, 16).ok()?;
                (text.len() as u32 == octets * 2).then(|| json!(number >> (octets * 8 - bits)))
            }
            Self::Address => {
                let octets = unhex(jer.get("value")?).ok()?;
                let v4 = |octets: &[u8]| {
                    Some(std::net::Ipv4Addr::from(<[u8; 4]>::try_from(octets).ok()?))
                };
                let v6 = |octets: &[u8]| {
                    Some(std::net::Ipv6Addr::from(<[u8; 16]>::try_from(octets).ok()?))
                };
                Some(json!(match (jer.get("length")?.as_u64()?, octets.len()) {
                    (32, 4) => v4(&octets)?.to_string(),
                    (128, 16) => v6(&octets)?.to_string(),
                    (160, 20) => format!("{},{}", v4(&octets[..4])?, v6(&octets[4..])?),
                    _ => return None,
                }))
            }
        }
    }

    /// The value as JER has it, from its readable form. A value in another form is kept
    /// as it is: the JER form is still taken, and the codec refuses what is neither.
    fn write(self, shown: &Value) -> Result<Value, String> {
        let number = |bits: u32, number: u64| {
            if bits < 64 && number >> bits != 0 {
                return Err(format!("{number} does not fit in {bits} bits"));
            }
            let octets = bits.div_ceil(8);
            let width = octets as usize * 2;
            Ok(json!(format!("{:0width$X}", number << (octets * 8 - bits))))
        };
        let Some(text) = shown.as_str() else {
            return match (self, shown.as_u64()) {
                (Self::Number(bits), Some(value)) => number(bits, value),
                _ => Ok(shown.clone()),
            };
        };
        let bcd = |digits: &str| digits.bytes().map(|digit| digit & 15).collect::<Vec<_>>();
        let all_digits = |text: &str| text.bytes().all(|b| b.is_ascii_digit());
        match self {
            Self::Plmn if text.contains('-') => {
                let (mcc, mnc) = text.split_once('-').unwrap_or_default();
                if mcc.len() != 3
                    || !(2..=3).contains(&mnc.len())
                    || !all_digits(&text.replacen('-', "", 1))
                {
                    return Err(format!("{text}: a PLMN identity is MCC-MNC, as 208-93"));
                }
                let (mcc, mnc) = (bcd(mcc), bcd(mnc));
                let filler = mnc.get(2).copied().unwrap_or(15);
                let octets = [
                    mcc[1] << 4 | mcc[0],
                    filler << 4 | mcc[2],
                    mnc[1] << 4 | mnc[0],
                ];
                Ok(json!(hex(&octets)))
            }
            Self::Digits if all_digits(text) => {
                let pairs = bcd(text);
                let pairs = pairs.chunks(2);
                let octets = pairs.map(|pair| pair.get(1).copied().unwrap_or(15) << 4 | pair[0]);
                Ok(json!(hex(&octets.collect::<Vec<_>>())))
            }
            Self::Number(bits) if text.starts_with("0x") => {
                let value = u64::from_str_radix(&text[2..], 16);
                number(bits, value.map_err(|error| format!("{text}: {error}"))?)
            }
            Self::Address if text.contains(['.', ':']) => {
                let mut octets = Vec::new();
                for part in text.split(',') {
                    match (octets.len(), part.trim().parse()) {
                        (0, Ok(std::net::IpAddr::V4(address))) => octets.extend(address.octets()),
                        (0 | 4, Ok(std::net::IpAddr::V6(address))) => {
                            octets.extend(address.octets())
                        }
                        _ => {
                            return Err(format!(
                                "{text}: a transport layer address is an IPv4 address, an IPv6 address, or the two with a comma"
                            ));
                        }
                    }
                }
                Ok(json!({"value": hex(&octets), "length": octets.len() * 8}))
            }
            _ => Ok(shown.clone()),
        }
    }
}

/// Show `value`, or each value of a list, in the readable form that it has: only a value
/// that the form gives back as it was.
fn show(form: Form, value: &mut Value) {
    match value {
        Value::Array(values) => values.iter_mut().for_each(|value| show(form, value)),
        value => {
            let shown = form.read(value);
            if let Some(shown) =
                shown.filter(|shown| form.write(shown).is_ok_and(|jer| jer == *value))
            {
                *value = shown;
            }
        }
    }
}

/// `value`, or each value of a list, as JER has it again.
fn unshow(form: Form, value: &mut Value) -> Result<(), String> {
    match value {
        Value::Array(values) => values.iter_mut().try_for_each(|value| unshow(form, value)),
        value => {
            *value = form.write(value)?;
            Ok(())
        }
    }
}

/// `value` with the name that JER did not find among those of an ENUMERATED type as the
/// specification spells it: a name is the same whatever its case, with `-`, `_` and
/// space taken as the same.
fn respelled(value: &Value, error: &rasn::error::DecodeError) -> Option<Value> {
    use rasn::error::{CodecDecodeError, DecodeErrorKind, JerDecodeErrorKind};
    // The error of a member is in the errors of the fields that lead to it.
    let mut error = error;
    while let DecodeErrorKind::FieldError { nested, .. } = &*error.kind {
        error = nested;
    }
    let DecodeErrorKind::CodecSpecific {
        inner: CodecDecodeError::Jer(JerDecodeErrorKind::InvalidEnumDiscriminant { discriminant }),
    } = &*error.kind
    else {
        return None;
    };
    let letter = |c: u8| match c {
        b'-' | b'_' | b' ' => b'-',
        c => c.to_ascii_lowercase(),
    };
    let written = discriminant.bytes().map(letter);
    let same = |name: &&&str| name.bytes().map(letter).eq(written.clone());
    // Two types may spell a name differently: the next decoding tries the other.
    let mut names = registry::ENUMERATED.iter().filter(same);
    let name = names.find(|name| **name != discriminant.as_str())?;
    fn rename(value: &mut Value, from: &str, to: &str) {
        match value {
            Value::String(text) if text == from => *text = to.into(),
            Value::Array(values) => values.iter_mut().for_each(|value| rename(value, from, to)),
            Value::Object(members) => members
                .values_mut()
                .for_each(|value| rename(value, from, to)),
            _ => {}
        }
    }
    let mut value = value.clone();
    rename(&mut value, discriminant, name);
    Some(value)
}

/// The value that the octets `raw` contain, beside them, or why they did not decode.
fn contained(raw: Value, typed: Result<Typed, String>) -> Value {
    match typed.and_then(|typed| (typed.decode)(&unhex(&raw)?)) {
        Ok(decoded) => json!({"_raw_transfer": raw, "decoded": decoded}),
        Err(error) => json!({"_raw_transfer": raw, "_decode_error": error}),
    }
}

/// How deep in a tree an IE or a transfer is still decoded.
const NESTING: usize = 64;

/// Why what is deeper keeps its octets.
fn too_deep<T>() -> Result<T, String> {
    Err(format!("inspection nesting exceeds {NESTING}"))
}

/// Decode the IEs and the transfers of `value`, which is `depth` deep in its tree. What
/// is deeper than [`NESTING`] keeps its octets, with why.
fn expand(value: &mut Value, depth: usize) {
    let deep = depth > NESTING;
    if let Some(object) = value.as_object_mut() {
        let field = if object.contains_key("value") {
            "value"
        } else {
            "extensionValue"
        };
        if let Some(id) = object
            .get("id")
            .and_then(Value::as_u64)
            .and_then(|v| u16::try_from(v).ok())
            && let Some(raw) = object.get(field).filter(|v| v.is_string()).cloned()
        {
            object.insert("_raw_value".into(), raw.clone());
            object.insert("_original_id".into(), json!(id));
            if let Some((_, name)) = ie_names().iter().find(|(key, _)| *key == id) {
                object.insert("_ie_name".into(), json!(name));
            }
            let decoded = match deep {
                true => too_deep(),
                false => unhex(&raw).and_then(|bytes| (registry::ie(id)?.decode)(&bytes)),
            };
            match decoded {
                Ok(decoded) => {
                    // An OCTET STRING (CONTAINING ...) has the form of a transfer.
                    let mut decoded = match registry::ie_contents(id) {
                        Some(contents) => contained(decoded, Ok(contents)),
                        None => decoded,
                    };
                    if let Some(form) = registry::ie_form(id) {
                        show(form, &mut decoded);
                    }
                    object.insert(field.into(), decoded);
                }
                Err(error) => {
                    object.insert("_decode_error".into(), json!(error));
                }
            }
        }
        for (key, child) in object {
            if !key.starts_with('_') {
                if registry::TRANSFER_FIELDS.contains(&key.as_str()) && child.is_string() {
                    let typed = match deep {
                        true => too_deep(),
                        false => registry::transfer(key),
                    };
                    *child = contained(child.take(), typed);
                }
                expand(child, depth + 1);
                if let Some(form) = registry::member_form(key) {
                    show(form, child);
                }
            }
        }
    } else if let Some(array) = value.as_array_mut() {
        for child in array {
            expand(child, depth + 1);
        }
    }
}

/// `member` is the name of the member that holds `value`, or the list it is in.
fn collapse(value: &mut Value, member: &str, depth: usize) -> Result<(), String> {
    // What is decoded at the limit of the inspection has values of its own under it.
    if depth > 4 * NESTING {
        return Err(format!("inspection nesting exceeds {}", 4 * NESTING));
    }
    if let Some(object) = value.as_object_mut() {
        let id = object
            .get("id")
            .and_then(Value::as_u64)
            .and_then(|v| u16::try_from(v).ok());
        for (key, child) in object.iter_mut() {
            if !key.starts_with('_') {
                collapse(child, key, depth + 1)?;
                if let Some(form) = registry::member_form(key) {
                    unshow(form, child)?;
                }
                if let Some(raw) = child.get("_raw_transfer").cloned() {
                    // Nothing that is written is left out: a transfer has its decoded
                    // value beside the octets that it was received as, and no other.
                    let known = ["_raw_transfer", "decoded", "_decode_error", EDITED];
                    let mut members = child.as_object().into_iter().flatten();
                    if let Some((other, _)) =
                        members.find(|(name, _)| !known.contains(&name.as_str()))
                    {
                        return Err(format!(
                            "{key} has no member {other:?}: a transfer has its decoded value"
                        ));
                    }
                    if let Some(edited) = child.get("decoded") {
                        let bytes = unhex(&raw)?;
                        // An IE's identifier gives the type of what it contains, and
                        // the name of a transfer field the type of the transfer.
                        let transfer = match key.as_str() {
                            "value" | "extensionValue" => id
                                .and_then(registry::ie_contents)
                                .ok_or("this IE does not contain a type")?,
                            _ => registry::transfer(key)?,
                        };
                        let original = (transfer.decode)(&bytes)?;
                        if &original == edited {
                            *child = raw;
                        } else {
                            if (transfer.encode)(&original)? != bytes {
                                return Err(format!(
                                    "{key} does not encode back to the octets received"
                                ));
                            }
                            *child = json!(hex(&(transfer.encode)(edited)?));
                        }
                    } else {
                        *child = raw;
                    }
                } else if let Some(written) = child.get("decoded") {
                    // What was not received is encoded from the value written.
                    let transfer = match key.as_str() {
                        "value" | "extensionValue" => id.and_then(registry::ie_contents),
                        key if registry::TRANSFER_FIELDS.contains(&key) => {
                            Some(registry::transfer(key)?)
                        }
                        _ => None,
                    };
                    if let Some(transfer) = transfer {
                        if child.as_object().is_some_and(|members| members.len() != 1) {
                            return Err(format!(
                                "{key} is written with its decoded value alone, or as its octets"
                            ));
                        }
                        *child = json!(hex(&(transfer.encode)(written)?));
                    }
                }
            }
        }
        let raw = object.remove("_raw_value");
        let original_id = object.remove("_original_id");
        let undecoded = object.remove("_decode_error").is_some();
        object.remove("_ie_name");
        object.remove(EDITED);
        if let Some(id) = id {
            // The value of an IE of an extension container has another name.
            let field = match member.starts_with("iE-Extension") {
                true => "extensionValue",
                false => "value",
            };
            let typed = |id: u16| {
                let octets = format!("give its octets as _raw_value, without {field}");
                registry::ie(id).map_err(|error| format!("{error}: {octets}"))
            };
            if !undecoded
                && let Some(form) = registry::ie_form(id)
                && let Some(value) = object.get_mut(field)
            {
                unshow(form, value)?;
            }
            let wire = match (raw, object.get(field)) {
                // The octets alone are sent as they are.
                (Some(raw), None) => raw,
                // The IE was shown as its octets.
                (_, Some(edited)) if undecoded => {
                    if !edited.is_string() {
                        return Err(format!(
                            "IE {id} did not decode: its value is its octets in hexadecimal"
                        ));
                    }
                    edited.clone()
                }
                (Some(raw), Some(edited)) => {
                    let original_id = original_id
                        .as_ref()
                        .and_then(Value::as_u64)
                        .and_then(|v| u16::try_from(v).ok())
                        .unwrap_or(id);
                    let bytes = unhex(&raw)?;
                    let received = registry::ie(original_id)?;
                    let original = (received.decode)(&bytes)?;
                    if &original == edited && id == original_id {
                        raw
                    } else if (received.encode)(&original)? != bytes {
                        return Err(format!(
                            "IE {original_id} does not encode back to the octets received; replace its octets instead"
                        ));
                    } else {
                        json!(hex(&(typed(id)?.encode)(edited)?))
                    }
                }
                // An IE that was not received.
                (None, Some(new)) => json!(hex(&(typed(id)?.encode)(new)?)),
                (None, None) => return Err(format!("IE {id} has no {field} and no _raw_value")),
            };
            object.insert(field.into(), wire);
        }
    } else if let Some(array) = value.as_array_mut() {
        for child in array {
            collapse(child, member, depth + 1)?;
        }
    }
    Ok(())
}

/// The tree of a PDU. An error when its message does not decode.
pub fn inspect_pdu(pdu: &NGAP_PDU) -> Result<Value, String> {
    let top: Value = serde_json::from_str(&rasn::jer::encode(pdu).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let body = top
        .as_object()
        .and_then(|o| o.values().next())
        .ok_or("invalid PDU choice")?;
    let code = pdu.procedure_code();
    let direction = pdu.direction();
    let raw = body["value"].clone();
    let mut message = (message_type(direction, code)?.decode)(&unhex(&raw)?)?;
    expand(&mut message, 0);
    Ok(
        json!({"procedure_code": code, "direction": direction, "criticality": body["criticality"], "message": message, "_raw_message": raw}),
    )
}

/// The PDU of a tree, with the octets received for what was not edited.
pub fn encode_pdu(tree: &Value) -> Result<NGAP_PDU, String> {
    if tree.as_object().is_none_or(|o| {
        o.keys().any(|key| {
            ![
                "procedure_code",
                "direction",
                "criticality",
                "message",
                "_raw_message",
            ]
            .contains(&key.as_str())
        })
    }) {
        return Err("unknown PDU inspection root field".into());
    }
    let code = tree["procedure_code"]
        .as_u64()
        .and_then(|c| u8::try_from(c).ok())
        .ok_or("procedure_code must be u8")?;
    let direction = tree["direction"]
        .as_str()
        .ok_or("direction must be a string")?;
    let variant = match direction {
        "InitiatingMessage" => "initiatingMessage",
        "SuccessfulOutcome" => "successfulOutcome",
        "UnsuccessfulOutcome" => "unsuccessfulOutcome",
        _ => return Err("invalid direction".into()),
    };
    let mut message = tree.get("message").ok_or("missing message")?.clone();
    collapse(&mut message, "", 0)?;
    let raw = if let Some(original) = tree.get("_raw_message") {
        let original = unhex(original)?;
        let typed = message_type(direction, code)?;
        let decoded = (typed.decode)(&original)?;
        if decoded == message {
            original
        } else {
            if (typed.encode)(&decoded)? != original {
                return Err("the message does not encode back to the octets received".into());
            }
            (typed.encode)(&message)?
        }
    } else {
        (message_type(direction, code)?.encode)(&message)?
    };
    let top = json!({variant: {"procedureCode": code, "criticality": tree["criticality"], "value": hex(&raw)}});
    let (mut pdu, _) = decode_jer::<NGAP_PDU>(&top)?;
    pdu.repair_open_types()?;
    Ok(pdu)
}
