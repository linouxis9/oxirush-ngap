//! The errors of the crate, each where a caller meets it: its kind, which is what a
//! caller matches on, and what its message says.

use oxirush_ngap::ngap::*;
use oxirush_ngap::{
    Error, ErrorKind, Message, build_ngap, build_ngap_ie, extract_ngap_ies, with_ngap_ie_mut,
};
use rasn::types::Any;

/// A UE CONTEXT RELEASE REQUEST with its three IEs, the message of the README.
const RELEASE_REQUEST: &str = "002a4015000003000a0002002a005500020007000f40020500";

fn release_request() -> NGAP_PDU {
    NGAP_PDU::decode(&hex::decode(RELEASE_REQUEST).unwrap()).unwrap()
}

/// What an error has for the one who handles it.
fn source(error: &Error) -> Option<String> {
    std::error::Error::source(error).map(|source| source.to_string())
}

#[test]
fn a_pdu_is_decoded_as_the_message_that_it_holds_and_as_no_other() {
    let pdu = release_request();
    let request: UEContextReleaseRequest = pdu.decode_value().unwrap();
    assert_eq!(request.protocol_ies.0.len(), 3);

    // The octets of the message are those of an NG SETUP REQUEST too, which is a list
    // of IEs as well: nothing but the procedure of the PDU says which message it is.
    let NGAP_PDU::initiatingMessage(held) = &pdu else {
        panic!("an initiating message");
    };
    let mistaken: NGSetupRequest = decode_open_type(&held.value).unwrap();
    assert_eq!(mistaken.protocol_ies.0.len(), 3);

    let error = pdu.decode_value::<NGSetupRequest>().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::WrongMessage);
    assert_eq!(
        error.to_string(),
        "the message of this PDU is UEContextReleaseRequest, not NGSetupRequest: kind() \
         says which message a PDU holds"
    );
    assert_eq!(source(&error), None);
}

#[test]
fn a_message_of_the_same_procedure_in_another_direction_is_another_message() {
    // An NG SETUP FAILURE: the unsuccessful outcome of the procedure 21.
    let failure = NGAP_PDU::decode(&hex::decode("40150008000001000f400188").unwrap()).unwrap();
    assert!(failure.decode_value::<NGSetupFailure>().is_ok());
    for kind in [
        failure.decode_value::<NGSetupRequest>().unwrap_err().kind(),
        failure
            .decode_value::<NGSetupResponse>()
            .unwrap_err()
            .kind(),
    ] {
        assert_eq!(kind, ErrorKind::WrongMessage);
    }
}

#[test]
fn a_pdu_of_a_procedure_that_the_specification_does_not_have_is_no_message() {
    let NGAP_PDU::initiatingMessage(mut held) = release_request() else {
        panic!("an initiating message");
    };
    held.procedure_code = ProcedureCode(200);
    let pdu = NGAP_PDU::initiatingMessage(held);
    let error = pdu.decode_value::<UEContextReleaseRequest>().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::WrongMessage);
    assert_eq!(
        error.to_string(),
        "the message of this PDU is not UEContextReleaseRequest: the specification has no \
         InitiatingMessage of procedure 200"
    );
}

#[test]
fn the_type_of_a_message_says_which_message_it_is() {
    assert_eq!(UEContextReleaseRequest::NAME, "UEContextReleaseRequest");
    assert_eq!(UEContextReleaseRequest::DIRECTION, "InitiatingMessage");
    assert_eq!(UEContextReleaseRequest::PROCEDURE_CODE, 42);
    // The name is the one that ASN.1 gives the message, which is not always that of
    // its procedure.
    assert_eq!(NGSetupFailure::NAME, "NGSetupFailure");
    assert_eq!(NGSetupFailure::DIRECTION, "UnsuccessfulOutcome");
    assert_eq!(NGSetupFailure::PROCEDURE_CODE, 21);
    let pdu = release_request();
    let of = |direction, code| (pdu.direction(), pdu.procedure_code()) == (direction, code);
    assert!(of(
        UEContextReleaseRequest::DIRECTION,
        UEContextReleaseRequest::PROCEDURE_CODE
    ));
}

#[test]
fn octets_that_are_no_pdu_are_a_decode_error() {
    let octets = hex::decode(RELEASE_REQUEST).unwrap();

    // No octet, octets that end early, and an octet after the PDU.
    let none = NGAP_PDU::decode(&[]).unwrap_err();
    assert_eq!(none.kind(), ErrorKind::Decode);
    assert_eq!(
        none.to_string(),
        "these octets do not decode as NGAP-PDU: there are no octets"
    );
    for cut in 1..octets.len() {
        let short = NGAP_PDU::decode(&octets[..cut]).unwrap_err();
        assert_eq!(short.kind(), ErrorKind::Decode, "{cut}");
        let text = short.to_string();
        assert!(
            text.starts_with("these octets do not decode as NGAP-PDU: "),
            "{text}"
        );
        // What rasn says is the source, and is not the message.
        assert!(!text.contains("Codec"), "{text}");
        assert!(source(&short).is_some_and(|source| source.contains("Codec: APER")));
        // And is that error itself, for the one who reads its kind.
        let rasn = std::error::Error::source(&short).unwrap();
        assert!(rasn.downcast_ref::<rasn::error::DecodeError>().is_some());
    }
    let long = NGAP_PDU::decode(&[&octets[..], &[0]].concat()).unwrap_err();
    assert_eq!(long.kind(), ErrorKind::Decode);
    assert_eq!(
        long.to_string(),
        "these octets do not decode as NGAP-PDU: the value is followed by 1 octet"
    );
}

#[test]
fn an_open_type_that_is_not_the_value_asked_for_is_a_decode_error() {
    // An AMF UE NGAP ID is an INTEGER (0..1099511627775): a length of one octet, then
    // the octet.
    let id: AMFUENGAPID = decode_open_type(&Any::new(vec![0x00, 0x2a])).unwrap();
    assert_eq!(id.0, 42);
    let short = decode_open_type::<AMFUENGAPID>(&Any::new(vec![0x40, 0x2a])).unwrap_err();
    assert_eq!(short.kind(), ErrorKind::Decode);
    assert_eq!(
        short.to_string(),
        "these octets do not decode as AMF-UE-NGAP-ID: the octets end before the value does"
    );
    let long = decode_open_type::<AMFUENGAPID>(&Any::new(vec![0x00, 0x2a, 0x00, 0x00]));
    assert_eq!(
        long.unwrap_err().to_string(),
        "these octets do not decode as AMF-UE-NGAP-ID: the value is followed by 2 octets"
    );
}

#[test]
fn a_value_outside_its_constraint_is_an_encode_error() {
    let error = encode_open_type(&AMFUENGAPID(u64::MAX)).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Encode);
    assert_eq!(
        error.to_string(),
        "a value of AMF-UE-NGAP-ID does not encode: 18446744073709551615 is outside its \
         constraint (0..1099511627775)"
    );
    assert!(source(&error).is_some_and(|source| source.contains("Codec: APER")));
}

#[test]
fn an_error_is_the_error_of_a_function_of_the_caller() {
    // With `?`, in a function whose error is this one or holds any error.
    fn code(octets: &[u8]) -> Result<u8, Error> {
        Ok(NGAP_PDU::decode(octets)?.procedure_code())
    }
    fn boxed(octets: &[u8]) -> Result<u8, Box<dyn std::error::Error + Send + Sync>> {
        Ok(NGAP_PDU::decode(octets)?.procedure_code())
    }
    assert_eq!(code(&hex::decode(RELEASE_REQUEST).unwrap()), Ok(42));
    assert_eq!(code(&[]).unwrap_err().kind(), ErrorKind::Decode);
    let error = boxed(&[]).unwrap_err();
    let error = error
        .downcast_ref::<Error>()
        .expect("the error of the crate");
    assert_eq!(error.kind(), ErrorKind::Decode);
}

/// What the macro reads in a request: an identifier that it requires, and one that
/// the request may be without.
fn ids(request: &UEContextReleaseRequest) -> Result<(u64, Option<u32>), Error> {
    extract_ngap_ies!(request, UEContextReleaseRequest,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        opt ran_id: u32 = RAN_UE_NGAP_ID(id),
    );
    Ok((amf_id, ran_id))
}

fn request(ies: Vec<ProtocolIEField>) -> UEContextReleaseRequest {
    UEContextReleaseRequest::new(ProtocolIEContainer(ies))
}

/// The entry of an AMF UE NGAP ID and that of a RAN UE NGAP ID.
fn amf_and_ran() -> (ProtocolIEField, ProtocolIEField) {
    let amf = build_ngap_ie!(UEContextReleaseRequest, REJECT AMF_UE_NGAP_ID(42u64));
    let ran = build_ngap_ie!(UEContextReleaseRequest, REJECT RAN_UE_NGAP_ID(7u32));
    (amf.unwrap(), ran.unwrap())
}

/// An entry with the identifier of an IE, whose octets are not the value of that IE.
fn undecodable(id: u16) -> ProtocolIEField {
    ProtocolIEField::new(id, Criticality::reject, Any::new(vec![0xff]))
}

#[test]
fn a_required_ie_that_is_absent_is_missing_and_an_optional_one_is_none() {
    let (amf, ran) = amf_and_ran();
    assert_eq!(
        ids(&request(vec![amf.clone(), ran.clone()])),
        Ok((42, Some(7)))
    );
    assert_eq!(ids(&request(vec![amf])), Ok((42, None)));

    let missing = ids(&request(vec![ran])).unwrap_err();
    assert_eq!(missing.kind(), ErrorKind::MissingIe);
    assert_eq!(
        missing.to_string(),
        "this UEContextReleaseRequest has no AMF-UE-NGAP-ID, which the field `amf_id` requires"
    );
    assert_eq!(source(&missing), None);
}

#[test]
fn an_ie_that_does_not_decode_is_an_error_for_a_required_and_for_an_optional_field() {
    let (amf, ran) = amf_and_ran();

    // The one that is required.
    let required = ids(&request(vec![undecodable(10), ran])).unwrap_err();
    assert_eq!(required.kind(), ErrorKind::UndecodableIe);
    assert_eq!(
        required.to_string(),
        "the AMF-UE-NGAP-ID of this UEContextReleaseRequest does not decode: the octets end \
         before the value does"
    );
    assert!(source(&required).is_some_and(|source| source.contains("Codec: APER")));

    // The one that may be absent is there: it is not `None`.
    let optional = ids(&request(vec![amf, undecodable(85)])).unwrap_err();
    assert_eq!(optional.kind(), ErrorKind::UndecodableIe);
    let text = optional.to_string();
    assert!(
        text.starts_with("the RAN-UE-NGAP-ID of this UEContextReleaseRequest does not decode: "),
        "{text}"
    );
}

#[test]
fn a_value_outside_its_constraint_is_an_error_of_the_macros_that_build() {
    // An AMF UE NGAP ID is an INTEGER (0..1099511627775), TS 38.413 clause 9.3.3.1, and
    // the macro takes a `u64`.
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(u64::MAX),
        REJECT RAN_UE_NGAP_ID(7u32),
    );
    let error = pdu.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Encode);
    assert_eq!(
        error.to_string(),
        "a value of AMF-UE-NGAP-ID does not encode: 18446744073709551615 is outside its \
         constraint (0..1099511627775)"
    );
    let ie = build_ngap_ie!(UEContextReleaseRequest, REJECT AMF_UE_NGAP_ID(u64::MAX));
    assert_eq!(ie.unwrap_err(), error);

    // The last value of the constraint is built.
    let last = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID((1u64 << 40) - 1),
    );
    assert!(last.is_ok());
}

#[test]
fn changing_an_ie_says_whether_it_did_or_why_it_did_not() {
    let (amf, _) = amf_and_ran();
    let mut message = request(vec![amf]);

    // The IE is changed, another one is not there, and the expression says no.
    let changed = with_ngap_ie_mut!(message, UEContextReleaseRequest, AMF_UE_NGAP_ID(id) = 43u64);
    assert_eq!(changed, Ok(true));
    let absent = with_ngap_ie_mut!(message, UEContextReleaseRequest, RAN_UE_NGAP_ID(id) = 8u32);
    assert_eq!(absent, Ok(false));
    let refused =
        with_ngap_ie_mut!(message, UEContextReleaseRequest, AMF_UE_NGAP_ID(id) => id.0 == 0);
    assert_eq!(refused, Ok(false));
    assert_eq!(ids(&message), Ok((43, None)));

    // A value that does not encode: the message is as it was.
    let before = message.clone();
    let outside = with_ngap_ie_mut!(
        message,
        UEContextReleaseRequest,
        AMF_UE_NGAP_ID(id) = u64::MAX
    );
    assert_eq!(outside.unwrap_err().kind(), ErrorKind::Encode);
    assert_eq!(message, before);

    // An IE that does not decode is not changed either.
    let mut message = request(vec![undecodable(10)]);
    let before = message.clone();
    let unread = with_ngap_ie_mut!(message, UEContextReleaseRequest, AMF_UE_NGAP_ID(id) = 1u64);
    assert_eq!(unread.unwrap_err().kind(), ErrorKind::UndecodableIe);
    assert_eq!(message, before);
}
