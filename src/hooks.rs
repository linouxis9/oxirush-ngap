//! What the fuzz targets of `fuzz/` take from the inside of the crate. It is compiled
//! with `--cfg oxirush_ngap_hooks`, which the fuzz targets of the crate are built with, and is no
//! part of the crate without: the fuzz build of another program, for which cargo-fuzz
//! sets `fuzzing`, has the crate as every other build has it.
//!
//! A target compares what the crate does with what the codec of rasn does, which the
//! oracle of the tests has. The oracle is no dependency of the crate, so the comparison
//! is in the target: this module gives it what needs the type of a message, which no
//! function gives from a PDU, and each value that a view decodes.

use crate::Error;
use crate::ngap::NGAPPDU;

/// The message of `pdu` as its type: the name of the type in the bindings, and the
/// octets that the message encodes to once `decode_value` has read it, or why it does
/// not decode. `None` when the protocol has no message for the PDU.
pub fn message_read_and_written(
    pdu: &NGAPPDU<'_>,
) -> Option<(&'static str, Result<Vec<u8>, Error>)> {
    let name = crate::registry::message_type_name(pdu.direction(), pdu.procedure_code())?;
    Some((name, crate::registry::message_read_and_written(pdu)?))
}

/// From here on, `observer` is told of each value that a view of this thread decodes
/// as a type of the registry: the name of the type in the bindings, the octets, and
/// the JSON form of the value, or why the octets are none.
#[cfg(feature = "inspect")]
pub fn observe(observer: fn(&'static str, &[u8], &Result<serde_json::Value, Error>)) {
    crate::inspect::OBSERVER.set(Some(observer));
}
