//! What the fuzz targets of `fuzz/` take from the inside of the crate. It is compiled
//! with `--cfg fuzzing`, which cargo-fuzz sets, and is no part of the crate without.
//!
//! A target compares what the crate does itself with what the codec of rasn does
//! alone. What the functions of the crate say of a PDU is compared in the target; this
//! module has what needs the type of a message, which no function gives from a PDU,
//! and the comparison that the JSON forms make where they are written.

use core::fmt::Debug;

use crate::Message;
use crate::registry::{Pdu, with_message_type};

/// What is done with the type of a message.
pub(crate) trait Reader {
    fn read<T: Message + PartialEq + Debug>(&mut self);
}

/// The message of `pdu` as its type: `decode_value` reads the value that the codec of
/// rasn alone reads in the octets of the message, or both refuse them. What was read
/// is written, and read again as the same message.
pub fn message_as_rasn_reads_it(pdu: &Pdu) {
    struct Compare<'a>(&'a Pdu);
    impl Reader for Compare<'_> {
        fn read<T: Message + PartialEq + Debug>(&mut self) {
            let pdu = self.0;
            let ours = pdu.decode_value::<T>();
            let theirs = crate::per::decode_complete::<T>(pdu.value().as_bytes());
            assert_eq!(ours.as_ref().ok(), theirs.as_ref().ok(), "{pdu:?}");
            let Ok(message) = ours else {
                return;
            };
            let written = match crate::per::encode_open_type(&message) {
                Ok(written) => written,
                Err(error) => panic!("a message that was decoded does not encode: {error}"),
            };
            let again = crate::per::decode_complete::<T>(written.as_bytes());
            assert_eq!(again.as_ref(), Ok(&message), "{written:?}");
        }
    }
    with_message_type(pdu.direction(), pdu.procedure_code(), &mut Compare(pdu));
}

/// From here on, each value that a view of this thread shows is compared with what
/// the JSON Encoding Rules of rasn write of it, and is read back from its JSON form.
#[cfg(feature = "inspect")]
pub fn compare_json_with_rasn() {
    crate::json::COMPARED.set(Some(0));
}
