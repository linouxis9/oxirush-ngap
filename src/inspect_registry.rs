// Auto-generated from ASN.1 by the generator of oxirush-aper; do not edit.
use crate::inspect::{Form, Typed, carried, transfers};
use crate::inspect_paths::types;
pub(crate) use crate::registry::{
    IE_NAMES, IE_TYPES, ITEM_CRITICALITIES, MESSAGE_IE_CRITICALITIES, MESSAGES,
    PROCEDURE_CRITICALITIES, ie, ie_contents,
};
pub(crate) const PROTOCOL: &str = "NGAP";
pub(crate) const ROOT: &str = "ngap";
transfers! {
    "handoverCommandTransfer" crate::ngap::HandoverCommandTransfer;
    "handoverPreparationUnsuccessfulTransfer" crate::ngap::HandoverPreparationUnsuccessfulTransfer;
    "handoverRequestAcknowledgeTransfer" crate::ngap::HandoverRequestAcknowledgeTransfer;
    "handoverRequestTransfer" crate::ngap::PDUSessionResourceSetupRequestTransfer;
    "handoverRequiredTransfer" crate::ngap::HandoverRequiredTransfer;
    "handoverResourceAllocationUnsuccessfulTransfer" crate::ngap::HandoverResourceAllocationUnsuccessfulTransfer;
    "pDUSessionResourceModifyConfirmTransfer" crate::ngap::PDUSessionResourceModifyConfirmTransfer;
    "pDUSessionResourceModifyIndicationTransfer" crate::ngap::PDUSessionResourceModifyIndicationTransfer;
    "pDUSessionResourceModifyIndicationUnsuccessfulTransfer" crate::ngap::PDUSessionResourceModifyIndicationUnsuccessfulTransfer;
    "pDUSessionResourceModifyRequestTransfer" crate::ngap::PDUSessionResourceModifyRequestTransfer;
    "pDUSessionResourceModifyResponseTransfer" crate::ngap::PDUSessionResourceModifyResponseTransfer;
    "pDUSessionResourceModifyUnsuccessfulTransfer" crate::ngap::PDUSessionResourceModifyUnsuccessfulTransfer;
    "pDUSessionResourceNotifyReleasedTransfer" crate::ngap::PDUSessionResourceNotifyReleasedTransfer;
    "pDUSessionResourceNotifyTransfer" crate::ngap::PDUSessionResourceNotifyTransfer;
    "pDUSessionResourceReleaseCommandTransfer" crate::ngap::PDUSessionResourceReleaseCommandTransfer;
    "pDUSessionResourceReleaseResponseTransfer" crate::ngap::PDUSessionResourceReleaseResponseTransfer;
    "pDUSessionResourceSetupRequestTransfer" crate::ngap::PDUSessionResourceSetupRequestTransfer;
    "pDUSessionResourceSetupResponseTransfer" crate::ngap::PDUSessionResourceSetupResponseTransfer;
    "pDUSessionResourceSetupUnsuccessfulTransfer" crate::ngap::PDUSessionResourceSetupUnsuccessfulTransfer;
    "pathSwitchRequestAcknowledgeTransfer" crate::ngap::PathSwitchRequestAcknowledgeTransfer;
    "pathSwitchRequestSetupFailedTransfer" crate::ngap::PathSwitchRequestSetupFailedTransfer;
    "pathSwitchRequestTransfer" crate::ngap::PathSwitchRequestTransfer;
    "pathSwitchRequestUnsuccessfulTransfer" crate::ngap::PathSwitchRequestUnsuccessfulTransfer;
    "secondaryRATDataUsageReportTransfer" crate::ngap::SecondaryRATDataUsageReportTransfer;
    "uEContextResumeRequestTransfer" crate::ngap::UEContextResumeRequestTransfer;
    "uEContextResumeResponseTransfer" crate::ngap::UEContextResumeResponseTransfer;
    "uEContextSuspendRequestTransfer" crate::ngap::UEContextSuspendRequestTransfer;
}
carried! {
    101 "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer" crate::ngap::SourceNGRANNodeToTargetNGRANNodeTransparentContainer;
    106 "TargetNGRANNode-ToSourceNGRANNode-TransparentContainer" crate::ngap::TargetNGRANNodeToSourceNGRANNodeTransparentContainer;
    262 "TargetNGRANNode-ToSourceNGRANNode-FailureTransparentContainer" crate::ngap::TargetNGRANNodeToSourceNGRANNodeFailureTransparentContainer;
}
pub(crate) fn type_form(name: &str) -> Option<Form> {
    Some(match name {
        // AMFPointer
        "AMFPointer" => Form::Number(6),
        // AMFRegionID
        "AMFRegionID" => Form::Number(8),
        // AMFSetID
        "AMFSetID" => Form::Number(10),
        // EPS-TAC
        "EPSTAC" => Form::Number(16),
        // EUTRACellIdentity
        "EUTRACellIdentity" => Form::Number(28),
        // FiveG-TMSI
        "FiveGTMSI" => Form::Number(32),
        // GTP-TEID
        "GTPTEID" => Form::Number(32),
        // LAC
        "LAC" => Form::Number(16),
        // NRCellIdentity
        "NRCellIdentity" => Form::Number(36),
        // PLMNIdentity
        "PLMNIdentity" => Form::Plmn,
        // PortNumber
        "PortNumber" => Form::Number(16),
        // SST
        "SST" => Form::Number(8),
        // TAC
        "TAC" => Form::Number(24),
        // TransportLayerAddress
        "TransportLayerAddress" => Form::Address,
        // UL-NAS-Count
        "ULNASCount" => Form::Number(5),
        _ => return None,
    })
}
/// The names of the values of each ENUMERATED, by the name of its type in the bindings, in the order of the names of the types.
pub(crate) const ENUMERATIONS: &[(&str, &[&str])] = &[
    (
        "A2XPC5QoSFlowItemA2XRange",
        &[
            "m50", "m80", "m180", "m200", "m350", "m400", "m500", "m700", "m1000",
        ],
    ),
    ("AIoTFollowonCommandIndication", &["true"]),
    ("AIoTSupport", &["aIoT-only", "aIoT-and-NRUu"]),
    ("AUN3DeviceAccessInfoAUN3DeviceAccess", &["true"]),
    (
        "AUN3DeviceAccessInfoExistingNGConnectionwithSameLocation",
        &["true", "false"],
    ),
    ("ActivationStatus", &["active", "not-active"]),
    ("AdditionalQosFlowInformation", &["more-likely"]),
    ("AerialUEFlightInformationOngoingReportingFailed", &["true"]),
    (
        "AerialUEReportingPeriodicity",
        &[
            "ms120", "ms240", "ms480", "ms640", "ms1024", "ms2048", "ms5120", "ms10240", "ms20480",
            "ms40960", "min1", "min6", "min12", "min30",
        ],
    ),
    (
        "AerialUEsubscriptionInformation",
        &["allowed", "not-allowed"],
    ),
    (
        "AllowedPNINPNItemPNINPNRestricted",
        &["restricted", "not-restricted"],
    ),
    (
        "AssociatedQosFlowItemQosFlowMappingIndication",
        &["ul", "dl"],
    ),
    ("AuthenticatedIndication", &["true"]),
    (
        "AvailableRANVisibleQoEMetricsApplicationLayerBufferLevelList",
        &["true"],
    ),
    (
        "AvailableRANVisibleQoEMetricsPlayoutDelayForMediaStartup",
        &["true"],
    ),
    ("BeamMeasurementsReportQuantityRSRP", &["true", "false"]),
    ("BeamMeasurementsReportQuantityRSRQ", &["true", "false"]),
    ("BeamMeasurementsReportQuantitySINR", &["true", "false"]),
    ("BluetoothMeasConfig", &["setup"]),
    ("BluetoothMeasurementConfigurationBtRssi", &["true"]),
    ("CEmodeBSupportIndicator", &["supported"]),
    ("CEmodeBrestricted", &["restricted", "not-restricted"]),
    ("CNMTCommunicationHandling", &["supported"]),
    (
        "CNTypeRestrictionsForEquivalentItemCnType",
        &["epc-forbidden", "fiveGC-forbidden"],
    ),
    ("CNTypeRestrictionsForServing", &["epc-forbidden"]),
    ("CancelAllWarningMessages", &["true"]),
    (
        "CauseMisc",
        &[
            "control-processing-overload",
            "not-enough-user-plane-processing-resources",
            "hardware-failure",
            "om-intervention",
            "unknown-PLMN-or-SNPN",
            "unspecified",
        ],
    ),
    (
        "CauseNas",
        &[
            "normal-release",
            "authentication-failure",
            "deregister",
            "unspecified",
            "uE-not-in-PLMN-serving-area",
            "mobile-IAB-not-authorized",
            "iAB-not-authorized",
            "no-aiot-NAS-response",
        ],
    ),
    (
        "CauseProtocol",
        &[
            "transfer-syntax-error",
            "abstract-syntax-error-reject",
            "abstract-syntax-error-ignore-and-notify",
            "message-not-compatible-with-receiver-state",
            "semantic-error",
            "abstract-syntax-error-falsely-constructed-message",
            "unspecified",
        ],
    ),
    (
        "CauseRadioNetwork",
        &[
            "unspecified",
            "txnrelocoverall-expiry",
            "successful-handover",
            "release-due-to-ngran-generated-reason",
            "release-due-to-5gc-generated-reason",
            "handover-cancelled",
            "partial-handover",
            "ho-failure-in-target-5GC-ngran-node-or-target-system",
            "ho-target-not-allowed",
            "tngrelocoverall-expiry",
            "tngrelocprep-expiry",
            "cell-not-available",
            "unknown-targetID",
            "no-radio-resources-available-in-target-cell",
            "unknown-local-UE-NGAP-ID",
            "inconsistent-remote-UE-NGAP-ID",
            "handover-desirable-for-radio-reason",
            "time-critical-handover",
            "resource-optimisation-handover",
            "reduce-load-in-serving-cell",
            "user-inactivity",
            "radio-connection-with-ue-lost",
            "radio-resources-not-available",
            "invalid-qos-combination",
            "failure-in-radio-interface-procedure",
            "interaction-with-other-procedure",
            "unknown-PDU-session-ID",
            "unkown-qos-flow-ID",
            "multiple-PDU-session-ID-instances",
            "multiple-qos-flow-ID-instances",
            "encryption-and-or-integrity-protection-algorithms-not-supported",
            "ng-intra-system-handover-triggered",
            "ng-inter-system-handover-triggered",
            "xn-handover-triggered",
            "not-supported-5QI-value",
            "ue-context-transfer",
            "ims-voice-eps-fallback-or-rat-fallback-triggered",
            "up-integrity-protection-not-possible",
            "up-confidentiality-protection-not-possible",
            "slice-not-supported",
            "ue-in-rrc-inactive-state-not-reachable",
            "redirection",
            "resources-not-available-for-the-slice",
            "ue-max-integrity-protected-data-rate-reason",
            "release-due-to-cn-detected-mobility",
            "n26-interface-not-available",
            "release-due-to-pre-emption",
            "multiple-location-reporting-reference-ID-instances",
            "rsn-not-available-for-the-up",
            "npn-access-denied",
            "cag-only-access-denied",
            "insufficient-ue-capabilities",
            "redcap-ue-not-supported",
            "unknown-MBS-Session-ID",
            "indicated-MBS-session-area-information-not-served-by-the-gNB",
            "inconsistent-slice-info-for-the-session",
            "misaligned-association-for-multicast-unicast",
            "eredcap-ue-not-supported",
            "two-rx-xr-ue-not-supported",
            "aerial-ue-flight-information-reporting-initiation-failure",
            "unknown-RAN-AIoT-Device-NGAP-ID",
            "requested-AIoT-service-area-information-not-served-by-the-gNB",
            "unknown-AIoT-session",
            "aiot-device-not-reachable",
            "multiple-AIoT-session-ID-instances",
        ],
    ),
    (
        "CauseTransport",
        &["transport-resource-unavailable", "unspecified"],
    ),
    ("CellSize", &["verysmall", "small", "medium", "large"]),
    ("ClockQualityAcceptanceCriteriaTraceabletoGNSS", &["true"]),
    ("ClockQualityAcceptanceCriteriaTraceabletoUTC", &["true"]),
    (
        "CommServiceType",
        &["mbs-broadcast", "mbs-multicast", "unicast"],
    ),
    ("ConcurrentWarningMessageInd", &["true"]),
    (
        "ConfidentialityProtectionIndication",
        &["required", "preferred", "not-needed"],
    ),
    (
        "ConfidentialityProtectionResult",
        &["performed", "not-performed"],
    ),
    ("ConfiguredTACIndication", &["true"]),
    (
        "CongestionInformationRequest",
        &["ul", "dl", "both", "stop"],
    ),
    ("Criticality", &["reject", "ignore", "notify"]),
    ("DAPSRequestInfoDAPSIndicator", &["daps-ho-required"]),
    (
        "DAPSResponseInfoDapsresponseindicator",
        &["daps-ho-accepted", "daps-ho-not-accepted"],
    ),
    ("DLForwarding", &["dl-forwarding-proposed"]),
    ("DLNGUTNLInformationReused", &["true"]),
    ("DLPDUSetInformationMarkingSupportIndication", &["true"]),
    ("DLSignalling", &["true"]),
    ("DataForwardingAccepted", &["data-forwarding-accepted"]),
    (
        "DataForwardingNotPossible",
        &["data-forwarding-not-possible"],
    ),
    ("DelayCritical", &["delay-critical", "non-delay-critical"]),
    (
        "DirectForwardingPathAvailability",
        &["direct-path-available"],
    ),
    ("ECNMarkingAtRANRequest", &["ul", "dl", "both", "stop"]),
    ("ECNMarkingAtUPFRequest", &["ul", "dl", "both", "stop"]),
    ("EDTSession", &["true"]),
    ("ERedCapIndication", &["true"]),
    (
        "EUTRAPagingEDRXCycle",
        &[
            "hfhalf", "hf1", "hf2", "hf4", "hf6", "hf8", "hf10", "hf12", "hf14", "hf16", "hf32",
            "hf64", "hf128", "hf256",
        ],
    ),
    (
        "EUTRAPagingTimeWindow",
        &[
            "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13",
            "s14", "s15", "s16",
        ],
    ),
    ("EarlyMeasurement", &["true"]),
    (
        "EmergencyFallbackRequestIndicator",
        &["emergency-fallback-requested"],
    ),
    ("EmergencyServiceTargetCN", &["fiveGC", "epc"]),
    ("EndIndication", &["no-further-data", "further-data-exists"]),
    ("EnhancedCoverageRestriction", &["restricted"]),
    ("EventTriggerOutOfCoverage", &["true"]),
    (
        "EventType",
        &[
            "direct",
            "change-of-serve-cell",
            "ue-presence-in-area-of-interest",
            "stop-change-of-serve-cell",
            "stop-ue-presence-in-area-of-interest",
            "cancel-location-reporting-for-the-ue",
            "change-of-serving-cell-and-UE-presence-in-the-Area-of-Interest",
            "report-the-Aerial-UE-flight-information",
            "cancel-the-Aerial-UE-flight-information-reporting",
        ],
    ),
    (
        "ExcessPacketDelayThresholdValue",
        &[
            "ms0dot25", "ms0dot5", "ms1", "ms2", "ms4", "ms5", "ms10", "ms20", "ms30", "ms40",
            "ms50", "ms60", "ms70", "ms80", "ms90", "ms100", "ms150", "ms300", "ms500",
        ],
    ),
    (
        "ExpectedHOInterval",
        &[
            "sec15",
            "sec30",
            "sec60",
            "sec90",
            "sec120",
            "sec180",
            "long-time",
        ],
    ),
    ("ExpectedUEMobility", &["stationary", "mobile"]),
    ("ExtendedReportIntervalMDT", &["ms20480", "ms40960"]),
    (
        "FiveGProSeDirectCommunication",
        &["authorized", "not-authorized"],
    ),
    (
        "FiveGProSeDirectDiscovery",
        &["authorized", "not-authorized"],
    ),
    (
        "FiveGProSeLayer2MHIntermediateUEtoNetworkRelay",
        &["authorized", "not-authorized"],
    ),
    (
        "FiveGProSeLayer2MHRemote",
        &["authorized", "not-authorized"],
    ),
    (
        "FiveGProSeLayer2MHUEtoNetworkRelay",
        &["authorized", "not-authorized"],
    ),
    (
        "FiveGProSeLayer2Multipath",
        &["authorized", "not-authorized"],
    ),
    (
        "FiveGProSeLayer2RemoteUE",
        &["authorized", "not-authorized"],
    ),
    (
        "FiveGProSeLayer2UEtoNetworkRelay",
        &["authorized", "not-authorized"],
    ),
    (
        "FiveGProSeLayer2UEtoUERelay",
        &["authorized", "not-authorized"],
    ),
    (
        "FiveGProSeLayer2UEtoUERemote",
        &["authorized", "not-authorized"],
    ),
    (
        "FiveGProSeLayer3MHUEtoNetworkRelay",
        &["authorized", "not-authorized"],
    ),
    (
        "FiveGProSeLayer3UEtoNetworkRelay",
        &["authorized", "not-authorized"],
    ),
    ("GUAMIType", &["native", "mapped"]),
    ("GWContextReleaseIndication", &["true"]),
    ("HLComDeactivateUEReachabilityIndication", &["true"]),
    (
        "HOReportHandoverReportType",
        &["ho-too-early", "ho-to-wrong-cell", "intersystem-ping-pong"],
    ),
    ("HandoverFlag", &["handover-preparation"]),
    (
        "HandoverType",
        &["intra5gs", "fivegs-to-eps", "eps-to-5gs", "fivegs-to-utran"],
    ),
    ("IABAuthorized", &["authorized", "not-authorized"]),
    ("IABNodeIndication", &["true"]),
    ("IABSupported", &["true"]),
    ("IMSVoiceSupportIndicator", &["supported", "not-supported"]),
    ("IncludeBeamMeasurementsIndication", &["true"]),
    ("IndicationOfBitrateAdaptation", &["uplink"]),
    (
        "IntegrityProtectionIndication",
        &["required", "preferred", "not-needed"],
    ),
    ("IntegrityProtectionResult", &["performed", "not-performed"]),
    ("IntersystemUnnecessaryHOEarlyIRATHO", &["true", "false"]),
    (
        "InventoryReportTransferInventoryCompleteIndication",
        &["true"],
    ),
    ("LPWUSDisableIndication", &["true"]),
    (
        "LTEA2XServicesAuthorizedAerialControllerUE",
        &["authorized", "not-authorized"],
    ),
    (
        "LTEA2XServicesAuthorizedAerialUE",
        &["authorized", "not-authorized"],
    ),
    ("LTEMIndication", &["lte-m"]),
    ("LineType", &["dsl", "pon"]),
    (
        "LinksToLog",
        &["uplink", "downlink", "both-uplink-and-downlink"],
    ),
    ("LocationReportingAdditionalInfo", &["includePSCell"]),
    (
        "LoggingDuration",
        &["m10", "m20", "m40", "m60", "m90", "m120"],
    ),
    (
        "LoggingInterval",
        &[
            "ms320", "ms640", "ms1280", "ms2560", "ms5120", "ms10240", "ms20480", "ms30720",
            "ms40960", "ms61440", "infinity",
        ],
    ),
    (
        "M1ReportingTrigger",
        &["periodic", "a2eventtriggered", "a2eventtriggered-periodic"],
    ),
    (
        "M4ReportAmountMDT",
        &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"],
    ),
    (
        "M4period",
        &["ms1024", "ms2048", "ms5120", "ms10240", "min1"],
    ),
    (
        "M5ReportAmountMDT",
        &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"],
    ),
    (
        "M5period",
        &["ms1024", "ms2048", "ms5120", "ms10240", "min1"],
    ),
    (
        "M6ReportAmountMDT",
        &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"],
    ),
    (
        "M6reportInterval",
        &[
            "ms120", "ms240", "ms480", "ms640", "ms1024", "ms2048", "ms5120", "ms10240", "ms20480",
            "ms40960", "min1", "min6", "min12", "min30",
        ],
    ),
    (
        "M7ReportAmountMDT",
        &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"],
    ),
    ("MBSAssistanceInformation", &["true"]),
    ("MBSSessionStatus", &["activated", "deactivated"]),
    (
        "MBSSupportIndicator",
        &[
            "multicast-supported",
            "multicast-supported-with-reception-in-RRC-inactive",
        ],
    ),
    ("MBSUPFailureIndication", &["ng-u-path-failure-detected"]),
    (
        "MDTActivation",
        &[
            "immediate-MDT-only",
            "logged-MDT-only",
            "immediate-MDT-and-Trace",
        ],
    ),
    ("MICOModeIndication", &["true"]),
    ("MNOnlyMDTCollection", &["mn-only"]),
    (
        "MaximumIntegrityProtectedDataRate",
        &["bitrate64kbs", "maximum-UE-rate"],
    ),
    ("MicoAllPLMN", &["true"]),
    ("MobileIABAuthorized", &["authorized", "not-authorized"]),
    ("MobileIABNodeIndication", &["true"]),
    ("MobileIABSupported", &["true"]),
    ("MonitoringRequest", &["ul", "dl", "both", "stop"]),
    (
        "NBIoTDefaultPagingDRX",
        &["rf128", "rf256", "rf512", "rf1024"],
    ),
    (
        "NBIoTPagingDRX",
        &["rf32", "rf64", "rf128", "rf256", "rf512", "rf1024"],
    ),
    (
        "NBIoTPagingEDRXCycle",
        &[
            "hf2", "hf4", "hf6", "hf8", "hf10", "hf12", "hf14", "hf16", "hf32", "hf64", "hf128",
            "hf256", "hf512", "hf1024",
        ],
    ),
    (
        "NBIoTPagingTimeWindow",
        &[
            "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13",
            "s14", "s15", "s16",
        ],
    ),
    (
        "NGAPIESupportInformationResponseItemNgapProtocolIEPresenceInfo",
        &["present", "not-present"],
    ),
    (
        "NGAPIESupportInformationResponseItemNgapProtocolIESupportInfo",
        &["supported", "not-supported"],
    ),
    (
        "NRA2XServicesAuthorizedAerialControllerUE",
        &["authorized", "not-authorized"],
    ),
    (
        "NRA2XServicesAuthorizedAerialUE",
        &["authorized", "not-authorized"],
    ),
    (
        "NRPagingEDRXCycle",
        &[
            "hfquarter",
            "hfhalf",
            "hf1",
            "hf2",
            "hf4",
            "hf8",
            "hf16",
            "hf32",
            "hf64",
            "hf128",
            "hf256",
            "hf512",
            "hf1024",
        ],
    ),
    (
        "NRPagingLongEDRXCycleForRRCINACTIVE",
        &[
            "hf2", "hf4", "hf8", "hf16", "hf32", "hf64", "hf128", "hf256", "hf512", "hf1024",
        ],
    ),
    (
        "NRPagingTimeWindow",
        &[
            "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13",
            "s14", "s15", "s16", "s17", "s18", "s19", "s20", "s21", "s22", "s23", "s24", "s25",
            "s26", "s27", "s28", "s29", "s30", "s31", "s32",
        ],
    ),
    (
        "NRPagingTimeWindowForRRCINACTIVE",
        &[
            "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13",
            "s14", "s15", "s16", "s17", "s18", "s19", "s20", "s21", "s22", "s23", "s24", "s25",
            "s26", "s27", "s28", "s29", "s30", "s31", "s32",
        ],
    ),
    (
        "NetworkControlledRepeaterAuthorized",
        &["authorized", "not-authorized"],
    ),
    ("NewSecurityContextInd", &["true"]),
    ("NextPagingAreaScope", &["same", "changed"]),
    ("NoPDUSessionIndication", &["true"]),
    (
        "NotificationCause",
        &[
            "fulfilled",
            "not-fulfilled",
            "not-fulfilled-DL",
            "not-fulfilled-UL",
        ],
    ),
    (
        "NotificationCellItemNotifyFlag",
        &["activated", "deactivated"],
    ),
    ("NotificationControl", &["notification-requested"]),
    ("NotifySourceNGRANNode", &["notifySource"]),
    (
        "NumberOfMeasurementReportingLevels",
        &["n2", "n3", "n4", "n5", "n10", "n0"],
    ),
    ("OnboardingSupport", &["true"]),
    (
        "OverloadAction",
        &[
            "reject-non-emergency-mo-dt",
            "reject-rrc-cr-signalling",
            "permit-emergency-sessions-and-mobile-terminated-services-only",
            "permit-high-priority-sessions-and-mobile-terminated-services-only",
        ],
    ),
    (
        "PDUSessionType",
        &["ipv4", "ipv6", "ipv4v6", "ethernet", "unstructured"],
    ),
    (
        "PDUSessionUsageReportRATType",
        &["nr", "eutra", "nr-unlicensed", "e-utra-unlicensed"],
    ),
    ("PDUSetbasedHandlingIndicator", &["supported"]),
    (
        "PDUsetQoSInformationPduSetIntegratedHandlingInformation",
        &["true", "false"],
    ),
    ("PagingCause", &["voice"]),
    ("PagingCauseIndicationForVoiceService", &["supported"]),
    ("PagingDRX", &["v32", "v64", "v128", "v256"]),
    ("PagingOrigin", &["non-3gpp"]),
    (
        "PagingPriority",
        &[
            "priolevel1",
            "priolevel2",
            "priolevel3",
            "priolevel4",
            "priolevel5",
            "priolevel6",
            "priolevel7",
            "priolevel8",
        ],
    ),
    (
        "PagingProbabilityInformation",
        &[
            "p00", "p05", "p10", "p15", "p20", "p25", "p30", "p35", "p40", "p45", "p50", "p55",
            "p60", "p65", "p70", "p75", "p80", "p85", "p90", "p95", "p100",
        ],
    ),
    (
        "ParentTImeSource",
        &[
            "synce",
            "ptp",
            "gnss",
            "atomicclock",
            "terrestrialradio",
            "serialtimecode",
            "ntp",
            "handset",
            "other",
        ],
    ),
    ("PedestrianUE", &["authorized", "not-authorized"]),
    (
        "PreEmptionCapability",
        &["shall-not-trigger-pre-emption", "may-trigger-pre-emption"],
    ),
    (
        "PreEmptionVulnerability",
        &["not-pre-emptable", "pre-emptable"],
    ),
    ("Presence", &["optional", "conditional", "mandatory"]),
    ("PrivacyIndicator", &["immediate-MDT", "logged-MDT"]),
    ("QoERVQoEReportingPathsQoEReportingPath", &["srb4", "srb5"]),
    (
        "QoERVQoEReportingPathsRVQoEReportingPath",
        &["srb4", "srb5"],
    ),
    (
        "QoSFlowsUsageReportItemRATType",
        &["nr", "eutra", "nr-unlicensed", "e-utra-unlicensed"],
    ),
    ("QosMonitoringRequest", &["ul", "dl", "both", "stop"]),
    ("RANTSSRequestType", &["start", "stop"]),
    (
        "RANTimingSynchronisationStatusInfoSynchronisationState",
        &["locked", "holdover", "freerun"],
    ),
    (
        "RANTimingSynchronisationStatusInfoTraceabletoGNSS",
        &["true", "false"],
    ),
    (
        "RANTimingSynchronisationStatusInfoTraceabletoUTC",
        &["true", "false"],
    ),
    (
        "RANfeedbacktypeReactiveCapabilityForBATAdaptation",
        &["true"],
    ),
    (
        "RATInformation",
        &[
            "unlicensed",
            "nb-IoT",
            "nR-LEO",
            "nR-MEO",
            "nR-GEO",
            "nR-OTHERSAT",
        ],
    ),
    (
        "RIMInformationRIMRSDetection",
        &["rs-detected", "rs-disappeared"],
    ),
    (
        "RRCEstablishmentCause",
        &[
            "emergency",
            "highPriorityAccess",
            "mt-Access",
            "mo-Signalling",
            "mo-Data",
            "mo-VoiceCall",
            "mo-VideoCall",
            "mo-SMS",
            "mps-PriorityAccess",
            "mcs-PriorityAccess",
            "notAvailable",
            "mo-ExceptionData",
        ],
    ),
    (
        "RRCInactiveTransitionReportRequest",
        &[
            "subsequent-state-transition-report",
            "single-rrc-connected-state-report",
            "cancel-report",
        ],
    ),
    ("RRCState", &["inactive", "connected"]),
    ("RSN", &["v1", "v2"]),
    (
        "Range",
        &[
            "m50", "m80", "m180", "m200", "m350", "m400", "m500", "m700", "m1000",
        ],
    ),
    ("RedCapIndication", &["redcap"]),
    ("RedirectionVoiceFallback", &["possible", "not-possible"]),
    ("RedundantQosFlowIndicator", &["true", "false"]),
    ("ReflectiveQosAttribute", &["subject-to"]),
    (
        "ReportAmountMDT",
        &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "rinfinity"],
    ),
    ("ReportArea", &["cell"]),
    (
        "ReportIntervalMDT",
        &[
            "ms120", "ms240", "ms480", "ms640", "ms1024", "ms2048", "ms5120", "ms10240", "min1",
            "min6", "min12", "min30", "min60",
        ],
    ),
    (
        "ReportingPeriodicity",
        &["stop", "single", "ms1000", "ms2000", "ms5000", "ms10000"],
    ),
    ("ResetAll", &["reset-all"]),
    (
        "SLPositioningRangingAuthorized",
        &["authorized", "not-authorized"],
    ),
    ("SONInformationRequest", &["xn-TNL-configuration-info"]),
    ("SRVCCOperationPossible", &["possible", "notPossible"]),
    ("SensorMeasConfig", &["setup"]),
    ("SensorNameConfigUeOrientationConfig", &["true"]),
    ("SensorNameConfigUeSpeedConfig", &["true"]),
    ("SensorNameConfigUncompensatedBarometricConfig", &["true"]),
    ("ServiceType", &["streaming", "mTSI", "vR"]),
    (
        "SourceOfUEActivityBehaviourInformation",
        &["subscription-information", "statistics"],
    ),
    (
        "SupportedUEType",
        &["non-RedCap-eRedCap-UE", "redCap-eRedCap-UE"],
    ),
    ("SuspendIndicator", &["true"]),
    ("SuspendRequestIndication", &["suspend-requested"]),
    ("SuspendResponseIndication", &["suspend-indicated"]),
    ("TNLAssociationUsage", &["ue", "non-ue", "both"]),
    (
        "TimeSyncAssistanceInfoTimeDistributionIndication",
        &["enabled", "disabled"],
    ),
    (
        "TimeToTrigger",
        &[
            "ms0", "ms40", "ms64", "ms80", "ms100", "ms128", "ms160", "ms256", "ms320", "ms480",
            "ms512", "ms640", "ms1024", "ms1280", "ms2560", "ms5120",
        ],
    ),
    ("TimeToWait", &["v1s", "v2s", "v5s", "v10s", "v20s", "v60s"]),
    ("TimerApproachForGUAMIRemoval", &["apply-timer"]),
    (
        "TraceDepth",
        &[
            "minimum",
            "medium",
            "maximum",
            "minimumWithoutVendorSpecificExtension",
            "mediumWithoutVendorSpecificExtension",
            "maximumWithoutVendorSpecificExtension",
            "minimumOnlyVendorSpecificTraceRecord",
            "mediumOnlyVendorSpecificTraceRecord",
            "maximumOnlyVendorSpecificTraceRecord",
        ],
    ),
    (
        "TriggeringMessage",
        &[
            "initiating-message",
            "successful-outcome",
            "unsuccessful-outcome",
        ],
    ),
    ("TypeOfError", &["not-understood", "missing"]),
    ("UEAppLayerMeasConfigInfoQoEMeasurementStatus", &["ongoing"]),
    ("UECapabilityInfoRequest", &["requested"]),
    ("UEContextRequest", &["requested"]),
    (
        "UEDifferentiationInfoBatteryIndication",
        &[
            "battery-powered",
            "battery-powered-not-rechargeable-or-replaceable",
            "not-battery-powered",
        ],
    ),
    (
        "UEDifferentiationInfoPeriodicCommunicationIndicator",
        &["periodically", "ondemand"],
    ),
    (
        "UEDifferentiationInfoStationaryIndication",
        &["stationary", "mobile"],
    ),
    (
        "UEDifferentiationInfoTrafficProfile",
        &["single-packet", "dual-packets", "multiple-packets"],
    ),
    ("UEPresence", &["in", "out", "unknown"]),
    ("UERetentionInformation", &["ues-retained"]),
    ("UEUPCIoTSupport", &["supported"]),
    ("ULForwarding", &["ul-forwarding-proposed"]),
    (
        "UserPlaneErrorIndicator",
        &["gTP-U-error-indication-received"],
    ),
    (
        "UserPlaneFailureIndicationReport",
        &[
            "qos-flows-and-tunnel-to-be-released",
            "qoS-flows-to-be-moved",
            "new-transport-address-allocated",
        ],
    ),
    (
        "UserPlaneFailureType",
        &["gtp-u-error-indication-received", "up-path-failure"],
    ),
    ("VehicleUE", &["authorized", "not-authorized"]),
    ("WLANMeasConfig", &["setup"]),
    ("WLANMeasurementConfigurationWlanRssi", &["true"]),
    ("WLANMeasurementConfigurationWlanRtt", &["true"]),
    ("XrDeviceWith2Rx", &["true"]),
];
types! {
    A2XPC5FlowBitRates sequence { "a2X-GuaranteedFlowBitRate": _, "a2X-MaximumFlowBitRate": _, "iE-Extensions"? extensions };
    A2XPC5QoSFlowItem sequence { "a2X-PQI": _, "a2X-PC5-FlowBitRates"? A2XPC5FlowBitRates, "a2X-Range"? A2XPC5QoSFlowItemA2XRange, "iE-Extensions"? extensions };
    A2XPC5QoSFlowList = [A2XPC5QoSFlowItem];
    A2XPC5QoSParameters sequence { "a2X-PC5-QoS-Flow-List": A2XPC5QoSFlowList, "a2X-PC5-LinkAggregateBitRates"? _, "iE-Extensions"? extensions };
    AIOTFName sequence { "aIOTFNameVisibleString"? _, "aIOTFNameUTF8String"? _, "iE-Extensions"? extensions };
    AIOTSessionReleaseCommand sequence { "protocolIEs": ies };
    AIOTSessionReleaseCommandTransfer sequence { "protocolIEs": ies };
    AIOTSessionReleaseComplete sequence { "protocolIEs": ies };
    AIOTSessionReleaseCompleteTransfer sequence { "correlationIdentifier": _, "criticalityDiagnostics"? CriticalityDiagnostics, "iE-Extensions"? extensions };
    AIOTSessionReleaseRequest sequence { "protocolIEs": ies };
    AIOTSessionReleaseRequestTransfer sequence { "correlationIdentifier": _, "cause": Cause, "iE-Extensions"? extensions };
    AIoTAreaID sequence { "pLMNidentity": PLMNIdentity, "nID"? _, "aIoTAreaCode": _, "iE-Extensions"? extensions };
    AIoTCommandAssistanceInformation sequence { "estimateofExpectedD2RMsgSize": _, "iE-Extensions"? extensions };
    AIoTDeviceIdentificationRequested choice { "singleDevice": bits, "groupDevices": bits, "allDevices": null, "choice-Extensions": ie };
    AIoTDeviceReportItem sequence { "aIoT-NASPDU": _, "rAN-AIOT-Device-NGAP-ID"? _, "iE-Extensions"? extensions };
    AIoTDeviceReportList = [AIoTDeviceReportItem];
    AIoTInventoryAssistanceInformation sequence { "expectedD2RMsgSize": _, "approximateNoofTargetDevices"? _, "timeInterval"? _, "iE-Extensions"? extensions };
    AIoTReaderReportItem sequence { "readerIndex": _, "deviceReportList": AIoTDeviceReportList, "iE-Extensions"? extensions };
    AIoTReaderReportList = [AIoTReaderReportItem];
    AIoTRequestedAIoTAreaList = [AIoTAreaID];
    AIoTRequestedReaderItem sequence { "globalgNB-ID": GlobalGNBID, "readerIndex": _, "iE-Extensions"? extensions };
    AIoTRequestedReaderList = [AIoTRequestedReaderItem];
    AIoTRequestedServiceAreaInformation sequence { "requestedReaderList"? AIoTRequestedReaderList, "requestedAIoTAreaList"? AIoTRequestedAIoTAreaList, "iE-Extensions"? extensions };
    AIoTSecurityInfo choice { "a-IoT-RAND-n": bits, "choice-Extensions": ie };
    AMFCPRelocationIndication sequence { "protocolIEs": ies };
    AMFConfigurationUpdate sequence { "protocolIEs": ies };
    AMFConfigurationUpdateAcknowledge sequence { "protocolIEs": ies };
    AMFConfigurationUpdateFailure sequence { "protocolIEs": ies };
    AMFPagingTarget choice { "globalRANNodeID": GlobalRANNodeID, "tAI": TAI, "choice-Extensions": ie };
    AMFStatusIndication sequence { "protocolIEs": ies };
    AMFTNLAssociationSetupItem sequence { "aMF-TNLAssociationAddress": CPTransportLayerInformation, "iE-Extensions"? extensions };
    AMFTNLAssociationSetupList = [AMFTNLAssociationSetupItem];
    AMFTNLAssociationToAddItem sequence { "aMF-TNLAssociationAddress": CPTransportLayerInformation, "tNLAssociationUsage"? TNLAssociationUsage, "tNLAddressWeightFactor": _, "iE-Extensions"? extensions };
    AMFTNLAssociationToAddList = [AMFTNLAssociationToAddItem];
    AMFTNLAssociationToRemoveItem sequence { "aMF-TNLAssociationAddress": CPTransportLayerInformation, "iE-Extensions"? extensions };
    AMFTNLAssociationToRemoveList = [AMFTNLAssociationToRemoveItem];
    AMFTNLAssociationToUpdateItem sequence { "aMF-TNLAssociationAddress": CPTransportLayerInformation, "tNLAssociationUsage"? TNLAssociationUsage, "tNLAddressWeightFactor"? _, "iE-Extensions"? extensions };
    AMFTNLAssociationToUpdateList = [AMFTNLAssociationToUpdateItem];
    AUN3DeviceAccessInfo sequence { "aUN3DeviceAccess": AUN3DeviceAccessInfoAUN3DeviceAccess, "existingNGConnectionwithSameLocation": AUN3DeviceAccessInfoExistingNGConnectionwithSameLocation, "iE-Extensions"? extensions };
    ActivatedCellList = [NGRANCGI];
    AdditionalCancelledlocationReportingReferenceIDItem sequence { "locationReportingReferenceIDToBeCancelled": _, "iE-Extensions"? extensions };
    AdditionalCancelledlocationReportingReferenceIDList = [AdditionalCancelledlocationReportingReferenceIDItem];
    AdditionalDLUPTNLInformationForHOItem sequence { "additionalDL-NGU-UP-TNLInformation": UPTransportLayerInformation, "additionalQosFlowSetupResponseList": QosFlowListWithDataForwarding, "additionalDLForwardingUPTNLInformation"? UPTransportLayerInformation, "iE-Extensions"? extensions };
    AdditionalDLUPTNLInformationForHOList = [AdditionalDLUPTNLInformationForHOItem];
    AdditionalULI sequence { "nRCGI": NRCGI, "tAI": TAI, "iE-Extensions"? extensions };
    AerialUEFlightInformationReporting sequence { "altitude": _, "timeStamp": _, "iE-Extensions"? extensions };
    AerialUEFlightInformationReportingControl sequence { "higher-Altitude-Threshold": _, "lower-Altitude-Threshold": _, "aerial-UE-report-periodicity"? AerialUEReportingPeriodicity, "areaID": AreaID, "iE-Extensions"? extensions };
    AerialUEFlightInformationReportingControlItem sequence { "aerial-UE-FlightInformationReportingControl": AerialUEFlightInformationReportingControl, "iE-Extensions"? extensions };
    AerialUEFlightInformationReportingControlList = [AerialUEFlightInformationReportingControlItem];
    AllocationAndRetentionPriority sequence { "priorityLevelARP": _, "pre-emptionCapability": PreEmptionCapability, "pre-emptionVulnerability": PreEmptionVulnerability, "iE-Extensions"? extensions };
    AllowedCAGListPerPLMN = [_];
    AllowedNSSAI = [AllowedNSSAIItem];
    AllowedNSSAIItem sequence { "s-NSSAI": SNSSAI, "iE-Extensions"? extensions };
    AllowedPNINPNItem sequence { "pLMNIdentity": PLMNIdentity, "pNI-NPN-restricted": AllowedPNINPNItemPNINPNRestricted, "allowed-CAG-List-per-PLMN": AllowedCAGListPerPLMN, "iE-Extensions"? extensions };
    AllowedPNINPNList = [AllowedPNINPNItem];
    AllowedPeriodicityList = [_];
    AllowedTACs = [TAC];
    AlternativeQoSParaSetItem sequence { "alternativeQoSParaSetIndex": _, "guaranteedFlowBitRateDL"? _, "guaranteedFlowBitRateUL"? _, "packetDelayBudget"? _, "packetErrorRate"? PacketErrorRate, "iE-Extensions"? extensions };
    AlternativeQoSParaSetList = [AlternativeQoSParaSetItem];
    AnonymousPrivateMessagePrivateIEs sequence { "id": PrivateIEID, "criticality": Criticality, "value": _ };
    AreaID choice { "tAI": TAI, "globalRANNodeID": GlobalRANNodeID, "nR-CGI": NRCGI, "choice-Extensions": ie };
    AreaOfInterest sequence { "areaOfInterestTAIList"? AreaOfInterestTAIList, "areaOfInterestCellList"? AreaOfInterestCellList, "areaOfInterestRANNodeList"? AreaOfInterestRANNodeList, "iE-Extensions"? extensions };
    AreaOfInterestCellItem sequence { "nGRAN-CGI": NGRANCGI, "iE-Extensions"? extensions };
    AreaOfInterestCellList = [AreaOfInterestCellItem];
    AreaOfInterestItem sequence { "areaOfInterest": AreaOfInterest, "locationReportingReferenceID": _, "iE-Extensions"? extensions };
    AreaOfInterestList = [AreaOfInterestItem];
    AreaOfInterestRANNodeItem sequence { "globalRANNodeID": GlobalRANNodeID, "iE-Extensions"? extensions };
    AreaOfInterestRANNodeList = [AreaOfInterestRANNodeItem];
    AreaOfInterestTAIItem sequence { "tAI": TAI, "iE-Extensions"? extensions };
    AreaOfInterestTAIList = [AreaOfInterestTAIItem];
    AreaScopeOfMDTEUTRA choice { "cellBased": CellBasedMDTEUTRA, "tABased": TABasedMDT, "pLMNWide": null, "tAIBased": TAIBasedMDT, "choice-Extensions": ie };
    AreaScopeOfMDTNR choice { "cellBased": CellBasedMDTNR, "tABased": TABasedMDT, "pLMNWide": null, "tAIBased": TAIBasedMDT, "choice-Extensions": ie };
    AreaScopeOfNeighCellsItem sequence { "nrFrequencyInfo": NRFrequencyInfo, "pciListForMDT"? PCIListForMDT, "iE-Extensions"? extensions };
    AreaScopeOfNeighCellsList = [AreaScopeOfNeighCellsItem];
    AreaScopeOfQMC choice { "cellBased": CellBasedQMC, "tABased": TABasedQMC, "tAIBased": TAIBasedQMC, "pLMNAreaBased": PLMNAreaBasedQMC, "choice-Extensions": ie };
    AssistanceDataForPaging sequence { "assistanceDataForRecommendedCells"? AssistanceDataForRecommendedCells, "pagingAttemptInformation"? PagingAttemptInformation, "iE-Extensions"? extensions };
    AssistanceDataForRecommendedCells sequence { "recommendedCellsForPaging": RecommendedCellsForPaging, "iE-Extensions"? extensions };
    AssociatedMBSQosFlowSetupRequestItem sequence { "mBS-QosFlowIdentifier": _, "associatedUnicastQosFlowIdentifier": _, "iE-Extensions"? extensions };
    AssociatedMBSQosFlowSetupRequestList = [AssociatedMBSQosFlowSetupRequestItem];
    AssociatedMBSQosFlowSetuporModifyRequestItem sequence { "mBS-QosFlowIdentifier": _, "associatedUnicastQosFlowIdentifier": _, "iE-Extensions"? extensions };
    AssociatedMBSQosFlowSetuporModifyRequestList = [AssociatedMBSQosFlowSetuporModifyRequestItem];
    AssociatedQosFlowItem sequence { "qosFlowIdentifier": _, "qosFlowMappingIndication"? AssociatedQosFlowItemQosFlowMappingIndication, "iE-Extensions"? extensions };
    AssociatedQosFlowList = [AssociatedQosFlowItem];
    AvailableBitrateReportThresholdItem sequence { "reportingThreshold": _, "iE-Extensions"? extensions };
    AvailableBitrateReportThresholdList = [AvailableBitrateReportThresholdItem];
    AvailableRANVisibleQoEMetrics sequence { "applicationLayerBufferLevelList"? AvailableRANVisibleQoEMetricsApplicationLayerBufferLevelList, "playoutDelayForMediaStartup"? AvailableRANVisibleQoEMetricsPlayoutDelayForMediaStartup, "iE-Extensions"? extensions };
    BeamMeasurementsReportConfiguration sequence { "beamMeasurementsReportQuantity"? BeamMeasurementsReportQuantity, "maxNrofRS-IndexesToReport"? _, "iE-Extensions"? extensions };
    BeamMeasurementsReportQuantity sequence { "rSRP": BeamMeasurementsReportQuantityRSRP, "rSRQ": BeamMeasurementsReportQuantityRSRQ, "sINR": BeamMeasurementsReportQuantitySINR, "iE-Extensions"? extensions };
    BluetoothMeasConfigNameItem sequence { "bluetoothName": _, "iE-Extensions"? extensions };
    BluetoothMeasConfigNameList = [BluetoothMeasConfigNameItem];
    BluetoothMeasurementConfiguration sequence { "bluetoothMeasConfig": BluetoothMeasConfig, "bluetoothMeasConfigNameList"? BluetoothMeasConfigNameList, "bt-rssi"? BluetoothMeasurementConfigurationBtRssi, "iE-Extensions"? extensions };
    BroadcastCancelledAreaList choice { "cellIDCancelledEUTRA": CellIDCancelledEUTRA, "tAICancelledEUTRA": TAICancelledEUTRA, "emergencyAreaIDCancelledEUTRA": EmergencyAreaIDCancelledEUTRA, "cellIDCancelledNR": CellIDCancelledNR, "tAICancelledNR": TAICancelledNR, "emergencyAreaIDCancelledNR": EmergencyAreaIDCancelledNR, "choice-Extensions": ie };
    BroadcastCompletedAreaList choice { "cellIDBroadcastEUTRA": CellIDBroadcastEUTRA, "tAIBroadcastEUTRA": TAIBroadcastEUTRA, "emergencyAreaIDBroadcastEUTRA": EmergencyAreaIDBroadcastEUTRA, "cellIDBroadcastNR": CellIDBroadcastNR, "tAIBroadcastNR": TAIBroadcastNR, "emergencyAreaIDBroadcastNR": EmergencyAreaIDBroadcastNR, "choice-Extensions": ie };
    BroadcastPLMNItem sequence { "pLMNIdentity": PLMNIdentity, "tAISliceSupportList": SliceSupportList, "iE-Extensions"? extensions };
    BroadcastPLMNList = [BroadcastPLMNItem];
    BroadcastSessionModificationFailure sequence { "protocolIEs": ies };
    BroadcastSessionModificationRequest sequence { "protocolIEs": ies };
    BroadcastSessionModificationResponse sequence { "protocolIEs": ies };
    BroadcastSessionReleaseRequest sequence { "protocolIEs": ies };
    BroadcastSessionReleaseRequired sequence { "protocolIEs": ies };
    BroadcastSessionReleaseResponse sequence { "protocolIEs": ies };
    BroadcastSessionSetupFailure sequence { "protocolIEs": ies };
    BroadcastSessionSetupRequest sequence { "protocolIEs": ies };
    BroadcastSessionSetupResponse sequence { "protocolIEs": ies };
    BroadcastSessionTransportFailure sequence { "protocolIEs": ies };
    BroadcastSessionTransportRequest sequence { "protocolIEs": ies };
    BroadcastSessionTransportResponse sequence { "protocolIEs": ies };
    BroadcastTransportFailureTransfer sequence { "mBS-SessionID": MBSSessionID, "cause": Cause, "criticalityDiagnostics"? CriticalityDiagnostics, "iE-Extensions"? extensions };
    BroadcastTransportRequestTransfer sequence { "mBS-SessionID": MBSSessionID, "mBS-SessionTNLInfoNGRAN": MBSSessionTNLInfoNGRAN, "iE-Extensions"? extensions };
    BroadcastTransportResponseTransfer sequence { "mBS-SessionID": MBSSessionID, "iE-Extensions"? extensions };
    BurstArrivalTimeWindow sequence { "burstArrivalTimeWindowStart": _, "burstArrivalTimeWindowEnd": _, "iE-Extension"? extensions };
    CAGListforMDT = [CAGListforMDTItem];
    CAGListforMDTItem sequence { "plmnID": PLMNIdentity, "cAGID": _, "iE-Extensions"? extensions };
    CNAssistedRANTuning sequence { "expectedUEBehaviour"? ExpectedUEBehaviour, "iE-Extensions"? extensions };
    CNTypeRestrictionsForEquivalent = [CNTypeRestrictionsForEquivalentItem];
    CNTypeRestrictionsForEquivalentItem sequence { "plmnIdentity": PLMNIdentity, "cn-Type": CNTypeRestrictionsForEquivalentItemCnType, "iE-Extensions"? extensions };
    COUNTValueForPDCPSN12 sequence { "pDCP-SN12": _, "hFN-PDCP-SN12": _, "iE-Extensions"? extensions };
    COUNTValueForPDCPSN18 sequence { "pDCP-SN18": _, "hFN-PDCP-SN18": _, "iE-Extensions"? extensions };
    CPTransportLayerInformation choice { "endpointIPAddress": TransportLayerAddress, "choice-Extensions": ie };
    CancelledCellsInEAIEUTRA = [CancelledCellsInEAIEUTRAItem];
    CancelledCellsInEAIEUTRAItem sequence { "eUTRA-CGI": EUTRACGI, "numberOfBroadcasts": _, "iE-Extensions"? extensions };
    CancelledCellsInEAINR = [CancelledCellsInEAINRItem];
    CancelledCellsInEAINRItem sequence { "nR-CGI": NRCGI, "numberOfBroadcasts": _, "iE-Extensions"? extensions };
    CancelledCellsInTAIEUTRA = [CancelledCellsInTAIEUTRAItem];
    CancelledCellsInTAIEUTRAItem sequence { "eUTRA-CGI": EUTRACGI, "numberOfBroadcasts": _, "iE-Extensions"? extensions };
    CancelledCellsInTAINR = [CancelledCellsInTAINRItem];
    CancelledCellsInTAINRItem sequence { "nR-CGI": NRCGI, "numberOfBroadcasts": _, "iE-Extensions"? extensions };
    CandidateCell choice { "candidateCGI": CandidateCellID, "candidatePCI": CandidatePCI, "choice-Extensions": ie };
    CandidateCellID sequence { "candidateCellID": NRCGI, "iE-Extensions"? extensions };
    CandidateCellItem sequence { "candidateCell": CandidateCell, "iE-Extensions"? extensions };
    CandidateCellList = [CandidateCellItem];
    CandidatePCI sequence { "candidatePCI": _, "candidateNRARFCN": _, "iE-Extensions"? extensions };
    CandidateRelayUEInformationItem sequence { "candidateRelayUE-Id": _, "iE-Extensions"? extensions };
    CandidateRelayUEInformationList = [CandidateRelayUEInformationItem];
    Cause choice { "radioNetwork": CauseRadioNetwork, "transport": CauseTransport, "nas": CauseNas, "protocol": CauseProtocol, "misc": CauseMisc, "choice-Extensions": ie };
    CellBasedMDTEUTRA sequence { "cellIdListforMDT": CellIdListforMDTEUTRA, "iE-Extensions"? extensions };
    CellBasedMDTNR sequence { "cellIdListforMDT": CellIdListforMDTNR, "iE-Extensions"? extensions };
    CellBasedQMC sequence { "cellIdListforQMC": CellIdListforQMC, "iE-Extensions"? extensions };
    CellCAGInformation sequence { "nGRAN-CGI": NGRANCGI, "cellCAGList": CellCAGList, "iE-Extensions"? extensions };
    CellCAGList = [_];
    CellIDBroadcastEUTRA = [CellIDBroadcastEUTRAItem];
    CellIDBroadcastEUTRAItem sequence { "eUTRA-CGI": EUTRACGI, "iE-Extensions"? extensions };
    CellIDBroadcastNR = [CellIDBroadcastNRItem];
    CellIDBroadcastNRItem sequence { "nR-CGI": NRCGI, "iE-Extensions"? extensions };
    CellIDCancelledEUTRA = [CellIDCancelledEUTRAItem];
    CellIDCancelledEUTRAItem sequence { "eUTRA-CGI": EUTRACGI, "numberOfBroadcasts": _, "iE-Extensions"? extensions };
    CellIDCancelledNR = [CellIDCancelledNRItem];
    CellIDCancelledNRItem sequence { "nR-CGI": NRCGI, "numberOfBroadcasts": _, "iE-Extensions"? extensions };
    CellIDListForRestart choice { "eUTRA-CGIListforRestart": EUTRACGIList, "nR-CGIListforRestart": NRCGIList, "choice-Extensions": ie };
    CellIdListforMDTEUTRA = [EUTRACGI];
    CellIdListforMDTNR = [NRCGI];
    CellIdListforQMC = [NGRANCGI];
    CellTrafficTrace sequence { "protocolIEs": ies };
    CellType sequence { "cellSize": CellSize, "iE-Extensions"? extensions };
    CellsToActivateList = [NGRANCGI];
    ClockAccuracy choice { "clockAccuracyValue": _, "clockAccuracyIndex": _, "choice-Extensions": ie };
    ClockQualityAcceptanceCriteria sequence { "synchronisationState"? bits, "traceabletoUTC"? ClockQualityAcceptanceCriteriaTraceabletoUTC, "traceabletoGNSS"? ClockQualityAcceptanceCriteriaTraceabletoGNSS, "clockFrequencyStability"? bits, "clockAccuracy"? _, "parentTImeSource"? bits, "iE-Extensions"? extensions };
    ClockQualityAcceptanceIndication sequence { "clockQualityAcceptanceCriteria": ClockQualityAcceptanceCriteria, "iE-Extensions"? extensions };
    ClockQualityDetailLevel choice { "clockQualityMetrics": null, "clockQualityAcceptanceIndication": ClockQualityAcceptanceIndication, "choice-extensions": ie };
    ClockQualityReportingControlInfo sequence { "clockQualityDetailLevel": ClockQualityDetailLevel, "iE-Extensions"? extensions };
    CommandFailure sequence { "protocolIEs": ies };
    CommandFailureTransfer sequence { "correlationIdentifier": _, "rAN-AIOT-Device-NGAP-ID": _, "cause": Cause, "criticalityDiagnostics"? CriticalityDiagnostics, "iE-Extensions"? extensions };
    CommandRequest sequence { "protocolIEs": ies };
    CommandRequestTransfer sequence { "protocolIEs": ies };
    CommandResponse sequence { "protocolIEs": ies };
    CommandResponseTransfer sequence { "correlationIdentifier": _, "rAN-AIOT-Device-NGAP-ID": _, "aIoT-NASPDU": _, "criticalityDiagnostics"? CriticalityDiagnostics, "iE-Extensions"? extensions };
    CompletedCellsInEAIEUTRA = [CompletedCellsInEAIEUTRAItem];
    CompletedCellsInEAIEUTRAItem sequence { "eUTRA-CGI": EUTRACGI, "iE-Extensions"? extensions };
    CompletedCellsInEAINR = [CompletedCellsInEAINRItem];
    CompletedCellsInEAINRItem sequence { "nR-CGI": NRCGI, "iE-Extensions"? extensions };
    CompletedCellsInTAIEUTRA = [CompletedCellsInTAIEUTRAItem];
    CompletedCellsInTAIEUTRAItem sequence { "eUTRA-CGI": EUTRACGI, "iE-Extensions"? extensions };
    CompletedCellsInTAINR = [CompletedCellsInTAINRItem];
    CompletedCellsInTAINRItem sequence { "nR-CGI": NRCGI, "iE-Extensions"? extensions };
    CompositeAvailableCapacity sequence { "cellCapacityClassValue"? _, "capacityValue": _, "iE-Extensions"? extensions };
    ConnectionEstablishmentIndication sequence { "protocolIEs": ies };
    CoreNetworkAssistanceInformationForInactive sequence { "uEIdentityIndexValue": UEIdentityIndexValue, "uESpecificDRX"? PagingDRX, "periodicRegistrationUpdateTimer": _, "mICOModeIndication"? MICOModeIndication, "tAIListForInactive": TAIListForInactive, "expectedUEBehaviour"? ExpectedUEBehaviour, "iE-Extensions"? extensions };
    CriticalityDiagnostics sequence { "procedureCode"? _, "triggeringMessage"? TriggeringMessage, "procedureCriticality"? Criticality, "iEsCriticalityDiagnostics"? CriticalityDiagnosticsIEList, "iE-Extensions"? extensions };
    CriticalityDiagnosticsIEItem sequence { "iECriticality": Criticality, "iE-ID": _, "typeOfError": TypeOfError, "iE-Extensions"? extensions };
    CriticalityDiagnosticsIEList = [CriticalityDiagnosticsIEItem];
    DAPSRequestInfo sequence { "dAPSIndicator": DAPSRequestInfoDAPSIndicator, "iE-Extensions"? extensions };
    DAPSResponseInfo sequence { "dapsresponseindicator": DAPSResponseInfoDapsresponseindicator, "iE-Extensions"? extensions };
    DAPSResponseInfoItem sequence { "dRB-ID": _, "dAPSResponseInfo": DAPSResponseInfo, "iE-Extension"? extensions };
    DAPSResponseInfoList = [DAPSResponseInfoItem];
    DLCPSecurityInformation sequence { "dl-NAS-MAC": _, "iE-Extensions"? extensions };
    DLCountChoice choice { "count12bits": COUNTValueForPDCPSN12, "count18bits": COUNTValueForPDCPSN18, "choice-extension": ie };
    DLDiscarding sequence { "dRBsSubjectToDLDiscarding": DRBsSubjectToDLDiscardingList, "iE-Extension"? extensions };
    DRBStatusDL choice { "dRBStatusDL12": DRBStatusDL12, "dRBStatusDL18": DRBStatusDL18, "choice-Extensions": ie };
    DRBStatusDL12 sequence { "dL-COUNTValue": COUNTValueForPDCPSN12, "iE-Extension"? extensions };
    DRBStatusDL18 sequence { "dL-COUNTValue": COUNTValueForPDCPSN18, "iE-Extension"? extensions };
    DRBStatusUL choice { "dRBStatusUL12": DRBStatusUL12, "dRBStatusUL18": DRBStatusUL18, "choice-Extensions": ie };
    DRBStatusUL12 sequence { "uL-COUNTValue": COUNTValueForPDCPSN12, "receiveStatusOfUL-PDCP-SDUs"? bits, "iE-Extension"? extensions };
    DRBStatusUL18 sequence { "uL-COUNTValue": COUNTValueForPDCPSN18, "receiveStatusOfUL-PDCP-SDUs"? bits, "iE-Extension"? extensions };
    DRBsSubjectToDLDiscardingItem sequence { "drbID": _, "dlCount": DLCountChoice, "iE-Extension"? extensions };
    DRBsSubjectToDLDiscardingList = [DRBsSubjectToDLDiscardingItem];
    DRBsSubjectToEarlyStatusTransferItem sequence { "dRB-ID": _, "firstDLCOUNT": DRBStatusDL, "iE-Extension"? extensions };
    DRBsSubjectToEarlyStatusTransferList = [DRBsSubjectToEarlyStatusTransferItem];
    DRBsSubjectToStatusTransferItem sequence { "dRB-ID": _, "dRBStatusUL": DRBStatusUL, "dRBStatusDL": DRBStatusDL, "iE-Extension"? extensions };
    DRBsSubjectToStatusTransferList = [DRBsSubjectToStatusTransferItem];
    DRBsToQosFlowsMappingItem sequence { "dRB-ID": _, "associatedQosFlowList": AssociatedQosFlowList, "iE-Extensions"? extensions };
    DRBsToQosFlowsMappingList = [DRBsToQosFlowsMappingItem];
    DataForwardingResponseDRBItem sequence { "dRB-ID": _, "dLForwardingUP-TNLInformation"? UPTransportLayerInformation, "uLForwardingUP-TNLInformation"? UPTransportLayerInformation, "iE-Extensions"? extensions };
    DataForwardingResponseDRBList = [DataForwardingResponseDRBItem];
    DataForwardingResponseERABList = [DataForwardingResponseERABListItem];
    DataForwardingResponseERABListItem sequence { "e-RAB-ID": _, "dLForwardingUP-TNLInformation": UPTransportLayerInformation, "iE-Extensions"? extensions };
    DeactivateTrace sequence { "protocolIEs": ies };
    DistributionReleaseRequest sequence { "protocolIEs": ies };
    DistributionReleaseResponse sequence { "protocolIEs": ies };
    DistributionSetupFailure sequence { "protocolIEs": ies };
    DistributionSetupRequest sequence { "protocolIEs": ies };
    DistributionSetupResponse sequence { "protocolIEs": ies };
    DownlinkNASTransport sequence { "protocolIEs": ies };
    DownlinkNonUEAssociatedNRPPaTransport sequence { "protocolIEs": ies };
    DownlinkRANConfigurationTransfer sequence { "protocolIEs": ies };
    DownlinkRANEarlyStatusTransfer sequence { "protocolIEs": ies };
    DownlinkRANStatusTransfer sequence { "protocolIEs": ies };
    DownlinkRIMInformationTransfer sequence { "protocolIEs": ies };
    DownlinkUEAssociatedNRPPaTransport sequence { "protocolIEs": ies };
    Dynamic5QIDescriptor sequence { "priorityLevelQos": _, "packetDelayBudget": _, "packetErrorRate": PacketErrorRate, "fiveQI"? _, "delayCritical"? DelayCritical, "averagingWindow"? _, "maximumDataBurstVolume"? _, "iE-Extensions"? extensions };
    ECNMarkingorCongestionInformationReportingRequest choice { "eCNMarkingAtRANRequest": ECNMarkingAtRANRequest, "eCNMarkingAtUPFRequest": ECNMarkingAtUPFRequest, "congestionInformationRequest": CongestionInformationRequest, "choice-Extensions": ie };
    ECNMarkingorCongestionInformationReportingStatus = [ECNMarkingorCongestionInformationReportingStatusItem];
    ECNMarkingorCongestionInformationReportingStatusItem sequence { "qosFlowIdentifier": _, "activationStatus": ActivationStatus, "iE-Extension"? extensions };
    ENBID choice { "macroENB-ID": _, "homeENB-ID": _, "short-macroENB-ID": _, "long-macroENB-ID": _, "choice-Extensions": ie };
    EPSTAI sequence { "pLMNIdentity": PLMNIdentity, "ePS-TAC": EPSTAC, "iE-Extensions"? extensions };
    ERABInformationItem sequence { "e-RAB-ID": _, "dLForwarding"? DLForwarding, "iE-Extensions"? extensions };
    ERABInformationList = [ERABInformationItem];
    EUTRACGI sequence { "pLMNIdentity": PLMNIdentity, "eUTRACellIdentity": EUTRACellIdentity, "iE-Extensions"? extensions };
    EUTRACGIList = [EUTRACGI];
    EUTRACGIListForWarning = [EUTRACGI];
    EUTRANCellReportItem sequence { "eCGI": EUTRACGI, "eUTRAN-CompositeAvailableCapacityGroup": EUTRANCompositeAvailableCapacityGroup, "eUTRAN-NumberOfActiveUEs"? _, "eUTRAN-NoofRRCConnections"? _, "eUTRAN-RadioResourceStatus"? EUTRANRadioResourceStatus, "iE-Extensions"? extensions };
    EUTRANCellReportList = [EUTRANCellReportItem];
    EUTRANCellToReportItem sequence { "eCGI": EUTRACGI, "iE-Extensions"? extensions };
    EUTRANCellToReportList = [EUTRANCellToReportItem];
    EUTRANCompositeAvailableCapacityGroup sequence { "dL-CompositeAvailableCapacity": CompositeAvailableCapacity, "uL-CompositeAvailableCapacity": CompositeAvailableCapacity, "iE-Extensions"? extensions };
    EUTRANRadioResourceStatus sequence { "dL-GBR-PRB-usage": _, "uL-GBR-PRB-usage": _, "dL-non-GBR-PRB-usage": _, "uL-non-GBR-PRB-usage": _, "dL-Total-PRB-usage": _, "uL-Total-PRB-usage": _, "dL-scheduling-PDCCH-CCE-usage"? _, "uL-scheduling-PDCCH-CCE-usage"? _, "iE-Extensions"? extensions };
    EUTRANReportingStatusIEs sequence { "eUTRAN-CellReportList": EUTRANCellReportList, "iE-Extensions"? extensions };
    EUTRANReportingSystemIEs sequence { "eUTRAN-CellToReportList": EUTRANCellToReportList, "iE-Extensions"? extensions };
    EUTRAPagingeDRXInformation sequence { "eUTRA-paging-eDRX-Cycle": EUTRAPagingEDRXCycle, "eUTRA-paging-Time-Window"? EUTRAPagingTimeWindow, "iE-Extensions"? extensions };
    EUTRAencryptionAlgorithms = bits;
    EUTRAintegrityProtectionAlgorithms = bits;
    EarlyStatusTransferTransparentContainer sequence { "procedureStage": ProcedureStageChoice, "iE-Extensions"? extensions };
    EmergencyAreaIDBroadcastEUTRA = [EmergencyAreaIDBroadcastEUTRAItem];
    EmergencyAreaIDBroadcastEUTRAItem sequence { "emergencyAreaID": _, "completedCellsInEAI-EUTRA": CompletedCellsInEAIEUTRA, "iE-Extensions"? extensions };
    EmergencyAreaIDBroadcastNR = [EmergencyAreaIDBroadcastNRItem];
    EmergencyAreaIDBroadcastNRItem sequence { "emergencyAreaID": _, "completedCellsInEAI-NR": CompletedCellsInEAINR, "iE-Extensions"? extensions };
    EmergencyAreaIDCancelledEUTRA = [EmergencyAreaIDCancelledEUTRAItem];
    EmergencyAreaIDCancelledEUTRAItem sequence { "emergencyAreaID": _, "cancelledCellsInEAI-EUTRA": CancelledCellsInEAIEUTRA, "iE-Extensions"? extensions };
    EmergencyAreaIDCancelledNR = [EmergencyAreaIDCancelledNRItem];
    EmergencyAreaIDCancelledNRItem sequence { "emergencyAreaID": _, "cancelledCellsInEAI-NR": CancelledCellsInEAINR, "iE-Extensions"? extensions };
    EmergencyAreaIDList = [_];
    EmergencyAreaIDListForRestart = [_];
    EmergencyFallbackIndicator sequence { "emergencyFallbackRequestIndicator": EmergencyFallbackRequestIndicator, "emergencyServiceTargetCN"? EmergencyServiceTargetCN, "iE-Extensions"? extensions };
    EndpointIPAddressAndPort sequence { "endpointIPAddress": TransportLayerAddress, "portNumber": PortNumber, "iE-Extensions"? extensions };
    EquivalentPLMNs = [PLMNIdentity];
    EquivalentSNPNsItem sequence { "plmnIdentity": PLMNIdentity, "nID": _, "iE-Extensions"? extensions };
    EquivalentSNPNsList = [EquivalentSNPNsItem];
    ErrorIndication sequence { "protocolIEs": ies };
    EventBasedReportingIEs sequence { "intersystemResourceThresholdLow": _, "intersystemResourceThresholdHigh": _, "numberOfMeasurementReportingLevels": NumberOfMeasurementReportingLevels, "iE-Extensions"? extensions };
    EventL1LoggedMDTConfig sequence { "l1Threshold": MeasurementThresholdL1LoggedMDT, "hysteresis": _, "timeToTrigger": TimeToTrigger, "iE-Extensions"? extensions };
    EventTrigger choice { "outOfCoverage": EventTriggerOutOfCoverage, "eventL1LoggedMDTConfig": EventL1LoggedMDTConfig, "choice-Extensions": ie };
    ExcessPacketDelayThresholdConfiguration = [ExcessPacketDelayThresholdItem];
    ExcessPacketDelayThresholdItem sequence { "fiveQi": _, "excessPacketDelayThresholdValue": ExcessPacketDelayThresholdValue, "iE-Extensions"? extensions };
    ExpectedUEActivityBehaviour sequence { "expectedActivityPeriod"? _, "expectedIdlePeriod"? _, "sourceOfUEActivityBehaviourInformation"? SourceOfUEActivityBehaviourInformation, "iE-Extensions"? extensions };
    ExpectedUEBehaviour sequence { "expectedUEActivityBehaviour"? ExpectedUEActivityBehaviour, "expectedHOInterval"? ExpectedHOInterval, "expectedUEMobility"? ExpectedUEMobility, "expectedUEMovingTrajectory"? ExpectedUEMovingTrajectory, "iE-Extensions"? extensions };
    ExpectedUEMovingTrajectory = [ExpectedUEMovingTrajectoryItem];
    ExpectedUEMovingTrajectoryItem sequence { "nGRAN-CGI": NGRANCGI, "timeStayedInCell"? _, "iE-Extensions"? extensions };
    ExtendedAMFName sequence { "aMFNameVisibleString"? _, "aMFNameUTF8String"? _, "iE-Extensions"? extensions };
    ExtendedRANNodeName sequence { "rANNodeNameVisibleString"? _, "rANNodeNameUTF8String"? _, "iE-Extensions"? extensions };
    ExtendedRATRestrictionInformation sequence { "primaryRATRestriction": bits, "secondaryRATRestriction": bits, "iE-Extensions"? extensions };
    ExtendedSliceSupportList = [SliceSupportItem];
    FailureIndication sequence { "uERLFReportContainer": UERLFReportContainer, "iE-Extensions"? extensions };
    FirstDLCount sequence { "dRBsSubjectToEarlyStatusTransfer": DRBsSubjectToEarlyStatusTransferList, "iE-Extension"? extensions };
    FiveGCAction choice { "hLComActivate": HLComActivate, "hLComDeactivate": HLComDeactivate, "choice-Extensions": ie };
    FiveGProSeAuthorized sequence { "fiveGProSeDirectDiscovery"? FiveGProSeDirectDiscovery, "fiveGProSeDirectCommunication"? FiveGProSeDirectCommunication, "fiveGProSeLayer2UEtoNetworkRelay"? FiveGProSeLayer2UEtoNetworkRelay, "fiveGProSeLayer3UEtoNetworkRelay"? FiveGProSeLayer3UEtoNetworkRelay, "fiveGProSeLayer2RemoteUE"? FiveGProSeLayer2RemoteUE, "iE-Extensions"? extensions };
    FiveGProSePC5FlowBitRates sequence { "fiveGproSeguaranteedFlowBitRate": _, "fiveGproSemaximumFlowBitRate": _, "iE-Extensions"? extensions };
    FiveGProSePC5QoSFlowItem sequence { "fiveGproSepQI": _, "fiveGproSepc5FlowBitRates"? FiveGProSePC5FlowBitRates, "fiveGproSerange"? Range, "iE-Extensions"? extensions };
    FiveGProSePC5QoSFlowList = [FiveGProSePC5QoSFlowItem];
    FiveGProSePC5QoSParameters sequence { "fiveGProSepc5QoSFlowList": FiveGProSePC5QoSFlowList, "fiveGProSepc5LinkAggregateBitRates"? _, "iE-Extensions"? extensions };
    FiveGSTMSI sequence { "aMFSetID": AMFSetID, "aMFPointer": AMFPointer, "fiveG-TMSI": FiveGTMSI, "iE-Extensions"? extensions };
    ForbiddenAreaInformation = [ForbiddenAreaInformationItem];
    ForbiddenAreaInformationItem sequence { "pLMNIdentity": PLMNIdentity, "forbiddenTACs": ForbiddenTACs, "iE-Extensions"? extensions };
    ForbiddenTACs = [TAC];
    FromEUTRANtoNGRAN sequence { "sourceeNBID": IntersystemSONeNBID, "targetNGRANnodeID": IntersystemSONNGRANnodeID, "iE-Extensions"? extensions };
    FromNGRANtoEUTRAN sequence { "sourceNGRANnodeID": IntersystemSONNGRANnodeID, "targeteNBID": IntersystemSONeNBID, "iE-Extensions"? extensions };
    GBRQosInformation sequence { "maximumFlowBitRateDL": _, "maximumFlowBitRateUL": _, "guaranteedFlowBitRateDL": _, "guaranteedFlowBitRateUL": _, "notificationControl"? NotificationControl, "maximumPacketLossRateDL"? _, "maximumPacketLossRateUL"? _, "iE-Extensions"? extensions };
    GNBID choice { "gNB-ID": bits, "choice-Extensions": ie };
    GTPTunnel sequence { "transportLayerAddress": TransportLayerAddress, "gTP-TEID": GTPTEID, "iE-Extensions"? extensions };
    GUAMI sequence { "pLMNIdentity": PLMNIdentity, "aMFRegionID": AMFRegionID, "aMFSetID": AMFSetID, "aMFPointer": AMFPointer, "iE-Extensions"? extensions };
    GUAMIList = [GUAMIListItem];
    GUAMIListItem sequence { "gUAMI": GUAMI, "iE-Extensions"? extensions };
    GeographyBasedMDT sequence { "nTN-Geographical-Area": NTNGeographicalAreaList, "nTN-PLMN-List"? MDTPLMNList, "iE-Extensions"? extensions };
    GlobalCableIDNew sequence { "globalCable-ID": _, "tAI": TAI, "iE-Extensions"? extensions };
    GlobalENBID sequence { "pLMNidentity": PLMNIdentity, "eNB-ID": ENBID, "iE-Extensions"? extensions };
    GlobalGNBID sequence { "pLMNIdentity": PLMNIdentity, "gNB-ID": GNBID, "iE-Extensions"? extensions };
    GlobalLineID sequence { "globalLineIdentity": _, "lineType"? LineType, "iE-Extensions"? extensions };
    GlobalN3IWFID sequence { "pLMNIdentity": PLMNIdentity, "n3IWF-ID": N3IWFID, "iE-Extensions"? extensions };
    GlobalNgENBID sequence { "pLMNIdentity": PLMNIdentity, "ngENB-ID": NgENBID, "iE-Extensions"? extensions };
    GlobalRANNodeID choice { "globalGNB-ID": GlobalGNBID, "globalNgENB-ID": GlobalNgENBID, "globalN3IWF-ID": GlobalN3IWFID, "choice-Extensions": ie };
    GlobalTNGFID sequence { "pLMNIdentity": PLMNIdentity, "tNGF-ID": TNGFID, "iE-Extensions"? extensions };
    GlobalTWIFID sequence { "pLMNIdentity": PLMNIdentity, "tWIF-ID": TWIFID, "iE-Extensions"? extensions };
    GlobalWAGFID sequence { "pLMNIdentity": PLMNIdentity, "w-AGF-ID": WAGFID, "iE-Extensions"? extensions };
    HFCNodeIDNew sequence { "hFCNode-ID": _, "tAI": TAI, "iE-Extensions"? extensions };
    HLComActivate sequence { "nR-Paging-Long-eDRX-Information-for-RRC-INACTIVE": NRPagingLongEDRXInformationForRRCINACTIVE, "iE-Extensions"? extensions };
    HLComDeactivate sequence { "uEReachabilityIndication": HLComDeactivateUEReachabilityIndication, "iE-Extensions"? extensions };
    HOReport sequence { "handoverReportType": HOReportHandoverReportType, "handoverCause": Cause, "sourcecellCGI": NGRANCGI, "targetcellCGI": NGRANCGI, "reestablishmentcellCGI"? NGRANCGI, "sourcecellC-RNTI"? bits, "targetcellinE-UTRAN"? EUTRACGI, "mobilityInformation"? _, "uERLFReportContainer"? UERLFReportContainer, "iE-Extensions"? extensions };
    HandoverCancel sequence { "protocolIEs": ies };
    HandoverCancelAcknowledge sequence { "protocolIEs": ies };
    HandoverCommand sequence { "protocolIEs": ies };
    HandoverCommandTransfer sequence { "dLForwardingUP-TNLInformation"? UPTransportLayerInformation, "qosFlowToBeForwardedList"? QosFlowToBeForwardedList, "dataForwardingResponseDRBList"? DataForwardingResponseDRBList, "iE-Extensions"? extensions };
    HandoverFailure sequence { "protocolIEs": ies };
    HandoverNotify sequence { "protocolIEs": ies };
    HandoverPreparationFailure sequence { "protocolIEs": ies };
    HandoverPreparationUnsuccessfulTransfer sequence { "cause": Cause, "iE-Extensions"? extensions };
    HandoverRequest sequence { "protocolIEs": ies };
    HandoverRequestAcknowledge sequence { "protocolIEs": ies };
    HandoverRequestAcknowledgeTransfer sequence { "dL-NGU-UP-TNLInformation": UPTransportLayerInformation, "dLForwardingUP-TNLInformation"? UPTransportLayerInformation, "securityResult"? SecurityResult, "qosFlowSetupResponseList": QosFlowListWithDataForwarding, "qosFlowFailedToSetupList"? QosFlowListWithCause, "dataForwardingResponseDRBList"? DataForwardingResponseDRBList, "iE-Extensions"? extensions };
    HandoverRequired sequence { "protocolIEs": ies };
    HandoverRequiredTransfer sequence { "directForwardingPathAvailability"? DirectForwardingPathAvailability, "iE-Extensions"? extensions };
    HandoverResourceAllocationUnsuccessfulTransfer sequence { "cause": Cause, "criticalityDiagnostics"? CriticalityDiagnostics, "iE-Extensions"? extensions };
    HandoverSuccess sequence { "protocolIEs": ies };
    HashedUEIdentityIndexValue = bits;
    ImmediateMDTNr sequence { "measurementsToActivate": _, "m1Configuration"? M1Configuration, "m4Configuration"? M4Configuration, "m5Configuration"? M5Configuration, "m6Configuration"? M6Configuration, "m7Configuration"? M7Configuration, "bluetoothMeasurementConfiguration"? BluetoothMeasurementConfiguration, "wLANMeasurementConfiguration"? WLANMeasurementConfiguration, "mDT-Location-Info"? MDTLocationInfo, "sensorMeasurementConfiguration"? SensorMeasurementConfiguration, "iE-Extensions"? extensions };
    InfoOnRecommendedCellsAndRANNodesForPaging sequence { "recommendedCellsForPaging": RecommendedCellsForPaging, "recommendRANNodesForPaging": RecommendedRANNodesForPaging, "iE-Extensions"? extensions };
    InitialContextSetupFailure sequence { "protocolIEs": ies };
    InitialContextSetupRequest sequence { "protocolIEs": ies };
    InitialContextSetupResponse sequence { "protocolIEs": ies };
    InitialUEMessage sequence { "protocolIEs": ies };
    InitiatingMessage sequence { "procedureCode": _, "criticality": Criticality, "value": _ };
    InterSystemFailureIndication sequence { "uERLFReportContainer"? UERLFReportContainer, "iE-Extensions"? extensions };
    InterSystemHOReport sequence { "handoverReportType": InterSystemHandoverReportType, "iE-Extensions"? extensions };
    InterSystemHandoverReportType choice { "tooearlyIntersystemHO": TooearlyIntersystemHO, "intersystemUnnecessaryHO": IntersystemUnnecessaryHO, "choice-Extensions": ie };
    IntersystemCellActivationReply sequence { "activatedCellList": ActivatedCellList, "activation-ID": _, "iE-Extensions"? extensions };
    IntersystemCellActivationRequest sequence { "activationID": _, "cellsToActivateList": CellsToActivateList, "iE-Extensions"? extensions };
    IntersystemCellStateIndication sequence { "notificationCellList": NotificationCellList, "iE-Extensions"? extensions };
    IntersystemMobilityFailureforVoiceFallback sequence { "sourcecellID": NGRANCGI, "targetcellID": EUTRACGI, "reconnectCellID"? EUTRACGI, "uERLFReportContainer"? UERLFReportContainer, "iE-Extensions"? extensions };
    IntersystemResourceStatusReply sequence { "reportingsystem": ReportingSystem, "iE-Extensions"? extensions };
    IntersystemResourceStatusReport sequence { "reportingSystem": ResourceStatusReportingSystem, "iE-Extensions"? extensions };
    IntersystemResourceStatusRequest sequence { "reportingSystem": ReportingSystem, "reportCharacteristics": _, "reportType": ReportType, "iE-Extensions"? extensions };
    IntersystemSONConfigurationTransfer sequence { "transferType": IntersystemSONTransferType, "intersystemSONInformation": IntersystemSONInformation, "iE-Extensions"? extensions };
    IntersystemSONInformation choice { "intersystemSONInformationReport": IntersystemSONInformationReport, "choice-Extensions": ie };
    IntersystemSONInformationReply choice { "nGRAN-CellActivation": IntersystemCellActivationReply, "resourceStatus": IntersystemResourceStatusReply, "choice-Extensions": ie };
    IntersystemSONInformationReport choice { "hOReportInformation": InterSystemHOReport, "failureIndicationInformation": InterSystemFailureIndication, "choice-Extensions": ie };
    IntersystemSONInformationRequest choice { "nGRAN-CellActivation": IntersystemCellActivationRequest, "resourceStatus": IntersystemResourceStatusRequest, "choice-Extensions": ie };
    IntersystemSONNGRANnodeID sequence { "globalRANNodeID": GlobalRANNodeID, "selectedTAI": TAI, "iE-Extensions"? extensions };
    IntersystemSONTransferType choice { "fromEUTRANtoNGRAN": FromEUTRANtoNGRAN, "fromNGRANtoEUTRAN": FromNGRANtoEUTRAN, "choice-Extensions": ie };
    IntersystemSONeNBID sequence { "globaleNBID": GlobalENBID, "selectedEPSTAI": EPSTAI, "iE-Extensions"? extensions };
    IntersystemUnnecessaryHO sequence { "sourcecellID": NGRANCGI, "targetcellID": EUTRACGI, "earlyIRATHO": IntersystemUnnecessaryHOEarlyIRATHO, "candidateCellList": CandidateCellList, "iE-Extensions"? extensions };
    InventoryFailure sequence { "protocolIEs": ies };
    InventoryFailureTransfer sequence { "correlationIdentifier": _, "cause": Cause, "criticalityDiagnostics"? CriticalityDiagnostics, "iE-Extensions"? extensions };
    InventoryReport sequence { "protocolIEs": ies };
    InventoryReportTransfer sequence { "correlationIdentifier": _, "globalgNB-ID": GlobalGNBID, "readerReportList"? AIoTReaderReportList, "inventoryCompleteIndication"? InventoryReportTransferInventoryCompleteIndication, "iE-Extensions"? extensions };
    InventoryRequest sequence { "protocolIEs": ies };
    InventoryRequestTransfer sequence { "protocolIEs": ies };
    InventoryResponse sequence { "protocolIEs": ies };
    InventoryResponseTransfer sequence { "correlationIdentifier": _, "criticalityDiagnostics"? CriticalityDiagnostics, "iE-Extensions"? extensions };
    LAI sequence { "pLMNidentity": PLMNIdentity, "lAC": LAC, "iE-Extensions"? extensions };
    LPWUSPSAssistanceInformation sequence { "lPWUScNsubgroupID": _, "iE-Extensions"? extensions };
    LTEA2XServicesAuthorized sequence { "aerialUE"? LTEA2XServicesAuthorizedAerialUE, "aerialControllerUE"? LTEA2XServicesAuthorizedAerialControllerUE, "iE-Extensions"? extensions };
    LTEUESidelinkAggregateMaximumBitrate sequence { "uESidelinkAggregateMaximumBitRate": _, "iE-Extensions"? extensions };
    LTEV2XServicesAuthorized sequence { "vehicleUE"? VehicleUE, "pedestrianUE"? PedestrianUE, "iE-Extensions"? extensions };
    LastVisitedCellInformation choice { "nGRANCell": LastVisitedNGRANCellInformation, "eUTRANCell": _, "uTRANCell": _, "gERANCell": _, "choice-Extensions": ie };
    LastVisitedCellItem sequence { "lastVisitedCellInformation": LastVisitedCellInformation, "iE-Extensions"? extensions };
    LastVisitedNGRANCellInformation sequence { "globalCellID": NGRANCGI, "cellType": CellType, "timeUEStayedInCell": _, "timeUEStayedInCellEnhancedGranularity"? _, "hOCauseValue"? Cause, "iE-Extensions"? extensions };
    LastVisitedPSCellInformation sequence { "pSCellID"? NGRANCGI, "timeStay": _, "iE-Extensions"? extensions };
    LastVisitedPSCellList = [LastVisitedPSCellInformation];
    LocationReport sequence { "protocolIEs": ies };
    LocationReportingControl sequence { "protocolIEs": ies };
    LocationReportingFailureIndication sequence { "protocolIEs": ies };
    LocationReportingRequestType sequence { "eventType": EventType, "reportArea": ReportArea, "areaOfInterestList"? AreaOfInterestList, "locationReportingReferenceIDToBeCancelled"? _, "iE-Extensions"? extensions };
    LoggedMDTNr sequence { "loggingInterval": LoggingInterval, "loggingDuration": LoggingDuration, "loggedMDTTrigger": LoggedMDTTrigger, "bluetoothMeasurementConfiguration"? BluetoothMeasurementConfiguration, "wLANMeasurementConfiguration"? WLANMeasurementConfiguration, "sensorMeasurementConfiguration"? SensorMeasurementConfiguration, "areaScopeOfNeighCellsList"? AreaScopeOfNeighCellsList, "iE-Extensions"? extensions };
    LoggedMDTTrigger choice { "periodical": null, "eventTrigger": EventTrigger, "choice-Extensions": ie };
    M1Configuration sequence { "m1reportingTrigger": M1ReportingTrigger, "m1thresholdEventA2"? M1ThresholdEventA2, "m1periodicReporting"? M1PeriodicReporting, "iE-Extensions"? extensions };
    M1PeriodicReporting sequence { "reportInterval": ReportIntervalMDT, "reportAmount": ReportAmountMDT, "iE-Extensions"? extensions };
    M1ThresholdEventA2 sequence { "m1ThresholdType": M1ThresholdType, "iE-Extensions"? extensions };
    M1ThresholdType choice { "threshold-RSRP": _, "threshold-RSRQ": _, "threshold-SINR": _, "choice-Extensions": ie };
    M4Configuration sequence { "m4period": M4period, "m4-links-to-log": LinksToLog, "iE-Extensions"? extensions };
    M5Configuration sequence { "m5period": M5period, "m5-links-to-log": LinksToLog, "iE-Extensions"? extensions };
    M6Configuration sequence { "m6report-Interval": M6reportInterval, "m6-links-to-log": LinksToLog, "iE-Extensions"? extensions };
    M7Configuration sequence { "m7period": _, "m7-links-to-log": LinksToLog, "iE-Extensions"? extensions };
    MBSActiveSessionInformationSourcetoTargetItem sequence { "mBS-SessionID": MBSSessionID, "mBS-AreaSessionID"? _, "mBS-ServiceArea"? MBSServiceArea, "mBS-QoSFlowsToBeSetupList": MBSQoSFlowsToBeSetupList, "mBS-MappingandDataForwardingRequestList"? MBSMappingandDataForwardingRequestList, "iE-Extensions"? extensions };
    MBSActiveSessionInformationSourcetoTargetList = [MBSActiveSessionInformationSourcetoTargetItem];
    MBSActiveSessionInformationTargettoSourceItem sequence { "mBS-SessionID": MBSSessionID, "mBS-DataForwardingResponseMRBList"? MBSDataForwardingResponseMRBList, "iE-Extensions"? extensions };
    MBSActiveSessionInformationTargettoSourceList = [MBSActiveSessionInformationTargettoSourceItem];
    MBSAreaTAIList = [TAI];
    MBSDataForwardingResponseMRBItem sequence { "mRB-ID": _, "dL-Forwarding-UPTNLInformation": UPTransportLayerInformation, "mRB-ProgressInformation"? MRBProgressInformation, "iE-Extensions"? extensions };
    MBSDataForwardingResponseMRBList = [MBSDataForwardingResponseMRBItem];
    MBSDistributionReleaseRequestTransfer sequence { "mBS-SessionID": MBSSessionID, "mBS-AreaSessionID"? _, "sharedNGU-UnicastTNLInformation"? UPTransportLayerInformation, "cause": Cause, "iE-Extensions"? extensions };
    MBSDistributionSetupRequestTransfer sequence { "mBS-SessionID": MBSSessionID, "mBS-AreaSessionID"? _, "sharedNGU-UnicastTNLInformation"? UPTransportLayerInformation, "iE-Extensions"? extensions };
    MBSDistributionSetupResponseTransfer sequence { "mBS-SessionID": MBSSessionID, "mBS-AreaSessionID"? _, "sharedNGU-MulticastTNLInformation"? SharedNGUMulticastTNLInformation, "mBS-QoSFlowsToBeSetupList": MBSQoSFlowsToBeSetupList, "mBSSessionStatus": MBSSessionStatus, "mBS-ServiceArea"? MBSServiceArea, "iE-Extensions"? extensions };
    MBSDistributionSetupUnsuccessfulTransfer sequence { "mBS-SessionID": MBSSessionID, "mBS-AreaSessionID"? _, "cause": Cause, "criticalityDiagnostics"? CriticalityDiagnostics, "iE-Extensions"? extensions };
    MBSIntendedServiceAreaList = [_];
    MBSMappingandDataForwardingRequestItem sequence { "mRB-ID": _, "mBS-QoSFlowList": MBSQoSFlowList, "mRB-ProgressInformation"? MRBProgressInformation, "iE-Extensions"? extensions };
    MBSMappingandDataForwardingRequestList = [MBSMappingandDataForwardingRequestItem];
    MBSNGUFailureIndication choice { "locationindependent": MBSUPFailureIndication, "locationdependent": MBSUPFailureIndicationList, "choice-Extensions": ie };
    MBSQoSFlowList = [_];
    MBSQoSFlowsToBeSetupItem sequence { "mBSqosFlowIdentifier": _, "mBSqosFlowLevelQosParameters": QosFlowLevelQosParameters, "iE-Extensions"? extensions };
    MBSQoSFlowsToBeSetupList = [MBSQoSFlowsToBeSetupItem];
    MBSServiceArea choice { "locationindependent": MBSServiceAreaInformation, "locationdependent": MBSServiceAreaInformationList, "choice-Extensions": ie };
    MBSServiceAreaCellList = [NRCGI];
    MBSServiceAreaInformation sequence { "mBS-ServiceAreaCellList"? MBSServiceAreaCellList, "mBS-ServiceAreaTAIList"? MBSServiceAreaTAIList, "iE-Extensions"? extensions };
    MBSServiceAreaInformationItem sequence { "mBS-AreaSessionID": _, "mBS-ServiceAreaInformation": MBSServiceAreaInformation, "iE-Extensions"? extensions };
    MBSServiceAreaInformationList = [MBSServiceAreaInformationItem];
    MBSServiceAreaTAIList = [TAI];
    MBSSessionFSAIDList = [_];
    MBSSessionFailedtoSetupItem sequence { "mBS-SessionID": MBSSessionID, "mBS-AreaSessionID"? _, "cause": Cause, "iE-Extensions"? extensions };
    MBSSessionFailedtoSetupList = [MBSSessionFailedtoSetupItem];
    MBSSessionID sequence { "tMGI": _, "nID"? _, "iE-Extensions"? extensions };
    MBSSessionReleaseResponseTransfer sequence { "mBS-SessionTNLInfoNGRAN"? MBSSessionTNLInfoNGRAN, "iE-Extensions"? extensions };
    MBSSessionSetupOrModFailureTransfer sequence { "cause": Cause, "criticalityDiagnostics"? CriticalityDiagnostics, "iE-Extensions"? extensions };
    MBSSessionSetupOrModRequestTransfer sequence { "protocolIEs": ies };
    MBSSessionSetupOrModResponseTransfer sequence { "mBS-SessionTNLInfoNGRAN"? MBSSessionTNLInfoNGRAN, "iE-Extensions"? extensions };
    MBSSessionSetupRequestItem sequence { "mBS-SessionID": MBSSessionID, "mBS-AreaSessionID"? _, "associatedMBSQosFlowSetupRequestList"? AssociatedMBSQosFlowSetupRequestList, "iE-Extensions"? extensions };
    MBSSessionSetupRequestList = [MBSSessionSetupRequestItem];
    MBSSessionSetupResponseItem sequence { "mBS-SessionID": MBSSessionID, "mBS-AreaSessionID"? _, "iE-Extensions"? extensions };
    MBSSessionSetupResponseList = [MBSSessionSetupResponseItem];
    MBSSessionSetuporModifyRequestItem sequence { "mBS-SessionID": MBSSessionID, "mBS-AreaSessionID"? _, "associatedMBSQosFlowSetuporModifyRequestList"? AssociatedMBSQosFlowSetuporModifyRequestList, "mBS-QosFlowToReleaseList"? QosFlowListWithCause, "iE-Extensions"? extensions };
    MBSSessionSetuporModifyRequestList = [MBSSessionSetuporModifyRequestItem];
    MBSSessionTNLInfo5GC choice { "locationindependent": SharedNGUMulticastTNLInformation, "locationdependent": MBSSessionTNLInfo5GCList, "choice-Extensions": ie };
    MBSSessionTNLInfo5GCItem sequence { "mBS-AreaSessionID": _, "sharedNGU-MulticastTNLInformation": SharedNGUMulticastTNLInformation, "iE-Extensions"? extensions };
    MBSSessionTNLInfo5GCList = [MBSSessionTNLInfo5GCItem];
    MBSSessionTNLInfoNGRAN choice { "locationindependent": UPTransportLayerInformation, "locationdependent": MBSSessionTNLInfoNGRANList, "choice-Extensions": ie };
    MBSSessionTNLInfoNGRANItem sequence { "mBS-AreaSessionID": _, "sharedNGU-UnicastTNLInformation"? UPTransportLayerInformation, "iE-Extensions"? extensions };
    MBSSessionTNLInfoNGRANList = [MBSSessionTNLInfoNGRANItem];
    MBSSessionToReleaseItem sequence { "mBS-SessionID": MBSSessionID, "cause": Cause, "iE-Extensions"? extensions };
    MBSSessionToReleaseList = [MBSSessionToReleaseItem];
    MBSUPFailureIndicationItem sequence { "mBS-AreaSessionID": _, "mBS-UP-FailureIndication": MBSUPFailureIndication, "iE-Extensions"? extensions };
    MBSUPFailureIndicationList = [MBSUPFailureIndicationItem];
    MDTAlignmentInfo choice { "s-basedMDT": _, "choice-Extensions": ie };
    MDTCircle sequence { "reference-location": _, "distance-radius": _, "iE-Extensions"? extensions };
    MDTConfiguration sequence { "mdt-Config-NR"? MDTConfigurationNR, "mdt-Config-EUTRA"? MDTConfigurationEUTRA, "iE-Extensions"? extensions };
    MDTConfigurationEUTRA sequence { "mdt-Activation": MDTActivation, "areaScopeOfMDT": AreaScopeOfMDTEUTRA, "mDTMode": _, "signallingBasedMDTPLMNList"? MDTPLMNList, "iE-Extensions"? extensions };
    MDTConfigurationNR sequence { "mdt-Activation": MDTActivation, "areaScopeOfMDT": AreaScopeOfMDTNR, "mDTModeNr": MDTModeNr, "signallingBasedMDTPLMNList"? MDTPLMNList, "iE-Extensions"? extensions };
    MDTLocationInfo sequence { "mDT-Location-Information": _, "iE-Extensions"? extensions };
    MDTModeNr choice { "immediateMDTNr": ImmediateMDTNr, "loggedMDTNr": LoggedMDTNr, "choice-Extensions": ie };
    MDTPLMNList = [PLMNIdentity];
    MDTPLMNModificationList = [PLMNIdentity];
    MRBProgressInformation choice { "pDCP-SN-Length12": _, "pDCP-SN-Length18": _, "choice-Extensions": ie };
    MTCommunicationHandlingFailure sequence { "protocolIEs": ies };
    MTCommunicationHandlingRequest sequence { "protocolIEs": ies };
    MTCommunicationHandlingResponse sequence { "protocolIEs": ies };
    MeasurementThresholdL1LoggedMDT choice { "threshold-RSRP": _, "threshold-RSRQ": _, "choice-Extensions": ie };
    MobileIABMTUserLocationInformation sequence { "nRCGI": NRCGI, "tAI": TAI, "iE-Extensions"? extensions };
    MobilityRestrictionList sequence { "servingPLMN": PLMNIdentity, "equivalentPLMNs"? EquivalentPLMNs, "rATRestrictions"? RATRestrictions, "forbiddenAreaInformation"? ForbiddenAreaInformation, "serviceAreaInformation"? ServiceAreaInformation, "iE-Extensions"? extensions };
    MonitoringRequestonAvailableBitrate sequence { "monitoringRequest": MonitoringRequest, "dlAvailableBitrateReportThresholds"? AvailableBitrateReportThresholdList, "ulAvailableBitrateReportThresholds"? AvailableBitrateReportThresholdList, "iE-Extensions"? extensions };
    MulticastGroupPaging sequence { "protocolIEs": ies };
    MulticastGroupPagingArea sequence { "mBS-AreaTAIList": MBSAreaTAIList, "iE-Extensions"? extensions };
    MulticastGroupPagingAreaItem sequence { "multicastGroupPagingArea": MulticastGroupPagingArea, "uE-PagingList"? UEPagingList, "iE-Extensions"? extensions };
    MulticastGroupPagingAreaList = [MulticastGroupPagingAreaItem];
    MulticastSessionActivationFailure sequence { "protocolIEs": ies };
    MulticastSessionActivationRequest sequence { "protocolIEs": ies };
    MulticastSessionActivationRequestTransfer sequence { "mBS-SessionID": MBSSessionID, "iE-Extensions"? extensions };
    MulticastSessionActivationResponse sequence { "protocolIEs": ies };
    MulticastSessionDeactivationRequest sequence { "protocolIEs": ies };
    MulticastSessionDeactivationRequestTransfer sequence { "mBS-SessionID": MBSSessionID, "iE-Extensions"? extensions };
    MulticastSessionDeactivationResponse sequence { "protocolIEs": ies };
    MulticastSessionUpdateFailure sequence { "protocolIEs": ies };
    MulticastSessionUpdateRequest sequence { "protocolIEs": ies };
    MulticastSessionUpdateRequestTransfer sequence { "protocolIEs": ies };
    MulticastSessionUpdateResponse sequence { "protocolIEs": ies };
    N3IWFID choice { "n3IWF-ID": _, "choice-Extensions": ie };
    N6JitterInformation sequence { "n6JitterLowerBound": _, "n6JitterUpperBound": _, "iE-Extensions"? extensions };
    NASNonDeliveryIndication sequence { "protocolIEs": ies };
    NBIoTPagingEDRXInfo sequence { "nB-IoT-Paging-eDRXCycle": NBIoTPagingEDRXCycle, "nB-IoT-Paging-TimeWindow"? NBIoTPagingTimeWindow, "iE-Extensions"? extensions };
    NGAPIESupportInformationRequestItem sequence { "ngap-ProtocolIE-Id": _, "iE-Extensions"? extensions };
    NGAPIESupportInformationRequestList = [NGAPIESupportInformationRequestItem];
    NGAPIESupportInformationResponseItem sequence { "ngap-ProtocolIE-Id": _, "ngap-ProtocolIESupportInfo": NGAPIESupportInformationResponseItemNgapProtocolIESupportInfo, "ngap-ProtocolIEPresenceInfo": NGAPIESupportInformationResponseItemNgapProtocolIEPresenceInfo, "iE-Extensions"? extensions };
    NGAPIESupportInformationResponseList = [NGAPIESupportInformationResponseItem];
    NGAPPDU choice { "initiatingMessage": InitiatingMessage, "successfulOutcome": SuccessfulOutcome, "unsuccessfulOutcome": UnsuccessfulOutcome };
    NGRANCGI choice { "nR-CGI": NRCGI, "eUTRA-CGI": EUTRACGI, "choice-Extensions": ie };
    NGRANCellReportItem sequence { "nGRAN-CGI": NGRANCGI, "nGRAN-CompositeAvailableCapacityGroup": EUTRANCompositeAvailableCapacityGroup, "nGRAN-NumberOfActiveUEs"? _, "nGRAN-NoofRRCConnections"? _, "nGRAN-RadioResourceStatus"? NGRANRadioResourceStatus, "iE-Extensions"? extensions };
    NGRANCellReportList = [NGRANCellReportItem];
    NGRANCellToReportItem sequence { "nGRAN-CGI": NGRANCGI, "iE-Extensions"? extensions };
    NGRANCellToReportList = [NGRANCellToReportItem];
    NGRANRadioResourceStatus sequence { "dL-GBR-PRB-usage-for-MIMO": _, "uL-GBR-PRB-usage-for-MIMO": _, "dL-non-GBR-PRB-usage-for-MIMO": _, "uL-non-GBR-PRB-usage-for-MIMO": _, "dL-Total-PRB-usage-for-MIMO": _, "uL-Total-PRB-usage-for-MIMO": _, "iE-Extensions"? extensions };
    NGRANReportingStatusIEs sequence { "nGRAN-CellReportList": NGRANCellReportList, "iE-Extensions"? extensions };
    NGRANReportingSystemIEs sequence { "nGRAN-CellToReportList": NGRANCellToReportList, "iE-Extensions"? extensions };
    NGRANTNLAssociationToRemoveItem sequence { "tNLAssociationTransportLayerAddress": CPTransportLayerInformation, "tNLAssociationTransportLayerAddressAMF"? CPTransportLayerInformation, "iE-Extensions"? extensions };
    NGRANTNLAssociationToRemoveList = [NGRANTNLAssociationToRemoveItem];
    NGRemovalFailure sequence { "protocolIEs": ies };
    NGRemovalRequest sequence { "protocolIEs": ies };
    NGRemovalResponse sequence { "protocolIEs": ies };
    NGReset sequence { "protocolIEs": ies };
    NGResetAcknowledge sequence { "protocolIEs": ies };
    NGSetupFailure sequence { "protocolIEs": ies };
    NGSetupRequest sequence { "protocolIEs": ies };
    NGSetupResponse sequence { "protocolIEs": ies };
    NPNAccessInformation choice { "pNI-NPN-Access-Information": CellCAGList, "choice-Extensions": ie };
    NPNMobilityInformation choice { "sNPN-MobilityInformation": SNPNMobilityInformation, "pNI-NPN-MobilityInformation": PNINPNMobilityInformation, "choice-Extensions": ie };
    NPNPagingAssistanceInformation choice { "pNI-NPN-PagingAssistance": AllowedPNINPNList, "choice-Extensions": ie };
    NPNSupport choice { "sNPN": _, "choice-Extensions": ie };
    NRA2XServicesAuthorized sequence { "aerialUE"? NRA2XServicesAuthorizedAerialUE, "aerialControllerUE"? NRA2XServicesAuthorizedAerialControllerUE, "iE-Extensions"? extensions };
    NRCGI sequence { "pLMNIdentity": PLMNIdentity, "nRCellIdentity": NRCellIdentity, "iE-Extensions"? extensions };
    NRCGIList = [NRCGI];
    NRCGIListForWarning = [NRCGI];
    NRFrequencyBandItem sequence { "nr-frequency-band": _, "iE-Extension"? extensions };
    NRFrequencyBandList = [NRFrequencyBandItem];
    NRFrequencyInfo sequence { "nrARFCN": _, "frequencyBand-List": NRFrequencyBandList, "iE-Extension"? extensions };
    NRNTNTAIInformation sequence { "servingPLMN": PLMNIdentity, "tACListInNRNTN": TACListInNRNTN, "uELocationDerivedTACInNRNTN"? TAC, "iE-Extensions"? extensions };
    NRPagingLongEDRXInformationForRRCINACTIVE sequence { "nR-paging-Long-eDRX-Cycle-for-RRC-INACTIVE": NRPagingLongEDRXCycleForRRCINACTIVE, "nR-paging-Time-Window-for-RRC-INACTIVE": NRPagingTimeWindowForRRCINACTIVE, "iE-Extensions"? extensions };
    NRPagingeDRXInformation sequence { "nR-paging-eDRX-Cycle": NRPagingEDRXCycle, "nR-paging-Time-Window"? NRPagingTimeWindow, "iE-Extensions"? extensions };
    NRUESidelinkAggregateMaximumBitrate sequence { "uESidelinkAggregateMaximumBitRate": _, "iE-Extensions"? extensions };
    NRV2XServicesAuthorized sequence { "vehicleUE"? VehicleUE, "pedestrianUE"? PedestrianUE, "iE-Extensions"? extensions };
    NRencryptionAlgorithms = bits;
    NRintegrityProtectionAlgorithms = bits;
    NTNGeographicalAreaItem choice { "circle": MDTCircle, "polygon": _, "choice-Extensions": ie };
    NTNGeographicalAreaList = [NTNGeographicalAreaItem];
    NetworkSliceAreaScopeofMDT sequence { "networkSliceListforMDT": NetworkSliceListforMDT, "iE-Extensions"? extensions };
    NetworkSliceItemforMDT sequence { "plmnID": PLMNIdentity, "sliceMDTList": SliceMDTList, "iE-Extensions"? extensions };
    NetworkSliceListforMDT = [NetworkSliceItemforMDT];
    NgENBID choice { "macroNgENB-ID": _, "shortMacroNgENB-ID": _, "longMacroNgENB-ID": _, "choice-Extensions": ie };
    NonDynamic5QIDescriptor sequence { "fiveQI": _, "priorityLevelQos"? _, "averagingWindow"? _, "maximumDataBurstVolume"? _, "iE-Extensions"? extensions };
    NotAllowedTACs = [TAC];
    NotificationCellItem sequence { "nGRAN-CGI": NGRANCGI, "notifyFlag": NotificationCellItemNotifyFlag, "iE-Extensions"? extensions };
    NotificationCellList = [NotificationCellItem];
    OverloadResponse choice { "overloadAction": OverloadAction, "choice-Extensions": ie };
    OverloadStart sequence { "protocolIEs": ies };
    OverloadStartNSSAIItem sequence { "sliceOverloadList": SliceOverloadList, "sliceOverloadResponse"? OverloadResponse, "sliceTrafficLoadReductionIndication"? _, "iE-Extensions"? extensions };
    OverloadStartNSSAIList = [OverloadStartNSSAIItem];
    OverloadStop sequence { "protocolIEs": ies };
    PC5FlowBitRates sequence { "guaranteedFlowBitRate": _, "maximumFlowBitRate": _, "iE-Extensions"? extensions };
    PC5QoSFlowItem sequence { "pQI": _, "pc5FlowBitRates"? PC5FlowBitRates, "range"? Range, "iE-Extensions"? extensions };
    PC5QoSFlowList = [PC5QoSFlowItem];
    PC5QoSParameters sequence { "pc5QoSFlowList": PC5QoSFlowList, "pc5LinkAggregateBitRates"? _, "iE-Extensions"? extensions };
    PCIListForMDT = [_];
    PDUSessionAggregateMaximumBitRate sequence { "pDUSessionAggregateMaximumBitRateDL": _, "pDUSessionAggregateMaximumBitRateUL": _, "iE-Extensions"? extensions };
    PDUSessionForPagingItem sequence { "pDUSessionID": _, "pagingPolicyDifferentiationList": PagingPolicyDifferentiationList, "iE-Extensions"? extensions };
    PDUSessionForPagingList = [PDUSessionForPagingItem];
    PDUSessionListMTCommHReq = [PDUSessionListMTCommHReqItem];
    PDUSessionListMTCommHReqItem sequence { "pDUSessionID": _, "iE-Extensions"? extensions };
    PDUSessionResourceAdmittedItem sequence { "pDUSessionID": _, "handoverRequestAcknowledgeTransfer": (HandoverRequestAcknowledgeTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceAdmittedList = [PDUSessionResourceAdmittedItem];
    PDUSessionResourceFailedToModifyItemModCfm sequence { "pDUSessionID": _, "pDUSessionResourceModifyIndicationUnsuccessfulTransfer": (PDUSessionResourceModifyIndicationUnsuccessfulTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceFailedToModifyItemModRes sequence { "pDUSessionID": _, "pDUSessionResourceModifyUnsuccessfulTransfer": (PDUSessionResourceModifyUnsuccessfulTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceFailedToModifyListModCfm = [PDUSessionResourceFailedToModifyItemModCfm];
    PDUSessionResourceFailedToModifyListModRes = [PDUSessionResourceFailedToModifyItemModRes];
    PDUSessionResourceFailedToResumeItemRESReq sequence { "pDUSessionID": _, "cause": Cause, "iE-Extensions"? extensions };
    PDUSessionResourceFailedToResumeItemRESRes sequence { "pDUSessionID": _, "cause": Cause, "iE-Extensions"? extensions };
    PDUSessionResourceFailedToResumeListRESReq = [PDUSessionResourceFailedToResumeItemRESReq];
    PDUSessionResourceFailedToResumeListRESRes = [PDUSessionResourceFailedToResumeItemRESRes];
    PDUSessionResourceFailedToSetupItemCxtFail sequence { "pDUSessionID": _, "pDUSessionResourceSetupUnsuccessfulTransfer": (PDUSessionResourceSetupUnsuccessfulTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceFailedToSetupItemCxtRes sequence { "pDUSessionID": _, "pDUSessionResourceSetupUnsuccessfulTransfer": (PDUSessionResourceSetupUnsuccessfulTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceFailedToSetupItemHOAck sequence { "pDUSessionID": _, "handoverResourceAllocationUnsuccessfulTransfer": (HandoverResourceAllocationUnsuccessfulTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceFailedToSetupItemPSReq sequence { "pDUSessionID": _, "pathSwitchRequestSetupFailedTransfer": (PathSwitchRequestSetupFailedTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceFailedToSetupItemSURes sequence { "pDUSessionID": _, "pDUSessionResourceSetupUnsuccessfulTransfer": (PDUSessionResourceSetupUnsuccessfulTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceFailedToSetupListCxtFail = [PDUSessionResourceFailedToSetupItemCxtFail];
    PDUSessionResourceFailedToSetupListCxtRes = [PDUSessionResourceFailedToSetupItemCxtRes];
    PDUSessionResourceFailedToSetupListHOAck = [PDUSessionResourceFailedToSetupItemHOAck];
    PDUSessionResourceFailedToSetupListPSReq = [PDUSessionResourceFailedToSetupItemPSReq];
    PDUSessionResourceFailedToSetupListSURes = [PDUSessionResourceFailedToSetupItemSURes];
    PDUSessionResourceHandoverItem sequence { "pDUSessionID": _, "handoverCommandTransfer": (HandoverCommandTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceHandoverList = [PDUSessionResourceHandoverItem];
    PDUSessionResourceInformationItem sequence { "pDUSessionID": _, "qosFlowInformationList": QosFlowInformationList, "dRBsToQosFlowsMappingList"? DRBsToQosFlowsMappingList, "iE-Extensions"? extensions };
    PDUSessionResourceInformationList = [PDUSessionResourceInformationItem];
    PDUSessionResourceItemCxtRelCpl sequence { "pDUSessionID": _, "iE-Extensions"? extensions };
    PDUSessionResourceItemCxtRelReq sequence { "pDUSessionID": _, "iE-Extensions"? extensions };
    PDUSessionResourceItemHORqd sequence { "pDUSessionID": _, "handoverRequiredTransfer": (HandoverRequiredTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceListCxtRelCpl = [PDUSessionResourceItemCxtRelCpl];
    PDUSessionResourceListCxtRelReq = [PDUSessionResourceItemCxtRelReq];
    PDUSessionResourceListHORqd = [PDUSessionResourceItemHORqd];
    PDUSessionResourceModifyConfirm sequence { "protocolIEs": ies };
    PDUSessionResourceModifyConfirmTransfer sequence { "qosFlowModifyConfirmList": QosFlowModifyConfirmList, "uLNGU-UP-TNLInformation": UPTransportLayerInformation, "additionalNG-UUPTNLInformation"? UPTransportLayerInformationPairList, "qosFlowFailedToModifyList"? QosFlowListWithCause, "iE-Extensions"? extensions };
    PDUSessionResourceModifyIndication sequence { "protocolIEs": ies };
    PDUSessionResourceModifyIndicationTransfer sequence { "dLQosFlowPerTNLInformation": QosFlowPerTNLInformation, "additionalDLQosFlowPerTNLInformation"? QosFlowPerTNLInformationList, "iE-Extensions"? extensions };
    PDUSessionResourceModifyIndicationUnsuccessfulTransfer sequence { "cause": Cause, "iE-Extensions"? extensions };
    PDUSessionResourceModifyItemModCfm sequence { "pDUSessionID": _, "pDUSessionResourceModifyConfirmTransfer": (PDUSessionResourceModifyConfirmTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceModifyItemModInd sequence { "pDUSessionID": _, "pDUSessionResourceModifyIndicationTransfer": (PDUSessionResourceModifyIndicationTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceModifyItemModReq sequence { "pDUSessionID": _, "nAS-PDU"? _, "pDUSessionResourceModifyRequestTransfer": (PDUSessionResourceModifyRequestTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceModifyItemModRes sequence { "pDUSessionID": _, "pDUSessionResourceModifyResponseTransfer": (PDUSessionResourceModifyResponseTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceModifyListModCfm = [PDUSessionResourceModifyItemModCfm];
    PDUSessionResourceModifyListModInd = [PDUSessionResourceModifyItemModInd];
    PDUSessionResourceModifyListModReq = [PDUSessionResourceModifyItemModReq];
    PDUSessionResourceModifyListModRes = [PDUSessionResourceModifyItemModRes];
    PDUSessionResourceModifyRequest sequence { "protocolIEs": ies };
    PDUSessionResourceModifyRequestTransfer sequence { "protocolIEs": ies };
    PDUSessionResourceModifyResponse sequence { "protocolIEs": ies };
    PDUSessionResourceModifyResponseTransfer sequence { "dL-NGU-UP-TNLInformation"? UPTransportLayerInformation, "uL-NGU-UP-TNLInformation"? UPTransportLayerInformation, "qosFlowAddOrModifyResponseList"? QosFlowAddOrModifyResponseList, "additionalDLQosFlowPerTNLInformation"? QosFlowPerTNLInformationList, "qosFlowFailedToAddOrModifyList"? QosFlowListWithCause, "iE-Extensions"? extensions };
    PDUSessionResourceModifyUnsuccessfulTransfer sequence { "cause": Cause, "criticalityDiagnostics"? CriticalityDiagnostics, "iE-Extensions"? extensions };
    PDUSessionResourceNotify sequence { "protocolIEs": ies };
    PDUSessionResourceNotifyItem sequence { "pDUSessionID": _, "pDUSessionResourceNotifyTransfer": (PDUSessionResourceNotifyTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceNotifyList = [PDUSessionResourceNotifyItem];
    PDUSessionResourceNotifyReleasedTransfer sequence { "cause": Cause, "iE-Extensions"? extensions };
    PDUSessionResourceNotifyTransfer sequence { "qosFlowNotifyList"? QosFlowNotifyList, "qosFlowReleasedList"? QosFlowListWithCause, "iE-Extensions"? extensions };
    PDUSessionResourceReleaseCommand sequence { "protocolIEs": ies };
    PDUSessionResourceReleaseCommandTransfer sequence { "cause": Cause, "iE-Extensions"? extensions };
    PDUSessionResourceReleaseResponse sequence { "protocolIEs": ies };
    PDUSessionResourceReleaseResponseTransfer sequence { "iE-Extensions"? extensions };
    PDUSessionResourceReleasedItemNot sequence { "pDUSessionID": _, "pDUSessionResourceNotifyReleasedTransfer": (PDUSessionResourceNotifyReleasedTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceReleasedItemPSAck sequence { "pDUSessionID": _, "pathSwitchRequestUnsuccessfulTransfer": (PathSwitchRequestUnsuccessfulTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceReleasedItemPSFail sequence { "pDUSessionID": _, "pathSwitchRequestUnsuccessfulTransfer": (PathSwitchRequestUnsuccessfulTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceReleasedItemRelRes sequence { "pDUSessionID": _, "pDUSessionResourceReleaseResponseTransfer": (PDUSessionResourceReleaseResponseTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceReleasedListNot = [PDUSessionResourceReleasedItemNot];
    PDUSessionResourceReleasedListPSAck = [PDUSessionResourceReleasedItemPSAck];
    PDUSessionResourceReleasedListPSFail = [PDUSessionResourceReleasedItemPSFail];
    PDUSessionResourceReleasedListRelRes = [PDUSessionResourceReleasedItemRelRes];
    PDUSessionResourceResumeItemRESReq sequence { "pDUSessionID": _, "uEContextResumeRequestTransfer": (UEContextResumeRequestTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceResumeItemRESRes sequence { "pDUSessionID": _, "uEContextResumeResponseTransfer": (UEContextResumeResponseTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceResumeListRESReq = [PDUSessionResourceResumeItemRESReq];
    PDUSessionResourceResumeListRESRes = [PDUSessionResourceResumeItemRESRes];
    PDUSessionResourceSecondaryRATUsageItem sequence { "pDUSessionID": _, "secondaryRATDataUsageReportTransfer": (SecondaryRATDataUsageReportTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceSecondaryRATUsageList = [PDUSessionResourceSecondaryRATUsageItem];
    PDUSessionResourceSetupItemCxtReq sequence { "pDUSessionID": _, "nAS-PDU"? _, "s-NSSAI": SNSSAI, "pDUSessionResourceSetupRequestTransfer": (PDUSessionResourceSetupRequestTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceSetupItemCxtRes sequence { "pDUSessionID": _, "pDUSessionResourceSetupResponseTransfer": (PDUSessionResourceSetupResponseTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceSetupItemHOReq sequence { "pDUSessionID": _, "s-NSSAI": SNSSAI, "handoverRequestTransfer": (PDUSessionResourceSetupRequestTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceSetupItemSUReq sequence { "pDUSessionID": _, "pDUSessionNAS-PDU"? _, "s-NSSAI": SNSSAI, "pDUSessionResourceSetupRequestTransfer": (PDUSessionResourceSetupRequestTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceSetupItemSURes sequence { "pDUSessionID": _, "pDUSessionResourceSetupResponseTransfer": (PDUSessionResourceSetupResponseTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceSetupListCxtReq = [PDUSessionResourceSetupItemCxtReq];
    PDUSessionResourceSetupListCxtRes = [PDUSessionResourceSetupItemCxtRes];
    PDUSessionResourceSetupListHOReq = [PDUSessionResourceSetupItemHOReq];
    PDUSessionResourceSetupListSUReq = [PDUSessionResourceSetupItemSUReq];
    PDUSessionResourceSetupListSURes = [PDUSessionResourceSetupItemSURes];
    PDUSessionResourceSetupRequest sequence { "protocolIEs": ies };
    PDUSessionResourceSetupRequestTransfer sequence { "protocolIEs": ies };
    PDUSessionResourceSetupResponse sequence { "protocolIEs": ies };
    PDUSessionResourceSetupResponseTransfer sequence { "dLQosFlowPerTNLInformation": QosFlowPerTNLInformation, "additionalDLQosFlowPerTNLInformation"? QosFlowPerTNLInformationList, "securityResult"? SecurityResult, "qosFlowFailedToSetupList"? QosFlowListWithCause, "iE-Extensions"? extensions };
    PDUSessionResourceSetupUnsuccessfulTransfer sequence { "cause": Cause, "criticalityDiagnostics"? CriticalityDiagnostics, "iE-Extensions"? extensions };
    PDUSessionResourceSuspendItemSUSReq sequence { "pDUSessionID": _, "uEContextSuspendRequestTransfer": (UEContextSuspendRequestTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceSuspendListSUSReq = [PDUSessionResourceSuspendItemSUSReq];
    PDUSessionResourceSwitchedItem sequence { "pDUSessionID": _, "pathSwitchRequestAcknowledgeTransfer": (PathSwitchRequestAcknowledgeTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceSwitchedList = [PDUSessionResourceSwitchedItem];
    PDUSessionResourceToBeSwitchedDLItem sequence { "pDUSessionID": _, "pathSwitchRequestTransfer": (PathSwitchRequestTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceToBeSwitchedDLList = [PDUSessionResourceToBeSwitchedDLItem];
    PDUSessionResourceToReleaseItemHOCmd sequence { "pDUSessionID": _, "handoverPreparationUnsuccessfulTransfer": (HandoverPreparationUnsuccessfulTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceToReleaseItemRelCmd sequence { "pDUSessionID": _, "pDUSessionResourceReleaseCommandTransfer": (PDUSessionResourceReleaseCommandTransfer), "iE-Extensions"? extensions };
    PDUSessionResourceToReleaseListHOCmd = [PDUSessionResourceToReleaseItemHOCmd];
    PDUSessionResourceToReleaseListRelCmd = [PDUSessionResourceToReleaseItemRelCmd];
    PDUSessionUsageReport sequence { "rATType": PDUSessionUsageReportRATType, "pDUSessionTimedReportList": VolumeTimedReportList, "iE-Extensions"? extensions };
    PDUsetQoSInformation sequence { "pduSetDelayBudget"? _, "pduSetErrorRate"? PacketErrorRate, "pduSetIntegratedHandlingInformation"? PDUsetQoSInformationPduSetIntegratedHandlingInformation, "iE-Extensions"? extensions };
    PDUsetQoSParameters sequence { "ulPDUSetQoSInformation"? PDUsetQoSInformation, "dlPDUSetQoSInformation"? PDUsetQoSInformation, "iE-Extensions"? extensions };
    PEIPSassistanceInformation sequence { "cNsubgroupID": _, "iE-Extensions"? extensions };
    PLMNAreaBasedQMC sequence { "plmnListforQMC": PLMNListforQMC, "iE-Extensions"? extensions };
    PLMNListforQMC = [PLMNIdentity];
    PLMNSupportItem sequence { "pLMNIdentity": PLMNIdentity, "sliceSupportList": SliceSupportList, "iE-Extensions"? extensions };
    PLMNSupportList = [PLMNSupportItem];
    PNINPNAreaScopeofMDT sequence { "cAGListforMDT": CAGListforMDT, "iE-Extensions"? extensions };
    PNINPNBasedMDT sequence { "cAGListforMDT": CAGListforMDT, "iE-Extensions"? extensions };
    PNINPNMobilityInformation sequence { "allowed-PNI-NPI-List": AllowedPNINPNList, "iE-Extensions"? extensions };
    PWSCancelRequest sequence { "protocolIEs": ies };
    PWSCancelResponse sequence { "protocolIEs": ies };
    PWSFailedCellIDList choice { "eUTRA-CGI-PWSFailedList": EUTRACGIList, "nR-CGI-PWSFailedList": NRCGIList, "choice-Extensions": ie };
    PWSFailureIndication sequence { "protocolIEs": ies };
    PWSRestartIndication sequence { "protocolIEs": ies };
    PacketErrorRate sequence { "pERScalar": _, "pERExponent": _, "iE-Extensions"? extensions };
    Paging sequence { "protocolIEs": ies };
    PagingAssisDataforCEcapabUE sequence { "eUTRA-CGI": EUTRACGI, "coverageEnhancementLevel": _, "iE-Extensions"? extensions };
    PagingAttemptInformation sequence { "pagingAttemptCount": _, "intendedNumberOfPagingAttempts": _, "nextPagingAreaScope"? NextPagingAreaScope, "iE-Extensions"? extensions };
    PagingPolicyDifferentiation sequence { "pDUSessionForPagingList": PDUSessionForPagingList, "iE-Extensions"? extensions };
    PagingPolicyDifferentiationItem sequence { "qosFlowIdentifier"? _, "pagingPolicyIndicator"? _, "allocationAndRetentionPriority"? AllocationAndRetentionPriority, "fiveQI"? _, "dl-DataSize"? _, "iE-Extensions"? extensions };
    PagingPolicyDifferentiationList = [PagingPolicyDifferentiationItem];
    PartiallyAllowedNSSAI = [PartiallyAllowedNSSAIItem];
    PartiallyAllowedNSSAIItem sequence { "s-NSSAI": SNSSAI, "iE-Extensions"? extensions };
    PathSwitchRequest sequence { "protocolIEs": ies };
    PathSwitchRequestAcknowledge sequence { "protocolIEs": ies };
    PathSwitchRequestAcknowledgeTransfer sequence { "uL-NGU-UP-TNLInformation"? UPTransportLayerInformation, "securityIndication"? SecurityIndication, "iE-Extensions"? extensions };
    PathSwitchRequestFailure sequence { "protocolIEs": ies };
    PathSwitchRequestSetupFailedTransfer sequence { "cause": Cause, "iE-Extensions"? extensions };
    PathSwitchRequestTransfer sequence { "dL-NGU-UP-TNLInformation": UPTransportLayerInformation, "dL-NGU-TNLInformationReused"? DLNGUTNLInformationReused, "userPlaneSecurityInformation"? UserPlaneSecurityInformation, "qosFlowAcceptedList": QosFlowAcceptedList, "iE-Extensions"? extensions };
    PathSwitchRequestUnsuccessfulTransfer sequence { "cause": Cause, "iE-Extensions"? extensions };
    PeriodicReportingIEs sequence { "reportingPeriodicity": ReportingPeriodicity, "iE-Extensions"? extensions };
    PeriodicityBound sequence { "periodicityLowerBound": _, "periodicityUpperBound": _, "iE-Extensions"? extensions };
    PeriodicityList sequence { "allowedPeriodicityList": AllowedPeriodicityList, "iE-Extensions"? extensions };
    PeriodicityRange choice { "periodicityBound": PeriodicityBound, "periodicityList": AllowedPeriodicityList, "choice-extensions": ie };
    PrivateIEID choice { "local": _, "global": _ };
    PrivateMessage sequence { "privateIEs": PrivateMessagePrivateIEs };
    PrivateMessagePrivateIEs = [AnonymousPrivateMessagePrivateIEs];
    ProcedureStageChoice choice { "first-dl-count": FirstDLCount, "choice-Extensions": ie };
    QMCConfigInfo sequence { "uEAppLayerMeasInfoList": UEAppLayerMeasInfoList, "iE-Extensions"? extensions };
    QMCDeactivation sequence { "qoEReferenceList": QoEReferenceList, "iE-Extensions"? extensions };
    QoERVQoEReportingPaths sequence { "qoEReportingPath"? QoERVQoEReportingPathsQoEReportingPath, "rVQoEReportingPath"? QoERVQoEReportingPathsRVQoEReportingPath, "iE-Extensions"? extensions };
    QoEReferenceList = [_];
    QoSFlowList = [_];
    QoSFlowTSCItem sequence { "qosFlowIdentifier": _, "tSCTrafficCharacteristicsFeedback"? TSCTrafficCharacteristicsFeedback, "aNPacketDelayBudgetUL"? _, "iE-Extensions"? extensions };
    QoSFlowTSCList = [QoSFlowTSCItem];
    QoSFlowsUsageReportItem sequence { "qosFlowIdentifier": _, "rATType": QoSFlowsUsageReportItemRATType, "qoSFlowsTimedReportList": VolumeTimedReportList, "iE-Extensions"? extensions };
    QoSFlowsUsageReportList = [QoSFlowsUsageReportItem];
    QosCharacteristics choice { "nonDynamic5QI": NonDynamic5QIDescriptor, "dynamic5QI": Dynamic5QIDescriptor, "choice-Extensions": ie };
    QosFlowAcceptedItem sequence { "qosFlowIdentifier": _, "iE-Extensions"? extensions };
    QosFlowAcceptedList = [QosFlowAcceptedItem];
    QosFlowAddOrModifyRequestItem sequence { "qosFlowIdentifier": _, "qosFlowLevelQosParameters"? QosFlowLevelQosParameters, "e-RAB-ID"? _, "iE-Extensions"? extensions };
    QosFlowAddOrModifyRequestList = [QosFlowAddOrModifyRequestItem];
    QosFlowAddOrModifyResponseItem sequence { "qosFlowIdentifier": _, "iE-Extensions"? extensions };
    QosFlowAddOrModifyResponseList = [QosFlowAddOrModifyResponseItem];
    QosFlowAdditionalInfoItemRelCom sequence { "qosFlowIdentifier": _, "downlinkTLContainer"? _, "iE-Extensions"? extensions };
    QosFlowAdditionalInfoItemRelRes sequence { "qosFlowIdentifier": _, "uplinkTLContainer"? _, "iE-Extensions"? extensions };
    QosFlowAdditionalInfoListRelCom = [QosFlowAdditionalInfoItemRelCom];
    QosFlowAdditionalInfoListRelRes = [QosFlowAdditionalInfoItemRelRes];
    QosFlowFeedbackItem sequence { "qosFlowIdentifier": _, "updateFeedback"? UpdateFeedback, "cNpacketDelayBudgetDL"? _, "cNpacketDelayBudgetUL"? _, "iE-Extensions"? extensions };
    QosFlowFeedbackList = [QosFlowFeedbackItem];
    QosFlowInformationItem sequence { "qosFlowIdentifier": _, "dLForwarding"? DLForwarding, "iE-Extensions"? extensions };
    QosFlowInformationList = [QosFlowInformationItem];
    QosFlowItemWithDataForwarding sequence { "qosFlowIdentifier": _, "dataForwardingAccepted"? DataForwardingAccepted, "iE-Extensions"? extensions };
    QosFlowLevelQosParameters sequence { "qosCharacteristics": QosCharacteristics, "allocationAndRetentionPriority": AllocationAndRetentionPriority, "gBR-QosInformation"? GBRQosInformation, "reflectiveQosAttribute"? ReflectiveQosAttribute, "additionalQosFlowInformation"? AdditionalQosFlowInformation, "iE-Extensions"? extensions };
    QosFlowListWithCause = [QosFlowWithCauseItem];
    QosFlowListWithDataForwarding = [QosFlowItemWithDataForwarding];
    QosFlowModifyConfirmItem sequence { "qosFlowIdentifier": _, "iE-Extensions"? extensions };
    QosFlowModifyConfirmList = [QosFlowModifyConfirmItem];
    QosFlowNotifyItem sequence { "qosFlowIdentifier": _, "notificationCause": NotificationCause, "iE-Extensions"? extensions };
    QosFlowNotifyList = [QosFlowNotifyItem];
    QosFlowParametersItem sequence { "qosFlowIdentifier": _, "alternativeQoSParaSetList"? AlternativeQoSParaSetList, "iE-Extensions"? extensions };
    QosFlowParametersList = [QosFlowParametersItem];
    QosFlowPerTNLInformation sequence { "uPTransportLayerInformation": UPTransportLayerInformation, "associatedQosFlowList": AssociatedQosFlowList, "iE-Extensions"? extensions };
    QosFlowPerTNLInformationItem sequence { "qosFlowPerTNLInformation": QosFlowPerTNLInformation, "iE-Extensions"? extensions };
    QosFlowPerTNLInformationList = [QosFlowPerTNLInformationItem];
    QosFlowSetupRequestItem sequence { "qosFlowIdentifier": _, "qosFlowLevelQosParameters": QosFlowLevelQosParameters, "e-RAB-ID"? _, "iE-Extensions"? extensions };
    QosFlowSetupRequestList = [QosFlowSetupRequestItem];
    QosFlowToBeForwardedItem sequence { "qosFlowIdentifier": _, "iE-Extensions"? extensions };
    QosFlowToBeForwardedList = [QosFlowToBeForwardedItem];
    QosFlowWithCauseItem sequence { "qosFlowIdentifier": _, "cause": Cause, "iE-Extensions"? extensions };
    RANCPRelocationIndication sequence { "protocolIEs": ies };
    RANConfigurationUpdate sequence { "protocolIEs": ies };
    RANConfigurationUpdateAcknowledge sequence { "protocolIEs": ies };
    RANConfigurationUpdateFailure sequence { "protocolIEs": ies };
    RANPagingRequest sequence { "protocolIEs": ies };
    RANStatusTransferTransparentContainer sequence { "dRBsSubjectToStatusTransferList": DRBsSubjectToStatusTransferList, "iE-Extensions"? extensions };
    RANTSSCellItem sequence { "nRCGI": NRCGI, "iE-Extensions"? extensions };
    RANTSSCellList = [RANTSSCellItem];
    RANTSSScope choice { "rANNodeLevel": GlobalGNBID, "cellListLevel": RANTSSCellList, "choice-Extensions": ie };
    RANTimingSynchronisationStatusInfo sequence { "synchronisationState"? RANTimingSynchronisationStatusInfoSynchronisationState, "traceabletoUTC"? RANTimingSynchronisationStatusInfoTraceabletoUTC, "traceabletoGNSS"? RANTimingSynchronisationStatusInfoTraceabletoGNSS, "clockFrequencyStability"? bits, "clockAccuracy"? ClockAccuracy, "parentTImeSource"? ParentTImeSource, "iE-Extensions"? extensions };
    RANfeedbacktype choice { "proactive": RANfeedbacktypeProactive, "reactive": RANfeedbacktypeReactive, "choice-extensions": ie };
    RANfeedbacktypeProactive sequence { "burstArrivalTimeWindow": BurstArrivalTimeWindow, "periodicityRange"? PeriodicityRange, "iE-Extension"? extensions };
    RANfeedbacktypeReactive sequence { "capabilityForBATAdaptation": RANfeedbacktypeReactiveCapabilityForBATAdaptation, "iE-Extension"? extensions };
    RATRestrictionInformation = bits;
    RATRestrictions = [RATRestrictionsItem];
    RATRestrictionsItem sequence { "pLMNIdentity": PLMNIdentity, "rATRestrictionInformation": RATRestrictionInformation, "iE-Extensions"? extensions };
    RIMInformation sequence { "targetgNBSetID": _, "rIM-RSDetection": RIMInformationRIMRSDetection, "iE-Extensions"? extensions };
    RIMInformationTransfer sequence { "targetRANNodeID-RIM": TargetRANNodeIDRIM, "sourceRANNodeID": SourceRANNodeID, "rIMInformation": RIMInformation, "iE-Extensions"? extensions };
    RRCInactiveTransitionReport sequence { "protocolIEs": ies };
    RSPPFlowBitRates sequence { "guaranteedFlowBitRate": _, "maximumFlowBitRate": _, "iE-Extensions"? extensions };
    RSPPQoSFlowItem sequence { "pQI": _, "rSPPFlowBitRates"? RSPPFlowBitRates, "range"? Range, "iE-Extensions"? extensions };
    RSPPQoSFlowList = [RSPPQoSFlowItem];
    RecommendedCellItem sequence { "nGRAN-CGI": NGRANCGI, "timeStayedInCell"? _, "iE-Extensions"? extensions };
    RecommendedCellList = [RecommendedCellItem];
    RecommendedCellsForPaging sequence { "recommendedCellList": RecommendedCellList, "iE-Extensions"? extensions };
    RecommendedRANNodeItem sequence { "aMFPagingTarget": AMFPagingTarget, "iE-Extensions"? extensions };
    RecommendedRANNodeList = [RecommendedRANNodeItem];
    RecommendedRANNodesForPaging sequence { "recommendedRANNodeList": RecommendedRANNodeList, "iE-Extensions"? extensions };
    RedundantPDUSessionInformation sequence { "rSN": RSN, "iE-Extensions"? extensions };
    ReportType choice { "eventBasedReporting": EventBasedReportingIEs, "periodicReporting": PeriodicReportingIEs, "choice-Extensions": ie };
    ReportingSystem choice { "eUTRAN": EUTRANReportingSystemIEs, "nGRAN": NGRANReportingSystemIEs, "noReporting": null, "choice-Extensions": ie };
    RequestedNSSAI = [RequestedNSSAIItem];
    RequestedNSSAIItem sequence { "s-NSSAI": SNSSAI, "iE-Extensions"? extensions };
    RequestedTNLInfo sequence { "pLMNIdentity": PLMNIdentity, "iE-Extensions"? extensions };
    RerouteNASRequest sequence { "protocolIEs": ies };
    ResetType choice { "nG-Interface": ResetAll, "partOfNG-Interface": UEAssociatedLogicalNGConnectionList, "choice-Extensions": ie };
    ResourceStatusReportingSystem choice { "eUTRAN-ReportingStatus": EUTRANReportingStatusIEs, "nGRAN-ReportingStatus": NGRANReportingStatusIEs, "choice-Extensions": ie };
    RetrieveUEInformation sequence { "protocolIEs": ies };
    SCTPTLAs = [TransportLayerAddress];
    SLPositioningRangingQoSParameters sequence { "rSPPQoSFlowList": RSPPQoSFlowList, "rSPPLinkAggregateBitRates"? _, "iE-Extensions"? extensions };
    SLPositioningRangingServiceInfo sequence { "sLPositioningRangingAuthorized": SLPositioningRangingAuthorized, "sLPositioningRangingQoSParameters"? SLPositioningRangingQoSParameters, "iE-Extensions"? extensions };
    SNPNBasedMDT sequence { "sNPNListforMDT": SNPNListforMDT, "iE-Extensions"? extensions };
    SNPNCellBasedMDT sequence { "sNPNcellIdListforMDT": SNPNCellIdListforMDT, "iE-Extensions"? extensions };
    SNPNCellIdListforMDT = [SNPNCellIdListforMDTItem];
    SNPNCellIdListforMDTItem sequence { "nR-CGI": NRCGI, "nID": _, "iE-Extensions"? extensions };
    SNPNListforMDT = [SNPNListforMDTItem];
    SNPNListforMDTItem sequence { "pLMNIdentity": PLMNIdentity, "nID": _, "iE-Extensions"? extensions };
    SNPNMobilityInformation sequence { "serving-NID": _, "iE-Extensions"? extensions };
    SNPNTAIBasedMDT sequence { "sNPNTAIListforMDT": SNPNTAIListforMDT, "iE-Extensions"? extensions };
    SNPNTAIListforMDT = [SNPNTAIListforMDTItem];
    SNPNTAIListforMDTItem sequence { "tAI": TAI, "nID": _, "iE-Extensions"? extensions };
    SNSSAI sequence { "sST": SST, "sD"? _, "iE-Extensions"? extensions };
    SONConfigurationTransfer sequence { "targetRANNodeID-SON": TargetRANNodeIDSON, "sourceRANNodeID": SourceRANNodeID, "sONInformation": SONInformation, "xnTNLConfigurationInfo"? XnTNLConfigurationInfo, "iE-Extensions"? extensions };
    SONInformation choice { "sONInformationRequest": SONInformationRequest, "sONInformationReply": SONInformationReply, "choice-Extensions": ie };
    SONInformationReply sequence { "xnTNLConfigurationInfo"? XnTNLConfigurationInfo, "iE-Extensions"? extensions };
    SONInformationReport choice { "failureIndicationInformation": FailureIndication, "hOReportInformation": HOReport, "choice-Extensions": ie };
    ScheduledCommunicationTime sequence { "dayofWeek"? bits, "timeofDayStart"? _, "timeofDayEnd"? _, "iE-Extensions"? extensions };
    SecondaryRATDataUsageReport sequence { "protocolIEs": ies };
    SecondaryRATDataUsageReportTransfer sequence { "secondaryRATUsageInformation"? SecondaryRATUsageInformation, "iE-Extensions"? extensions };
    SecondaryRATUsageInformation sequence { "pDUSessionUsageReport"? PDUSessionUsageReport, "qosFlowsUsageReportList"? QoSFlowsUsageReportList, "iE-Extension"? extensions };
    SecurityContext sequence { "nextHopChainingCount": _, "nextHopNH": _, "iE-Extensions"? extensions };
    SecurityIndication sequence { "integrityProtectionIndication": IntegrityProtectionIndication, "confidentialityProtectionIndication": ConfidentialityProtectionIndication, "maximumIntegrityProtectedDataRate-UL"? MaximumIntegrityProtectedDataRate, "iE-Extensions"? extensions };
    SecurityResult sequence { "integrityProtectionResult": IntegrityProtectionResult, "confidentialityProtectionResult": ConfidentialityProtectionResult, "iE-Extensions"? extensions };
    SensorMeasConfigNameItem sequence { "sensorNameConfig": SensorNameConfig, "iE-Extensions"? extensions };
    SensorMeasConfigNameList = [SensorMeasConfigNameItem];
    SensorMeasurementConfiguration sequence { "sensorMeasConfig": SensorMeasConfig, "sensorMeasConfigNameList"? SensorMeasConfigNameList, "iE-Extensions"? extensions };
    SensorNameConfig choice { "uncompensatedBarometricConfig": SensorNameConfigUncompensatedBarometricConfig, "ueSpeedConfig": SensorNameConfigUeSpeedConfig, "ueOrientationConfig": SensorNameConfigUeOrientationConfig, "choice-Extensions": ie };
    ServedGUAMIItem sequence { "gUAMI": GUAMI, "backupAMFName"? _, "iE-Extensions"? extensions };
    ServedGUAMIList = [ServedGUAMIItem];
    ServiceAreaInformation = [ServiceAreaInformationItem];
    ServiceAreaInformationItem sequence { "pLMNIdentity": PLMNIdentity, "allowedTACs"? AllowedTACs, "notAllowedTACs"? NotAllowedTACs, "iE-Extensions"? extensions };
    SharedNGUMulticastTNLInformation sequence { "iP-MulticastAddress": TransportLayerAddress, "iP-SourceAddress": TransportLayerAddress, "gTP-TEID": GTPTEID, "iE-Extensions"? extensions };
    SliceMDTItem sequence { "sNSSAI": SNSSAI, "iE-Extensions"? extensions };
    SliceMDTList = [SliceMDTItem];
    SliceOverloadItem sequence { "s-NSSAI": SNSSAI, "iE-Extensions"? extensions };
    SliceOverloadList = [SliceOverloadItem];
    SliceSupportItem sequence { "s-NSSAI": SNSSAI, "iE-Extensions"? extensions };
    SliceSupportList = [SliceSupportItem];
    SliceSupportListQMC = [SliceSupportQMCItem];
    SliceSupportQMCItem sequence { "s-NSSAI": SNSSAI, "iE-Extensions"? extensions };
    SourceNGRANNodeToTargetNGRANNodeTransparentContainer sequence { "rRCContainer": _, "pDUSessionResourceInformationList"? PDUSessionResourceInformationList, "e-RABInformationList"? ERABInformationList, "targetCell-ID": NGRANCGI, "indexToRFSP"? _, "uEHistoryInformation": UEHistoryInformation, "iE-Extensions"? extensions };
    SourceNodeID choice { "sourceengNB-ID": GlobalGNBID, "choice-Extensions": ie };
    SourceRANNodeID sequence { "globalRANNodeID": GlobalRANNodeID, "selectedTAI": TAI, "iE-Extensions"? extensions };
    SourceToTargetAMFInformationReroute sequence { "configuredNSSAI"? _, "rejectedNSSAIinPLMN"? _, "rejectedNSSAIinTA"? _, "iE-Extensions"? extensions };
    SuccessfulHandoverReportItem sequence { "successfulHOReportContainer": _, "iE-Extensions"? extensions };
    SuccessfulHandoverReportList = [SuccessfulHandoverReportItem];
    SuccessfulOutcome sequence { "procedureCode": _, "criticality": Criticality, "value": _ };
    SuccessfulPSCellChangeReportItem sequence { "successfulPSCellChangeReportContainer": _, "iE-Extensions"? extensions };
    SuccessfulPSCellChangeReportList = [SuccessfulPSCellChangeReportItem];
    SupportedTAItem sequence { "tAC": TAC, "broadcastPLMNList": BroadcastPLMNList, "iE-Extensions"? extensions };
    SupportedTAList = [SupportedTAItem];
    SupportedUETypeList = [SupportedUEType];
    TABasedMDT sequence { "tAListforMDT": TAListforMDT, "iE-Extensions"? extensions };
    TABasedQMC sequence { "tAListforQMC": TAListforQMC, "iE-Extensions"? extensions };
    TACListInNRNTN = [TAC];
    TAI sequence { "pLMNIdentity": PLMNIdentity, "tAC": TAC, "iE-Extensions"? extensions };
    TAIBasedMDT sequence { "tAIListforMDT": TAIListforMDT, "iE-Extensions"? extensions };
    TAIBasedQMC sequence { "tAIListforQMC": TAIListforQMC, "iE-Extensions"? extensions };
    TAIBroadcastEUTRA = [TAIBroadcastEUTRAItem];
    TAIBroadcastEUTRAItem sequence { "tAI": TAI, "completedCellsInTAI-EUTRA": CompletedCellsInTAIEUTRA, "iE-Extensions"? extensions };
    TAIBroadcastNR = [TAIBroadcastNRItem];
    TAIBroadcastNRItem sequence { "tAI": TAI, "completedCellsInTAI-NR": CompletedCellsInTAINR, "iE-Extensions"? extensions };
    TAICancelledEUTRA = [TAICancelledEUTRAItem];
    TAICancelledEUTRAItem sequence { "tAI": TAI, "cancelledCellsInTAI-EUTRA": CancelledCellsInTAIEUTRA, "iE-Extensions"? extensions };
    TAICancelledNR = [TAICancelledNRItem];
    TAICancelledNRItem sequence { "tAI": TAI, "cancelledCellsInTAI-NR": CancelledCellsInTAINR, "iE-Extensions"? extensions };
    TAIListForInactive = [TAIListForInactiveItem];
    TAIListForInactiveItem sequence { "tAI": TAI, "iE-Extensions"? extensions };
    TAIListForPaging = [TAIListForPagingItem];
    TAIListForPagingItem sequence { "tAI": TAI, "iE-Extensions"? extensions };
    TAIListForRestart = [TAI];
    TAIListForWarning = [TAI];
    TAIListforMDT = [TAI];
    TAIListforQMC = [TAI];
    TAIMBSSupportItem sequence { "tAI": TAI, "iE-Extensions"? extensions };
    TAIMBSSupportList = [TAIMBSSupportItem];
    TAINSAGSupportItem sequence { "nSAG-ID": _, "nSAGSliceSupportList": ExtendedSliceSupportList, "iE-Extensions"? extensions };
    TAINSAGSupportList = [TAINSAGSupportItem];
    TAListforMDT = [TAC];
    TAListforQMC = [TAC];
    TNGFID choice { "tNGF-ID": bits, "choice-Extensions": ie };
    TNLAssociationItem sequence { "tNLAssociationAddress": CPTransportLayerInformation, "cause": Cause, "iE-Extensions"? extensions };
    TNLAssociationList = [TNLAssociationItem];
    TSCAssistanceInformation sequence { "periodicity": _, "burstArrivalTime"? _, "iE-Extensions"? extensions };
    TSCFeedbackInformation sequence { "burstArrivalTimeOffset": _, "adjustedPeriodicity"? _, "iE-Extensions"? extensions };
    TSCTrafficCharacteristics sequence { "tSCAssistanceInformationDL"? TSCAssistanceInformation, "tSCAssistanceInformationUL"? TSCAssistanceInformation, "iE-Extensions"? extensions };
    TSCTrafficCharacteristicsFeedback sequence { "tSCFeedbackInformationDL"? TSCFeedbackInformation, "tSCFeedbackInformationUL"? TSCFeedbackInformation, "iE-Extensions"? extensions };
    TWIFID choice { "tWIF-ID": bits, "choice-Extensions": ie };
    TargetHomeENBID sequence { "pLMNidentity": PLMNIdentity, "homeENB-ID": _, "selected-EPS-TAI": EPSTAI, "iE-Extensions"? extensions };
    TargetID choice { "targetRANNodeID": TargetRANNodeID, "targeteNB-ID": TargeteNBID, "choice-Extensions": ie };
    TargetNGRANNodeToSourceNGRANNodeFailureTransparentContainer sequence { "cell-CAGInformation"? CellCAGInformation, "iE-Extensions"? extensions };
    TargetNGRANNodeToSourceNGRANNodeTransparentContainer sequence { "rRCContainer": _, "iE-Extensions"? extensions };
    TargetNSSAI = [TargetNSSAIItem];
    TargetNSSAIInformation sequence { "targetNSSAI": TargetNSSAI, "indexToRFSP": _, "iE-Extensions"? extensions };
    TargetNSSAIItem sequence { "s-NSSAI": SNSSAI, "iE-Extensions"? extensions };
    TargetRANNodeID sequence { "globalRANNodeID": GlobalRANNodeID, "selectedTAI": TAI, "iE-Extensions"? extensions };
    TargetRANNodeIDRIM sequence { "globalRANNodeID": GlobalRANNodeID, "selectedTAI": TAI, "iE-Extensions"? extensions };
    TargetRANNodeIDSON sequence { "globalRANNodeID": GlobalRANNodeID, "selectedTAI": TAI, "iE-Extensions"? extensions };
    TargetRNCID sequence { "lAI": LAI, "rNC-ID": _, "extendedRNC-ID"? _, "iE-Extensions"? extensions };
    TargeteNBID sequence { "globalENB-ID": GlobalNgENBID, "selected-EPS-TAI": EPSTAI, "iE-Extensions"? extensions };
    TimeBasedHandoverInformation sequence { "hOWindowStart": _, "hOWindowDuration": _, "iE-Extensions"? extensions };
    TimeSyncAssistanceInfo sequence { "timeDistributionIndication": TimeSyncAssistanceInfoTimeDistributionIndication, "uUTimeSyncErrorBudget"? _, "iE-Extensions"? extensions };
    TimingSynchronisationStatusFailure sequence { "protocolIEs": ies };
    TimingSynchronisationStatusReport sequence { "protocolIEs": ies };
    TimingSynchronisationStatusRequest sequence { "protocolIEs": ies };
    TimingSynchronisationStatusResponse sequence { "protocolIEs": ies };
    TooearlyIntersystemHO sequence { "sourcecellID": EUTRACGI, "failurecellID": NGRANCGI, "uERLFReportContainer"? UERLFReportContainer, "iE-Extensions"? extensions };
    TraceActivation sequence { "nGRANTraceID": _, "interfacesToTrace": _, "traceDepth": TraceDepth, "traceCollectionEntityIPAddress": TransportLayerAddress, "iE-Extensions"? extensions };
    TraceFailureIndication sequence { "protocolIEs": ies };
    TraceStart sequence { "protocolIEs": ies };
    TransportLayerAddress = bits;
    UEAggregateMaximumBitRate sequence { "uEAggregateMaximumBitRateDL": _, "uEAggregateMaximumBitRateUL": _, "iE-Extensions"? extensions };
    UEAppLayerMeasConfigInfo sequence { "qoEReference": _, "serviceType": ServiceType, "areaScopeOfQMC"? AreaScopeOfQMC, "measCollEntityIPAddress": TransportLayerAddress, "qoEMeasurementStatus"? UEAppLayerMeasConfigInfoQoEMeasurementStatus, "containerForAppLayerMeasConfig"? _, "measConfigAppLayerID"? _, "sliceSupportListQMC"? SliceSupportListQMC, "mDT-AlignmentInfo"? MDTAlignmentInfo, "availableRANVisibleQoEMetrics"? AvailableRANVisibleQoEMetrics, "iE-Extensions"? extensions };
    UEAppLayerMeasInfoItem sequence { "uEAppLayerMeasConfigInfo": UEAppLayerMeasConfigInfo, "iE-Extensions"? extensions };
    UEAppLayerMeasInfoList = [UEAppLayerMeasInfoItem];
    UEAssociatedLogicalNGConnectionItem sequence { "aMF-UE-NGAP-ID"? _, "rAN-UE-NGAP-ID"? _, "iE-Extensions"? extensions };
    UEAssociatedLogicalNGConnectionList = [UEAssociatedLogicalNGConnectionItem];
    UEContextModificationFailure sequence { "protocolIEs": ies };
    UEContextModificationRequest sequence { "protocolIEs": ies };
    UEContextModificationResponse sequence { "protocolIEs": ies };
    UEContextReleaseCommand sequence { "protocolIEs": ies };
    UEContextReleaseComplete sequence { "protocolIEs": ies };
    UEContextReleaseRequest sequence { "protocolIEs": ies };
    UEContextResumeFailure sequence { "protocolIEs": ies };
    UEContextResumeRequest sequence { "protocolIEs": ies };
    UEContextResumeRequestTransfer sequence { "qosFlowFailedToResumeList"? QosFlowListWithCause, "iE-Extensions"? extensions };
    UEContextResumeResponse sequence { "protocolIEs": ies };
    UEContextResumeResponseTransfer sequence { "qosFlowFailedToResumeList"? QosFlowListWithCause, "iE-Extensions"? extensions };
    UEContextSuspendFailure sequence { "protocolIEs": ies };
    UEContextSuspendRequest sequence { "protocolIEs": ies };
    UEContextSuspendRequestTransfer sequence { "suspendIndicator"? SuspendIndicator, "iE-Extensions"? extensions };
    UEContextSuspendResponse sequence { "protocolIEs": ies };
    UEDifferentiationInfo sequence { "periodicCommunicationIndicator"? UEDifferentiationInfoPeriodicCommunicationIndicator, "periodicTime"? _, "scheduledCommunicationTime"? ScheduledCommunicationTime, "stationaryIndication"? UEDifferentiationInfoStationaryIndication, "trafficProfile"? UEDifferentiationInfoTrafficProfile, "batteryIndication"? UEDifferentiationInfoBatteryIndication, "iE-Extensions"? extensions };
    UEHistoryInformation = [LastVisitedCellItem];
    UEHistoryInformationFromTheUE choice { "nR": _, "choice-Extensions": ie };
    UEIdentityIndexValue choice { "indexLength10": _, "choice-Extensions": ie };
    UEInformationTransfer sequence { "protocolIEs": ies };
    UENGAPIDPair sequence { "aMF-UE-NGAP-ID": _, "rAN-UE-NGAP-ID": _, "iE-Extensions"? extensions };
    UENGAPIDs choice { "uE-NGAP-ID-pair": UENGAPIDPair, "aMF-UE-NGAP-ID": _, "choice-Extensions": ie };
    UEPagingIdentity choice { "fiveG-S-TMSI": FiveGSTMSI, "choice-Extensions": ie };
    UEPagingItem sequence { "uEIdentityIndexValue": UEIdentityIndexValue, "pagingDRX"? PagingDRX, "iE-Extensions"? extensions };
    UEPagingList = [UEPagingItem];
    UEPresenceInAreaOfInterestItem sequence { "locationReportingReferenceID": _, "uEPresence": UEPresence, "iE-Extensions"? extensions };
    UEPresenceInAreaOfInterestList = [UEPresenceInAreaOfInterestItem];
    UERLFReportContainer choice { "nR": _, "lTE": _, "choice-Extensions": ie };
    UERadioCapabilityCheckRequest sequence { "protocolIEs": ies };
    UERadioCapabilityCheckResponse sequence { "protocolIEs": ies };
    UERadioCapabilityForPaging sequence { "uERadioCapabilityForPagingOfNR"? _, "uERadioCapabilityForPagingOfEUTRA"? _, "iE-Extensions"? extensions };
    UERadioCapabilityIDMappingRequest sequence { "protocolIEs": ies };
    UERadioCapabilityIDMappingResponse sequence { "protocolIEs": ies };
    UERadioCapabilityInfoIndication sequence { "protocolIEs": ies };
    UESecurityCapabilities sequence { "nRencryptionAlgorithms": NRencryptionAlgorithms, "nRintegrityProtectionAlgorithms": NRintegrityProtectionAlgorithms, "eUTRAencryptionAlgorithms": EUTRAencryptionAlgorithms, "eUTRAintegrityProtectionAlgorithms": EUTRAintegrityProtectionAlgorithms, "iE-Extensions"? extensions };
    UESliceMaximumBitRateItem sequence { "s-NSSAI": SNSSAI, "uESliceMaximumBitRateDL": _, "uESliceMaximumBitRateUL": _, "iE-Extensions"? extensions };
    UESliceMaximumBitRateList = [UESliceMaximumBitRateItem];
    UETNLABindingReleaseRequest sequence { "protocolIEs": ies };
    ULCPSecurityInformation sequence { "ul-NAS-MAC": _, "ul-NAS-Count": ULNASCount, "iE-Extensions"? extensions };
    ULNGUUPTNLModifyItem sequence { "uL-NGU-UP-TNLInformation": UPTransportLayerInformation, "dL-NGU-UP-TNLInformation": UPTransportLayerInformation, "iE-Extensions"? extensions };
    ULNGUUPTNLModifyList = [ULNGUUPTNLModifyItem];
    UPTransportLayerInformation choice { "gTPTunnel": GTPTunnel, "choice-Extensions": ie };
    UPTransportLayerInformationItem sequence { "nGU-UP-TNLInformation": UPTransportLayerInformation, "iE-Extensions"? extensions };
    UPTransportLayerInformationList = [UPTransportLayerInformationItem];
    UPTransportLayerInformationPairItem sequence { "uL-NGU-UP-TNLInformation": UPTransportLayerInformation, "dL-NGU-UP-TNLInformation": UPTransportLayerInformation, "iE-Extensions"? extensions };
    UPTransportLayerInformationPairList = [UPTransportLayerInformationPairItem];
    UnavailableGUAMIItem sequence { "gUAMI": GUAMI, "timerApproachForGUAMIRemoval"? TimerApproachForGUAMIRemoval, "backupAMFName"? _, "iE-Extensions"? extensions };
    UnavailableGUAMIList = [UnavailableGUAMIItem];
    UnsuccessfulOutcome sequence { "procedureCode": _, "criticality": Criticality, "value": _ };
    UpdateFeedback = bits;
    UplinkNASTransport sequence { "protocolIEs": ies };
    UplinkNonUEAssociatedNRPPaTransport sequence { "protocolIEs": ies };
    UplinkRANConfigurationTransfer sequence { "protocolIEs": ies };
    UplinkRANEarlyStatusTransfer sequence { "protocolIEs": ies };
    UplinkRANStatusTransfer sequence { "protocolIEs": ies };
    UplinkRIMInformationTransfer sequence { "protocolIEs": ies };
    UplinkUEAssociatedNRPPaTransport sequence { "protocolIEs": ies };
    UserLocationInformation choice { "userLocationInformationEUTRA": UserLocationInformationEUTRA, "userLocationInformationNR": UserLocationInformationNR, "userLocationInformationN3IWF-with-PortNumber": UserLocationInformationN3IWFWithPortNumber, "choice-Extensions": ie };
    UserLocationInformationEUTRA sequence { "eUTRA-CGI": EUTRACGI, "tAI": TAI, "timeStamp"? _, "iE-Extensions"? extensions };
    UserLocationInformationN3IWFWithPortNumber sequence { "iPAddress": TransportLayerAddress, "portNumber": PortNumber, "iE-Extensions"? extensions };
    UserLocationInformationN3IWFWithoutPortNumber sequence { "iPAddress": TransportLayerAddress, "tAI"? TAI, "iE-Extensions"? extensions };
    UserLocationInformationNR sequence { "nR-CGI": NRCGI, "tAI": TAI, "timeStamp"? _, "iE-Extensions"? extensions };
    UserLocationInformationTNGF sequence { "tNAP-ID": _, "iPAddress": TransportLayerAddress, "portNumber"? PortNumber, "iE-Extensions"? extensions };
    UserLocationInformationTWIF sequence { "tWAP-ID": _, "iPAddress": TransportLayerAddress, "portNumber"? PortNumber, "iE-Extensions"? extensions };
    UserLocationInformationWAGF choice { "globalLine-ID": GlobalLineID, "hFCNode-ID": _, "choice-Extensions": ie };
    UserPlaneFailureIndication sequence { "userPlaneFailureType": UserPlaneFailureType, "uL-NGU-UP-TNLInformation": UPTransportLayerInformation, "dL-NGU-UP-TNLInformation": UPTransportLayerInformation, "iE-Extensions"? extensions };
    UserPlaneSecurityInformation sequence { "securityResult": SecurityResult, "securityIndication": SecurityIndication, "iE-Extensions"? extensions };
    VolumeTimedReportItem sequence { "startTimeStamp": _, "endTimeStamp": _, "usageCountUL": _, "usageCountDL": _, "iE-Extensions"? extensions };
    VolumeTimedReportList = [VolumeTimedReportItem];
    WAGFID choice { "w-AGF-ID": bits, "choice-Extensions": ie };
    WLANMeasConfigNameItem sequence { "wLANName": _, "iE-Extensions"? extensions };
    WLANMeasConfigNameList = [WLANMeasConfigNameItem];
    WLANMeasurementConfiguration sequence { "wlanMeasConfig": WLANMeasConfig, "wlanMeasConfigNameList"? WLANMeasConfigNameList, "wlan-rssi"? WLANMeasurementConfigurationWlanRssi, "wlan-rtt"? WLANMeasurementConfigurationWlanRtt, "iE-Extensions"? extensions };
    WUSAssistanceInformation sequence { "pagingProbabilityInformation": PagingProbabilityInformation, "iE-Extensions"? extensions };
    WarningAreaList choice { "eUTRA-CGIListForWarning": EUTRACGIListForWarning, "nR-CGIListForWarning": NRCGIListForWarning, "tAIListForWarning": TAIListForWarning, "emergencyAreaIDList": EmergencyAreaIDList, "choice-Extensions": ie };
    WriteReplaceWarningRequest sequence { "protocolIEs": ies };
    WriteReplaceWarningResponse sequence { "protocolIEs": ies };
    XnExtTLAItem sequence { "iPsecTLA"? TransportLayerAddress, "gTP-TLAs"? XnGTPTLAs, "iE-Extensions"? extensions };
    XnExtTLAs = [XnExtTLAItem];
    XnGTPTLAs = [TransportLayerAddress];
    XnTLAs = [TransportLayerAddress];
    XnTNLConfigurationInfo sequence { "xnTransportLayerAddresses": XnTLAs, "xnExtendedTransportLayerAddresses"? XnExtTLAs, "iE-Extensions"? extensions };
}
