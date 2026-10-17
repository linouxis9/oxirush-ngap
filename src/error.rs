//! The errors of the crate that name what the registry has: the message of a PDU, and
//! an IE by its identifier. The error type is that of `oxirush-aper`.

use oxirush_aper::ErrorIe;

use crate::Error;
use crate::message::__Ies;
use crate::registry::Criticality;

/// A PDU of the message of `direction` and `code`, which is decoded as the message
/// that ASN.1 names `expected`.
pub(crate) fn wrong_message(expected: &str, direction: &str, code: u8) -> Error {
    let held = crate::registry::message_name(direction, code);
    Error::wrong_message(expected, held, direction, code)
}

/// The IE `id`, which the field `field` requires, is not in a message, or a transfer,
/// of the type `M`. The error is about that IE, with the criticality that the object set
/// of the message assigns it.
pub(crate) fn missing_ie<'a, M: __Ies<'a>>(id: u16, field: &str) -> Error {
    let error = Error::missing_ie(M::NAME, &ie(id), field);
    match M::__assigned(id) {
        Some(assigned) => error.about_ie(ErrorIe::new(id, assigned, None)),
        // An IE that the object set of the message does not have, or of a transfer.
        None => error,
    }
}

/// `error` is why the value of the IE `id` does not decode, in a message that ASN.1
/// names `message`. The error is about that IE, with the criticality that it was
/// received with and its position in the message.
pub(crate) fn undecodable_ie(
    error: Error,
    message: &str,
    id: u16,
    criticality: Criticality,
    position: usize,
) -> Error {
    let about = ErrorIe::new(id, criticality.into(), Some(position));
    error.undecodable_ie(message, &ie(id)).about_ie(about)
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
