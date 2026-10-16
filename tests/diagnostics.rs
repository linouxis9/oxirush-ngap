//! What the receiver of a message that deviates answers it with: the cause and the
//! Criticality Diagnostics of TS 38.413 clause 10, from what `check` reports of the
//! message, and from the error of the macro that extracts its IEs.
//!
//! Clause 10.2 has an IE whose value does not decode as a transfer syntax error.
//! Clause 10.3.4 has the receiver act on an IE that it does not comprehend by the
//! criticality that the IE was received with, and clause 10.3.5 on a missing IE by
//! the criticality that the specification assigns it. The Criticality Diagnostics name
//! each IE with that criticality, its identifier and the type of the error.

use oxirush_ngap::aper::{self, ErrorIe};
use oxirush_ngap::helpers::{global_gnb_id, plmn, s_nssai};
use oxirush_ngap::ngap::*;
use oxirush_ngap::{Deviation, Error, ErrorKind, build_ngap, extract_ngap_ies};

/// The identifiers of two IEs of an NG SETUP REQUEST: SupportedTAList, which is
/// mandatory with the criticality `reject`, and DefaultPagingDRX, which is mandatory
/// with the criticality `ignore`.
const SUPPORTED_TA_LIST: u16 = 102;
const DEFAULT_PAGING_DRX: u16 = 21;

fn areas() -> Result<SupportedTAList, Error> {
    let slices = SliceSupportList(vec![SliceSupportItem::new(s_nssai(1, None), None)]);
    let networks = BroadcastPLMNList(vec![BroadcastPLMNItem::new(
        plmn("208", "93")?,
        slices,
        None,
    )]);
    let area = SupportedTAItem::new(TAC([0, 0, 1]), networks, None);
    Ok(SupportedTAList(vec![area]))
}

/// An NG SETUP REQUEST as a gNB sends it, whose last IE is the DefaultPagingDRX.
fn request() -> Result<Vec<u8>, Error> {
    let node = GlobalRANNodeID::globalGNB_ID(global_gnb_id(plmn("208", "93")?, 1)?);
    build_ngap!(InitiatingMessage, NGSetup, REJECT, NGSetupRequest,
        REJECT GlobalRANNodeID(node),
        IGNORE RANNodeName(RANNodeName("gnb-1".into())),
        REJECT SupportedTAList(areas()?),
        IGNORE DefaultPagingDRX(PagingDRX::v128),
    )?
    .encode()
}

/// The same request without its SupportedTAList.
fn without_areas() -> Result<Vec<u8>, Error> {
    let node = GlobalRANNodeID::globalGNB_ID(global_gnb_id(plmn("208", "93")?, 1)?);
    build_ngap!(InitiatingMessage, NGSetup, REJECT, NGSetupRequest,
        REJECT GlobalRANNodeID(node),
        IGNORE RANNodeName(RANNodeName("gnb-1".into())),
        IGNORE DefaultPagingDRX(PagingDRX::v128),
    )?
    .encode()
}

/// The request with a DefaultPagingDRX that is no value of its type: its one octet
/// says that the value is an extension, and there is none.
fn spoiled() -> Result<Vec<u8>, Error> {
    let mut octets = request()?;
    *octets.last_mut().unwrap() = 0xff;
    Ok(octets)
}

/// What an AMF reads of the request: the first IE that is missing or does not decode
/// is its error.
fn accept(request: &NGSetupRequest) -> Result<usize, Error> {
    extract_ngap_ies!(request, NGSetupRequest,
        req node: GlobalRANNodeID = GlobalRANNodeID(node) => node,
        req areas: SupportedTAList = SupportedTAList(areas) => areas,
        req drx: PagingDRX = DefaultPagingDRX(drx) => drx,
        opt name: RANNodeName = RANNodeName(name) => name,
    );
    let _ = (node, drx, name);
    Ok(areas.0.len())
}

/// The Criticality Diagnostics of an NG SETUP REQUEST, with these items.
fn diagnostics(items: Vec<CriticalityDiagnosticsIEItem>) -> CriticalityDiagnostics {
    CriticalityDiagnostics::new(
        Some(ProcedureCode(21)),
        Some(TriggeringMessage::initiating_message),
        Some(Criticality::reject),
        Some(CriticalityDiagnosticsIEList(items)),
        None,
    )
}

/// The NG SETUP FAILURE that answers a request which deviates: the cause of its first
/// deviation that has one, and the Criticality Diagnostics of its IEs.
fn failure(pdu: &NGAP_PDU) -> Result<NGAP_PDU, Error> {
    let deviations = pdu.check()?;
    let cause = deviations.iter().find_map(Deviation::cause);
    let items = deviations.iter().filter_map(Deviation::diagnostics_item);
    build_ngap!(UnsuccessfulOutcome, NGSetup, REJECT, NGSetupFailure,
        IGNORE Cause(Cause::protocol(cause.unwrap_or(CauseProtocol::unspecified))),
        IGNORE CriticalityDiagnostics(diagnostics(items.collect())),
    )
}

/// The cause and the Criticality Diagnostics of a failure, as its receiver reads them.
fn answered(failure: &NGAP_PDU) -> Result<(Cause, CriticalityDiagnostics), Error> {
    let received = NGAP_PDU::decode(&failure.encode()?)?;
    let message: NGSetupFailure = received.decode_value()?;
    extract_ngap_ies!(message, NGSetupFailure,
        req cause: Cause = Cause(cause) => cause,
        req diagnostics: CriticalityDiagnostics = CriticalityDiagnostics(diagnostics) => diagnostics,
    );
    Ok((cause, diagnostics))
}

#[test]
fn a_request_as_the_specification_has_it_deviates_in_nothing() -> Result<(), Error> {
    let pdu = NGAP_PDU::decode(&request()?)?;
    assert_eq!(pdu.check()?, []);
    assert_eq!(accept(&pdu.decode_value()?)?, 1);
    Ok(())
}

#[test]
fn the_error_of_a_missing_ie_has_the_criticality_that_is_assigned() -> Result<(), Error> {
    let pdu = NGAP_PDU::decode(&without_areas()?)?;
    let error = accept(&pdu.decode_value()?).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::MissingIe);
    let ie = ErrorIe::new(SUPPORTED_TA_LIST, aper::Criticality::Reject, None);
    assert_eq!(error.ie(), Some(ie));

    // The deviation of the error is the one that `check` reports.
    let missing = Deviation::MissingIe {
        id: SUPPORTED_TA_LIST,
        name: "SupportedTAList",
        criticality: Criticality::reject,
    };
    assert_eq!(Deviation::of(&error), Some(missing.clone()));
    assert_eq!(pdu.check()?, std::slice::from_ref(&missing));

    // Clause 10.3.5: rejected, and reported as missing with the assigned criticality.
    assert_eq!(
        missing.cause(),
        Some(CauseProtocol::abstract_syntax_error_reject)
    );
    assert_eq!(missing.type_of_error(), Some(TypeOfError::missing));
    let item = CriticalityDiagnosticsIEItem::new(
        Criticality::reject,
        ProtocolIEID(SUPPORTED_TA_LIST),
        TypeOfError::missing,
        None,
    );
    assert_eq!(missing.diagnostics_item(), Some(item.clone()));
    let cause = Cause::protocol(CauseProtocol::abstract_syntax_error_reject);
    assert_eq!(answered(&failure(&pdu)?)?, (cause, diagnostics(vec![item])));
    Ok(())
}

#[test]
fn the_error_of_an_ie_that_does_not_decode_has_the_criticality_that_was_received()
-> Result<(), Error> {
    let pdu = NGAP_PDU::decode(&spoiled()?)?;
    let error = accept(&pdu.decode_value()?).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::UndecodableIe);
    assert_eq!(
        error.to_string(),
        "the DefaultPagingDRX of this NGSetupRequest does not decode: the octets end before \
         the value does"
    );
    // The fourth IE of the message, which was sent with the criticality `ignore`.
    let ie = ErrorIe::new(DEFAULT_PAGING_DRX, aper::Criticality::Ignore, Some(3));
    assert_eq!(error.ie(), Some(ie));

    // The deviation of the error is the one that `check` reports.
    let undecodable = Deviation::of(&error).unwrap();
    assert!(matches!(
        undecodable,
        Deviation::UndecodableIe {
            id: DEFAULT_PAGING_DRX,
            name: "DefaultPagingDRX",
            position: 3,
            criticality: Criticality::ignore,
            ..
        }
    ));
    assert_eq!(pdu.check()?, std::slice::from_ref(&undecodable));
    assert_eq!(
        undecodable.to_string(),
        "DefaultPagingDRX (IE 21), at position 3, does not decode: the octets end before the \
         value does"
    );

    // Clause 10.2: a transfer syntax error. The failure names the IE.
    assert_eq!(
        undecodable.cause(),
        Some(CauseProtocol::transfer_syntax_error)
    );
    assert_eq!(
        undecodable.type_of_error(),
        Some(TypeOfError::not_understood)
    );
    let item = CriticalityDiagnosticsIEItem::new(
        Criticality::ignore,
        ProtocolIEID(DEFAULT_PAGING_DRX),
        TypeOfError::not_understood,
        None,
    );
    let cause = Cause::protocol(CauseProtocol::transfer_syntax_error);
    assert_eq!(answered(&failure(&pdu)?)?, (cause, diagnostics(vec![item])));
    Ok(())
}

#[test]
fn check_reports_every_ie_that_is_missing_or_does_not_decode() -> Result<(), Error> {
    // Without its SupportedTAList, and with a DefaultPagingDRX that does not decode:
    // the macro stops at the first, and `check` has both.
    let mut octets = without_areas()?;
    *octets.last_mut().unwrap() = 0xff;
    let pdu = NGAP_PDU::decode(&octets)?;
    // The first of its fields that has no value: the areas, which are missing.
    let first = accept(&pdu.decode_value()?).unwrap_err();
    assert_eq!(first.kind(), ErrorKind::MissingIe);

    let deviations = pdu.check()?;
    let ies: Vec<_> = deviations.iter().map(Deviation::ie).collect();
    assert_eq!(
        ies,
        [
            Some((DEFAULT_PAGING_DRX, Criticality::ignore)),
            Some((SUPPORTED_TA_LIST, Criticality::reject)),
        ]
    );
    let types: Vec<_> = deviations.iter().map(Deviation::type_of_error).collect();
    assert_eq!(
        types,
        [
            Some(TypeOfError::not_understood),
            Some(TypeOfError::missing)
        ]
    );
    let items: Vec<_> = (deviations.iter())
        .filter_map(Deviation::diagnostics_item)
        .collect();
    assert_eq!(items.len(), 2);
    let cause = Cause::protocol(CauseProtocol::transfer_syntax_error);
    assert_eq!(answered(&failure(&pdu)?)?, (cause, diagnostics(items)));
    Ok(())
}

/// Clause 10.3.4.2: an IE that the receiver does not comprehend is acted on by the
/// criticality that it was received with, and clause 10.3.6 rejects a message with an
/// IE that is there twice or in a wrong order.
#[test]
fn each_deviation_has_the_cause_of_its_clause() -> Result<(), Error> {
    // An UPLINK NAS TRANSPORT with an IE of no message, received as `notify`, another
    // as `ignore`, and its identifiers in the other order.
    let ie =
        |id: u16, criticality| ProtocolIEField::new(ProtocolIEID(id), criticality, vec![0].into());
    let mut pdu = build_ngap!(InitiatingMessage, UplinkNASTransport, IGNORE, UplinkNASTransport,
        REJECT RAN_UE_NGAP_ID(7u32),
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT NAS_PDU(vec![0x7e, 0x00, 0x5d]),
        IGNORE UserLocationInformation(location()?),
    )?;
    let mut ies = pdu.protocol_ies()?.into_owned();
    ies.0.push(ie(60_000, Criticality::notify));
    ies.0.push(ie(60_001, Criticality::ignore));
    pdu.set_protocol_ies(ies)?;

    let deviations = pdu.check()?;
    let causes: Vec<_> = deviations.iter().map(Deviation::cause).collect();
    assert_eq!(
        causes,
        [
            Some(CauseProtocol::abstract_syntax_error_falsely_constructed_message),
            Some(CauseProtocol::abstract_syntax_error_ignore_and_notify),
            None,
        ],
        "{deviations:?}"
    );
    let items: Vec<_> = (deviations.iter())
        .filter_map(Deviation::diagnostics_item)
        .collect();
    let item = |id: u16, criticality| {
        CriticalityDiagnosticsIEItem::new(
            criticality,
            ProtocolIEID(id),
            TypeOfError::not_understood,
            None,
        )
    };
    assert_eq!(
        items,
        [
            item(60_000, Criticality::notify),
            item(60_001, Criticality::ignore)
        ]
    );
    Ok(())
}

/// TS 38.413 clause 9.3.3.21: an AMF name is a PrintableString (SIZE (1..150, ...)). A
/// name of no character encodes, as a value of a later version of the type, which a
/// sender of this version does not send. Clause 10.3.1 has a receiver handle a value
/// that its version does not define as an IE that it does not comprehend.
#[test]
fn a_value_outside_the_root_of_an_extensible_constraint_is_reported() -> Result<(), Error> {
    // It encodes, and an encoding that is strict refuses it.
    let none = AMFName("".into());
    assert_eq!(aper::encode(&none)?, [0x80, 0x00]);
    let refused = aper::encode_strict(&none).unwrap_err();
    assert_eq!(refused.kind(), ErrorKind::Encode);
    assert_eq!(
        refused.to_string(),
        "a value of AMFName does not encode: a size of 0 is outside the root of its \
         constraint (1..150, ...)"
    );

    // An NG SETUP RESPONSE with that name is built as any other, and `check` says it.
    let response = |name: &'static str| {
        let network = plmn("208", "93")?;
        let served =
            ServedGUAMIItem::new(oxirush_ngap::helpers::guami(network, 1, 1, 0)?, None, None);
        let slices = SliceSupportList(vec![SliceSupportItem::new(s_nssai(1, None), None)]);
        let supported = PLMNSupportItem::new(network, slices, None);
        build_ngap!(SuccessfulOutcome, NGSetup, REJECT, NGSetupResponse,
            REJECT AMFName(AMFName(name.into())),
            REJECT ServedGUAMIList(ServedGUAMIList(vec![served])),
            IGNORE RelativeAMFCapacity(RelativeAMFCapacity(255)),
            REJECT PLMNSupportList(PLMNSupportList(vec![supported])),
        )
    };
    assert_eq!(response("amf-1")?.check()?, []);
    let deviations = response("")?.check()?;
    assert!(matches!(
        deviations[..],
        [Deviation::OutsideRoot {
            id: 1,
            name: "AMFName",
            position: 0,
            criticality: Criticality::reject,
            ..
        }]
    ));
    assert_eq!(
        deviations[0].to_string(),
        "AMFName (IE 1), at position 0, has a value of a later version of its type: a size \
         of 0 is outside the root of its constraint (1..150, ...)"
    );
    assert_eq!(deviations[0].ie(), Some((1, Criticality::reject)));
    assert_eq!(
        deviations[0].type_of_error(),
        Some(TypeOfError::not_understood)
    );
    assert_eq!(
        deviations[0].cause(),
        Some(CauseProtocol::abstract_syntax_error_reject)
    );
    Ok(())
}

/// Where the UE is: an NR cell and its tracking area.
fn location() -> Result<UserLocationInformation<'static>, Error> {
    let network = plmn("208", "93")?;
    let cell = oxirush_ngap::helpers::nr_cgi(network, 1, 1)?;
    let area = oxirush_ngap::helpers::tai(network, [0, 0, 1]);
    let nr = UserLocationInformationNR::new(cell, area, None, None);
    Ok(UserLocationInformation::userLocationInformationNR(nr))
}
