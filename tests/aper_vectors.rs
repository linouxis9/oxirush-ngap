//! Independent APER vectors for every procedure outcome in 3GPP TS 38.413 v19.4.0.
use oxirush_ngap::Octets;
use oxirush_ngap::ngap::*;
use std::collections::BTreeSet;

fn value(pdu: &NGAP_PDU) -> &Octets<'static> {
    match pdu {
        NGAP_PDU::initiatingMessage(message) => &message.value,
        NGAP_PDU::successfulOutcome(message) => &message.value,
        NGAP_PDU::unsuccessfulOutcome(message) => &message.value,
        _ => panic!("unexpected NGAP_PDU alternative"),
    }
}

/// Where the octets of a value are cut short: at each octet for a value of the size
/// that a network sends, and for a longer one at its first and its last octets, in its
/// middle, and around the lengths at which a length determinant changes its form.
fn cuts(length: usize) -> Vec<usize> {
    if length <= 2048 {
        return (0..length).collect();
    }
    let forms = [127, 128, 16383, 16384, 32768, 49152, 65536];
    let around = forms.iter().flat_map(|form| form - 2..form + 6);
    let ends = (0..64).chain(length - 64..length).chain([length / 2]);
    let mut cuts: Vec<_> = ends.chain(around).filter(|cut| *cut < length).collect();
    cuts.sort_unstable();
    cuts.dedup();
    cuts
}

/// The message of a PDU as its type `$message`: it is written back as the octets that
/// it was read from, and these octets cut short or with one more are refused. A type
/// that borrows is named without its lifetime, which each use gives it.
macro_rules! check_typed {
    ($message:ident, $pdu:expr, $name:expr) => {{
        let (pdu, name): (&NGAP_PDU, &str) = ($pdu, $name);
        let raw = value(pdu);
        let decoded: $message = pdu
            .decode_value()
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(encode_open_type(&decoded).unwrap(), *raw, "{name}");
        for cut in cuts(raw.len()) {
            assert!(
                decode_open_type::<$message>(&raw[..cut]).is_err(),
                "{name}: prefix {cut}"
            );
        }
        for octet in [0, 0xff] {
            let mut bytes = raw.to_vec();
            bytes.push(octet);
            assert!(
                decode_open_type::<$message>(&bytes).is_err(),
                "{name}: trailing octet"
            );
            let mut extra = pdu.clone();
            match &mut extra {
                NGAP_PDU::initiatingMessage(message) => message.value = bytes.into(),
                NGAP_PDU::successfulOutcome(message) => message.value = bytes.into(),
                NGAP_PDU::unsuccessfulOutcome(message) => message.value = bytes.into(),
                _ => panic!("unexpected NGAP_PDU alternative"),
            }
            assert!(
                extra.decode_value::<$message>().is_err(),
                "{name}: convenience API trailing octet"
            );
        }
    }};
}

macro_rules! message_types {
    ($($message:ident),+ $(,)?) => {
        fn check_message(name: &str, pdu: &NGAP_PDU) {
            match name.replace('-', "_").as_str() {
                $(stringify!($message) => check_typed!($message, pdu, name),)+
                _ => panic!("missing message type: {name}"),
            }
        }
    };
}

message_types!(
    AMFConfigurationUpdate,
    AMFConfigurationUpdateAcknowledge,
    AMFConfigurationUpdateFailure,
    BroadcastSessionModificationRequest,
    BroadcastSessionModificationResponse,
    BroadcastSessionModificationFailure,
    BroadcastSessionReleaseRequest,
    BroadcastSessionReleaseResponse,
    BroadcastSessionSetupRequest,
    BroadcastSessionSetupResponse,
    BroadcastSessionSetupFailure,
    BroadcastSessionTransportRequest,
    BroadcastSessionTransportResponse,
    BroadcastSessionTransportFailure,
    DistributionSetupRequest,
    DistributionSetupResponse,
    DistributionSetupFailure,
    DistributionReleaseRequest,
    DistributionReleaseResponse,
    HandoverCancel,
    HandoverCancelAcknowledge,
    HandoverRequired,
    HandoverCommand,
    HandoverPreparationFailure,
    HandoverRequest,
    HandoverRequestAcknowledge,
    HandoverFailure,
    InitialContextSetupRequest,
    InitialContextSetupResponse,
    InitialContextSetupFailure,
    MTCommunicationHandlingRequest,
    MTCommunicationHandlingResponse,
    MTCommunicationHandlingFailure,
    MulticastSessionActivationRequest,
    MulticastSessionActivationResponse,
    MulticastSessionActivationFailure,
    MulticastSessionDeactivationRequest,
    MulticastSessionDeactivationResponse,
    MulticastSessionUpdateRequest,
    MulticastSessionUpdateResponse,
    MulticastSessionUpdateFailure,
    NGReset,
    NGResetAcknowledge,
    NGSetupRequest,
    NGSetupResponse,
    NGSetupFailure,
    PathSwitchRequest,
    PathSwitchRequestAcknowledge,
    PathSwitchRequestFailure,
    PDUSessionResourceModifyRequest,
    PDUSessionResourceModifyResponse,
    PDUSessionResourceModifyIndication,
    PDUSessionResourceModifyConfirm,
    PDUSessionResourceReleaseCommand,
    PDUSessionResourceReleaseResponse,
    PDUSessionResourceSetupRequest,
    PDUSessionResourceSetupResponse,
    PWSCancelRequest,
    PWSCancelResponse,
    RANConfigurationUpdate,
    RANConfigurationUpdateAcknowledge,
    RANConfigurationUpdateFailure,
    TimingSynchronisationStatusRequest,
    TimingSynchronisationStatusResponse,
    TimingSynchronisationStatusFailure,
    UEContextModificationRequest,
    UEContextModificationResponse,
    UEContextModificationFailure,
    UEContextReleaseCommand,
    UEContextReleaseComplete,
    UEContextResumeRequest,
    UEContextResumeResponse,
    UEContextResumeFailure,
    UEContextSuspendRequest,
    UEContextSuspendResponse,
    UEContextSuspendFailure,
    UERadioCapabilityCheckRequest,
    UERadioCapabilityCheckResponse,
    UERadioCapabilityIDMappingRequest,
    UERadioCapabilityIDMappingResponse,
    WriteReplaceWarningRequest,
    WriteReplaceWarningResponse,
    NGRemovalRequest,
    NGRemovalResponse,
    NGRemovalFailure,
    InventoryRequest,
    InventoryResponse,
    InventoryFailure,
    CommandRequest,
    CommandResponse,
    CommandFailure,
    AIOTSessionReleaseCommand,
    AIOTSessionReleaseComplete,
    AMFCPRelocationIndication,
    AMFStatusIndication,
    BroadcastSessionReleaseRequired,
    CellTrafficTrace,
    ConnectionEstablishmentIndication,
    DeactivateTrace,
    DownlinkNASTransport,
    DownlinkNonUEAssociatedNRPPaTransport,
    DownlinkRANConfigurationTransfer,
    DownlinkRANEarlyStatusTransfer,
    DownlinkRANStatusTransfer,
    DownlinkRIMInformationTransfer,
    DownlinkUEAssociatedNRPPaTransport,
    ErrorIndication,
    HandoverNotify,
    HandoverSuccess,
    InitialUEMessage,
    LocationReport,
    LocationReportingControl,
    LocationReportingFailureIndication,
    MulticastGroupPaging,
    NASNonDeliveryIndication,
    OverloadStart,
    OverloadStop,
    Paging,
    PDUSessionResourceNotify,
    PrivateMessage,
    PWSFailureIndication,
    PWSRestartIndication,
    RANCPRelocationIndication,
    RANPagingRequest,
    RerouteNASRequest,
    RetrieveUEInformation,
    RRCInactiveTransitionReport,
    SecondaryRATDataUsageReport,
    TimingSynchronisationStatusReport,
    TraceFailureIndication,
    TraceStart,
    UEContextReleaseRequest,
    UEInformationTransfer,
    UERadioCapabilityInfoIndication,
    UETNLABindingReleaseRequest,
    UplinkNASTransport,
    UplinkNonUEAssociatedNRPPaTransport,
    UplinkRANConfigurationTransfer,
    UplinkRANEarlyStatusTransfer,
    UplinkRANStatusTransfer,
    UplinkRIMInformationTransfer,
    UplinkUEAssociatedNRPPaTransport,
    InventoryReport,
    AIOTSessionReleaseRequest,
);

/// A vector of the fixtures: the name of its message, its octets, the code of its
/// procedure and the alternative of its PDU. It decodes to that message and encodes to
/// the same octets, and its octets cut short or with one more are refused. Returns the
/// alternative.
fn check_vector(name: &str, fields: &[&str]) -> &'static str {
    assert_eq!(fields.len(), 4, "{name}");
    let message = fields[0].split('/').next().unwrap();
    let wire = hex::decode(fields[1]).unwrap();
    let pdu = NGAP_PDU::decode(&wire).unwrap_or_else(|error| panic!("{name}: {error}"));
    assert_eq!(hex::encode(pdu.encode().unwrap()), fields[1], "{name}");
    assert_eq!(pdu.procedure_code().to_string(), fields[2], "{name}");
    let choice = match &pdu {
        NGAP_PDU::initiatingMessage(_) => "0",
        NGAP_PDU::successfulOutcome(_) => "1",
        NGAP_PDU::unsuccessfulOutcome(_) => "2",
        _ => panic!("unexpected NGAP_PDU alternative"),
    };
    assert_eq!(choice, fields[3], "{name}");
    check_message(message, &pdu);
    for cut in cuts(wire.len()) {
        assert!(
            NGAP_PDU::decode(&wire[..cut]).is_err(),
            "{name}: prefix {cut}"
        );
    }
    for octet in [0, 0xff] {
        let mut extra = wire.clone();
        extra.push(octet);
        assert!(NGAP_PDU::decode(&extra).is_err(), "{name}: trailing octet");
    }
    choice
}

#[test]
fn all_procedure_outcomes_round_trip_and_reject_incomplete_encodings() {
    let mut names = BTreeSet::new();
    let mut choices = BTreeSet::new();
    for line in include_str!("fixtures/messages.tsv").lines() {
        if line.starts_with('#') {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        let name = fields[0];
        assert!(names.insert(name), "duplicate message: {name}");
        choices.insert(check_vector(name, &fields));
    }
    assert_eq!(names.len(), 144);
    assert_eq!(choices.len(), 3);
}

/// `vectors.tsv` has each message with content, and vectors that are made for a length
/// or a size: `tests/fixtures/generate.py` says what each kind holds.
#[test]
fn the_vectors_with_content_round_trip_and_reject_incomplete_encodings() {
    let mut names = BTreeSet::new();
    let mut messages = BTreeSet::new();
    let mut kinds = BTreeSet::new();
    for line in include_str!("fixtures/vectors.tsv").lines() {
        if line.starts_with('#') {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        let name = fields[0];
        assert!(names.insert(name), "duplicate vector: {name}");
        let (message, variant) = name.split_once('/').unwrap();
        messages.insert(message);
        kinds.insert(variant.split('-').next().unwrap());
        check_vector(name, &fields);
    }
    // Each message of each procedure has content in one vector at least.
    assert_eq!(messages.len(), 144);
    let kinds: Vec<_> = kinds.into_iter().collect();
    assert_eq!(kinds, ["bits", "count", "cover", "largest", "length"]);
}

#[test]
fn complete_open_types_require_a_zero_octet_even_for_null() {
    for bytes in [vec![], vec![0xff], vec![0, 0], vec![0, 0xff]] {
        assert!(decode_open_type::<()>(&bytes).is_err());
    }
    decode_open_type::<()>(&[0]).unwrap();
    assert_eq!(encode_open_type(&()).unwrap(), [0]);
    // Fewer than eight unused bits retain the decoder's receive policy.
    assert!(decode_open_type::<bool>(&[0xff]).unwrap());
}
