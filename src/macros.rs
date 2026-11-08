//! Ergonomic macros for building, extracting, and mutating NGAP protocol IEs.
//!
//! The bindings have the value of an IE as the octets of an open type: the object
//! set of a message does not show in its type. The macros keep that inside: callers
//! construct and receive concrete NGAP types, while values are APER-encoded into
//! and decoded from the open type at the protocol boundary.
//!
//! # Builder macros
//!
//! ## `build_ngap!` — build a complete NGAP PDU
//!
//! IE IDs and procedure codes are those of `src/registry.rs`, the list generated
//! from the ASN.1 object sets, which the `inspect` feature reads too. Values
//! are converted with `.into()`, so primitive values such as `u32` can be passed
//! directly for generated newtypes.
//!
//! ```
//! use oxirush_ngap::{build_ngap, ngap::*};
//!
//! let pdu = build_ngap!(IGNORE UEContextReleaseRequest,
//!     REJECT AMF_UE_NGAP_ID(1u64),
//!     REJECT RAN_UE_NGAP_ID(7u32),
//!     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
//! )?;
//! assert_eq!(pdu.procedure_name(), "UEContextReleaseRequest");
//! # Ok::<(), oxirush_ngap::Error>(())
//! ```
//!
//! The macro gives a `Result`. A value that does not encode, as an integer outside
//! its constraint, is an [`Error`](crate::Error) of the kind `Encode`, and no panic.
//!
//! Arguments:
//! - `$direction` — `InitiatingMessage`, `SuccessfulOutcome`, or `UnsuccessfulOutcome`
//! - `$proc` — procedure name, used to derive its ASN.1 procedure code
//! - `$outer_crit` — outer criticality: `REJECT`, `IGNORE`, or `NOTIFY`, or none
//! - `$msg` — message struct name, such as `UEContextReleaseRequest`
//! - IEs — `$ie_crit $ie_name($ie_value)`, with the IE ID derived from its name, and
//!   `$ie_crit` as the outer criticality
//!
//! A criticality that is left out is the one that the specification assigns: that of
//! the procedure for the PDU, which is [`Message::CRITICALITY`](crate::Message) of its
//! message, and that of the object set of the message for an IE. An IE that the set does
//! not have has none, and is an error of the kind `Argument`.
//!
//! ```
//! use oxirush_ngap::{Message, build_ngap, ngap::*};
//!
//! let pdu = build_ngap!(UEContextReleaseRequest,
//!     AMF_UE_NGAP_ID(1u64),
//!     RAN_UE_NGAP_ID(7u32),
//!     Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
//! )?;
//! assert_eq!(pdu.criticality(), UEContextReleaseRequest::CRITICALITY);
//! assert!(pdu.check()?.is_empty());
//! # Ok::<(), oxirush_ngap::Error>(())
//! ```
//!
//! ## One IE: `Ie::field` and `Ie::field_of`
//!
//! The marker type of an IE builds the entry of a container that holds it, for an IE
//! that a message has on a condition or that is built apart: with a criticality, or
//! with the one that the object set of a message or of a transfer assigns it.
//!
//! ```
//! use oxirush_ngap::{Ie, ie, ngap::*};
//!
//! let ie = ie::AMF_UE_NGAP_ID::field(Criticality::Reject, 1u64)?;
//! assert_eq!(ie.criticality, Criticality::Reject);
//! let ie = ie::AMF_UE_NGAP_ID::field_of::<UEContextReleaseRequest>(1u64)?;
//! assert_eq!(ie.criticality, Criticality::Reject);
//! # Ok::<(), oxirush_ngap::Error>(())
//! ```
//!
//! An IE name is its ASN.1 identifier without `id-` and with `_` for `-`. The
//! name of a type is an alias only for the one IE of that type: the type of
//! both id-SONConfigurationTransferDL and id-SONConfigurationTransferUL is
//! addressed by the IE names.
//!
//! ```
//! use oxirush_ngap::{Ie, ie, ngap::*};
//!
//! fn downlink(transfer: SONConfigurationTransfer) -> Result<ProtocolIEField, oxirush_ngap::Error> {
//!     ie::SONConfigurationTransferDL::field(Criticality::Ignore, transfer)
//! }
//! ```
//!
//! ```compile_fail
//! use oxirush_ngap::{Ie, ie, ngap::*};
//!
//! fn downlink(transfer: SONConfigurationTransfer) -> Result<ProtocolIEField, oxirush_ngap::Error> {
//!     ie::SONConfigurationTransfer::field(Criticality::Ignore, transfer)
//! }
//! ```
//!
//! # Extraction macro
//!
//! ## `extract_ngap_ies!` — extract IEs from a decoded NGAP message
//!
//! The macro iterates through `protocol_ies`, matches generated ASN.1 IE IDs,
//! and APER-decodes each matching open type into its concrete generated type.
//! A `req` field is the value, and an `opt` or a `lax` field an `Option` of it:
//!
//! | The IE | A `req` field | An `opt` field | A `lax` field |
//! |--------|---------------|----------------|---------------|
//! | is in the message | the value | `Some` of the value | `Some` of the value |
//! | is not in the message | an error of the kind `MissingIe` | `None` | `None` |
//! | is there and does not decode | an error of the kind `UndecodableIe` | the same error | `None` |
//!
//! Which kind a field is says what the receiver does without the IE:
//!
//! - `req`: it cannot go on. The IE is one that the message always has, or one that
//!   the handling needs.
//! - `opt`: it goes on without an IE that was not sent, and not with one that it
//!   could not read.
//! - `lax`: it goes on as if the IE had not been received, whether it was not sent
//!   or was not read: the field takes the IE if it decodes.
//!
//! A `lax` field is for an IE that the receiver may ignore. TS 38.413 clause 10.3.4.2
//! has a receiver ignore the content of an IE that it does not comprehend and that
//! was received with the criticality `ignore`, and go on as if the IE had not been
//! received. With `notify` it does the same and reports the IE, and with `reject`
//! it rejects the procedure. The macro does not look at the criticality of an IE:
//! the caller reads it in `protocol_ies`, and takes with `lax` what it may ignore.
//!
//! The error is that of the enclosing function, which returns a `Result` whose
//! error is [`Error`](crate::Error) or has a `From` of it, as
//! `Box<dyn std::error::Error>` does: the macro converts the error as `?`
//! converts one. A call with `lax` fields alone has no error, and its function
//! returns what it likes. Without `=> expression`, extraction uses `binding.0`,
//! which unwraps the usual single-field generated newtype.
//!
//! What a field holds of octets is borrowed from the message: a NAS-PDU is a slice of
//! the PDU, and is not copied. A value that outlives the message is made with
//! `into_owned()`, or copied as the example below copies its NAS-PDU.
//!
//! ```
//! use oxirush_ngap::{Error, ErrorKind, Ie, extract_ngap_ies, ie, ngap::*};
//!
//! fn handle(msg: &UplinkNASTransport) -> Result<Vec<u8>, Error> {
//!     extract_ngap_ies!(msg, UplinkNASTransport,
//!         req amf_id: u64 = AMF_UE_NGAP_ID(id),
//!         req nas_pdu: Vec<u8> = NAS_PDU(pdu) => pdu.0.to_vec(),
//!         opt ran_id: u32 = RAN_UE_NGAP_ID(id),
//!     );
//!     let _ = (amf_id, ran_id);
//!     Ok(nas_pdu)
//! }
//!
//! let mut message = UplinkNASTransport { protocol_ies: ProtocolIEContainer(vec![
//!     ie::AMF_UE_NGAP_ID::field(Criticality::Reject, 1u64)?,
//!     ie::NAS_PDU::field(Criticality::Reject, vec![0x7e, 0x00])?,
//! ]) };
//! assert_eq!(handle(&message)?, [0x7e, 0x00]);
//!
//! // Without the NAS-PDU, which a field requires
//! message.protocol_ies.0.pop();
//! assert_eq!(handle(&message).unwrap_err().kind(), ErrorKind::MissingIe);
//! # Ok::<(), Error>(())
//! ```
//!
//! ## `set_ie` — change one IE of a message
//!
//! `set_ie::<T>(value)` of a message or of a transfer makes `value` the value of the IE
//! `T`, in its place and with its criticality, and says whether the message has it. It
//! gives an error for a value that does not encode (`Encode`).
//!
//! When a message has the same IE twice, `extract_ngap_ies!` reads the first and `set_ie`
//! changes it. `check()` of the PDU reports the second as `Deviation::RepeatedIe`, and
//! `ies::<T>()` of the message gives every one.
//!
//! ```
//! use oxirush_ngap::{Ie, ie, ngap::*};
//!
//! let mut message = UplinkNASTransport { protocol_ies: ProtocolIEContainer(vec![
//!     ie::AMF_UE_NGAP_ID::field(Criticality::Reject, 1u64)?,
//! ]) };
//! assert_eq!(message.set_ie::<ie::AMF_UE_NGAP_ID>(42u64), Ok(true));
//! # Ok::<(), oxirush_ngap::Error>(())
//! ```

/// Extract typed NGAP protocol IEs into `req`, `opt` and `lax` fields, from a message or
/// from a transfer of protocol IEs, as `PDUSessionResourceSetupRequestTransfer`.
///
/// | The IE | A `req` field | An `opt` field | A `lax` field |
/// |--------|---------------|----------------|---------------|
/// | is in the message | the value | `Some` of the value | `Some` of the value |
/// | is not in the message | an error of the kind `MissingIe` | `None` | `None` |
/// | is there and does not decode | an error of the kind `UndecodableIe` | the same error | `None` |
///
/// The enclosing function returns the error, converted as `?` converts one. An `opt`
/// field is `None` only when its IE is not in the message. A `lax` field takes the IE
/// if it decodes, and is `None` otherwise: it is for an IE that the receiver may
/// ignore, as TS 38.413 clause 10.3.4.2 has it ignore one that it does not comprehend
/// and that was received with the criticality `ignore`. The macro does not look at
/// the criticality, which the caller reads in `protocol_ies`. A call with `lax`
/// fields alone has no error, and its function need not return a `Result`.
///
/// A character string is checked when it is decoded: a name with a character that a
/// PrintableString does not have, as `_` or `@`, does not decode, where a decoder
/// that does not check its alphabet reads it. Such a name is an error for an `opt`
/// field and `None` for a `lax` one.
///
/// A field has the type of the IE, or the type of the one value that the IE is a name
/// for, as a number for an identifier. After `=>` comes an expression of the value of
/// the IE, which the field is then.
///
/// The IEs are read in one pass over the IEs of the message, which stops once each that
/// a field asks for is found, and only those are decoded. `NGAP_PDU` is taken too,
/// as the message that it holds, without `decode_value` first: the octets of its message
/// are read where they are, and a PDU of another message is an error of the kind
/// `WrongMessage` for a `req` and an `opt` field, as octets that are no container of
/// protocol IEs are an error of the kind `Decode`; `lax` fields are then `None`.
///
/// When the message has the IE twice, the field is the first one, which is the one
/// that `set_ie` changes, and the second is not decoded. `check()` of the PDU
/// reports it as `Deviation::RepeatedIe`, and `ies::<T>()` of the message gives every
/// one.
///
/// ```
/// use oxirush_ngap::{Error, build_ngap, extract_ngap_ies, ngap::*};
///
/// fn released(pdu: &NGAP_PDU) -> Result<(u64, bool, Option<u32>), Error> {
///     let message: UEContextReleaseRequest = pdu.decode_value()?;
///     extract_ngap_ies!(message, UEContextReleaseRequest,
///         // The one value that the IE is a name for
///         req amf: u64 = AMF_UE_NGAP_ID(id),
///         // An expression of the value after `=>`, which the field is
///         req inactive: bool = Cause(cause) =>
///             cause == Cause::radioNetwork(CauseRadioNetwork::user_inactivity),
///         // An IE that the message may not have
///         opt sessions: u32 = PDUSessionResourceListCxtRelReq(list) => list.0.len() as u32,
///     );
///     Ok((amf, inactive, sessions))
/// }
///
/// let pdu = build_ngap!(IGNORE UEContextReleaseRequest,
///     REJECT AMF_UE_NGAP_ID(1u64),
///     REJECT RAN_UE_NGAP_ID(7u32),
///     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
/// )?;
/// assert_eq!(released(&pdu)?, (1, true, None));
/// # Ok::<(), Error>(())
/// ```
///
/// A `lax` field, on a message whose Cause decodes and on one whose Cause does not:
///
/// ```
/// # use std::fmt::Write as _;
/// # let mut printed = String::new();
/// # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
/// use oxirush_ngap::{extract_ngap_ies, ngap::*};
///
/// // The cause of a request, when it has one that is read
/// fn cause(request: &UEContextReleaseRequest<'_>) -> Option<Cause> {
///     extract_ngap_ies!(request, UEContextReleaseRequest,
///         lax cause: Cause = Cause(value) => value,
///     );
///     cause
/// }
///
/// // A UE CONTEXT RELEASE REQUEST, then the same with an octet of its Cause spoiled
/// for octets in [
///     "002a4015000003000a0002002a005500020007000f40020500",
///     "002a4015000003000a0002002a005500020007000f4002e500",
/// ] {
///     let pdu = NGAP_PDU::decode(&hex::decode(octets).unwrap()).unwrap();
///     println!("{:?}", cause(&pdu.decode_value().unwrap()));
/// }
/// # assert_eq!(printed, r##"Some(radioNetwork(user_inactivity))
/// # None
/// # "##);
/// ```
///
/// ```text
/// Some(radioNetwork(user_inactivity))
/// None
/// ```
#[macro_export]
macro_rules! extract_ngap_ies {
    ($($arguments:tt)*) => {
        $crate::aper::__extract_ies!($crate, ngap, $($arguments)*)
    };
}

/// Build a complete `NGAP_PDU` from a direction, procedure, message, and IEs.
///
/// Gives a `Result<NGAP_PDU, Error>`: a value that does not encode into its open type,
/// such as an integer outside its constraint, is an error of the kind `Encode`. A
/// criticality that a call leaves out is the one that the specification assigns: that of
/// the procedure for the PDU, and that of the object set of the message for an IE, which
/// an IE that the set does not have has not: an error of the kind `Argument`.
///
/// ```
/// use oxirush_ngap::{build_ngap, ngap::*};
///
/// // The direction, the procedure and the message, then each IE: the criticalities
/// // are those of the specification, or written before what they are of
/// let pdu = build_ngap!(UEContextReleaseRequest,
///     AMF_UE_NGAP_ID(42u64),
///     RAN_UE_NGAP_ID(7u32),
///     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
/// )?;
/// assert_eq!(hex::encode(pdu.encode()?), "002a4015000003000a0002002a005500020007000f40020500");
/// # Ok::<(), oxirush_ngap::Error>(())
/// ```
#[macro_export]
macro_rules! build_ngap {
    ($($arguments:tt)*) => {
        $crate::aper::__build!($crate, ngap, "build_ngap!", $($arguments)*)
    };
}

#[cfg(test)]
mod tests {
    use crate::ngap::*;
    use crate::{Error, ErrorKind, Ie, ie};

    #[test]
    fn mutating_macro_updates_newtype_payload() -> Result<(), Error> {
        let mut pdu = build_ngap!(IGNORE UEContextReleaseRequest,
            REJECT AMF_UE_NGAP_ID(1u64),
            REJECT RAN_UE_NGAP_ID(7u32),
            IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
        )?;

        let NGAP_PDU::initiatingMessage(message) = &mut pdu else {
            panic!("expected initiating message");
        };
        let request: UEContextReleaseRequest =
            oxirush_aper::decode(&message.value).expect("decode request");
        let mut request = request;

        let changed = request.set_ie::<ie::AMF_UE_NGAP_ID>(42u64);
        assert_eq!(changed, Ok(true));

        let ie = request
            .protocol_ies
            .0
            .iter()
            .find(|ie| ie.id.0 == 10)
            .expect("AMF UE ID");
        let id: AMFUENGAPID = oxirush_aper::decode(&ie.value).expect("decode ID");
        assert_eq!(id.0, 42);
        Ok(())
    }

    /// The identifier of a request, as the macro reads it.
    fn id(request: &UEContextReleaseRequest) -> Result<u64, Error> {
        extract_ngap_ies!(request, UEContextReleaseRequest,
            req id: u64 = AMF_UE_NGAP_ID(id),
        );
        Ok(id)
    }

    #[test]
    fn an_ie_that_is_there_twice_is_read_and_changed_at_the_same_place() -> Result<(), Error> {
        let ids = |request: &UEContextReleaseRequest| -> Vec<Vec<u8>> {
            let ies = request.protocol_ies.0.iter();
            ies.map(|ie| ie.value.to_vec()).collect()
        };
        let entry = |id: u64| ie::AMF_UE_NGAP_ID::field(Criticality::Reject, id);
        let mut request = UEContextReleaseRequest {
            protocol_ies: ProtocolIEContainer(vec![entry(1)?, entry(2)?]),
        };
        // The first one is read, and is the one that is changed.
        assert_eq!(id(&request)?, 1);
        let before = ids(&request);
        let changed = request.set_ie::<ie::AMF_UE_NGAP_ID>(42u64);
        assert_eq!(changed, Ok(true));
        assert_eq!(id(&request)?, 42);
        let after = ids(&request);
        assert_ne!(after[0], before[0]);
        assert_eq!(after[1], before[1]);
        Ok(())
    }

    #[test]
    fn the_first_of_two_is_read_and_a_second_that_does_not_decode_is_not() -> Result<(), Error> {
        // An entry with the identifier of this IE whose octets are not its value,
        // after one that decodes and before it.
        let undecodable = || {
            let octets = crate::Octets::from(vec![0xff]);
            ProtocolIEField {
                id: ProtocolIEID(10),
                criticality: Criticality::Reject,
                value: octets,
            }
        };
        let entry = ie::AMF_UE_NGAP_ID::field(Criticality::Reject, 1u64)?;
        // The first one decodes: the macro and `set_ie` take it, and decode no other.
        let mut request = UEContextReleaseRequest {
            protocol_ies: ProtocolIEContainer(vec![entry.clone(), undecodable()]),
        };
        assert_eq!(id(&request)?, 1);
        let changed = request.set_ie::<ie::AMF_UE_NGAP_ID>(42u64);
        assert_eq!(changed, Ok(true));
        assert_eq!(id(&request)?, 42);
        // The first one does not decode: it is the error of the macro, and `set_ie`
        // replaces it, without reading it.
        let mut request = UEContextReleaseRequest {
            protocol_ies: ProtocolIEContainer(vec![undecodable(), entry.clone()]),
        };
        assert_eq!(id(&request).unwrap_err().kind(), ErrorKind::UndecodableIe);
        assert_eq!(request.set_ie::<ie::AMF_UE_NGAP_ID>(42u64), Ok(true));
        assert_eq!(id(&request)?, 42);
        Ok(())
    }

    #[test]
    fn builder_macros_cover_ie_and_outcome_forms() -> Result<(), Error> {
        let ie = ie::AMF_UE_NGAP_ID::field(Criticality::Reject, 1u64)?;
        assert_eq!(ie.id.0, 10);

        // The direction and the procedure code of a PDU are those of its message.
        let initiating = build_ngap!(REJECT NGSetupRequest,)?;
        assert_eq!(initiating.direction(), crate::Direction::Initiating);
        assert_eq!(initiating.procedure_code(), 21);

        let successful = build_ngap!(REJECT NGSetupResponse,)?;
        assert_eq!(successful.direction(), crate::Direction::Successful);
        assert_eq!(successful.procedure_code(), 21);

        let unsuccessful = build_ngap!(REJECT NGSetupFailure,)?;
        assert_eq!(unsuccessful.direction(), crate::Direction::Unsuccessful);
        assert_eq!(unsuccessful.procedure_code(), 21);
        Ok(())
    }

    /// The IEs whose type is an OCTET STRING (CONTAINING ...) carry the
    /// encoded transfer as octets.
    #[test]
    fn octet_string_containing_ies_have_macros() -> Result<(), Error> {
        let transfer = MBSSessionSetupOrModRequestTransfer {
            protocol_ies: ProtocolIEContainer(vec![]),
        };
        let octets = oxirush_aper::encode(&transfer).expect("encode transfer");
        let ie = ie::MBSSessionSetupRequestTransfer::field(Criticality::Reject, octets.clone())?;
        assert_eq!(ie.id.0, 315);

        let request = BroadcastSessionSetupRequest {
            protocol_ies: ProtocolIEContainer(vec![ie]),
        };
        extract_ngap_ies!(request, BroadcastSessionSetupRequest,
            req extracted: Vec<u8> = MBSSessionSetupRequestTransfer(value) => value.to_vec(),
        );
        assert_eq!(extracted, octets);
        let decoded: MBSSessionSetupOrModRequestTransfer =
            oxirush_aper::decode(&extracted).expect("decode transfer");
        assert_eq!(decoded, transfer);
        Ok(())
    }
}
