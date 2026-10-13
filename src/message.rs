//! What the registry says of the type of a message.

use oxirush_aper::{Decode, Encode};

/// A message of the protocol: the type of what the PDU of one procedure holds in one
/// direction. `'a` is how long the octets live that the message borrows from.
///
/// The registry of the crate implements it for the type of each message that the ASN.1
/// gives a procedure, and for no other type. So `decode_value` of a PDU takes the type
/// of a message, and not that of an IE or of a transfer, which would decode from the
/// same octets as another value.
///
/// The name that ASN.1 gives the message, as `UEContextReleaseRequest`, is the `NAME`
/// of its type, which every type of the bindings has.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is no message of the protocol",
    label = "the type of a message is expected here",
    note = "a message is the type of what a PDU holds, as `UEContextReleaseRequest`; an IE of a \
            list of single containers is built without the name of a type"
)]
pub trait Message<'a>: Decode<'a> + Encode + sealed::Sealed {
    /// The direction of the PDU that holds it, as `direction()` of a PDU says it:
    /// `"InitiatingMessage"`, `"SuccessfulOutcome"` or `"UnsuccessfulOutcome"`.
    const DIRECTION: &'static str;
    /// The code of its procedure.
    const PROCEDURE_CODE: u8;
    /// Whether the message is a container of protocol IEs, which every message but
    /// the private one is.
    #[doc(hidden)]
    const __OF_PROTOCOL_IES: bool;
    /// The message that `octets` hold, which borrows nothing from them: what it holds
    /// of octets is copied.
    #[doc(hidden)]
    fn __decode_owned(octets: &[u8]) -> Result<Self, oxirush_aper::Error>;
}

/// What a field of the macro that extracts IEs is given of the value of an IE: the
/// value itself, for a field of the type of the IE, and the one value that the type
/// of the IE is a name for, for a field of the type of that value. The bindings
/// implement it for each type that is a name for one value.
#[diagnostic::on_unimplemented(
    message = "a field of the type `{T}` does not take the value of this IE, which is a `{Self}`",
    label = "the value of the IE is a `{Self}`",
    note = "declare the field as `{Self}`, or as the type of the one value that a `{Self}` holds, \
            or write `=> expression` after the IE to make the field of its value"
)]
#[doc(hidden)]
pub trait __Field<T> {
    /// The field.
    fn __field(self) -> T;
}

impl<T> __Field<T> for T {
    #[inline]
    fn __field(self) -> T {
        self
    }
}

pub(crate) mod sealed {
    /// A type of the registry: no other type is a message.
    pub trait Sealed {}
}
