//! What the registry says of the type of a message.

/// A message of the protocol: the type of what the PDU of one procedure holds in one
/// direction.
///
/// The registry of the crate implements it for the type of each message that the ASN.1
/// gives a procedure, and for no other type. So `decode_value` of a PDU takes the type
/// of a message, and not that of an IE or of a transfer, which would decode from the
/// same octets as another value.
pub trait Message: rasn::Decode + rasn::Encode + sealed::Sealed {
    /// The name that ASN.1 gives the message, as `UEContextReleaseRequest`.
    const NAME: &'static str;
    /// The direction of the PDU that holds it, as `direction()` of a PDU says it:
    /// `"InitiatingMessage"`, `"SuccessfulOutcome"` or `"UnsuccessfulOutcome"`.
    const DIRECTION: &'static str;
    /// The code of its procedure.
    const PROCEDURE_CODE: u8;
}

pub(crate) mod sealed {
    /// A type of the registry: no other type is a message.
    pub trait Sealed {}
}
