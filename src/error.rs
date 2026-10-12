//! The errors of the crate that name what the registry has: the message of a PDU, and
//! an IE by its identifier. The error type is that of `oxirush-aper`.

use crate::Error;

/// A PDU of the message of `direction` and `code`, which is decoded as the message
/// that ASN.1 names `expected`.
pub(crate) fn wrong_message(expected: &str, direction: &str, code: u8) -> Error {
    let held = crate::registry::message_name(direction, code);
    Error::__wrong_message(expected, held, direction, code)
}

/// The IE of an identifier, as a message names it: the name that ASN.1 gives it after
/// `id-`, or its number for one that the protocol does not have.
pub(crate) fn ie(id: u16) -> String {
    let mut names = crate::registry::IE_NAMES.iter();
    match names.find(|(known, _)| *known == id) {
        Some((_, name)) => name.to_string(),
        None => format!("IE {id}"),
    }
}
