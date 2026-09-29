//! Independent APER vectors for every procedure outcome in 3GPP TS 38.413 v19.4.0.
use oxirush_ngap::ngap::*;
use rasn::types::Any;
use std::collections::BTreeSet;

fn value(pdu: &NGAP_PDU) -> &Any {
    match pdu {
        NGAP_PDU::initiatingMessage(message) => &message.value,
        NGAP_PDU::successfulOutcome(message) => &message.value,
        NGAP_PDU::unsuccessfulOutcome(message) => &message.value,
        _ => panic!("unexpected NGAP_PDU alternative"),
    }
}

fn check_typed<T: rasn::Decode + rasn::Encode>(pdu: &NGAP_PDU, name: &str) {
    let raw = value(pdu);
    let decoded: T = pdu
        .decode_value()
        .unwrap_or_else(|error| panic!("{name}: {error}"));
    assert_eq!(encode_open_type(&decoded).unwrap(), *raw, "{name}");
    for cut in 0..raw.as_bytes().len() {
        let partial = Any::new(raw.as_bytes()[..cut].to_vec());
        assert!(
            decode_open_type::<T>(&partial).is_err(),
            "{name}: prefix {cut}"
        );
    }
    for octet in [0, 0xff] {
        let mut bytes = raw.as_bytes().to_vec();
        bytes.push(octet);
        assert!(
            decode_open_type::<T>(&Any::new(bytes.clone())).is_err(),
            "{name}: trailing octet"
        );
        let mut extra = pdu.clone();
        match &mut extra {
            NGAP_PDU::initiatingMessage(message) => message.value = Any::new(bytes),
            NGAP_PDU::successfulOutcome(message) => message.value = Any::new(bytes),
            NGAP_PDU::unsuccessfulOutcome(message) => message.value = Any::new(bytes),
            _ => panic!("unexpected NGAP_PDU alternative"),
        }
        assert!(
            extra.decode_value::<T>().is_err(),
            "{name}: convenience API trailing octet"
        );
    }
}

macro_rules! message_types {
    ($($message:ident),+ $(,)?) => {
        fn check_message(name: &str, pdu: &NGAP_PDU) {
            match name.replace('-', "_").as_str() {
                $(stringify!($message) => check_typed::<$message>(pdu, name),)+
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

#[test]
fn all_procedure_outcomes_round_trip_and_reject_incomplete_encodings() {
    let mut names = BTreeSet::new();
    let mut choices = BTreeSet::new();
    for line in include_str!("fixtures/messages.tsv").lines() {
        if line.starts_with('#') {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 4);
        let name = fields[0];
        assert!(names.insert(name), "duplicate message: {name}");
        let wire = hex::decode(fields[1]).unwrap();
        let pdu = NGAP_PDU::decode(&wire).unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(pdu.encode().unwrap(), wire, "{name}");
        assert_eq!(pdu.procedure_code().to_string(), fields[2], "{name}");
        let choice = match &pdu {
            NGAP_PDU::initiatingMessage(_) => "0",
            NGAP_PDU::successfulOutcome(_) => "1",
            NGAP_PDU::unsuccessfulOutcome(_) => "2",
            _ => panic!("unexpected NGAP_PDU alternative"),
        };
        assert_eq!(choice, fields[3], "{name}");
        choices.insert(choice);
        check_message(name, &pdu);
        for cut in 0..wire.len() {
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
    }
    assert_eq!(names.len(), 144);
    assert_eq!(choices.len(), 3);
}

#[test]
fn complete_open_types_require_a_zero_octet_even_for_null() {
    for bytes in [vec![], vec![0xff], vec![0, 0], vec![0, 0xff]] {
        assert!(decode_open_type::<()>(&Any::new(bytes)).is_err());
    }
    decode_open_type::<()>(&Any::new(vec![0])).unwrap();
    assert_eq!(encode_open_type(&()).unwrap().as_bytes(), [0]);
    // Fewer than eight unused bits retain the decoder's receive policy.
    assert!(decode_open_type::<bool>(&Any::new(vec![0xff])).unwrap());
}
