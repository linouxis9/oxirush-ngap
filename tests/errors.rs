//! The errors of the crate, each where a caller meets it: its kind, which is what a
//! caller matches on, and what its message says.

use oxirush_ngap::ngap::*;
use oxirush_ngap::{
    AsnType, Error, ErrorKind, Message, Octets, build_ngap, build_ngap_ie, extract_ngap_ies,
    with_ngap_ie_mut,
};

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
        // The codec is that of the family: the error is whole, and has no source.
        assert_eq!(source(&short), None);
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
    let id: AMFUENGAPID = decode_open_type(&Octets::from(vec![0x00, 0x2a])).unwrap();
    assert_eq!(id.0, 42);
    let short = decode_open_type::<AMFUENGAPID>(&Octets::from(vec![0x40, 0x2a])).unwrap_err();
    assert_eq!(short.kind(), ErrorKind::Decode);
    assert_eq!(
        short.to_string(),
        "these octets do not decode as AMF-UE-NGAP-ID: the octets end before the value does"
    );
    let long = decode_open_type::<AMFUENGAPID>(&Octets::from(vec![0x00, 0x2a, 0x00, 0x00]));
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
    assert_eq!(source(&error), None);
}

#[test]
fn a_member_that_does_not_encode_is_named_as_one_that_does_not_decode_is() {
    // The priority level is an INTEGER (1..15), TS 38.413 clause 9.3.1.19.
    let priority = AllocationAndRetentionPriority::new(
        PriorityLevelARP(16),
        PreEmptionCapability::shall_not_trigger_pre_emption,
        PreEmptionVulnerability::not_pre_emptable,
        None,
    );
    let error = encode_open_type(&priority).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Encode);
    assert_eq!(
        error.to_string(),
        "a value of AllocationAndRetentionPriority does not encode: 16 is outside its \
         constraint (1..15), in its field `priority_level_arp`"
    );
    assert_eq!(error.fields().collect::<Vec<_>>(), ["priority_level_arp"]);

    // In a value that holds it, the fields are those on the way to the member.
    let characteristics = NonDynamic5QIDescriptor::new(FiveQI(9), None, None, None, None);
    let parameters = QosFlowLevelQosParameters::new(
        QosCharacteristics::nonDynamic5QI(characteristics),
        priority,
        None,
        None,
        None,
        None,
    );
    let error = encode_open_type(&parameters).unwrap_err();
    assert_eq!(
        error.to_string(),
        "a value of QosFlowLevelQosParameters does not encode: 16 is outside its constraint \
         (1..15), in its field `allocation_and_retention_priority.priority_level_arp`"
    );

    // A list of no item where the type has one at least, in a member that may be
    // absent: the history of a UE has 1 to 16 cells.
    let cell =
        oxirush_ngap::helpers::nr_cgi(oxirush_ngap::helpers::plmn("208", "93").unwrap(), 2, 1);
    let container = SourceNGRANNodeToTargetNGRANNodeTransparentContainer::new(
        RRCContainer(vec![0x01].into()),
        None,
        None,
        NGRANCGI::nR_CGI(cell.unwrap()),
        None,
        UEHistoryInformation(vec![]),
        None,
    );
    let error = encode_open_type(&container).unwrap_err();
    assert_eq!(
        error.to_string(),
        "a value of SourceNGRANNode-ToTargetNGRANNode-TransparentContainer does not encode: a \
         size of 0 is outside its constraint (1..16), in its field `ue_history_information`"
    );
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
fn amf_and_ran() -> (ProtocolIEField<'static>, ProtocolIEField<'static>) {
    let amf = build_ngap_ie!(UEContextReleaseRequest, REJECT AMF_UE_NGAP_ID(42u64));
    let ran = build_ngap_ie!(UEContextReleaseRequest, REJECT RAN_UE_NGAP_ID(7u32));
    (amf.unwrap(), ran.unwrap())
}

/// An entry with the identifier of an IE, whose octets are not the value of that IE.
fn undecodable(id: u16) -> ProtocolIEField<'static> {
    ProtocolIEField::new(id, Criticality::reject, Octets::from(vec![0xff]))
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
    assert_eq!(source(&required), None);

    // The one that may be absent is there: it is not `None`.
    let optional = ids(&request(vec![amf, undecodable(85)])).unwrap_err();
    assert_eq!(optional.kind(), ErrorKind::UndecodableIe);
    let text = optional.to_string();
    assert!(
        text.starts_with("the RAN-UE-NGAP-ID of this UEContextReleaseRequest does not decode: "),
        "{text}"
    );
}

/// The UE CONTEXT RELEASE REQUEST of the README with one octet of its Cause spoiled:
/// `e5` for `05`, which is the alternative 7 of a CHOICE of six. The IE has the
/// criticality "ignore", so a receiver that does not comprehend it goes on as if it had
/// not been received (TS 38.413 clause 10.3.4.2).
const SPOILED_CAUSE: &str = "002a4015000003000a0002002a005500020007000f4002e500";

fn spoiled_request() -> UEContextReleaseRequest<'static> {
    let pdu = NGAP_PDU::decode(&hex::decode(SPOILED_CAUSE).unwrap()).unwrap();
    let request: UEContextReleaseRequest = pdu.decode_value().unwrap();
    request.into_owned()
}

/// What a receiver reads in a request when it takes the cause if it can.
fn id_and_cause<'a>(
    request: &'a UEContextReleaseRequest<'a>,
) -> Result<(u64, Option<Cause>), Error> {
    extract_ngap_ies!(request, UEContextReleaseRequest,
        req id: u64 = AMF_UE_NGAP_ID(id),
        lax cause: Cause = Cause(cause) => cause,
    );
    Ok((id, cause))
}

/// A call with `lax` fields alone has no error: its function returns no `Result`.
fn cause_if_read<'a>(request: &'a UEContextReleaseRequest<'a>) -> Option<Cause> {
    extract_ngap_ies!(request, UEContextReleaseRequest,
        lax cause: Cause = Cause(cause) => cause,
    );
    cause
}

fn optional_cause<'a>(request: &'a UEContextReleaseRequest<'a>) -> Result<Option<Cause>, Error> {
    extract_ngap_ies!(request, UEContextReleaseRequest,
        opt cause: Cause = Cause(cause) => cause,
    );
    Ok(cause)
}

fn required_cause<'a>(request: &'a UEContextReleaseRequest<'a>) -> Result<Cause, Error> {
    extract_ngap_ies!(request, UEContextReleaseRequest,
        req cause: Cause = Cause(cause) => cause,
    );
    Ok(cause)
}

#[test]
fn a_lax_field_is_the_ie_if_it_decodes_and_none_otherwise() {
    let inactivity = Cause::radioNetwork(CauseRadioNetwork::user_inactivity);

    // The IE is there and decodes.
    let pdu = release_request();
    let received: UEContextReleaseRequest = pdu.decode_value().unwrap();
    assert_eq!(id_and_cause(&received), Ok((42, Some(inactivity.clone()))));
    assert_eq!(cause_if_read(&received), Some(inactivity));

    // It is absent.
    let (amf, _) = amf_and_ran();
    let without = request(vec![amf]);
    assert_eq!(id_and_cause(&without), Ok((42, None)));
    assert_eq!(cause_if_read(&without), None);

    // It is there and does not decode: the message that was received, with one octet
    // of the value of the IE spoiled.
    let spoiled = spoiled_request();
    assert_eq!(spoiled.protocol_ies.0[2].criticality, Criticality::ignore);
    assert_eq!(id_and_cause(&spoiled), Ok((42, None)));
    assert_eq!(cause_if_read(&spoiled), None);

    // The same octets are an error for an `opt` and for a `req` field.
    let optional = optional_cause(&spoiled).unwrap_err();
    let required = required_cause(&spoiled).unwrap_err();
    assert_eq!(optional, required);
    assert_eq!(optional.kind(), ErrorKind::UndecodableIe);
    assert_eq!(
        optional.to_string(),
        "the Cause of this UEContextReleaseRequest does not decode: 7 is no alternative of \
         the CHOICE"
    );
}

#[test]
fn the_last_of_an_ie_that_is_there_twice_decides_a_lax_field() {
    let inactivity = Cause::radioNetwork(CauseRadioNetwork::user_inactivity);
    let pdu = release_request();
    let received: UEContextReleaseRequest = pdu.decode_value().unwrap();
    let read = received.protocol_ies.0[2].clone();
    let unread = spoiled_request().protocol_ies.0[2].clone();

    // The last one does not decode: the field is `None`, though the first one decodes.
    let last_unread = request(vec![read.clone(), unread.clone()]);
    assert_eq!(cause_if_read(&last_unread), None);

    // The last one decodes: the field has it, whatever came before it.
    let last_read = request(vec![unread, read]);
    assert_eq!(cause_if_read(&last_read), Some(inactivity));

    // One that does not decode is an error for an `opt` and for a `req` field,
    // wherever it is.
    for request in [&last_unread, &last_read] {
        let optional = optional_cause(request).unwrap_err();
        assert_eq!(optional.kind(), ErrorKind::UndecodableIe);
        let required = required_cause(request).unwrap_err();
        assert_eq!(required.kind(), ErrorKind::UndecodableIe);
    }
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

#[test]
fn an_argument_that_a_helper_does_not_take_is_an_error_and_no_panic() {
    use oxirush_ngap::helpers::*;
    let network = || plmn("208", "93").unwrap();

    // What panicked: an MCC of two digits, an MNC of one, and a width over the 64 bits
    // of a number.
    let panicked = [
        plmn("20", "93").unwrap_err(),
        plmn("208", "9").unwrap_err(),
        oxirush_ngap::Bits::new(1, 65).unwrap_err(),
    ];
    // What was cut short: an AMF Set ID over 10 bits and an AMF Pointer over 6
    // (TS 38.413 clauses 9.3.3.12 and 9.3.3.19), a gNB ID over the 24 bits that the
    // helpers give it, a cell over the 12 bits that follow them in an NR Cell Identity
    // of 36 (clause 9.3.1.7), and a number over the width of its bit string.
    let cut_short = [
        guami(network(), 1, 2000, 0).unwrap_err(),
        guami(network(), 1, 1, 70).unwrap_err(),
        global_gnb_id(network(), 0x1ff_ffff).unwrap_err(),
        nr_cgi(network(), 0x1ff_ffff, 1).unwrap_err(),
        nr_cgi(network(), 1, 0x1000).unwrap_err(),
        oxirush_ngap::Bits::new(0x100, 8).unwrap_err(),
    ];
    for error in panicked.iter().chain(&cut_short) {
        assert_eq!(error.kind(), ErrorKind::Argument, "{error}");
        assert_eq!(source(error), None);
    }
    assert_eq!(
        cut_short[0].to_string(),
        "the AMF Set ID 2000 does not fit in 10 bits"
    );

    // A TAC is three octets (clause 9.3.3.10), which the type of the argument says.
    let area = tai(network(), [0, 0, 1]);
    assert_eq!(area.tac.0[..], [0, 0, 1]);
}

/// The errors of the `inspect` feature: a test for each kind, as a caller matches on it.
#[cfg(feature = "inspect")]
mod inspect_errors {
    use super::release_request;
    use oxirush_ngap::Octets;
    use oxirush_ngap::ngap::*;
    use oxirush_ngap::{Error, ErrorKind, inspect};
    use serde_json::{Value, json};

    /// The view of the UE CONTEXT RELEASE REQUEST.
    fn view() -> Value {
        release_request().to_view().unwrap()
    }

    /// The kind of the error that a call returned.
    fn kind<T: std::fmt::Debug>(result: Result<T, Error>) -> ErrorKind {
        result.unwrap_err().kind()
    }

    /// The PDU of a tree: a release request with these IEs.
    fn written(ies: Value) -> Result<NGAP_PDU, Error> {
        NGAP_PDU::from_tree(&json!({"message": "UEContextReleaseRequest", "ies": ies}))
    }

    /// A release request whose procedure code is one that the specification does not have.
    fn of_no_procedure() -> NGAP_PDU {
        let NGAP_PDU::initiatingMessage(mut held) = release_request() else {
            panic!("an initiating message");
        };
        held.procedure_code = ProcedureCode(200);
        NGAP_PDU::initiatingMessage(held)
    }

    #[test]
    fn a_message_that_the_protocol_does_not_have_is_unknown() {
        // A name that is no message, for a path and in a tree.
        let checked = inspect::check_path("NoSuchMessage", "/ngap/Cause");
        assert_eq!(kind(checked), ErrorKind::UnknownMessage);
        let names = inspect::enumerated_at("NoSuchMessage", "/ngap/Cause");
        assert_eq!(kind(names), ErrorKind::UnknownMessage);
        let tree = json!({"message": "NoSuchMessage", "ies": []});
        let error = NGAP_PDU::from_tree(&tree).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnknownMessage);
        assert_eq!(
            error.to_string(),
            "\"NoSuchMessage\" is not a message of NGAP"
        );

        // A procedure that the specification does not have has no tree, which names
        // its message. It has a view, of the IEs of its message.
        let error = of_no_procedure().to_tree().unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnknownMessage);
        assert_eq!(
            error.to_string(),
            "NGAP has no InitiatingMessage of procedure 200"
        );
        assert!(of_no_procedure().to_view().is_ok());
    }

    #[test]
    fn an_ie_that_the_protocol_does_not_have_is_unknown() {
        // A name that is no IE: after the root of a path, in a tree, and as the `id` of
        // an entry that an edit writes.
        let mut view = view();
        let error = inspect::select(&view, "/ngap/NoSuchIe/value").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnknownIe);
        assert_eq!(
            error.to_string(),
            "\"NoSuchIe\" is not an IE of NGAP, at /ngap/NoSuchIe/value"
        );
        assert_eq!(
            kind(written(json!([{"NoSuchIe": 1}]))),
            ErrorKind::UnknownIe
        );
        let entry = json!({"id": "NoSuchIe", "criticality": "ignore", "value": 1});
        assert_eq!(
            kind(inspect::insert(&mut view, "/ngap/-", entry)),
            ErrorKind::UnknownIe
        );

        // An identifier that has no type is written by its octets: its value has no
        // type to be encoded as.
        let error = written(json!([{"60000": 1, "criticality": "ignore"}])).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnknownIe);
        assert_eq!(
            error.to_string(),
            "NGAP IE 60000 is unknown: give its octets as _raw_value, without value"
        );
        let octets = json!([{"60000": {"octets": "c0ffee"}, "criticality": "ignore"}]);
        assert!(written(octets).is_ok());
    }

    #[test]
    fn a_path_that_is_not_well_formed_is_a_path_error() {
        let view = view();
        // No `/` before the first segment, `~` before what is no 0 and no 1, and
        // `@id=` before what is no identifier of an IE.
        for path in [
            "ngap/Cause",
            "/ngap/Cause/~2",
            "/ngap/@id=x",
            "/ngap/@id=70000",
        ] {
            assert_eq!(
                kind(inspect::select(&view, path)),
                ErrorKind::Path,
                "{path}"
            );
            let checked = inspect::check_path("UEContextReleaseRequest", path);
            assert_eq!(kind(checked), ErrorKind::Path, "{path}");
        }
        let error = inspect::select(&view, "ngap/Cause").unwrap_err();
        assert_eq!(
            error.to_string(),
            "\"ngap/Cause\" is no path: a path starts with \"/\", at ngap/Cause"
        );
        // The empty path is the view itself, as that of a JSON pointer is the document.
        assert_eq!(inspect::select(&view, ""), Ok(vec![&view]));
    }

    #[test]
    fn a_name_that_the_type_at_its_place_cannot_have_is_no_member() {
        let mut view = view();
        // A member that a CHOICE does not have, a member of a value that has none, the
        // identifier of an IE where there is no list, and a name where a list goes by
        // position.
        let error = inspect::select(&view, "/ngap/Cause/radioNetwrok").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::NoSuchMember);
        let text = error.to_string();
        assert!(
            text.starts_with("\"radioNetwrok\" is not a member of Cause, which has radioNetwork, "),
            "{text}"
        );
        for path in [
            "/ngap/Cause/radioNetwork/deeper",
            "/ngap/Cause/@id=1",
            "/message/protocolIEs/first",
        ] {
            assert_eq!(
                kind(inspect::select(&view, path)),
                ErrorKind::NoSuchMember,
                "{path}"
            );
        }
        // An edit at such a path, and the view is as it was.
        let before = view.clone();
        let edit = inspect::set(&mut view, "/ngap/Cause/radioNetwrok", json!(1));
        assert_eq!(kind(edit), ErrorKind::NoSuchMember);
        assert_eq!(view, before);
        // An IE that the object set of the message does not have, without a view.
        let checked = inspect::check_path("UEContextReleaseRequest", "/ngap/GUAMI");
        assert_eq!(kind(checked), ErrorKind::NoSuchMember);
        // A member that the type of a written value does not have.
        let error = NGAP_PDU::from_tree(&json!({"message": "InitialContextSetupRequest", "ies": [
            {"AllowedNSSAI": [{"s-NSSAI": {"sST": 1, "other": 2}}]},
        ]}))
        .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::NoSuchMember);
        assert_eq!(
            error.to_string(),
            "this is not a value of AllowedNSSAI, at 0/s-NSSAI: \"other\" is not a member that its type has"
        );
    }

    #[test]
    fn an_edit_whose_path_selects_nothing_is_an_error() {
        let mut view = view();
        let before = view.clone();
        // An IE that the message could have and does not have, and a position after
        // its last IE: a selection has nothing there, and an edit says so.
        for path in ["/ngap/PDUSessionResourceListCxtRelReq/value", "/ngap/7"] {
            assert_eq!(inspect::select(&view, path), Ok(vec![]), "{path}");
            let edit = inspect::set(&mut view, path, json!(1));
            assert_eq!(kind(edit), ErrorKind::NothingSelected, "{path}");
        }
        let removed = inspect::remove(&mut view, "/ngap/PDUSessionResourceListCxtRelReq");
        assert_eq!(kind(removed), ErrorKind::NothingSelected);
        assert_eq!(view, before);
    }

    #[test]
    fn octets_that_carry_a_type_are_not_opened_until_they_are() {
        let name = "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer";
        let required = json!({"message": "HandoverRequired", "ies": [
            {"SourceToTarget-TransparentContainer": {name: {
                "rRCContainer": "0102",
                "targetCell-ID": {"nR-CGI": {"pLMNIdentity": "208-93", "nRCellIdentity": 4660}},
                "uEHistoryInformation": [{"lastVisitedCellInformation": {"eUTRANCell": "00"}}],
            }}},
        ]});
        let pdu = NGAP_PDU::from_tree(&required).unwrap();
        let mut view = pdu.to_view().unwrap();
        let cell = format!(
            "/ngap/SourceToTarget-TransparentContainer/{name}/targetCell-ID/nR-CGI/nRCellIdentity"
        );

        let error = inspect::select(&view, &cell).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::NotOpened);
        let edit = inspect::set(&mut view, &cell, json!(7));
        assert_eq!(kind(edit), ErrorKind::NotOpened);

        // The kind says what to do about it.
        inspect::open(&mut view, &cell).unwrap();
        assert_eq!(inspect::select(&view, &cell), Ok(vec![&json!(4660)]));
    }

    #[test]
    fn a_path_into_what_did_not_decode_says_so() {
        // An IE whose identifier has no type keeps its octets in a view.
        let unknown = json!([{"65000": {"octets": "c0ffee"}, "criticality": "ignore"}]);
        let view = written(unknown).unwrap().to_view().unwrap();
        assert_eq!(
            inspect::select(&view, "/ngap/@id=65000/octets"),
            Ok(vec![&json!("C0FFEE")])
        );
        let error = inspect::select(&view, "/ngap/@id=65000/value/member").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UndecodableIe);
        assert_eq!(
            error.to_string(),
            "the IE did not decode, at /ngap/@id=65000/value/member"
        );
    }

    #[test]
    fn a_value_that_is_not_of_its_type_is_a_value_error() {
        // A string where the type has a number, a name that the ENUMERATED does not
        // have, a number outside the constraint of its type, and octets that are no
        // hexadecimal.
        let string = written(json!([{"AMF-UE-NGAP-ID": "forty-two"}])).unwrap_err();
        let name = written(json!([{"Cause": {"radioNetwork": "no-such-cause"}}])).unwrap_err();
        let outside = written(json!([{"AMF-UE-NGAP-ID": 1u64 << 40}])).unwrap_err();
        let octets = written(json!([{"AMF-UE-NGAP-ID": {"octets": "zz"}}])).unwrap_err();
        for error in [&string, &name, &outside, &octets] {
            assert_eq!(error.kind(), ErrorKind::Value, "{error}");
            assert!(!error.to_string().contains("Codec"), "{error}");
        }
        assert_eq!(
            string.to_string(),
            "this is not a value of AMF-UE-NGAP-ID: a number is expected, and \"forty-two\" is written"
        );
        assert_eq!(
            name.to_string(),
            "this is not a value of Cause, at radioNetwork: \"no-such-cause\" is no value of the \
             ENUMERATED"
        );
        assert_eq!(
            outside.to_string(),
            "this is not a value of AMF-UE-NGAP-ID: 1099511627776 is outside its constraint (0..1099511627775)"
        );
        assert_eq!(
            octets.to_string(),
            "this is not a value of UEContextReleaseRequest, at protocolIEs/0/value: octets in \
             hexadecimal, two digits for each, are expected, and \"zz\" is written"
        );

        // A value of a member, by the names that ASN.1 gives the members on the way:
        // a PLMN identity that is none, and a number where the type has a CHOICE.
        let setup = |plmn: Value, gnb: Value| {
            NGAP_PDU::from_tree(
                &json!({"message": "NGSetupRequest", "ies": [{"GlobalRANNodeID": {"globalGNB-ID": {
                    "pLMNIdentity": plmn,
                    "gNB-ID": gnb,
                }}}]}),
            )
        };
        assert!(
            setup(
                json!("208-93"),
                json!({"gNB-ID": {"value": "000004", "length": 22}})
            )
            .is_ok()
        );
        let plmn = setup(
            json!("20-93"),
            json!({"gNB-ID": {"value": "000004", "length": 22}}),
        )
        .unwrap_err();
        assert_eq!(plmn.kind(), ErrorKind::Value);
        assert_eq!(
            plmn.to_string(),
            "20-93: a PLMN identity is MCC-MNC, as 208-93"
        );
        // Without a hyphen it is its three octets: what is neither says both forms.
        for neither in ["20893", "2089", "208 93", "20893g"] {
            let plmn = setup(
                json!(neither),
                json!({"gNB-ID": {"value": "000004", "length": 22}}),
            )
            .unwrap_err();
            assert_eq!(plmn.kind(), ErrorKind::Value);
            assert_eq!(
                plmn.to_string(),
                format!(
                    "{neither}: a PLMN identity is MCC-MNC, as 208-93, or its three octets in \
                     hexadecimal"
                )
            );
        }
        assert!(
            setup(
                json!("02F839"),
                json!({"gNB-ID": {"value": "000004", "length": 22}})
            )
            .is_ok()
        );
        let member = setup(json!("208-93"), json!(5)).unwrap_err();
        assert_eq!(member.kind(), ErrorKind::Value);
        assert_eq!(
            member.to_string(),
            "this is not a value of GlobalRANNodeID, at globalGNB-ID/gNB-ID: an object with one \
             member is expected, the name of an alternative with its value, and 5 is written"
        );
        assert!(std::error::Error::source(&member).is_none());
    }

    #[test]
    fn json_that_is_no_tree_and_no_view_is_a_form_error() {
        // A tree is an object with the name of a message and its IEs, each one member:
        // an IE that the message does not have says its criticality.
        let message = "UEContextReleaseRequest";
        for tree in [
            json!([]),
            json!({"ies": []}),
            json!({"message": message}),
            json!({"message": message, "ies": [], "other": 1}),
            json!({"message": message, "ies": [{"a": 1, "b": 2}]}),
            json!({"message": message, "ies": [{"RANNodeName": "gnb-1"}]}),
        ] {
            let error = NGAP_PDU::from_tree(&tree).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Form, "{tree}: {error}");
        }

        // A view is an object with a procedure code, a direction, a criticality and a
        // message.
        let view = view();
        let without = |member: &str| {
            let mut view = view.clone();
            view.as_object_mut().unwrap().remove(member);
            view
        };
        let with = |member: &str, value: Value| {
            let mut view = view.clone();
            view[member] = value;
            view
        };
        for view in [
            json!(1),
            without("message"),
            without("direction"),
            without("procedure_code"),
            with("direction", json!("Sideways")),
            with("procedure_code", json!(256)),
            with("other", json!(1)),
        ] {
            let error = NGAP_PDU::from_view(&view).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Form, "{error}");
            // The PDU that the view is of refuses it as any PDU does.
            let error = release_request().with_view(&view).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Form, "{error}");
        }

        // The view of another message is not one of this PDU: another procedure code,
        // another direction, and a procedure that the specification does not have.
        let pdu = release_request();
        for (member, value, other) in [
            ("procedure_code", json!(21), "NGSetupRequest"),
            (
                "direction",
                json!("SuccessfulOutcome"),
                "the SuccessfulOutcome of procedure 42",
            ),
            (
                "procedure_code",
                json!(200),
                "the InitiatingMessage of procedure 200",
            ),
        ] {
            let error = pdu.with_view(&with(member, value)).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Form, "{error}");
            assert_eq!(
                error.to_string(),
                format!(
                    "this view is of {other}, and the PDU is UEContextReleaseRequest: with_view \
                     takes a view of the PDU itself, and from_view any view"
                )
            );
        }

        // An IE is written with its value or with its octets.
        let both = json!({"id": "Cause", "criticality": "ignore", "value": 1, "octets": "00"});
        let insert = inspect::insert(&mut view.clone(), "/ngap/-", both);
        assert_eq!(kind(insert), ErrorKind::Form);
    }

    #[test]
    fn an_edit_that_is_refused_is_an_edit_error() {
        let mut view = view();
        // A member that its type always has, the value of an IE and the alternative of
        // a CHOICE are not taken out, and nothing is inserted where there is no list.
        for path in [
            "/ngap/Cause/criticality",
            "/ngap/Cause/value",
            "/ngap/Cause/radioNetwork",
        ] {
            let removed = inspect::remove(&mut view, path);
            assert_eq!(kind(removed), ErrorKind::Edit, "{path}");
        }
        let inserted = inspect::insert(&mut view, "/ngap/Cause/criticality", json!("reject"));
        assert_eq!(kind(inserted), ErrorKind::Edit);

        // The octets of a value that an edit changed are no longer its octets.
        inspect::set(&mut view, "/ngap/Cause/nas", json!("normal-release")).unwrap();
        let octets = inspect::select(&view, "/ngap/Cause/octets");
        assert_eq!(kind(octets), ErrorKind::Edit);
    }

    #[test]
    fn what_is_over_the_limits_of_the_inspection_is_a_limit_error() {
        let view = view();
        // A path of more than 64 segments, and one of more than 4096 octets.
        let long = "/0".repeat(65);
        assert_eq!(kind(inspect::select(&view, &long)), ErrorKind::Limit);
        let wide = format!("/{}", "a".repeat(4096));
        assert_eq!(kind(inspect::select(&view, &wide)), ErrorKind::Limit);

        // A tree that is nested deeper than a tree is read.
        let mut deep = json!(1);
        for _ in 0..300 {
            deep = json!([deep]);
        }
        assert_eq!(
            kind(written(json!([{"AMF-UE-NGAP-ID": deep}]))),
            ErrorKind::Limit
        );

        // A private message has no list of IEs to show as a tree: its view has it.
        let private =
            NGAP_PDU::decode(&hex::decode("001f400a000000000000000211e8").unwrap()).unwrap();
        let error = private.to_tree().unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Limit);
        assert!(private.to_view().is_ok());
    }

    #[test]
    fn a_message_that_does_not_decode_is_a_decode_error() {
        // A PDU whose octets are not the message of its procedure.
        let NGAP_PDU::initiatingMessage(mut held) = release_request() else {
            panic!("an initiating message");
        };
        held.value = Octets::from(vec![0xff]);
        let pdu = NGAP_PDU::initiatingMessage(held);
        let error = pdu.to_view().unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Decode);
        assert_eq!(kind(pdu.to_tree()), ErrorKind::Decode);

        // The octets that a view was received as, replaced by others, and edited.
        let mut view = view();
        view["_raw_message"] = json!("ff");
        inspect::set(&mut view, "/ngap/Cause/nas", json!("normal-release")).unwrap();
        assert_eq!(kind(NGAP_PDU::from_view(&view)), ErrorKind::Decode);
    }
}
