//! What the fuzz targets of `fuzz/` and the tests of `oracle/` take from the inside of
//! the crate. It is compiled with `--cfg oxirush_ngap_hooks`, which those two packages
//! are built with, and is no part of the crate without: the fuzz build of another
//! program, for which cargo-fuzz sets `fuzzing`, has the crate as every other build
//! has it.
//!
//! Both compare what the crate does with what the codec of rasn does, which the oracle
//! has. The oracle is no dependency of the crate, so the comparison is in them: this
//! module gives them what needs the type of a message, which no function gives from a
//! PDU, each value that a view decodes, and the two ways in which the crate writes a
//! message.

use crate::Error;
use crate::ngap::{NGAPPDU, ProtocolIEContainer};

/// The bindings, by a name that is the same for both protocols.
pub use crate::ngap as bindings;
/// A PDU that owns its octets, by a name that is the same for both protocols.
pub use crate::ngap::NGAP_PDU as Pdu;
/// A PDU that borrows its message from the octets that it is decoded from.
pub use crate::ngap::NGAPPDU as BorrowedPdu;

/// The name of the type of a PDU in the bindings.
pub const PDU_TYPE: &str = "NGAPPDU";

/// The octets of the message of `pdu`.
pub fn message_octets<'p>(pdu: &'p NGAPPDU<'_>) -> &'p [u8] {
    pdu.value()
}

/// The octets of the message that has these protocol IEs, as `set_protocol_ies`
/// writes them, octet by octet, and as the codec of the type writes them.
pub fn message_octets_both_ways(
    ies: &ProtocolIEContainer<'_>,
) -> (Result<Vec<u8>, Error>, Result<Vec<u8>, Error>) {
    let as_its_type = crate::pdu::message_octets_as_its_type(ies);
    (crate::pdu::message_octets(ies), as_its_type)
}

/// The message of `pdu` as its type: the name of the type in the bindings, and the
/// octets that the message encodes to once `decode_value` has read it, or why it does
/// not decode. `None` when the protocol has no message for the PDU.
pub fn message_read_and_written(
    pdu: &NGAPPDU<'_>,
) -> Option<(&'static str, Result<Vec<u8>, Error>)> {
    let name = crate::registry::message_type_name(pdu.direction(), pdu.procedure_code())?;
    Some((name, crate::registry::message_read_and_written(pdu)?))
}

/// What is told of each value that a view decodes.
#[cfg(feature = "inspect")]
pub type Observer = fn(&'static str, &[u8], &Result<oxirush_aper::json::Value, Error>);

/// From here on, `observer` is told of each value that a view of this thread decodes
/// as a type of the registry: the name of the type in the bindings, the octets, and
/// the value in the form of the JSON Encoding Rules, which the oracle writes, or why
/// the octets are none. `None` ends it.
#[cfg(feature = "inspect")]
pub fn observe(observer: Option<Observer>) {
    crate::inspect::OBSERVER.set(observer);
}
