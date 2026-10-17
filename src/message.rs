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
    /// The criticality that the specification assigns to its procedure, which the PDU
    /// that holds it has: what `of_message` is given for it, and what the macro that
    /// builds a PDU gives one whose criticality it is not given.
    const CRITICALITY: crate::registry::Criticality;
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

/// A type whose value is a container of protocol IEs, whose IEs the macros read, change
/// and build: the type of a message, or of a transfer whose octets hold such a
/// container, as `PDUSessionResourceSetupRequestTransfer` of NGAP. The generator
/// implements it for the transfers, and every message has it.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is no container of protocol IEs",
    label = "the type of a message, or of a transfer of protocol IEs, is expected here",
    note = "the macros take the type of a message, as `UEContextReleaseRequest`, or of a \
            transfer of protocol IEs; an IE of a list of single containers is built without \
            the name of a type"
)]
#[doc(hidden)]
pub trait __Ies<'a>: Decode<'a> + Encode + sealed::Sealed {
    /// Whether the type is a container of protocol IEs: every message but the private
    /// one is, and every transfer of this trait.
    const __OF_PROTOCOL_IES: bool;
    /// The criticality that the object set of the type assigns to the IE `id`: `None`
    /// for an IE that the set does not have, and for a transfer, whose object set the
    /// tables of the crate do not have.
    fn __assigned(id: u16) -> Option<oxirush_aper::Criticality>;
}

impl<'a, M: Message<'a>> __Ies<'a> for M {
    const __OF_PROTOCOL_IES: bool = M::__OF_PROTOCOL_IES;

    fn __assigned(id: u16) -> Option<oxirush_aper::Criticality> {
        let ies = crate::check::object_set(M::DIRECTION, M::PROCEDURE_CODE)?.ies?;
        let found = ies.iter().find(|(known, ..)| *known == id);
        found.map(|(_, assigned, _)| (*assigned).into())
    }
}

pub(crate) mod sealed {
    /// A type of the registry, or a transfer of protocol IEs: no other type is a
    /// message or a container of IEs.
    pub trait Sealed {}
}
