//! Decoded PDUs as JSON trees that can be edited and encoded again.
//!
//! [`inspect_pdu`] returns `procedure_code`, `direction`, `criticality` and `message`: the
//! message in the ASN.1 JSON encoding (JER). An IE whose identifier has one type has its
//! typed `value` (`extensionValue` in an extension container) in place of its octets. A
//! contained transfer, and the value of an IE declared `OCTET STRING (CONTAINING ...)`, is
//! its `decoded` value beside its `_raw_transfer`. Repeated IEs stay ordered array entries.
//!
//! The members that start with `_` say what was received:
//!
//! - `_raw_value`, `_raw_transfer` and `_raw_message`: the octets, in hexadecimal;
//! - `_original_id` and `_ie_name`: the IE that the octets were decoded as;
//! - `_decode_error`: why an IE's `value` is still its octets in hexadecimal, or why a
//!   transfer has no `decoded` member. The identifier is unknown or has several types, the
//!   octets do not decode, or JER cannot represent the value.
//!
//! [`encode_pdu`] encodes the tree. What was not edited keeps the octets received. To edit:
//!
//! - change a typed `value` or the `decoded` member of a transfer, which is encoded again.
//!   This is refused when the octets received do not decode and encode back to themselves,
//!   as with an extension addition that the typed value does not keep;
//! - to send other octets as an IE, or to add an IE, write them in hexadecimal as `value`
//!   and leave `_raw_value` out; for a transfer, replace the member with them;
//! - an IE with a `_decode_error` has its octets as `value`: change them there.
//!
//! `_raw_value` and `_raw_message` are what an edit is compared with, so changing one alone
//! sends nothing else. A member that the ASN.1 type does not have is refused, and so is
//! hexadecimal in lower case.

use serde_json::{Value, json};

use crate::inspect_registry as registry;
use crate::ngap::NGAP_PDU;

/// ASN.1-derived IE identifiers and names, including extension IEs.
pub fn ie_names() -> &'static [(u16, &'static str)] {
    registry::IE_NAMES
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

fn encode_typed<T: rasn::Decode + rasn::Encode + RepairOpenTypes>(
    value: &Value,
) -> Result<Vec<u8>, String> {
    let mut typed = rasn::jer::decode::<T>(&value.to_string()).map_err(|e| e.to_string())?;
    typed.repair_open_types()?;
    let checked: Value =
        serde_json::from_str(&rasn::jer::encode(&typed).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if checked != *value {
        return Err("the value has a member that its ASN.1 type does not have, or is not written as JER writes it".into());
    }
    Ok(crate::ngap::encode_open_type(&typed)
        .map_err(|e| e.to_string())?
        .as_bytes()
        .to_vec())
}

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

/// The value that the octets `raw` contain, beside them, or why they did not decode.
fn contained(raw: Value, typed: Result<Typed, String>) -> Value {
    match typed.and_then(|typed| (typed.decode)(&unhex(&raw)?)) {
        Ok(decoded) => json!({"_raw_transfer": raw, "decoded": decoded}),
        Err(error) => json!({"_raw_transfer": raw, "_decode_error": error}),
    }
}

fn expand(value: &mut Value, depth: usize) -> Result<(), String> {
    if depth > 64 {
        return Err("inspection nesting exceeds 64".into());
    }
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
            match unhex(&raw).and_then(|bytes| (registry::ie(id)?.decode)(&bytes)) {
                Ok(decoded) => {
                    // An OCTET STRING (CONTAINING ...) has the form of a transfer.
                    let decoded = match registry::ie_contents(id) {
                        Some(contents) => contained(decoded, Ok(contents)),
                        None => decoded,
                    };
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
                    *child = contained(child.take(), registry::transfer(key));
                }
                expand(child, depth + 1)?;
            }
        }
    } else if let Some(array) = value.as_array_mut() {
        for child in array {
            expand(child, depth + 1)?;
        }
    }
    Ok(())
}

fn collapse(value: &mut Value, depth: usize) -> Result<(), String> {
    if depth > 64 {
        return Err("inspection nesting exceeds 64".into());
    }
    if let Some(object) = value.as_object_mut() {
        let id = object
            .get("id")
            .and_then(Value::as_u64)
            .and_then(|v| u16::try_from(v).ok());
        for (key, child) in object.iter_mut() {
            if !key.starts_with('_') {
                collapse(child, depth + 1)?;
                if let Some(raw) = child.get("_raw_transfer").cloned() {
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
                }
            }
        }
        let raw = object.remove("_raw_value");
        let original_id = object.remove("_original_id");
        let undecoded = object.remove("_decode_error").is_some();
        object.remove("_ie_name");
        if let Some(raw) = raw {
            let id = object
                .get("id")
                .and_then(Value::as_u64)
                .and_then(|v| u16::try_from(v).ok())
                .ok_or("IE id must be u16")?;
            let field = if object.contains_key("value") {
                "value"
            } else {
                "extensionValue"
            };
            let edited = object.get(field).ok_or("IE has no value")?;
            let original_id = original_id
                .as_ref()
                .and_then(Value::as_u64)
                .and_then(|v| u16::try_from(v).ok())
                .unwrap_or(id);
            let wire = if undecoded {
                // The IE was shown as its octets.
                if !edited.is_string() {
                    return Err(format!(
                        "IE {id} did not decode: its value is its octets in hexadecimal"
                    ));
                }
                edited.clone()
            } else {
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
                    json!(hex(&(registry::ie(id)?.encode)(edited)?))
                }
            };
            object.insert(field.into(), wire);
        }
    } else if let Some(array) = value.as_array_mut() {
        for child in array {
            collapse(child, depth + 1)?;
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
    let mut message = (registry::message(direction, code)?.decode)(&unhex(&raw)?)?;
    expand(&mut message, 0)?;
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
    collapse(&mut message, 0)?;
    let raw = if let Some(original) = tree.get("_raw_message") {
        let original = unhex(original)?;
        let typed = registry::message(direction, code)?;
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
        (registry::message(direction, code)?.encode)(&message)?
    };
    let top = json!({variant: {"procedureCode": code, "criticality": tree["criticality"], "value": hex(&raw)}});
    let mut pdu: NGAP_PDU = rasn::jer::decode(&top.to_string()).map_err(|e| e.to_string())?;
    pdu.repair_open_types()?;
    Ok(pdu)
}
