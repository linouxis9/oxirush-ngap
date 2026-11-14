// Auto-generated from ASN.1 by the generator of oxirush-aper; do not edit.
pub(crate) use crate::registry::{
    IE_NAMES, IE_TYPES, ITEM_CRITICALITIES, MESSAGES, ie, ie_contents,
};
use oxirush_aper::inspect::Form;
pub(crate) const PROTOCOL: &str = "NGAP";
pub(crate) const ROOT: &str = "ngap";
oxirush_aper::__transfers! {
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
oxirush_aper::__carried! {
    101 "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer" crate::ngap::SourceNGRANNodeToTargetNGRANNodeTransparentContainer;
    106 "TargetNGRANNode-ToSourceNGRANNode-TransparentContainer" crate::ngap::TargetNGRANNodeToSourceNGRANNodeTransparentContainer;
    262 "TargetNGRANNode-ToSourceNGRANNode-FailureTransparentContainer" crate::ngap::TargetNGRANNodeToSourceNGRANNodeFailureTransparentContainer;
    of "S1AP" 101 "SourceeNB-ToTargeteNB-TransparentContainer";
    of "S1AP" 106 "TargeteNB-ToSourceeNB-TransparentContainer";
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
oxirush_aper::__types! {
    types {
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
    enumerations {
        A2XPC5QoSFlowItemA2XRange ["m50", "m80", "m180", "m200", "m350", "m400", "m500", "m700", "m1000"];
        AIoTFollowonCommandIndication ["true"];
        AIoTSupport ["aIoT-only", "aIoT-and-NRUu"];
        AUN3DeviceAccessInfoAUN3DeviceAccess ["true"];
        AUN3DeviceAccessInfoExistingNGConnectionwithSameLocation ["true", "false"];
        ActivationStatus ["active", "not-active"];
        AdditionalQosFlowInformation ["more-likely"];
        AerialUEFlightInformationOngoingReportingFailed ["true"];
        AerialUEReportingPeriodicity ["ms120", "ms240", "ms480", "ms640", "ms1024", "ms2048", "ms5120", "ms10240", "ms20480", "ms40960", "min1", "min6", "min12", "min30"];
        AerialUEsubscriptionInformation ["allowed", "not-allowed"];
        AllowedPNINPNItemPNINPNRestricted ["restricted", "not-restricted"];
        AssociatedQosFlowItemQosFlowMappingIndication ["ul", "dl"];
        AuthenticatedIndication ["true"];
        AvailableRANVisibleQoEMetricsApplicationLayerBufferLevelList ["true"];
        AvailableRANVisibleQoEMetricsPlayoutDelayForMediaStartup ["true"];
        BeamMeasurementsReportQuantityRSRP ["true", "false"];
        BeamMeasurementsReportQuantityRSRQ ["true", "false"];
        BeamMeasurementsReportQuantitySINR ["true", "false"];
        BluetoothMeasConfig ["setup"];
        BluetoothMeasurementConfigurationBtRssi ["true"];
        CEmodeBSupportIndicator ["supported"];
        CEmodeBrestricted ["restricted", "not-restricted"];
        CNMTCommunicationHandling ["supported"];
        CNTypeRestrictionsForEquivalentItemCnType ["epc-forbidden", "fiveGC-forbidden"];
        CNTypeRestrictionsForServing ["epc-forbidden"];
        CancelAllWarningMessages ["true"];
        CauseMisc ["control-processing-overload", "not-enough-user-plane-processing-resources", "hardware-failure", "om-intervention", "unknown-PLMN-or-SNPN", "unspecified"];
        CauseNas ["normal-release", "authentication-failure", "deregister", "unspecified", "uE-not-in-PLMN-serving-area", "mobile-IAB-not-authorized", "iAB-not-authorized", "no-aiot-NAS-response"];
        CauseProtocol ["transfer-syntax-error", "abstract-syntax-error-reject", "abstract-syntax-error-ignore-and-notify", "message-not-compatible-with-receiver-state", "semantic-error", "abstract-syntax-error-falsely-constructed-message", "unspecified"];
        CauseRadioNetwork ["unspecified", "txnrelocoverall-expiry", "successful-handover", "release-due-to-ngran-generated-reason", "release-due-to-5gc-generated-reason", "handover-cancelled", "partial-handover", "ho-failure-in-target-5GC-ngran-node-or-target-system", "ho-target-not-allowed", "tngrelocoverall-expiry", "tngrelocprep-expiry", "cell-not-available", "unknown-targetID", "no-radio-resources-available-in-target-cell", "unknown-local-UE-NGAP-ID", "inconsistent-remote-UE-NGAP-ID", "handover-desirable-for-radio-reason", "time-critical-handover", "resource-optimisation-handover", "reduce-load-in-serving-cell", "user-inactivity", "radio-connection-with-ue-lost", "radio-resources-not-available", "invalid-qos-combination", "failure-in-radio-interface-procedure", "interaction-with-other-procedure", "unknown-PDU-session-ID", "unkown-qos-flow-ID", "multiple-PDU-session-ID-instances", "multiple-qos-flow-ID-instances", "encryption-and-or-integrity-protection-algorithms-not-supported", "ng-intra-system-handover-triggered", "ng-inter-system-handover-triggered", "xn-handover-triggered", "not-supported-5QI-value", "ue-context-transfer", "ims-voice-eps-fallback-or-rat-fallback-triggered", "up-integrity-protection-not-possible", "up-confidentiality-protection-not-possible", "slice-not-supported", "ue-in-rrc-inactive-state-not-reachable", "redirection", "resources-not-available-for-the-slice", "ue-max-integrity-protected-data-rate-reason", "release-due-to-cn-detected-mobility", "n26-interface-not-available", "release-due-to-pre-emption", "multiple-location-reporting-reference-ID-instances", "rsn-not-available-for-the-up", "npn-access-denied", "cag-only-access-denied", "insufficient-ue-capabilities", "redcap-ue-not-supported", "unknown-MBS-Session-ID", "indicated-MBS-session-area-information-not-served-by-the-gNB", "inconsistent-slice-info-for-the-session", "misaligned-association-for-multicast-unicast", "eredcap-ue-not-supported", "two-rx-xr-ue-not-supported", "aerial-ue-flight-information-reporting-initiation-failure", "unknown-RAN-AIoT-Device-NGAP-ID", "requested-AIoT-service-area-information-not-served-by-the-gNB", "unknown-AIoT-session", "aiot-device-not-reachable", "multiple-AIoT-session-ID-instances"];
        CauseTransport ["transport-resource-unavailable", "unspecified"];
        CellSize ["verysmall", "small", "medium", "large"];
        ClockQualityAcceptanceCriteriaTraceabletoGNSS ["true"];
        ClockQualityAcceptanceCriteriaTraceabletoUTC ["true"];
        CommServiceType ["mbs-broadcast", "mbs-multicast", "unicast"];
        ConcurrentWarningMessageInd ["true"];
        ConfidentialityProtectionIndication ["required", "preferred", "not-needed"];
        ConfidentialityProtectionResult ["performed", "not-performed"];
        ConfiguredTACIndication ["true"];
        CongestionInformationRequest ["ul", "dl", "both", "stop"];
        Criticality ["reject", "ignore", "notify"];
        DAPSRequestInfoDAPSIndicator ["daps-ho-required"];
        DAPSResponseInfoDapsresponseindicator ["daps-ho-accepted", "daps-ho-not-accepted"];
        DLForwarding ["dl-forwarding-proposed"];
        DLNGUTNLInformationReused ["true"];
        DLPDUSetInformationMarkingSupportIndication ["true"];
        DLSignalling ["true"];
        DataForwardingAccepted ["data-forwarding-accepted"];
        DataForwardingNotPossible ["data-forwarding-not-possible"];
        DelayCritical ["delay-critical", "non-delay-critical"];
        DirectForwardingPathAvailability ["direct-path-available"];
        ECNMarkingAtRANRequest ["ul", "dl", "both", "stop"];
        ECNMarkingAtUPFRequest ["ul", "dl", "both", "stop"];
        EDTSession ["true"];
        ERedCapIndication ["true"];
        EUTRAPagingEDRXCycle ["hfhalf", "hf1", "hf2", "hf4", "hf6", "hf8", "hf10", "hf12", "hf14", "hf16", "hf32", "hf64", "hf128", "hf256"];
        EUTRAPagingTimeWindow ["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13", "s14", "s15", "s16"];
        EarlyMeasurement ["true"];
        EmergencyFallbackRequestIndicator ["emergency-fallback-requested"];
        EmergencyServiceTargetCN ["fiveGC", "epc"];
        EndIndication ["no-further-data", "further-data-exists"];
        EnhancedCoverageRestriction ["restricted"];
        EventTriggerOutOfCoverage ["true"];
        EventType ["direct", "change-of-serve-cell", "ue-presence-in-area-of-interest", "stop-change-of-serve-cell", "stop-ue-presence-in-area-of-interest", "cancel-location-reporting-for-the-ue", "change-of-serving-cell-and-UE-presence-in-the-Area-of-Interest", "report-the-Aerial-UE-flight-information", "cancel-the-Aerial-UE-flight-information-reporting"];
        ExcessPacketDelayThresholdValue ["ms0dot25", "ms0dot5", "ms1", "ms2", "ms4", "ms5", "ms10", "ms20", "ms30", "ms40", "ms50", "ms60", "ms70", "ms80", "ms90", "ms100", "ms150", "ms300", "ms500"];
        ExpectedHOInterval ["sec15", "sec30", "sec60", "sec90", "sec120", "sec180", "long-time"];
        ExpectedUEMobility ["stationary", "mobile"];
        ExtendedReportIntervalMDT ["ms20480", "ms40960"];
        FiveGProSeDirectCommunication ["authorized", "not-authorized"];
        FiveGProSeDirectDiscovery ["authorized", "not-authorized"];
        FiveGProSeLayer2MHIntermediateUEtoNetworkRelay ["authorized", "not-authorized"];
        FiveGProSeLayer2MHRemote ["authorized", "not-authorized"];
        FiveGProSeLayer2MHUEtoNetworkRelay ["authorized", "not-authorized"];
        FiveGProSeLayer2Multipath ["authorized", "not-authorized"];
        FiveGProSeLayer2RemoteUE ["authorized", "not-authorized"];
        FiveGProSeLayer2UEtoNetworkRelay ["authorized", "not-authorized"];
        FiveGProSeLayer2UEtoUERelay ["authorized", "not-authorized"];
        FiveGProSeLayer2UEtoUERemote ["authorized", "not-authorized"];
        FiveGProSeLayer3MHUEtoNetworkRelay ["authorized", "not-authorized"];
        FiveGProSeLayer3UEtoNetworkRelay ["authorized", "not-authorized"];
        GUAMIType ["native", "mapped"];
        GWContextReleaseIndication ["true"];
        HLComDeactivateUEReachabilityIndication ["true"];
        HOReportHandoverReportType ["ho-too-early", "ho-to-wrong-cell", "intersystem-ping-pong"];
        HandoverFlag ["handover-preparation"];
        HandoverType ["intra5gs", "fivegs-to-eps", "eps-to-5gs", "fivegs-to-utran"];
        IABAuthorized ["authorized", "not-authorized"];
        IABNodeIndication ["true"];
        IABSupported ["true"];
        IMSVoiceSupportIndicator ["supported", "not-supported"];
        IncludeBeamMeasurementsIndication ["true"];
        IndicationOfBitrateAdaptation ["uplink"];
        IntegrityProtectionIndication ["required", "preferred", "not-needed"];
        IntegrityProtectionResult ["performed", "not-performed"];
        IntersystemUnnecessaryHOEarlyIRATHO ["true", "false"];
        InventoryReportTransferInventoryCompleteIndication ["true"];
        LPWUSDisableIndication ["true"];
        LTEA2XServicesAuthorizedAerialControllerUE ["authorized", "not-authorized"];
        LTEA2XServicesAuthorizedAerialUE ["authorized", "not-authorized"];
        LTEMIndication ["lte-m"];
        LineType ["dsl", "pon"];
        LinksToLog ["uplink", "downlink", "both-uplink-and-downlink"];
        LocationReportingAdditionalInfo ["includePSCell"];
        LoggingDuration ["m10", "m20", "m40", "m60", "m90", "m120"];
        LoggingInterval ["ms320", "ms640", "ms1280", "ms2560", "ms5120", "ms10240", "ms20480", "ms30720", "ms40960", "ms61440", "infinity"];
        M1ReportingTrigger ["periodic", "a2eventtriggered", "a2eventtriggered-periodic"];
        M4ReportAmountMDT ["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"];
        M4period ["ms1024", "ms2048", "ms5120", "ms10240", "min1"];
        M5ReportAmountMDT ["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"];
        M5period ["ms1024", "ms2048", "ms5120", "ms10240", "min1"];
        M6ReportAmountMDT ["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"];
        M6reportInterval ["ms120", "ms240", "ms480", "ms640", "ms1024", "ms2048", "ms5120", "ms10240", "ms20480", "ms40960", "min1", "min6", "min12", "min30"];
        M7ReportAmountMDT ["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"];
        MBSAssistanceInformation ["true"];
        MBSSessionStatus ["activated", "deactivated"];
        MBSSupportIndicator ["multicast-supported", "multicast-supported-with-reception-in-RRC-inactive"];
        MBSUPFailureIndication ["ng-u-path-failure-detected"];
        MDTActivation ["immediate-MDT-only", "logged-MDT-only", "immediate-MDT-and-Trace"];
        MICOModeIndication ["true"];
        MNOnlyMDTCollection ["mn-only"];
        MaximumIntegrityProtectedDataRate ["bitrate64kbs", "maximum-UE-rate"];
        MicoAllPLMN ["true"];
        MobileIABAuthorized ["authorized", "not-authorized"];
        MobileIABNodeIndication ["true"];
        MobileIABSupported ["true"];
        MonitoringRequest ["ul", "dl", "both", "stop"];
        NBIoTDefaultPagingDRX ["rf128", "rf256", "rf512", "rf1024"];
        NBIoTPagingDRX ["rf32", "rf64", "rf128", "rf256", "rf512", "rf1024"];
        NBIoTPagingEDRXCycle ["hf2", "hf4", "hf6", "hf8", "hf10", "hf12", "hf14", "hf16", "hf32", "hf64", "hf128", "hf256", "hf512", "hf1024"];
        NBIoTPagingTimeWindow ["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13", "s14", "s15", "s16"];
        NGAPIESupportInformationResponseItemNgapProtocolIEPresenceInfo ["present", "not-present"];
        NGAPIESupportInformationResponseItemNgapProtocolIESupportInfo ["supported", "not-supported"];
        NRA2XServicesAuthorizedAerialControllerUE ["authorized", "not-authorized"];
        NRA2XServicesAuthorizedAerialUE ["authorized", "not-authorized"];
        NRPagingEDRXCycle ["hfquarter", "hfhalf", "hf1", "hf2", "hf4", "hf8", "hf16", "hf32", "hf64", "hf128", "hf256", "hf512", "hf1024"];
        NRPagingLongEDRXCycleForRRCINACTIVE ["hf2", "hf4", "hf8", "hf16", "hf32", "hf64", "hf128", "hf256", "hf512", "hf1024"];
        NRPagingTimeWindow ["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13", "s14", "s15", "s16", "s17", "s18", "s19", "s20", "s21", "s22", "s23", "s24", "s25", "s26", "s27", "s28", "s29", "s30", "s31", "s32"];
        NRPagingTimeWindowForRRCINACTIVE ["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13", "s14", "s15", "s16", "s17", "s18", "s19", "s20", "s21", "s22", "s23", "s24", "s25", "s26", "s27", "s28", "s29", "s30", "s31", "s32"];
        NetworkControlledRepeaterAuthorized ["authorized", "not-authorized"];
        NewSecurityContextInd ["true"];
        NextPagingAreaScope ["same", "changed"];
        NoPDUSessionIndication ["true"];
        NotificationCause ["fulfilled", "not-fulfilled", "not-fulfilled-DL", "not-fulfilled-UL"];
        NotificationCellItemNotifyFlag ["activated", "deactivated"];
        NotificationControl ["notification-requested"];
        NotifySourceNGRANNode ["notifySource"];
        NumberOfMeasurementReportingLevels ["n2", "n3", "n4", "n5", "n10", "n0"];
        OnboardingSupport ["true"];
        OverloadAction ["reject-non-emergency-mo-dt", "reject-rrc-cr-signalling", "permit-emergency-sessions-and-mobile-terminated-services-only", "permit-high-priority-sessions-and-mobile-terminated-services-only"];
        PDUSessionType ["ipv4", "ipv6", "ipv4v6", "ethernet", "unstructured"];
        PDUSessionUsageReportRATType ["nr", "eutra", "nr-unlicensed", "e-utra-unlicensed"];
        PDUSetbasedHandlingIndicator ["supported"];
        PDUsetQoSInformationPduSetIntegratedHandlingInformation ["true", "false"];
        PagingCause ["voice"];
        PagingCauseIndicationForVoiceService ["supported"];
        PagingDRX ["v32", "v64", "v128", "v256"];
        PagingOrigin ["non-3gpp"];
        PagingPriority ["priolevel1", "priolevel2", "priolevel3", "priolevel4", "priolevel5", "priolevel6", "priolevel7", "priolevel8"];
        PagingProbabilityInformation ["p00", "p05", "p10", "p15", "p20", "p25", "p30", "p35", "p40", "p45", "p50", "p55", "p60", "p65", "p70", "p75", "p80", "p85", "p90", "p95", "p100"];
        ParentTImeSource ["synce", "ptp", "gnss", "atomicclock", "terrestrialradio", "serialtimecode", "ntp", "handset", "other"];
        PedestrianUE ["authorized", "not-authorized"];
        PreEmptionCapability ["shall-not-trigger-pre-emption", "may-trigger-pre-emption"];
        PreEmptionVulnerability ["not-pre-emptable", "pre-emptable"];
        Presence ["optional", "conditional", "mandatory"];
        PrivacyIndicator ["immediate-MDT", "logged-MDT"];
        QoERVQoEReportingPathsQoEReportingPath ["srb4", "srb5"];
        QoERVQoEReportingPathsRVQoEReportingPath ["srb4", "srb5"];
        QoSFlowsUsageReportItemRATType ["nr", "eutra", "nr-unlicensed", "e-utra-unlicensed"];
        QosMonitoringRequest ["ul", "dl", "both", "stop"];
        RANTSSRequestType ["start", "stop"];
        RANTimingSynchronisationStatusInfoSynchronisationState ["locked", "holdover", "freerun"];
        RANTimingSynchronisationStatusInfoTraceabletoGNSS ["true", "false"];
        RANTimingSynchronisationStatusInfoTraceabletoUTC ["true", "false"];
        RANfeedbacktypeReactiveCapabilityForBATAdaptation ["true"];
        RATInformation ["unlicensed", "nb-IoT", "nR-LEO", "nR-MEO", "nR-GEO", "nR-OTHERSAT"];
        RIMInformationRIMRSDetection ["rs-detected", "rs-disappeared"];
        RRCEstablishmentCause ["emergency", "highPriorityAccess", "mt-Access", "mo-Signalling", "mo-Data", "mo-VoiceCall", "mo-VideoCall", "mo-SMS", "mps-PriorityAccess", "mcs-PriorityAccess", "notAvailable", "mo-ExceptionData"];
        RRCInactiveTransitionReportRequest ["subsequent-state-transition-report", "single-rrc-connected-state-report", "cancel-report"];
        RRCState ["inactive", "connected"];
        RSN ["v1", "v2"];
        Range ["m50", "m80", "m180", "m200", "m350", "m400", "m500", "m700", "m1000"];
        RedCapIndication ["redcap"];
        RedirectionVoiceFallback ["possible", "not-possible"];
        RedundantQosFlowIndicator ["true", "false"];
        ReflectiveQosAttribute ["subject-to"];
        ReportAmountMDT ["r1", "r2", "r4", "r8", "r16", "r32", "r64", "rinfinity"];
        ReportArea ["cell"];
        ReportIntervalMDT ["ms120", "ms240", "ms480", "ms640", "ms1024", "ms2048", "ms5120", "ms10240", "min1", "min6", "min12", "min30", "min60"];
        ReportingPeriodicity ["stop", "single", "ms1000", "ms2000", "ms5000", "ms10000"];
        ResetAll ["reset-all"];
        SLPositioningRangingAuthorized ["authorized", "not-authorized"];
        SONInformationRequest ["xn-TNL-configuration-info"];
        SRVCCOperationPossible ["possible", "notPossible"];
        SensorMeasConfig ["setup"];
        SensorNameConfigUeOrientationConfig ["true"];
        SensorNameConfigUeSpeedConfig ["true"];
        SensorNameConfigUncompensatedBarometricConfig ["true"];
        ServiceType ["streaming", "mTSI", "vR"];
        SourceOfUEActivityBehaviourInformation ["subscription-information", "statistics"];
        SupportedUEType ["non-RedCap-eRedCap-UE", "redCap-eRedCap-UE"];
        SuspendIndicator ["true"];
        SuspendRequestIndication ["suspend-requested"];
        SuspendResponseIndication ["suspend-indicated"];
        TNLAssociationUsage ["ue", "non-ue", "both"];
        TimeSyncAssistanceInfoTimeDistributionIndication ["enabled", "disabled"];
        TimeToTrigger ["ms0", "ms40", "ms64", "ms80", "ms100", "ms128", "ms160", "ms256", "ms320", "ms480", "ms512", "ms640", "ms1024", "ms1280", "ms2560", "ms5120"];
        TimeToWait ["v1s", "v2s", "v5s", "v10s", "v20s", "v60s"];
        TimerApproachForGUAMIRemoval ["apply-timer"];
        TraceDepth ["minimum", "medium", "maximum", "minimumWithoutVendorSpecificExtension", "mediumWithoutVendorSpecificExtension", "maximumWithoutVendorSpecificExtension", "minimumOnlyVendorSpecificTraceRecord", "mediumOnlyVendorSpecificTraceRecord", "maximumOnlyVendorSpecificTraceRecord"];
        TriggeringMessage ["initiating-message", "successful-outcome", "unsuccessful-outcome"];
        TypeOfError ["not-understood", "missing"];
        UEAppLayerMeasConfigInfoQoEMeasurementStatus ["ongoing"];
        UECapabilityInfoRequest ["requested"];
        UEContextRequest ["requested"];
        UEDifferentiationInfoBatteryIndication ["battery-powered", "battery-powered-not-rechargeable-or-replaceable", "not-battery-powered"];
        UEDifferentiationInfoPeriodicCommunicationIndicator ["periodically", "ondemand"];
        UEDifferentiationInfoStationaryIndication ["stationary", "mobile"];
        UEDifferentiationInfoTrafficProfile ["single-packet", "dual-packets", "multiple-packets"];
        UEPresence ["in", "out", "unknown"];
        UERetentionInformation ["ues-retained"];
        UEUPCIoTSupport ["supported"];
        ULForwarding ["ul-forwarding-proposed"];
        UserPlaneErrorIndicator ["gTP-U-error-indication-received"];
        UserPlaneFailureIndicationReport ["qos-flows-and-tunnel-to-be-released", "qoS-flows-to-be-moved", "new-transport-address-allocated"];
        UserPlaneFailureType ["gtp-u-error-indication-received", "up-path-failure"];
        VehicleUE ["authorized", "not-authorized"];
        WLANMeasConfig ["setup"];
        WLANMeasurementConfigurationWlanRssi ["true"];
        WLANMeasurementConfigurationWlanRtt ["true"];
        XrDeviceWith2Rx ["true"];
    }
}
use oxirush_aper::UNBOUNDED;
#[allow(unused_imports)]
use oxirush_aper::inspect::schema::{Field as F, Kind as K, Member as M, Text, Type as T};
/// The types of the protocol, in the order of the names that ASN.1 gives them, which the walker of the trees reads: a type is named by its row.
#[rustfmt::skip]
pub(crate) static SCHEMA: &[T] = &[
/* 0 */ T { name: "A2X-PC5-FlowBitRates", kind: K::Sequence { names: &["a2X-GuaranteedFlowBitRate", "a2X-MaximumFlowBitRate", "iE-Extensions"], members: &[M { field: "a2x_guaranteed_flow_bit_rate", ty: F::Named(124), optional: false }, M { field: "a2x_maximum_flow_bit_rate", ty: F::Named(124), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1 */ T { name: "A2X-PC5-QoS-Flow-Item", kind: K::Sequence { names: &["a2X-PQI", "a2X-PC5-FlowBitRates", "a2X-Range", "iE-Extensions"], members: &[M { field: "a2x_pqi", ty: F::Unsigned(0, 255, true), optional: false }, M { field: "a2x_pc5_flow_bit_rates", ty: F::Named(0), optional: true }, M { field: "a2x_range", ty: F::Named(4), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 2 */ T { name: "A2X-PC5-QoS-Flow-List", kind: K::Alias { field: F::List(&F::Named(1), 1, 2048), form: None, secret: false } },
/* 3 */ T { name: "A2X-PC5-QoS-Parameters", kind: K::Sequence { names: &["a2X-PC5-QoS-Flow-List", "a2X-PC5-LinkAggregateBitRates", "iE-Extensions"], members: &[M { field: "a2x_pc5_qos_flow_list", ty: F::Named(2), optional: false }, M { field: "a2x_pc5_link_aggregate_bit_rates", ty: F::Named(124), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 4 */ T { name: "A2XPC5QoSFlowItemA2XRange", kind: K::Enumerated { names: &["m50", "m80", "m180", "m200", "m350", "m400", "m500", "m700", "m1000"], root: 9, extensible: true } },
/* 5 */ T { name: "AIOTFIdentifier", kind: K::Alias { field: F::Octets(16, 36, true), form: None, secret: false } },
/* 6 */ T { name: "AIOTFName", kind: K::Sequence { names: &["aIOTFNameVisibleString", "aIOTFNameUTF8String", "iE-Extensions"], members: &[M { field: "aiotf_name_visible_string", ty: F::Named(8), optional: true }, M { field: "aiotf_name_utf8_string", ty: F::Named(7), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 7 */ T { name: "AIOTFNameUTF8String", kind: K::Alias { field: F::Text(Text::Utf8, 1, 150, true), form: None, secret: false } },
/* 8 */ T { name: "AIOTFNameVisibleString", kind: K::Alias { field: F::Text(Text::Visible, 1, 150, true), form: None, secret: false } },
/* 9 */ T { name: "AIOTSessionReleaseCommand", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 10 */ T { name: "AIOTSessionReleaseCommandTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 11 */ T { name: "AIOTSessionReleaseComplete", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 12 */ T { name: "AIOTSessionReleaseCompleteTransfer", kind: K::Sequence { names: &["correlationIdentifier", "criticalityDiagnostics", "iE-Extensions"], members: &[M { field: "correlation_identifier", ty: F::Named(16), optional: false }, M { field: "criticality_diagnostics", ty: F::Named(246), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 13 */ T { name: "AIOTSessionReleaseRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 14 */ T { name: "AIOTSessionReleaseRequestTransfer", kind: K::Sequence { names: &["correlationIdentifier", "cause", "iE-Extensions"], members: &[M { field: "correlation_identifier", ty: F::Named(16), optional: false }, M { field: "cause", ty: F::Named(185), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 15 */ T { name: "AIoT-CommandAssistanceInformation", kind: K::Sequence { names: &["estimateofExpectedD2RMsgSize", "iE-Extensions"], members: &[M { field: "estimateof_expected_d2r_msg_size", ty: F::Unsigned(1, 256, true), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 16 */ T { name: "AIoT-CorrelationIdentifier", kind: K::Alias { field: F::Unsigned(0, 65535, true), form: None, secret: false } },
/* 17 */ T { name: "AIoT-DeviceIdentificationRequested", kind: K::Choice { names: &["singleDevice", "groupDevices", "allDevices", "choice-Extensions"], alternatives: &[F::BitString(1, 1024, true), F::BitString(1, 1024, true), F::Null, F::Named(948)], root: 4, extensible: false } },
/* 18 */ T { name: "AIoT-DeviceReportItem", kind: K::Sequence { names: &["aIoT-NASPDU", "rAN-AIOT-Device-NGAP-ID", "iE-Extensions"], members: &[M { field: "aiot_nas_pdu", ty: F::Named(22), optional: false }, M { field: "ran_aiot_device_ngap_id", ty: F::Named(999), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 19 */ T { name: "AIoT-DeviceReportList", kind: K::Alias { field: F::List(&F::Named(18), 1, 65535), form: None, secret: false } },
/* 20 */ T { name: "AIoT-FollowonCommandIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 21 */ T { name: "AIoT-InventoryAssistanceInformation", kind: K::Sequence { names: &["expectedD2RMsgSize", "approximateNoofTargetDevices", "timeInterval", "iE-Extensions"], members: &[M { field: "expected_d2r_msg_size", ty: F::Unsigned(1, 256, true), optional: false }, M { field: "approximate_noof_target_devices", ty: F::Unsigned(1, 65535, true), optional: true }, M { field: "time_interval", ty: F::Unsigned(1, 256, true), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 22 */ T { name: "AIoT-NASPDU", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 23 */ T { name: "AIoT-ReaderIndex", kind: K::Alias { field: F::Unsigned(1, 65535, true), form: None, secret: false } },
/* 24 */ T { name: "AIoT-ReaderReportItem", kind: K::Sequence { names: &["readerIndex", "deviceReportList", "iE-Extensions"], members: &[M { field: "reader_index", ty: F::Named(23), optional: false }, M { field: "device_report_list", ty: F::Named(19), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 25 */ T { name: "AIoT-ReaderReportList", kind: K::Alias { field: F::List(&F::Named(24), 1, 65535), form: None, secret: false } },
/* 26 */ T { name: "AIoT-RequestedAIoTAreaList", kind: K::Alias { field: F::List(&F::Named(32), 1, 256), form: None, secret: false } },
/* 27 */ T { name: "AIoT-RequestedReaderItem", kind: K::Sequence { names: &["globalgNB-ID", "readerIndex", "iE-Extensions"], members: &[M { field: "global_gnb_id", ty: F::Named(425), optional: false }, M { field: "reader_index", ty: F::Named(23), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 28 */ T { name: "AIoT-RequestedReaderList", kind: K::Alias { field: F::List(&F::Named(27), 1, 65535), form: None, secret: false } },
/* 29 */ T { name: "AIoT-RequestedServiceAreaInformation", kind: K::Sequence { names: &["requestedReaderList", "requestedAIoTAreaList", "iE-Extensions"], members: &[M { field: "requested_reader_list", ty: F::Named(28), optional: true }, M { field: "requested_aiot_area_list", ty: F::Named(26), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 30 */ T { name: "AIoT-SecurityInfo", kind: K::Choice { names: &["a-IoT-RAND-n", "choice-Extensions"], alternatives: &[F::BitString(128, 128, true), F::Named(948)], root: 2, extensible: false } },
/* 31 */ T { name: "AIoT-Support", kind: K::Enumerated { names: &["aIoT-only", "aIoT-and-NRUu"], root: 2, extensible: true } },
/* 32 */ T { name: "AIoTAreaID", kind: K::Sequence { names: &["pLMNidentity", "nID", "aIoTAreaCode", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "nid", ty: F::Named(705), optional: true }, M { field: "aiot_area_code", ty: F::OctetArray(3), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 33 */ T { name: "AMF-TNLAssociationSetupItem", kind: K::Sequence { names: &["aMF-TNLAssociationAddress", "iE-Extensions"], members: &[M { field: "amf_tnl_association_address", ty: F::Named(167), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 34 */ T { name: "AMF-TNLAssociationSetupList", kind: K::Alias { field: F::List(&F::Named(33), 1, 32), form: None, secret: false } },
/* 35 */ T { name: "AMF-TNLAssociationToAddItem", kind: K::Sequence { names: &["aMF-TNLAssociationAddress", "tNLAssociationUsage", "tNLAddressWeightFactor", "iE-Extensions"], members: &[M { field: "amf_tnl_association_address", ty: F::Named(167), optional: false }, M { field: "tnl_association_usage", ty: F::Named(1186), optional: true }, M { field: "tnl_address_weight_factor", ty: F::Named(1183), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 36 */ T { name: "AMF-TNLAssociationToAddList", kind: K::Alias { field: F::List(&F::Named(35), 1, 32), form: None, secret: false } },
/* 37 */ T { name: "AMF-TNLAssociationToRemoveItem", kind: K::Sequence { names: &["aMF-TNLAssociationAddress", "iE-Extensions"], members: &[M { field: "amf_tnl_association_address", ty: F::Named(167), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 38 */ T { name: "AMF-TNLAssociationToRemoveList", kind: K::Alias { field: F::List(&F::Named(37), 1, 32), form: None, secret: false } },
/* 39 */ T { name: "AMF-TNLAssociationToUpdateItem", kind: K::Sequence { names: &["aMF-TNLAssociationAddress", "tNLAssociationUsage", "tNLAddressWeightFactor", "iE-Extensions"], members: &[M { field: "amf_tnl_association_address", ty: F::Named(167), optional: false }, M { field: "tnl_association_usage", ty: F::Named(1186), optional: true }, M { field: "tnl_address_weight_factor", ty: F::Named(1183), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 40 */ T { name: "AMF-TNLAssociationToUpdateList", kind: K::Alias { field: F::List(&F::Named(39), 1, 32), form: None, secret: false } },
/* 41 */ T { name: "AMF-UE-NGAP-ID", kind: K::Alias { field: F::Unsigned(0, 1099511627775, false), form: None, secret: false } },
/* 42 */ T { name: "AMFCPRelocationIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 43 */ T { name: "AMFConfigurationUpdate", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 44 */ T { name: "AMFConfigurationUpdateAcknowledge", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 45 */ T { name: "AMFConfigurationUpdateFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 46 */ T { name: "AMFName", kind: K::Alias { field: F::Text(Text::Printable, 1, 150, true), form: None, secret: false } },
/* 47 */ T { name: "AMFNameUTF8String", kind: K::Alias { field: F::Text(Text::Utf8, 1, 150, true), form: None, secret: false } },
/* 48 */ T { name: "AMFNameVisibleString", kind: K::Alias { field: F::Text(Text::Visible, 1, 150, true), form: None, secret: false } },
/* 49 */ T { name: "AMFPagingTarget", kind: K::Choice { names: &["globalRANNodeID", "tAI", "choice-Extensions"], alternatives: &[F::Named(430), F::Named(1154), F::Named(948)], root: 3, extensible: false } },
/* 50 */ T { name: "AMFPointer", kind: K::Alias { field: F::Bits(6, 6), form: Some(Form::Number(6)), secret: false } },
/* 51 */ T { name: "AMFRegionID", kind: K::Alias { field: F::Bits(8, 8), form: Some(Form::Number(8)), secret: false } },
/* 52 */ T { name: "AMFSetID", kind: K::Alias { field: F::Bits(10, 10), form: Some(Form::Number(10)), secret: false } },
/* 53 */ T { name: "AMFStatusIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 54 */ T { name: "AUN3DeviceAccessInfo", kind: K::Sequence { names: &["aUN3DeviceAccess", "existingNGConnectionwithSameLocation", "iE-Extensions"], members: &[M { field: "aun3_device_access", ty: F::Named(55), optional: false }, M { field: "existing_ng_connectionwith_same_location", ty: F::Named(56), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 55 */ T { name: "AUN3DeviceAccessInfoAUN3DeviceAccess", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 56 */ T { name: "AUN3DeviceAccessInfoExistingNGConnectionwithSameLocation", kind: K::Enumerated { names: &["true", "false"], root: 2, extensible: true } },
/* 57 */ T { name: "ActivatedCellList", kind: K::Alias { field: F::List(&F::Named(684), 1, 16384), form: None, secret: false } },
/* 58 */ T { name: "ActivationStatus", kind: K::Enumerated { names: &["active", "not-active"], root: 2, extensible: true } },
/* 59 */ T { name: "AdditionalCancelledlocationReportingReferenceIDItem", kind: K::Sequence { names: &["locationReportingReferenceIDToBeCancelled", "iE-Extensions"], members: &[M { field: "location_reporting_reference_id_to_be_cancelled", ty: F::Named(537), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 60 */ T { name: "AdditionalCancelledlocationReportingReferenceIDList", kind: K::Alias { field: F::List(&F::Named(59), 1, 63), form: None, secret: false } },
/* 61 */ T { name: "AdditionalDLUPTNLInformationForHOItem", kind: K::Sequence { names: &["additionalDL-NGU-UP-TNLInformation", "additionalQosFlowSetupResponseList", "additionalDLForwardingUPTNLInformation", "iE-Extensions"], members: &[M { field: "additional_dl_ngu_up_tnl_information", ty: F::Named(1298), optional: false }, M { field: "additional_qos_flow_setup_response_list", ty: F::Named(982), optional: false }, M { field: "additional_dl_forwarding_up_tnl_information", ty: F::Named(1298), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 62 */ T { name: "AdditionalDLUPTNLInformationForHOList", kind: K::Alias { field: F::List(&F::Named(61), 1, 3), form: None, secret: false } },
/* 63 */ T { name: "AdditionalQosFlowInformation", kind: K::Enumerated { names: &["more-likely"], root: 1, extensible: true } },
/* 64 */ T { name: "AdditionalULI", kind: K::Sequence { names: &["nRCGI", "tAI", "iE-Extensions"], members: &[M { field: "nr_cgi", ty: F::Named(711), optional: false }, M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 65 */ T { name: "Aerial-UE-FlightInformationOngoingReporting-failed", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 66 */ T { name: "Aerial-UE-FlightInformationReporting", kind: K::Sequence { names: &["altitude", "timeStamp", "iE-Extensions"], members: &[M { field: "altitude", ty: F::Named(85), optional: false }, M { field: "time_stamp", ty: F::Named(1212), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 67 */ T { name: "Aerial-UE-FlightInformationReportingControl", kind: K::Sequence { names: &["higher-Altitude-Threshold", "lower-Altitude-Threshold", "aerial-UE-report-periodicity", "areaID", "iE-Extensions"], members: &[M { field: "higher_altitude_threshold", ty: F::Named(85), optional: false }, M { field: "lower_altitude_threshold", ty: F::Named(85), optional: false }, M { field: "aerial_ue_report_periodicity", ty: F::Named(70), optional: true }, M { field: "area_id", ty: F::Named(86), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 68 */ T { name: "Aerial-UE-FlightInformationReportingControlItem", kind: K::Sequence { names: &["aerial-UE-FlightInformationReportingControl", "iE-Extensions"], members: &[M { field: "aerial_ue_flight_information_reporting_control", ty: F::Named(67), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 69 */ T { name: "Aerial-UE-FlightInformationReportingControlList", kind: K::Alias { field: F::List(&F::Named(68), 1, 64), form: None, secret: false } },
/* 70 */ T { name: "AerialUEReportingPeriodicity", kind: K::Enumerated { names: &["ms120", "ms240", "ms480", "ms640", "ms1024", "ms2048", "ms5120", "ms10240", "ms20480", "ms40960", "min1", "min6", "min12", "min30"], root: 14, extensible: true } },
/* 71 */ T { name: "AerialUEsubscriptionInformation", kind: K::Enumerated { names: &["allowed", "not-allowed"], root: 2, extensible: true } },
/* 72 */ T { name: "AllocationAndRetentionPriority", kind: K::Sequence { names: &["priorityLevelARP", "pre-emptionCapability", "pre-emptionVulnerability", "iE-Extensions"], members: &[M { field: "priority_level_arp", ty: F::Named(936), optional: false }, M { field: "pre_emption_capability", ty: F::Named(933), optional: false }, M { field: "pre_emption_vulnerability", ty: F::Named(934), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 73 */ T { name: "Allowed-CAG-List-per-PLMN", kind: K::Alias { field: F::List(&F::Named(153), 1, 256), form: None, secret: false } },
/* 74 */ T { name: "Allowed-PNI-NPN-Item", kind: K::Sequence { names: &["pLMNIdentity", "pNI-NPN-restricted", "allowed-CAG-List-per-PLMN", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "pni_npn_restricted", ty: F::Named(78), optional: false }, M { field: "allowed_cag_list_per_plmn", ty: F::Named(73), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 75 */ T { name: "Allowed-PNI-NPN-List", kind: K::Alias { field: F::List(&F::Named(74), 1, 16), form: None, secret: false } },
/* 76 */ T { name: "AllowedNSSAI", kind: K::Alias { field: F::List(&F::Named(77), 1, 8), form: None, secret: false } },
/* 77 */ T { name: "AllowedNSSAI-Item", kind: K::Sequence { names: &["s-NSSAI", "iE-Extensions"], members: &[M { field: "s_nssai", ty: F::Named(1074), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 78 */ T { name: "AllowedPNINPNItemPNINPNRestricted", kind: K::Enumerated { names: &["restricted", "not-restricted"], root: 2, extensible: true } },
/* 79 */ T { name: "AllowedPeriodicityList", kind: K::Alias { field: F::List(&F::Named(928), 1, 8), form: None, secret: false } },
/* 80 */ T { name: "AllowedTACs", kind: K::Alias { field: F::List(&F::Named(1152), 1, 16), form: None, secret: false } },
/* 81 */ T { name: "AlternativeQoSParaSetIndex", kind: K::Alias { field: F::Unsigned(1, 8, true), form: None, secret: false } },
/* 82 */ T { name: "AlternativeQoSParaSetItem", kind: K::Sequence { names: &["alternativeQoSParaSetIndex", "guaranteedFlowBitRateDL", "guaranteedFlowBitRateUL", "packetDelayBudget", "packetErrorRate", "iE-Extensions"], members: &[M { field: "alternative_qos_para_set_index", ty: F::Named(81), optional: false }, M { field: "guaranteed_flow_bit_rate_dl", ty: F::Named(124), optional: true }, M { field: "guaranteed_flow_bit_rate_ul", ty: F::Named(124), optional: true }, M { field: "packet_delay_budget", ty: F::Named(898), optional: true }, M { field: "packet_error_rate", ty: F::Named(899), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 83 */ T { name: "AlternativeQoSParaSetList", kind: K::Alias { field: F::List(&F::Named(82), 1, 8), form: None, secret: false } },
/* 84 */ T { name: "AlternativeQoSParaSetNotifyIndex", kind: K::Alias { field: F::Unsigned(0, 8, true), form: None, secret: false } },
/* 85 */ T { name: "Altitude", kind: K::Alias { field: F::Signed(-420, 10000, true), form: None, secret: false } },
/* 86 */ T { name: "Area-ID", kind: K::Choice { names: &["tAI", "globalRANNodeID", "nR-CGI", "choice-Extensions"], alternatives: &[F::Named(1154), F::Named(430), F::Named(711), F::Named(948)], root: 4, extensible: false } },
/* 87 */ T { name: "AreaOfInterest", kind: K::Sequence { names: &["areaOfInterestTAIList", "areaOfInterestCellList", "areaOfInterestRANNodeList", "iE-Extensions"], members: &[M { field: "area_of_interest_tai_list", ty: F::Named(95), optional: true }, M { field: "area_of_interest_cell_list", ty: F::Named(89), optional: true }, M { field: "area_of_interest_ran_node_list", ty: F::Named(93), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 88 */ T { name: "AreaOfInterestCellItem", kind: K::Sequence { names: &["nGRAN-CGI", "iE-Extensions"], members: &[M { field: "ngran_cgi", ty: F::Named(684), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 89 */ T { name: "AreaOfInterestCellList", kind: K::Alias { field: F::List(&F::Named(88), 1, 256), form: None, secret: false } },
/* 90 */ T { name: "AreaOfInterestItem", kind: K::Sequence { names: &["areaOfInterest", "locationReportingReferenceID", "iE-Extensions"], members: &[M { field: "area_of_interest", ty: F::Named(87), optional: false }, M { field: "location_reporting_reference_id", ty: F::Named(537), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 91 */ T { name: "AreaOfInterestList", kind: K::Alias { field: F::List(&F::Named(90), 1, 64), form: None, secret: false } },
/* 92 */ T { name: "AreaOfInterestRANNodeItem", kind: K::Sequence { names: &["globalRANNodeID", "iE-Extensions"], members: &[M { field: "global_ran_node_id", ty: F::Named(430), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 93 */ T { name: "AreaOfInterestRANNodeList", kind: K::Alias { field: F::List(&F::Named(92), 1, 64), form: None, secret: false } },
/* 94 */ T { name: "AreaOfInterestTAIItem", kind: K::Sequence { names: &["tAI", "iE-Extensions"], members: &[M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 95 */ T { name: "AreaOfInterestTAIList", kind: K::Alias { field: F::List(&F::Named(94), 1, 16), form: None, secret: false } },
/* 96 */ T { name: "AreaScopeOfMDT-EUTRA", kind: K::Choice { names: &["cellBased", "tABased", "pLMNWide", "tAIBased", "choice-Extensions"], alternatives: &[F::Named(192), F::Named(1150), F::Null, F::Named(1155), F::Named(948)], root: 5, extensible: false } },
/* 97 */ T { name: "AreaScopeOfMDT-NR", kind: K::Choice { names: &["cellBased", "tABased", "pLMNWide", "tAIBased", "choice-Extensions"], alternatives: &[F::Named(193), F::Named(1150), F::Null, F::Named(1155), F::Named(948)], root: 5, extensible: false } },
/* 98 */ T { name: "AreaScopeOfNeighCellsItem", kind: K::Sequence { names: &["nrFrequencyInfo", "pciListForMDT", "iE-Extensions"], members: &[M { field: "nr_frequency_info", ty: F::Named(728), optional: false }, M { field: "pci_list_for_mdt", ty: F::Named(773), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 99 */ T { name: "AreaScopeOfNeighCellsList", kind: K::Alias { field: F::List(&F::Named(98), 1, 8), form: None, secret: false } },
/* 100 */ T { name: "AreaScopeOfQMC", kind: K::Choice { names: &["cellBased", "tABased", "tAIBased", "pLMNAreaBased", "choice-Extensions"], alternatives: &[F::Named(194), F::Named(1151), F::Named(1156), F::Named(885), F::Named(948)], root: 5, extensible: false } },
/* 101 */ T { name: "AssistanceDataForPaging", kind: K::Sequence { names: &["assistanceDataForRecommendedCells", "pagingAttemptInformation", "iE-Extensions"], members: &[M { field: "assistance_data_for_recommended_cells", ty: F::Named(102), optional: true }, M { field: "paging_attempt_information", ty: F::Named(904), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 102 */ T { name: "AssistanceDataForRecommendedCells", kind: K::Sequence { names: &["recommendedCellsForPaging", "iE-Extensions"], members: &[M { field: "recommended_cells_for_paging", ty: F::Named(1044), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 103 */ T { name: "AssistanceInformationQoE-Meas", kind: K::Alias { field: F::Unsigned(1, 16, true), form: None, secret: false } },
/* 104 */ T { name: "AssociatedMBSQosFlowSetupRequestItem", kind: K::Sequence { names: &["mBS-QosFlowIdentifier", "associatedUnicastQosFlowIdentifier", "iE-Extensions"], members: &[M { field: "mbs_qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "associated_unicast_qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 105 */ T { name: "AssociatedMBSQosFlowSetupRequestList", kind: K::Alias { field: F::List(&F::Named(104), 1, 64), form: None, secret: false } },
/* 106 */ T { name: "AssociatedMBSQosFlowSetuporModifyRequestItem", kind: K::Sequence { names: &["mBS-QosFlowIdentifier", "associatedUnicastQosFlowIdentifier", "iE-Extensions"], members: &[M { field: "mbs_qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "associated_unicast_qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 107 */ T { name: "AssociatedMBSQosFlowSetuporModifyRequestList", kind: K::Alias { field: F::List(&F::Named(106), 1, 64), form: None, secret: false } },
/* 108 */ T { name: "AssociatedQosFlowItem", kind: K::Sequence { names: &["qosFlowIdentifier", "qosFlowMappingIndication", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "qos_flow_mapping_indication", ty: F::Named(109), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 109 */ T { name: "AssociatedQosFlowItemQosFlowMappingIndication", kind: K::Enumerated { names: &["ul", "dl"], root: 2, extensible: true } },
/* 110 */ T { name: "AssociatedQosFlowList", kind: K::Alias { field: F::List(&F::Named(108), 1, 64), form: None, secret: false } },
/* 111 */ T { name: "AssociatedSessionID", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 112 */ T { name: "AuthenticatedIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 113 */ T { name: "AvailableBitrateReportThresholdItem", kind: K::Sequence { names: &["reportingThreshold", "iE-Extensions"], members: &[M { field: "reporting_threshold", ty: F::Named(1064), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 114 */ T { name: "AvailableBitrateReportThresholdList", kind: K::Alias { field: F::List(&F::Named(113), 1, 8), form: None, secret: false } },
/* 115 */ T { name: "AvailableRANVisibleQoEMetrics", kind: K::Sequence { names: &["applicationLayerBufferLevelList", "playoutDelayForMediaStartup", "iE-Extensions"], members: &[M { field: "application_layer_buffer_level_list", ty: F::Named(116), optional: true }, M { field: "playout_delay_for_media_startup", ty: F::Named(117), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 116 */ T { name: "AvailableRANVisibleQoEMetricsApplicationLayerBufferLevelList", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 117 */ T { name: "AvailableRANVisibleQoEMetricsPlayoutDelayForMediaStartup", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 118 */ T { name: "AveragingWindow", kind: K::Alias { field: F::Unsigned(0, 4095, true), form: None, secret: false } },
/* 119 */ T { name: "BeamMeasurementsReportConfiguration", kind: K::Sequence { names: &["beamMeasurementsReportQuantity", "maxNrofRS-IndexesToReport", "iE-Extensions"], members: &[M { field: "beam_measurements_report_quantity", ty: F::Named(120), optional: true }, M { field: "max_nrof_rs_indexes_to_report", ty: F::Named(636), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 120 */ T { name: "BeamMeasurementsReportQuantity", kind: K::Sequence { names: &["rSRP", "rSRQ", "sINR", "iE-Extensions"], members: &[M { field: "rsrp", ty: F::Named(121), optional: false }, M { field: "rsrq", ty: F::Named(122), optional: false }, M { field: "sinr", ty: F::Named(123), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 121 */ T { name: "BeamMeasurementsReportQuantityRSRP", kind: K::Enumerated { names: &["true", "false"], root: 1, extensible: true } },
/* 122 */ T { name: "BeamMeasurementsReportQuantityRSRQ", kind: K::Enumerated { names: &["true", "false"], root: 1, extensible: true } },
/* 123 */ T { name: "BeamMeasurementsReportQuantitySINR", kind: K::Enumerated { names: &["true", "false"], root: 1, extensible: true } },
/* 124 */ T { name: "BitRate", kind: K::Alias { field: F::Unsigned(0, 4000000000000, true), form: None, secret: false } },
/* 125 */ T { name: "BluetoothMeasConfig", kind: K::Enumerated { names: &["setup"], root: 1, extensible: true } },
/* 126 */ T { name: "BluetoothMeasConfigNameItem", kind: K::Sequence { names: &["bluetoothName", "iE-Extensions"], members: &[M { field: "bluetooth_name", ty: F::Named(130), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 127 */ T { name: "BluetoothMeasConfigNameList", kind: K::Alias { field: F::List(&F::Named(126), 1, 4), form: None, secret: false } },
/* 128 */ T { name: "BluetoothMeasurementConfiguration", kind: K::Sequence { names: &["bluetoothMeasConfig", "bluetoothMeasConfigNameList", "bt-rssi", "iE-Extensions"], members: &[M { field: "bluetooth_meas_config", ty: F::Named(125), optional: false }, M { field: "bluetooth_meas_config_name_list", ty: F::Named(127), optional: true }, M { field: "bt_rssi", ty: F::Named(129), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 129 */ T { name: "BluetoothMeasurementConfigurationBtRssi", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 130 */ T { name: "BluetoothName", kind: K::Alias { field: F::Octets(1, 248, false), form: None, secret: false } },
/* 131 */ T { name: "BroadcastCancelledAreaList", kind: K::Choice { names: &["cellIDCancelledEUTRA", "tAICancelledEUTRA", "emergencyAreaIDCancelledEUTRA", "cellIDCancelledNR", "tAICancelledNR", "emergencyAreaIDCancelledNR", "choice-Extensions"], alternatives: &[F::Named(200), F::Named(1161), F::Named(340), F::Named(202), F::Named(1163), F::Named(342), F::Named(948)], root: 7, extensible: false } },
/* 132 */ T { name: "BroadcastCompletedAreaList", kind: K::Choice { names: &["cellIDBroadcastEUTRA", "tAIBroadcastEUTRA", "emergencyAreaIDBroadcastEUTRA", "cellIDBroadcastNR", "tAIBroadcastNR", "emergencyAreaIDBroadcastNR", "choice-Extensions"], alternatives: &[F::Named(196), F::Named(1157), F::Named(336), F::Named(198), F::Named(1159), F::Named(338), F::Named(948)], root: 7, extensible: false } },
/* 133 */ T { name: "BroadcastPLMNItem", kind: K::Sequence { names: &["pLMNIdentity", "tAISliceSupportList", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "tai_slice_support_list", ty: F::Named(1128), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 134 */ T { name: "BroadcastPLMNList", kind: K::Alias { field: F::List(&F::Named(133), 1, 12), form: None, secret: false } },
/* 135 */ T { name: "BroadcastSessionModificationFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 136 */ T { name: "BroadcastSessionModificationRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 137 */ T { name: "BroadcastSessionModificationResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 138 */ T { name: "BroadcastSessionReleaseRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 139 */ T { name: "BroadcastSessionReleaseRequired", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 140 */ T { name: "BroadcastSessionReleaseResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 141 */ T { name: "BroadcastSessionSetupFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 142 */ T { name: "BroadcastSessionSetupRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 143 */ T { name: "BroadcastSessionSetupResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 144 */ T { name: "BroadcastSessionTransportFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 145 */ T { name: "BroadcastSessionTransportRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 146 */ T { name: "BroadcastSessionTransportResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 147 */ T { name: "BroadcastTransportFailureTransfer", kind: K::Sequence { names: &["mBS-SessionID", "cause", "criticalityDiagnostics", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "cause", ty: F::Named(185), optional: false }, M { field: "criticality_diagnostics", ty: F::Named(246), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 148 */ T { name: "BroadcastTransportRequestTransfer", kind: K::Sequence { names: &["mBS-SessionID", "mBS-SessionTNLInfoNGRAN", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "mbs_session_tnl_info_ngran", ty: F::Named(592), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 149 */ T { name: "BroadcastTransportResponseTransfer", kind: K::Sequence { names: &["mBS-SessionID", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 150 */ T { name: "BurstArrivalTime", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 151 */ T { name: "BurstArrivalTimeWindow", kind: K::Sequence { names: &["burstArrivalTimeWindowStart", "burstArrivalTimeWindowEnd", "iE-Extension"], members: &[M { field: "burst_arrival_time_window_start", ty: F::Unsigned(0, 640000, true), optional: false }, M { field: "burst_arrival_time_window_end", ty: F::Unsigned(0, 640000, true), optional: false }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 152 */ T { name: "C-RNTI", kind: K::Alias { field: F::Bits(16, 16), form: None, secret: false } },
/* 153 */ T { name: "CAG-ID", kind: K::Alias { field: F::Bits(32, 32), form: None, secret: false } },
/* 154 */ T { name: "CAGListforMDT", kind: K::Alias { field: F::List(&F::Named(155), 1, 256), form: None, secret: false } },
/* 155 */ T { name: "CAGListforMDTItem", kind: K::Sequence { names: &["plmnID", "cAGID", "iE-Extensions"], members: &[M { field: "plmn_id", ty: F::Named(886), optional: false }, M { field: "cag_id", ty: F::Named(153), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 156 */ T { name: "CEmodeBSupport-Indicator", kind: K::Enumerated { names: &["supported"], root: 1, extensible: true } },
/* 157 */ T { name: "CEmodeBrestricted", kind: K::Enumerated { names: &["restricted", "not-restricted"], root: 2, extensible: true } },
/* 158 */ T { name: "CN-MT-CommunicationHandling", kind: K::Enumerated { names: &["supported"], root: 1, extensible: true } },
/* 159 */ T { name: "CNAssistedRANTuning", kind: K::Sequence { names: &["expectedUEBehaviour", "iE-Extensions"], members: &[M { field: "expected_ue_behaviour", ty: F::Named(368), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 160 */ T { name: "CNTypeRestrictionsForEquivalent", kind: K::Alias { field: F::List(&F::Named(161), 1, 15), form: None, secret: false } },
/* 161 */ T { name: "CNTypeRestrictionsForEquivalentItem", kind: K::Sequence { names: &["plmnIdentity", "cn-Type", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "cn_type", ty: F::Named(162), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 162 */ T { name: "CNTypeRestrictionsForEquivalentItemCnType", kind: K::Enumerated { names: &["epc-forbidden", "fiveGC-forbidden"], root: 2, extensible: true } },
/* 163 */ T { name: "CNTypeRestrictionsForServing", kind: K::Enumerated { names: &["epc-forbidden"], root: 1, extensible: true } },
/* 164 */ T { name: "CNsubgroupID", kind: K::Alias { field: F::Unsigned(0, 7, true), form: None, secret: false } },
/* 165 */ T { name: "COUNTValueForPDCP-SN12", kind: K::Sequence { names: &["pDCP-SN12", "hFN-PDCP-SN12", "iE-Extensions"], members: &[M { field: "pdcp_sn12", ty: F::Unsigned(0, 4095, false), optional: false }, M { field: "hfn_pdcp_sn12", ty: F::Unsigned(0, 1048575, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 166 */ T { name: "COUNTValueForPDCP-SN18", kind: K::Sequence { names: &["pDCP-SN18", "hFN-PDCP-SN18", "iE-Extensions"], members: &[M { field: "pdcp_sn18", ty: F::Unsigned(0, 262143, false), optional: false }, M { field: "hfn_pdcp_sn18", ty: F::Unsigned(0, 16383, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 167 */ T { name: "CPTransportLayerInformation", kind: K::Choice { names: &["endpointIPAddress", "choice-Extensions"], alternatives: &[F::Named(1230), F::Named(948)], root: 2, extensible: false } },
/* 168 */ T { name: "CancelAllWarningMessages", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 169 */ T { name: "CancelledCellsInEAI-EUTRA", kind: K::Alias { field: F::List(&F::Named(170), 1, 65535), form: None, secret: false } },
/* 170 */ T { name: "CancelledCellsInEAI-EUTRA-Item", kind: K::Sequence { names: &["eUTRA-CGI", "numberOfBroadcasts", "iE-Extensions"], members: &[M { field: "eutra_cgi", ty: F::Named(315), optional: false }, M { field: "number_of_broadcasts", ty: F::Named(758), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 171 */ T { name: "CancelledCellsInEAI-NR", kind: K::Alias { field: F::List(&F::Named(172), 1, 65535), form: None, secret: false } },
/* 172 */ T { name: "CancelledCellsInEAI-NR-Item", kind: K::Sequence { names: &["nR-CGI", "numberOfBroadcasts", "iE-Extensions"], members: &[M { field: "nr_cgi", ty: F::Named(711), optional: false }, M { field: "number_of_broadcasts", ty: F::Named(758), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 173 */ T { name: "CancelledCellsInTAI-EUTRA", kind: K::Alias { field: F::List(&F::Named(174), 1, 65535), form: None, secret: false } },
/* 174 */ T { name: "CancelledCellsInTAI-EUTRA-Item", kind: K::Sequence { names: &["eUTRA-CGI", "numberOfBroadcasts", "iE-Extensions"], members: &[M { field: "eutra_cgi", ty: F::Named(315), optional: false }, M { field: "number_of_broadcasts", ty: F::Named(758), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 175 */ T { name: "CancelledCellsInTAI-NR", kind: K::Alias { field: F::List(&F::Named(176), 1, 65535), form: None, secret: false } },
/* 176 */ T { name: "CancelledCellsInTAI-NR-Item", kind: K::Sequence { names: &["nR-CGI", "numberOfBroadcasts", "iE-Extensions"], members: &[M { field: "nr_cgi", ty: F::Named(711), optional: false }, M { field: "number_of_broadcasts", ty: F::Named(758), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 177 */ T { name: "CandidateCell", kind: K::Choice { names: &["candidateCGI", "candidatePCI", "choice-Extensions"], alternatives: &[F::Named(178), F::Named(181), F::Named(948)], root: 3, extensible: false } },
/* 178 */ T { name: "CandidateCellID", kind: K::Sequence { names: &["candidateCellID", "iE-Extensions"], members: &[M { field: "candidate_cell_id", ty: F::Named(711), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 179 */ T { name: "CandidateCellItem", kind: K::Sequence { names: &["candidateCell", "iE-Extensions"], members: &[M { field: "candidate_cell", ty: F::Named(177), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 180 */ T { name: "CandidateCellList", kind: K::Alias { field: F::List(&F::Named(179), 1, 32), form: None, secret: false } },
/* 181 */ T { name: "CandidatePCI", kind: K::Sequence { names: &["candidatePCI", "candidateNRARFCN", "iE-Extensions"], members: &[M { field: "candidate_pci", ty: F::Unsigned(0, 1007, true), optional: false }, M { field: "candidate_nr_arfcn", ty: F::Unsigned(0, 3279165, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 182 */ T { name: "CandidateRelayUE-ID", kind: K::Alias { field: F::Bits(24, 24), form: None, secret: false } },
/* 183 */ T { name: "CandidateRelayUEInformationItem", kind: K::Sequence { names: &["candidateRelayUE-Id", "iE-Extensions"], members: &[M { field: "candidate_relay_ue_id", ty: F::Named(182), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 184 */ T { name: "CandidateRelayUEInformationList", kind: K::Alias { field: F::List(&F::Named(183), 1, 32), form: None, secret: false } },
/* 185 */ T { name: "Cause", kind: K::Choice { names: &["radioNetwork", "transport", "nas", "protocol", "misc", "choice-Extensions"], alternatives: &[F::Named(189), F::Named(190), F::Named(187), F::Named(188), F::Named(186), F::Named(948)], root: 6, extensible: false } },
/* 186 */ T { name: "CauseMisc", kind: K::Enumerated { names: &["control-processing-overload", "not-enough-user-plane-processing-resources", "hardware-failure", "om-intervention", "unknown-PLMN-or-SNPN", "unspecified"], root: 6, extensible: true } },
/* 187 */ T { name: "CauseNas", kind: K::Enumerated { names: &["normal-release", "authentication-failure", "deregister", "unspecified", "uE-not-in-PLMN-serving-area", "mobile-IAB-not-authorized", "iAB-not-authorized", "no-aiot-NAS-response"], root: 4, extensible: true } },
/* 188 */ T { name: "CauseProtocol", kind: K::Enumerated { names: &["transfer-syntax-error", "abstract-syntax-error-reject", "abstract-syntax-error-ignore-and-notify", "message-not-compatible-with-receiver-state", "semantic-error", "abstract-syntax-error-falsely-constructed-message", "unspecified"], root: 7, extensible: true } },
/* 189 */ T { name: "CauseRadioNetwork", kind: K::Enumerated { names: &["unspecified", "txnrelocoverall-expiry", "successful-handover", "release-due-to-ngran-generated-reason", "release-due-to-5gc-generated-reason", "handover-cancelled", "partial-handover", "ho-failure-in-target-5GC-ngran-node-or-target-system", "ho-target-not-allowed", "tngrelocoverall-expiry", "tngrelocprep-expiry", "cell-not-available", "unknown-targetID", "no-radio-resources-available-in-target-cell", "unknown-local-UE-NGAP-ID", "inconsistent-remote-UE-NGAP-ID", "handover-desirable-for-radio-reason", "time-critical-handover", "resource-optimisation-handover", "reduce-load-in-serving-cell", "user-inactivity", "radio-connection-with-ue-lost", "radio-resources-not-available", "invalid-qos-combination", "failure-in-radio-interface-procedure", "interaction-with-other-procedure", "unknown-PDU-session-ID", "unkown-qos-flow-ID", "multiple-PDU-session-ID-instances", "multiple-qos-flow-ID-instances", "encryption-and-or-integrity-protection-algorithms-not-supported", "ng-intra-system-handover-triggered", "ng-inter-system-handover-triggered", "xn-handover-triggered", "not-supported-5QI-value", "ue-context-transfer", "ims-voice-eps-fallback-or-rat-fallback-triggered", "up-integrity-protection-not-possible", "up-confidentiality-protection-not-possible", "slice-not-supported", "ue-in-rrc-inactive-state-not-reachable", "redirection", "resources-not-available-for-the-slice", "ue-max-integrity-protected-data-rate-reason", "release-due-to-cn-detected-mobility", "n26-interface-not-available", "release-due-to-pre-emption", "multiple-location-reporting-reference-ID-instances", "rsn-not-available-for-the-up", "npn-access-denied", "cag-only-access-denied", "insufficient-ue-capabilities", "redcap-ue-not-supported", "unknown-MBS-Session-ID", "indicated-MBS-session-area-information-not-served-by-the-gNB", "inconsistent-slice-info-for-the-session", "misaligned-association-for-multicast-unicast", "eredcap-ue-not-supported", "two-rx-xr-ue-not-supported", "aerial-ue-flight-information-reporting-initiation-failure", "unknown-RAN-AIoT-Device-NGAP-ID", "requested-AIoT-service-area-information-not-served-by-the-gNB", "unknown-AIoT-session", "aiot-device-not-reachable", "multiple-AIoT-session-ID-instances"], root: 45, extensible: true } },
/* 190 */ T { name: "CauseTransport", kind: K::Enumerated { names: &["transport-resource-unavailable", "unspecified"], root: 2, extensible: true } },
/* 191 */ T { name: "Cell-CAGInformation", kind: K::Sequence { names: &["nGRAN-CGI", "cellCAGList", "iE-Extensions"], members: &[M { field: "ngran_cgi", ty: F::Named(684), optional: false }, M { field: "cell_cag_list", ty: F::Named(195), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 192 */ T { name: "CellBasedMDT-EUTRA", kind: K::Sequence { names: &["cellIdListforMDT", "iE-Extensions"], members: &[M { field: "cell_id_listfor_mdt", ty: F::Named(205), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 193 */ T { name: "CellBasedMDT-NR", kind: K::Sequence { names: &["cellIdListforMDT", "iE-Extensions"], members: &[M { field: "cell_id_listfor_mdt", ty: F::Named(206), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 194 */ T { name: "CellBasedQMC", kind: K::Sequence { names: &["cellIdListforQMC", "iE-Extensions"], members: &[M { field: "cell_id_listfor_qmc", ty: F::Named(207), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 195 */ T { name: "CellCAGList", kind: K::Alias { field: F::List(&F::Named(153), 1, 64), form: None, secret: false } },
/* 196 */ T { name: "CellIDBroadcastEUTRA", kind: K::Alias { field: F::List(&F::Named(197), 1, 65535), form: None, secret: false } },
/* 197 */ T { name: "CellIDBroadcastEUTRA-Item", kind: K::Sequence { names: &["eUTRA-CGI", "iE-Extensions"], members: &[M { field: "eutra_cgi", ty: F::Named(315), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 198 */ T { name: "CellIDBroadcastNR", kind: K::Alias { field: F::List(&F::Named(199), 1, 65535), form: None, secret: false } },
/* 199 */ T { name: "CellIDBroadcastNR-Item", kind: K::Sequence { names: &["nR-CGI", "iE-Extensions"], members: &[M { field: "nr_cgi", ty: F::Named(711), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 200 */ T { name: "CellIDCancelledEUTRA", kind: K::Alias { field: F::List(&F::Named(201), 1, 65535), form: None, secret: false } },
/* 201 */ T { name: "CellIDCancelledEUTRA-Item", kind: K::Sequence { names: &["eUTRA-CGI", "numberOfBroadcasts", "iE-Extensions"], members: &[M { field: "eutra_cgi", ty: F::Named(315), optional: false }, M { field: "number_of_broadcasts", ty: F::Named(758), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 202 */ T { name: "CellIDCancelledNR", kind: K::Alias { field: F::List(&F::Named(203), 1, 65535), form: None, secret: false } },
/* 203 */ T { name: "CellIDCancelledNR-Item", kind: K::Sequence { names: &["nR-CGI", "numberOfBroadcasts", "iE-Extensions"], members: &[M { field: "nr_cgi", ty: F::Named(711), optional: false }, M { field: "number_of_broadcasts", ty: F::Named(758), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 204 */ T { name: "CellIDListForRestart", kind: K::Choice { names: &["eUTRA-CGIListforRestart", "nR-CGIListforRestart", "choice-Extensions"], alternatives: &[F::Named(316), F::Named(712), F::Named(948)], root: 3, extensible: false } },
/* 205 */ T { name: "CellIdListforMDT-EUTRA", kind: K::Alias { field: F::List(&F::Named(315), 1, 32), form: None, secret: false } },
/* 206 */ T { name: "CellIdListforMDT-NR", kind: K::Alias { field: F::List(&F::Named(711), 1, 32), form: None, secret: false } },
/* 207 */ T { name: "CellIdListforQMC", kind: K::Alias { field: F::List(&F::Named(684), 1, 32), form: None, secret: false } },
/* 208 */ T { name: "CellSize", kind: K::Enumerated { names: &["verysmall", "small", "medium", "large"], root: 4, extensible: true } },
/* 209 */ T { name: "CellTrafficTrace", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 210 */ T { name: "CellType", kind: K::Sequence { names: &["cellSize", "iE-Extensions"], members: &[M { field: "cell_size", ty: F::Named(208), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 211 */ T { name: "CellsToActivateList", kind: K::Alias { field: F::List(&F::Named(684), 1, 16384), form: None, secret: false } },
/* 212 */ T { name: "ClockAccuracy", kind: K::Choice { names: &["clockAccuracyValue", "clockAccuracyIndex", "choice-Extensions"], alternatives: &[F::Unsigned(1, 40000000, true), F::Unsigned(32, 47, true), F::Named(948)], root: 3, extensible: false } },
/* 213 */ T { name: "ClockQualityAcceptanceCriteria", kind: K::Sequence { names: &["synchronisationState", "traceabletoUTC", "traceabletoGNSS", "clockFrequencyStability", "clockAccuracy", "parentTImeSource", "iE-Extensions"], members: &[M { field: "synchronisation_state", ty: F::BitString(8, 8, true), optional: true }, M { field: "traceableto_utc", ty: F::Named(215), optional: true }, M { field: "traceableto_gnss", ty: F::Named(214), optional: true }, M { field: "clock_frequency_stability", ty: F::Bits(16, 16), optional: true }, M { field: "clock_accuracy", ty: F::Unsigned(1, 40000000, true), optional: true }, M { field: "parent_time_source", ty: F::BitString(16, 16, true), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 214 */ T { name: "ClockQualityAcceptanceCriteriaTraceabletoGNSS", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 215 */ T { name: "ClockQualityAcceptanceCriteriaTraceabletoUTC", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 216 */ T { name: "ClockQualityAcceptanceIndication", kind: K::Sequence { names: &["clockQualityAcceptanceCriteria", "iE-Extensions"], members: &[M { field: "clock_quality_acceptance_criteria", ty: F::Named(213), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 217 */ T { name: "ClockQualityDetailLevel", kind: K::Choice { names: &["clockQualityMetrics", "clockQualityAcceptanceIndication", "choice-extensions"], alternatives: &[F::Null, F::Named(216), F::Named(948)], root: 3, extensible: false } },
/* 218 */ T { name: "ClockQualityReportingControlInfo", kind: K::Sequence { names: &["clockQualityDetailLevel", "iE-Extensions"], members: &[M { field: "clock_quality_detail_level", ty: F::Named(217), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 219 */ T { name: "CommServiceType", kind: K::Enumerated { names: &["mbs-broadcast", "mbs-multicast", "unicast"], root: 2, extensible: true } },
/* 220 */ T { name: "CommandFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 221 */ T { name: "CommandFailureTransfer", kind: K::Sequence { names: &["correlationIdentifier", "rAN-AIOT-Device-NGAP-ID", "cause", "criticalityDiagnostics", "iE-Extensions"], members: &[M { field: "correlation_identifier", ty: F::Named(16), optional: false }, M { field: "ran_aiot_device_ngap_id", ty: F::Named(999), optional: false }, M { field: "cause", ty: F::Named(185), optional: false }, M { field: "criticality_diagnostics", ty: F::Named(246), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 222 */ T { name: "CommandRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 223 */ T { name: "CommandRequestTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 224 */ T { name: "CommandResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 225 */ T { name: "CommandResponseTransfer", kind: K::Sequence { names: &["correlationIdentifier", "rAN-AIOT-Device-NGAP-ID", "aIoT-NASPDU", "criticalityDiagnostics", "iE-Extensions"], members: &[M { field: "correlation_identifier", ty: F::Named(16), optional: false }, M { field: "ran_aiot_device_ngap_id", ty: F::Named(999), optional: false }, M { field: "aiot_nas_pdu", ty: F::Named(22), optional: false }, M { field: "criticality_diagnostics", ty: F::Named(246), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 226 */ T { name: "CommonNetworkInstance", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 227 */ T { name: "CompletedCellsInEAI-EUTRA", kind: K::Alias { field: F::List(&F::Named(228), 1, 65535), form: None, secret: false } },
/* 228 */ T { name: "CompletedCellsInEAI-EUTRA-Item", kind: K::Sequence { names: &["eUTRA-CGI", "iE-Extensions"], members: &[M { field: "eutra_cgi", ty: F::Named(315), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 229 */ T { name: "CompletedCellsInEAI-NR", kind: K::Alias { field: F::List(&F::Named(230), 1, 65535), form: None, secret: false } },
/* 230 */ T { name: "CompletedCellsInEAI-NR-Item", kind: K::Sequence { names: &["nR-CGI", "iE-Extensions"], members: &[M { field: "nr_cgi", ty: F::Named(711), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 231 */ T { name: "CompletedCellsInTAI-EUTRA", kind: K::Alias { field: F::List(&F::Named(232), 1, 65535), form: None, secret: false } },
/* 232 */ T { name: "CompletedCellsInTAI-EUTRA-Item", kind: K::Sequence { names: &["eUTRA-CGI", "iE-Extensions"], members: &[M { field: "eutra_cgi", ty: F::Named(315), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 233 */ T { name: "CompletedCellsInTAI-NR", kind: K::Alias { field: F::List(&F::Named(234), 1, 65535), form: None, secret: false } },
/* 234 */ T { name: "CompletedCellsInTAI-NR-Item", kind: K::Sequence { names: &["nR-CGI", "iE-Extensions"], members: &[M { field: "nr_cgi", ty: F::Named(711), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 235 */ T { name: "CompositeAvailableCapacity", kind: K::Sequence { names: &["cellCapacityClassValue", "capacityValue", "iE-Extensions"], members: &[M { field: "cell_capacity_class_value", ty: F::Unsigned(1, 100, true), optional: true }, M { field: "capacity_value", ty: F::Unsigned(0, 100, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 236 */ T { name: "ConcurrentWarningMessageInd", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 237 */ T { name: "ConfidentialityProtectionIndication", kind: K::Enumerated { names: &["required", "preferred", "not-needed"], root: 3, extensible: true } },
/* 238 */ T { name: "ConfidentialityProtectionResult", kind: K::Enumerated { names: &["performed", "not-performed"], root: 2, extensible: true } },
/* 239 */ T { name: "ConfiguredNSSAI", kind: K::Alias { field: F::OctetArray(128), form: None, secret: false } },
/* 240 */ T { name: "ConfiguredTACIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 241 */ T { name: "CongestionInformationRequest", kind: K::Enumerated { names: &["ul", "dl", "both", "stop"], root: 4, extensible: true } },
/* 242 */ T { name: "ConnectionEstablishmentIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 243 */ T { name: "CoreNetworkAssistanceInformationForInactive", kind: K::Sequence { names: &["uEIdentityIndexValue", "uESpecificDRX", "periodicRegistrationUpdateTimer", "mICOModeIndication", "tAIListForInactive", "expectedUEBehaviour", "iE-Extensions"], members: &[M { field: "ue_identity_index_value", ty: F::Named(1269), optional: false }, M { field: "ue_specific_drx", ty: F::Named(907), optional: true }, M { field: "periodic_registration_update_timer", ty: F::Named(926), optional: false }, M { field: "mico_mode_indication", ty: F::Named(627), optional: true }, M { field: "tai_list_for_inactive", ty: F::Named(1165), optional: false }, M { field: "expected_ue_behaviour", ty: F::Named(368), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 244 */ T { name: "CoverageEnhancementLevel", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 245 */ T { name: "Criticality", kind: K::Enumerated { names: &["reject", "ignore", "notify"], root: 3, extensible: false } },
/* 246 */ T { name: "CriticalityDiagnostics", kind: K::Sequence { names: &["procedureCode", "triggeringMessage", "procedureCriticality", "iEsCriticalityDiagnostics", "iE-Extensions"], members: &[M { field: "procedure_code", ty: F::Named(942), optional: true }, M { field: "triggering_message", ty: F::Named(1231), optional: true }, M { field: "procedure_criticality", ty: F::Named(245), optional: true }, M { field: "ies_criticality_diagnostics", ty: F::Named(248), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 247 */ T { name: "CriticalityDiagnostics-IE-Item", kind: K::Sequence { names: &["iECriticality", "iE-ID", "typeOfError", "iE-Extensions"], members: &[M { field: "ie_criticality", ty: F::Named(245), optional: false }, M { field: "ie_id", ty: F::Named(949), optional: false }, M { field: "type_of_error", ty: F::Named(1232), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 248 */ T { name: "CriticalityDiagnostics-IE-List", kind: K::Alias { field: F::List(&F::Named(247), 1, 256), form: None, secret: false } },
/* 249 */ T { name: "DAPSRequestInfo", kind: K::Sequence { names: &["dAPSIndicator", "iE-Extensions"], members: &[M { field: "daps_indicator", ty: F::Named(250), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 250 */ T { name: "DAPSRequestInfoDAPSIndicator", kind: K::Enumerated { names: &["daps-ho-required"], root: 1, extensible: true } },
/* 251 */ T { name: "DAPSResponseInfo", kind: K::Sequence { names: &["dapsresponseindicator", "iE-Extensions"], members: &[M { field: "dapsresponseindicator", ty: F::Named(252), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 252 */ T { name: "DAPSResponseInfoDapsresponseindicator", kind: K::Enumerated { names: &["daps-ho-accepted", "daps-ho-not-accepted"], root: 2, extensible: true } },
/* 253 */ T { name: "DAPSResponseInfoItem", kind: K::Sequence { names: &["dRB-ID", "dAPSResponseInfo", "iE-Extension"], members: &[M { field: "drb_id", ty: F::Named(263), optional: false }, M { field: "daps_response_info", ty: F::Named(251), optional: false }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 254 */ T { name: "DAPSResponseInfoList", kind: K::Alias { field: F::List(&F::Named(253), 1, 32), form: None, secret: false } },
/* 255 */ T { name: "DL-CP-SecurityInformation", kind: K::Sequence { names: &["dl-NAS-MAC", "iE-Extensions"], members: &[M { field: "dl_nas_mac", ty: F::Named(256), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 256 */ T { name: "DL-NAS-MAC", kind: K::Alias { field: F::Bits(16, 16), form: None, secret: false } },
/* 257 */ T { name: "DL-NGU-TNLInformationReused", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 258 */ T { name: "DL-Signalling", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 259 */ T { name: "DLCountChoice", kind: K::Choice { names: &["count12bits", "count18bits", "choice-extension"], alternatives: &[F::Named(165), F::Named(166), F::Named(948)], root: 3, extensible: false } },
/* 260 */ T { name: "DLDiscarding", kind: K::Sequence { names: &["dRBsSubjectToDLDiscarding", "iE-Extension"], members: &[M { field: "drbs_subject_to_dl_discarding", ty: F::Named(271), optional: false }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 261 */ T { name: "DLForwarding", kind: K::Enumerated { names: &["dl-forwarding-proposed"], root: 1, extensible: true } },
/* 262 */ T { name: "DLPDUSetInformationMarkingSupportIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 263 */ T { name: "DRB-ID", kind: K::Alias { field: F::Unsigned(1, 32, true), form: None, secret: false } },
/* 264 */ T { name: "DRBStatusDL", kind: K::Choice { names: &["dRBStatusDL12", "dRBStatusDL18", "choice-Extensions"], alternatives: &[F::Named(265), F::Named(266), F::Named(948)], root: 3, extensible: false } },
/* 265 */ T { name: "DRBStatusDL12", kind: K::Sequence { names: &["dL-COUNTValue", "iE-Extension"], members: &[M { field: "dl_count_value", ty: F::Named(165), optional: false }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 266 */ T { name: "DRBStatusDL18", kind: K::Sequence { names: &["dL-COUNTValue", "iE-Extension"], members: &[M { field: "dl_count_value", ty: F::Named(166), optional: false }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 267 */ T { name: "DRBStatusUL", kind: K::Choice { names: &["dRBStatusUL12", "dRBStatusUL18", "choice-Extensions"], alternatives: &[F::Named(268), F::Named(269), F::Named(948)], root: 3, extensible: false } },
/* 268 */ T { name: "DRBStatusUL12", kind: K::Sequence { names: &["uL-COUNTValue", "receiveStatusOfUL-PDCP-SDUs", "iE-Extension"], members: &[M { field: "ul_count_value", ty: F::Named(165), optional: false }, M { field: "receive_status_of_ul_pdcp_sdus", ty: F::BitString(1, 2048, false), optional: true }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 269 */ T { name: "DRBStatusUL18", kind: K::Sequence { names: &["uL-COUNTValue", "receiveStatusOfUL-PDCP-SDUs", "iE-Extension"], members: &[M { field: "ul_count_value", ty: F::Named(166), optional: false }, M { field: "receive_status_of_ul_pdcp_sdus", ty: F::BitString(1, 131072, false), optional: true }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 270 */ T { name: "DRBsSubjectToDLDiscarding-Item", kind: K::Sequence { names: &["drbID", "dlCount", "iE-Extension"], members: &[M { field: "drb_id", ty: F::Named(263), optional: false }, M { field: "dl_count", ty: F::Named(259), optional: false }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 271 */ T { name: "DRBsSubjectToDLDiscarding-List", kind: K::Alias { field: F::List(&F::Named(270), 1, 32), form: None, secret: false } },
/* 272 */ T { name: "DRBsSubjectToEarlyStatusTransfer-Item", kind: K::Sequence { names: &["dRB-ID", "firstDLCOUNT", "iE-Extension"], members: &[M { field: "drb_id", ty: F::Named(263), optional: false }, M { field: "first_dl_count", ty: F::Named(264), optional: false }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 273 */ T { name: "DRBsSubjectToEarlyStatusTransfer-List", kind: K::Alias { field: F::List(&F::Named(272), 1, 32), form: None, secret: false } },
/* 274 */ T { name: "DRBsSubjectToStatusTransferItem", kind: K::Sequence { names: &["dRB-ID", "dRBStatusUL", "dRBStatusDL", "iE-Extension"], members: &[M { field: "drb_id", ty: F::Named(263), optional: false }, M { field: "drb_status_ul", ty: F::Named(267), optional: false }, M { field: "drb_status_dl", ty: F::Named(264), optional: false }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 275 */ T { name: "DRBsSubjectToStatusTransferList", kind: K::Alias { field: F::List(&F::Named(274), 1, 32), form: None, secret: false } },
/* 276 */ T { name: "DRBsToQosFlowsMappingItem", kind: K::Sequence { names: &["dRB-ID", "associatedQosFlowList", "iE-Extensions"], members: &[M { field: "drb_id", ty: F::Named(263), optional: false }, M { field: "associated_qos_flow_list", ty: F::Named(110), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 277 */ T { name: "DRBsToQosFlowsMappingList", kind: K::Alias { field: F::List(&F::Named(276), 1, 32), form: None, secret: false } },
/* 278 */ T { name: "DataCodingScheme", kind: K::Alias { field: F::Bits(8, 8), form: None, secret: false } },
/* 279 */ T { name: "DataForwardingAccepted", kind: K::Enumerated { names: &["data-forwarding-accepted"], root: 1, extensible: true } },
/* 280 */ T { name: "DataForwardingNotPossible", kind: K::Enumerated { names: &["data-forwarding-not-possible"], root: 1, extensible: true } },
/* 281 */ T { name: "DataForwardingResponseDRBItem", kind: K::Sequence { names: &["dRB-ID", "dLForwardingUP-TNLInformation", "uLForwardingUP-TNLInformation", "iE-Extensions"], members: &[M { field: "drb_id", ty: F::Named(263), optional: false }, M { field: "dl_forwarding_up_tnl_information", ty: F::Named(1298), optional: true }, M { field: "ul_forwarding_up_tnl_information", ty: F::Named(1298), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 282 */ T { name: "DataForwardingResponseDRBList", kind: K::Alias { field: F::List(&F::Named(281), 1, 32), form: None, secret: false } },
/* 283 */ T { name: "DataForwardingResponseERABList", kind: K::Alias { field: F::List(&F::Named(284), 1, 256), form: None, secret: false } },
/* 284 */ T { name: "DataForwardingResponseERABListItem", kind: K::Sequence { names: &["e-RAB-ID", "dLForwardingUP-TNLInformation", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(301), optional: false }, M { field: "dl_forwarding_up_tnl_information", ty: F::Named(1298), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 285 */ T { name: "DeactivateTrace", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 286 */ T { name: "DelayCritical", kind: K::Enumerated { names: &["delay-critical", "non-delay-critical"], root: 2, extensible: true } },
/* 287 */ T { name: "DirectForwardingPathAvailability", kind: K::Enumerated { names: &["direct-path-available"], root: 1, extensible: true } },
/* 288 */ T { name: "DistributionReleaseRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 289 */ T { name: "DistributionReleaseResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 290 */ T { name: "DistributionSetupFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 291 */ T { name: "DistributionSetupRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 292 */ T { name: "DistributionSetupResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 293 */ T { name: "DownlinkNASTransport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 294 */ T { name: "DownlinkNonUEAssociatedNRPPaTransport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 295 */ T { name: "DownlinkRANConfigurationTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 296 */ T { name: "DownlinkRANEarlyStatusTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 297 */ T { name: "DownlinkRANStatusTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 298 */ T { name: "DownlinkRIMInformationTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 299 */ T { name: "DownlinkUEAssociatedNRPPaTransport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 300 */ T { name: "Dynamic5QIDescriptor", kind: K::Sequence { names: &["priorityLevelQos", "packetDelayBudget", "packetErrorRate", "fiveQI", "delayCritical", "averagingWindow", "maximumDataBurstVolume", "iE-Extensions"], members: &[M { field: "priority_level_qos", ty: F::Named(937), optional: false }, M { field: "packet_delay_budget", ty: F::Named(898), optional: false }, M { field: "packet_error_rate", ty: F::Named(899), optional: false }, M { field: "five_qi", ty: F::Named(404), optional: true }, M { field: "delay_critical", ty: F::Named(286), optional: true }, M { field: "averaging_window", ty: F::Named(118), optional: true }, M { field: "maximum_data_burst_volume", ty: F::Named(637), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 301 */ T { name: "E-RAB-ID", kind: K::Alias { field: F::Unsigned(0, 15, true), form: None, secret: false } },
/* 302 */ T { name: "E-RABInformationItem", kind: K::Sequence { names: &["e-RAB-ID", "dLForwarding", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(301), optional: false }, M { field: "dl_forwarding", ty: F::Named(261), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 303 */ T { name: "E-RABInformationList", kind: K::Alias { field: F::List(&F::Named(302), 1, 256), form: None, secret: false } },
/* 304 */ T { name: "ECNMarkingAtRANRequest", kind: K::Enumerated { names: &["ul", "dl", "both", "stop"], root: 4, extensible: true } },
/* 305 */ T { name: "ECNMarkingAtUPFRequest", kind: K::Enumerated { names: &["ul", "dl", "both", "stop"], root: 4, extensible: true } },
/* 306 */ T { name: "ECNMarkingorCongestionInformationReportingRequest", kind: K::Choice { names: &["eCNMarkingAtRANRequest", "eCNMarkingAtUPFRequest", "congestionInformationRequest", "choice-Extensions"], alternatives: &[F::Named(304), F::Named(305), F::Named(241), F::Named(948)], root: 4, extensible: false } },
/* 307 */ T { name: "ECNMarkingorCongestionInformationReportingStatus", kind: K::Alias { field: F::List(&F::Named(308), 1, 64), form: None, secret: false } },
/* 308 */ T { name: "ECNMarkingorCongestionInformationReportingStatus-Item", kind: K::Sequence { names: &["qosFlowIdentifier", "activationStatus", "iE-Extension"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "activation_status", ty: F::Named(58), optional: false }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 309 */ T { name: "EDT-Session", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 310 */ T { name: "EN-DCSONConfigurationTransfer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 311 */ T { name: "ENB-ID", kind: K::Choice { names: &["macroENB-ID", "homeENB-ID", "short-macroENB-ID", "long-macroENB-ID", "choice-Extensions"], alternatives: &[F::Bits(20, 20), F::Bits(28, 28), F::Bits(18, 18), F::Bits(21, 21), F::Named(948)], root: 5, extensible: false } },
/* 312 */ T { name: "EPS-TAC", kind: K::Alias { field: F::OctetArray(2), form: Some(Form::Number(16)), secret: false } },
/* 313 */ T { name: "EPS-TAI", kind: K::Sequence { names: &["pLMNIdentity", "ePS-TAC", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "eps_tac", ty: F::Named(312), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 314 */ T { name: "ERedCapIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 315 */ T { name: "EUTRA-CGI", kind: K::Sequence { names: &["pLMNIdentity", "eUTRACellIdentity", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "eutra_cell_identity", ty: F::Named(321), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 316 */ T { name: "EUTRA-CGIList", kind: K::Alias { field: F::List(&F::Named(315), 1, 256), form: None, secret: false } },
/* 317 */ T { name: "EUTRA-CGIListForWarning", kind: K::Alias { field: F::List(&F::Named(315), 1, 65535), form: None, secret: false } },
/* 318 */ T { name: "EUTRA-Paging-Time-Window", kind: K::Enumerated { names: &["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13", "s14", "s15", "s16"], root: 16, extensible: true } },
/* 319 */ T { name: "EUTRA-Paging-eDRX-Cycle", kind: K::Enumerated { names: &["hfhalf", "hf1", "hf2", "hf4", "hf6", "hf8", "hf10", "hf12", "hf14", "hf16", "hf32", "hf64", "hf128", "hf256"], root: 14, extensible: true } },
/* 320 */ T { name: "EUTRA-PagingeDRXInformation", kind: K::Sequence { names: &["eUTRA-paging-eDRX-Cycle", "eUTRA-paging-Time-Window", "iE-Extensions"], members: &[M { field: "eutra_paging_edrx_cycle", ty: F::Named(319), optional: false }, M { field: "eutra_paging_time_window", ty: F::Named(318), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 321 */ T { name: "EUTRACellIdentity", kind: K::Alias { field: F::Bits(28, 28), form: Some(Form::Number(28)), secret: false } },
/* 322 */ T { name: "EUTRAN-CellReportItem", kind: K::Sequence { names: &["eCGI", "eUTRAN-CompositeAvailableCapacityGroup", "eUTRAN-NumberOfActiveUEs", "eUTRAN-NoofRRCConnections", "eUTRAN-RadioResourceStatus", "iE-Extensions"], members: &[M { field: "ecgi", ty: F::Named(315), optional: false }, M { field: "eutran_composite_available_capacity_group", ty: F::Named(326), optional: false }, M { field: "eutran_number_of_active_ues", ty: F::Named(327), optional: true }, M { field: "eutran_noof_rrc_connections", ty: F::Named(689), optional: true }, M { field: "eutran_radio_resource_status", ty: F::Named(328), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 323 */ T { name: "EUTRAN-CellReportList", kind: K::Alias { field: F::List(&F::Named(322), 1, 256), form: None, secret: false } },
/* 324 */ T { name: "EUTRAN-CellToReportItem", kind: K::Sequence { names: &["eCGI", "iE-Extensions"], members: &[M { field: "ecgi", ty: F::Named(315), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 325 */ T { name: "EUTRAN-CellToReportList", kind: K::Alias { field: F::List(&F::Named(324), 1, 256), form: None, secret: false } },
/* 326 */ T { name: "EUTRAN-CompositeAvailableCapacityGroup", kind: K::Sequence { names: &["dL-CompositeAvailableCapacity", "uL-CompositeAvailableCapacity", "iE-Extensions"], members: &[M { field: "dl_composite_available_capacity", ty: F::Named(235), optional: false }, M { field: "ul_composite_available_capacity", ty: F::Named(235), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 327 */ T { name: "EUTRAN-NumberOfActiveUEs", kind: K::Alias { field: F::Unsigned(0, 16777215, true), form: None, secret: false } },
/* 328 */ T { name: "EUTRAN-RadioResourceStatus", kind: K::Sequence { names: &["dL-GBR-PRB-usage", "uL-GBR-PRB-usage", "dL-non-GBR-PRB-usage", "uL-non-GBR-PRB-usage", "dL-Total-PRB-usage", "uL-Total-PRB-usage", "dL-scheduling-PDCCH-CCE-usage", "uL-scheduling-PDCCH-CCE-usage", "iE-Extensions"], members: &[M { field: "dl_gbr_prb_usage", ty: F::Unsigned(0, 100, false), optional: false }, M { field: "ul_gbr_prb_usage", ty: F::Unsigned(0, 100, false), optional: false }, M { field: "dl_non_gbr_prb_usage", ty: F::Unsigned(0, 100, false), optional: false }, M { field: "ul_non_gbr_prb_usage", ty: F::Unsigned(0, 100, false), optional: false }, M { field: "dl_total_prb_usage", ty: F::Unsigned(0, 100, false), optional: false }, M { field: "ul_total_prb_usage", ty: F::Unsigned(0, 100, false), optional: false }, M { field: "dl_scheduling_pdcch_cce_usage", ty: F::Unsigned(0, 100, false), optional: true }, M { field: "ul_scheduling_pdcch_cce_usage", ty: F::Unsigned(0, 100, false), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 329 */ T { name: "EUTRAN-ReportingStatusIEs", kind: K::Sequence { names: &["eUTRAN-CellReportList", "iE-Extensions"], members: &[M { field: "eutran_cell_report_list", ty: F::Named(323), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 330 */ T { name: "EUTRAN-ReportingSystemIEs", kind: K::Sequence { names: &["eUTRAN-CellToReportList", "iE-Extensions"], members: &[M { field: "eutran_cell_to_report_list", ty: F::Named(325), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 331 */ T { name: "EUTRAencryptionAlgorithms", kind: K::Alias { field: F::BitString(16, 16, true), form: None, secret: false } },
/* 332 */ T { name: "EUTRAintegrityProtectionAlgorithms", kind: K::Alias { field: F::BitString(16, 16, true), form: None, secret: false } },
/* 333 */ T { name: "EarlyMeasurement", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 334 */ T { name: "EarlyStatusTransfer-TransparentContainer", kind: K::Sequence { names: &["procedureStage", "iE-Extensions"], members: &[M { field: "procedure_stage", ty: F::Named(943), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 335 */ T { name: "EmergencyAreaID", kind: K::Alias { field: F::OctetArray(3), form: None, secret: false } },
/* 336 */ T { name: "EmergencyAreaIDBroadcastEUTRA", kind: K::Alias { field: F::List(&F::Named(337), 1, 65535), form: None, secret: false } },
/* 337 */ T { name: "EmergencyAreaIDBroadcastEUTRA-Item", kind: K::Sequence { names: &["emergencyAreaID", "completedCellsInEAI-EUTRA", "iE-Extensions"], members: &[M { field: "emergency_area_id", ty: F::Named(335), optional: false }, M { field: "completed_cells_in_eai_eutra", ty: F::Named(227), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 338 */ T { name: "EmergencyAreaIDBroadcastNR", kind: K::Alias { field: F::List(&F::Named(339), 1, 65535), form: None, secret: false } },
/* 339 */ T { name: "EmergencyAreaIDBroadcastNR-Item", kind: K::Sequence { names: &["emergencyAreaID", "completedCellsInEAI-NR", "iE-Extensions"], members: &[M { field: "emergency_area_id", ty: F::Named(335), optional: false }, M { field: "completed_cells_in_eai_nr", ty: F::Named(229), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 340 */ T { name: "EmergencyAreaIDCancelledEUTRA", kind: K::Alias { field: F::List(&F::Named(341), 1, 65535), form: None, secret: false } },
/* 341 */ T { name: "EmergencyAreaIDCancelledEUTRA-Item", kind: K::Sequence { names: &["emergencyAreaID", "cancelledCellsInEAI-EUTRA", "iE-Extensions"], members: &[M { field: "emergency_area_id", ty: F::Named(335), optional: false }, M { field: "cancelled_cells_in_eai_eutra", ty: F::Named(169), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 342 */ T { name: "EmergencyAreaIDCancelledNR", kind: K::Alias { field: F::List(&F::Named(343), 1, 65535), form: None, secret: false } },
/* 343 */ T { name: "EmergencyAreaIDCancelledNR-Item", kind: K::Sequence { names: &["emergencyAreaID", "cancelledCellsInEAI-NR", "iE-Extensions"], members: &[M { field: "emergency_area_id", ty: F::Named(335), optional: false }, M { field: "cancelled_cells_in_eai_nr", ty: F::Named(171), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 344 */ T { name: "EmergencyAreaIDList", kind: K::Alias { field: F::List(&F::Named(335), 1, 65535), form: None, secret: false } },
/* 345 */ T { name: "EmergencyAreaIDListForRestart", kind: K::Alias { field: F::List(&F::Named(335), 1, 256), form: None, secret: false } },
/* 346 */ T { name: "EmergencyFallbackIndicator", kind: K::Sequence { names: &["emergencyFallbackRequestIndicator", "emergencyServiceTargetCN", "iE-Extensions"], members: &[M { field: "emergency_fallback_request_indicator", ty: F::Named(347), optional: false }, M { field: "emergency_service_target_cn", ty: F::Named(348), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 347 */ T { name: "EmergencyFallbackRequestIndicator", kind: K::Enumerated { names: &["emergency-fallback-requested"], root: 1, extensible: true } },
/* 348 */ T { name: "EmergencyServiceTargetCN", kind: K::Enumerated { names: &["fiveGC", "epc"], root: 2, extensible: true } },
/* 349 */ T { name: "EndIndication", kind: K::Enumerated { names: &["no-further-data", "further-data-exists"], root: 2, extensible: true } },
/* 350 */ T { name: "EndpointIPAddressAndPort", kind: K::Sequence { names: &["endpointIPAddress", "portNumber", "iE-Extensions"], members: &[M { field: "endpoint_ip_address", ty: F::Named(1230), optional: false }, M { field: "port_number", ty: F::Named(932), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: false } },
/* 351 */ T { name: "Enhanced-CoverageRestriction", kind: K::Enumerated { names: &["restricted"], root: 1, extensible: true } },
/* 352 */ T { name: "EquivalentPLMNs", kind: K::Alias { field: F::List(&F::Named(886), 1, 15), form: None, secret: false } },
/* 353 */ T { name: "EquivalentSNPNsItem", kind: K::Sequence { names: &["plmnIdentity", "nID", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "nid", ty: F::Named(705), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 354 */ T { name: "EquivalentSNPNsList", kind: K::Alias { field: F::List(&F::Named(353), 1, 15), form: None, secret: false } },
/* 355 */ T { name: "ErrorIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 356 */ T { name: "EventBasedReportingIEs", kind: K::Sequence { names: &["intersystemResourceThresholdLow", "intersystemResourceThresholdHigh", "numberOfMeasurementReportingLevels", "iE-Extensions"], members: &[M { field: "intersystem_resource_threshold_low", ty: F::Named(491), optional: false }, M { field: "intersystem_resource_threshold_high", ty: F::Named(491), optional: false }, M { field: "number_of_measurement_reporting_levels", ty: F::Named(760), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 357 */ T { name: "EventL1LoggedMDTConfig", kind: K::Sequence { names: &["l1Threshold", "hysteresis", "timeToTrigger", "iE-Extensions"], members: &[M { field: "l1_threshold", ty: F::Named(639), optional: false }, M { field: "hysteresis", ty: F::Named(461), optional: false }, M { field: "time_to_trigger", ty: F::Named(1215), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 358 */ T { name: "EventTrigger", kind: K::Choice { names: &["outOfCoverage", "eventL1LoggedMDTConfig", "choice-Extensions"], alternatives: &[F::Named(359), F::Named(357), F::Named(948)], root: 3, extensible: false } },
/* 359 */ T { name: "EventTriggerOutOfCoverage", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 360 */ T { name: "EventType", kind: K::Enumerated { names: &["direct", "change-of-serve-cell", "ue-presence-in-area-of-interest", "stop-change-of-serve-cell", "stop-ue-presence-in-area-of-interest", "cancel-location-reporting-for-the-ue", "change-of-serving-cell-and-UE-presence-in-the-Area-of-Interest", "report-the-Aerial-UE-flight-information", "cancel-the-Aerial-UE-flight-information-reporting"], root: 6, extensible: true } },
/* 361 */ T { name: "ExcessPacketDelayThresholdConfiguration", kind: K::Alias { field: F::List(&F::Named(362), 1, 255), form: None, secret: false } },
/* 362 */ T { name: "ExcessPacketDelayThresholdItem", kind: K::Sequence { names: &["fiveQi", "excessPacketDelayThresholdValue", "iE-Extensions"], members: &[M { field: "five_qi", ty: F::Named(404), optional: false }, M { field: "excess_packet_delay_threshold_value", ty: F::Named(363), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 363 */ T { name: "ExcessPacketDelayThresholdValue", kind: K::Enumerated { names: &["ms0dot25", "ms0dot5", "ms1", "ms2", "ms4", "ms5", "ms10", "ms20", "ms30", "ms40", "ms50", "ms60", "ms70", "ms80", "ms90", "ms100", "ms150", "ms300", "ms500"], root: 19, extensible: true } },
/* 364 */ T { name: "ExpectedActivityPeriod", kind: K::Alias { field: F::Unsigned(1, 181, true), form: None, secret: false } },
/* 365 */ T { name: "ExpectedHOInterval", kind: K::Enumerated { names: &["sec15", "sec30", "sec60", "sec90", "sec120", "sec180", "long-time"], root: 7, extensible: true } },
/* 366 */ T { name: "ExpectedIdlePeriod", kind: K::Alias { field: F::Unsigned(1, 181, true), form: None, secret: false } },
/* 367 */ T { name: "ExpectedUEActivityBehaviour", kind: K::Sequence { names: &["expectedActivityPeriod", "expectedIdlePeriod", "sourceOfUEActivityBehaviourInformation", "iE-Extensions"], members: &[M { field: "expected_activity_period", ty: F::Named(364), optional: true }, M { field: "expected_idle_period", ty: F::Named(366), optional: true }, M { field: "source_of_ue_activity_behaviour_information", ty: F::Named(1133), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 368 */ T { name: "ExpectedUEBehaviour", kind: K::Sequence { names: &["expectedUEActivityBehaviour", "expectedHOInterval", "expectedUEMobility", "expectedUEMovingTrajectory", "iE-Extensions"], members: &[M { field: "expected_ue_activity_behaviour", ty: F::Named(367), optional: true }, M { field: "expected_ho_interval", ty: F::Named(365), optional: true }, M { field: "expected_ue_mobility", ty: F::Named(369), optional: true }, M { field: "expected_ue_moving_trajectory", ty: F::Named(370), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 369 */ T { name: "ExpectedUEMobility", kind: K::Enumerated { names: &["stationary", "mobile"], root: 2, extensible: true } },
/* 370 */ T { name: "ExpectedUEMovingTrajectory", kind: K::Alias { field: F::List(&F::Named(371), 1, 16), form: None, secret: false } },
/* 371 */ T { name: "ExpectedUEMovingTrajectoryItem", kind: K::Sequence { names: &["nGRAN-CGI", "timeStayedInCell", "iE-Extensions"], members: &[M { field: "ngran_cgi", ty: F::Named(684), optional: false }, M { field: "time_stayed_in_cell", ty: F::Unsigned(0, 4095, false), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 372 */ T { name: "Extended-AMFName", kind: K::Sequence { names: &["aMFNameVisibleString", "aMFNameUTF8String", "iE-Extensions"], members: &[M { field: "amf_name_visible_string", ty: F::Named(48), optional: true }, M { field: "amf_name_utf8_string", ty: F::Named(47), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 373 */ T { name: "Extended-ConnectedTime", kind: K::Alias { field: F::Unsigned(0, 255, false), form: None, secret: false } },
/* 374 */ T { name: "Extended-RANNodeName", kind: K::Sequence { names: &["rANNodeNameVisibleString", "rANNodeNameUTF8String", "iE-Extensions"], members: &[M { field: "ran_node_name_visible_string", ty: F::Named(1009), optional: true }, M { field: "ran_node_name_utf8_string", ty: F::Named(1008), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 375 */ T { name: "ExtendedMobilityInformation", kind: K::Alias { field: F::Bits(32, 32), form: None, secret: false } },
/* 376 */ T { name: "ExtendedPacketDelayBudget", kind: K::Alias { field: F::Unsigned(1, 65535, true), form: None, secret: false } },
/* 377 */ T { name: "ExtendedRATRestrictionInformation", kind: K::Sequence { names: &["primaryRATRestriction", "secondaryRATRestriction", "iE-Extensions"], members: &[M { field: "primary_rat_restriction", ty: F::BitString(8, 8, true), optional: false }, M { field: "secondary_rat_restriction", ty: F::BitString(8, 8, true), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 378 */ T { name: "ExtendedRNC-ID", kind: K::Alias { field: F::Unsigned(4096, 65535, false), form: None, secret: false } },
/* 379 */ T { name: "ExtendedReportIntervalMDT", kind: K::Enumerated { names: &["ms20480", "ms40960"], root: 2, extensible: true } },
/* 380 */ T { name: "ExtendedSliceSupportList", kind: K::Alias { field: F::List(&F::Named(1127), 1, 65535), form: None, secret: false } },
/* 381 */ T { name: "ExtendedUEIdentityIndexValue", kind: K::Alias { field: F::Bits(16, 16), form: None, secret: false } },
/* 382 */ T { name: "FailureIndication", kind: K::Sequence { names: &["uERLFReportContainer", "iE-Extensions"], members: &[M { field: "ue_rlf_report_container", ty: F::Named(1275), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 383 */ T { name: "FirstDLCount", kind: K::Sequence { names: &["dRBsSubjectToEarlyStatusTransfer", "iE-Extension"], members: &[M { field: "drbs_subject_to_early_status_transfer", ty: F::Named(273), optional: false }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 384 */ T { name: "FiveG-ProSeAuthorized", kind: K::Sequence { names: &["fiveGProSeDirectDiscovery", "fiveGProSeDirectCommunication", "fiveGProSeLayer2UEtoNetworkRelay", "fiveGProSeLayer3UEtoNetworkRelay", "fiveGProSeLayer2RemoteUE", "iE-Extensions"], members: &[M { field: "five_g_prose_direct_discovery", ty: F::Named(390), optional: true }, M { field: "five_g_prose_direct_communication", ty: F::Named(389), optional: true }, M { field: "five_g_prose_layer2_ue_to_network_relay", ty: F::Named(396), optional: true }, M { field: "five_g_prose_layer3_ue_to_network_relay", ty: F::Named(400), optional: true }, M { field: "five_g_prose_layer2_remote_ue", ty: F::Named(395), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 385 */ T { name: "FiveG-ProSePC5QoSParameters", kind: K::Sequence { names: &["fiveGProSepc5QoSFlowList", "fiveGProSepc5LinkAggregateBitRates", "iE-Extensions"], members: &[M { field: "five_g_prose_pc5_qos_flow_list", ty: F::Named(403), optional: false }, M { field: "five_g_prose_pc5_link_aggregate_bit_rates", ty: F::Named(124), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 386 */ T { name: "FiveG-S-TMSI", kind: K::Sequence { names: &["aMFSetID", "aMFPointer", "fiveG-TMSI", "iE-Extensions"], members: &[M { field: "amf_set_id", ty: F::Named(52), optional: false }, M { field: "amf_pointer", ty: F::Named(50), optional: false }, M { field: "five_g_tmsi", ty: F::Named(387), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 387 */ T { name: "FiveG-TMSI", kind: K::Alias { field: F::OctetArray(4), form: Some(Form::Number(32)), secret: false } },
/* 388 */ T { name: "FiveGCAction", kind: K::Choice { names: &["hLComActivate", "hLComDeactivate", "choice-Extensions"], alternatives: &[F::Named(436), F::Named(437), F::Named(948)], root: 3, extensible: false } },
/* 389 */ T { name: "FiveGProSeDirectCommunication", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 390 */ T { name: "FiveGProSeDirectDiscovery", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 391 */ T { name: "FiveGProSeLayer2MHIntermediateUEtoNetworkRelay", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 392 */ T { name: "FiveGProSeLayer2MHRemote", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 393 */ T { name: "FiveGProSeLayer2MHUEtoNetworkRelay", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 394 */ T { name: "FiveGProSeLayer2Multipath", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 395 */ T { name: "FiveGProSeLayer2RemoteUE", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 396 */ T { name: "FiveGProSeLayer2UEtoNetworkRelay", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 397 */ T { name: "FiveGProSeLayer2UEtoUERelay", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 398 */ T { name: "FiveGProSeLayer2UEtoUERemote", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 399 */ T { name: "FiveGProSeLayer3MHUEtoNetworkRelay", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 400 */ T { name: "FiveGProSeLayer3UEtoNetworkRelay", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 401 */ T { name: "FiveGProSePC5FlowBitRates", kind: K::Sequence { names: &["fiveGproSeguaranteedFlowBitRate", "fiveGproSemaximumFlowBitRate", "iE-Extensions"], members: &[M { field: "five_g_prose_guaranteed_flow_bit_rate", ty: F::Named(124), optional: false }, M { field: "five_g_prose_maximum_flow_bit_rate", ty: F::Named(124), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 402 */ T { name: "FiveGProSePC5QoSFlowItem", kind: K::Sequence { names: &["fiveGproSepQI", "fiveGproSepc5FlowBitRates", "fiveGproSerange", "iE-Extensions"], members: &[M { field: "five_g_prose_pqi", ty: F::Named(404), optional: false }, M { field: "five_g_prose_pc5_flow_bit_rates", ty: F::Named(401), optional: true }, M { field: "five_g_prose_range", ty: F::Named(1041), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 403 */ T { name: "FiveGProSePC5QoSFlowList", kind: K::Alias { field: F::List(&F::Named(402), 1, 2048), form: None, secret: false } },
/* 404 */ T { name: "FiveQI", kind: K::Alias { field: F::Unsigned(0, 255, true), form: None, secret: false } },
/* 405 */ T { name: "ForbiddenAreaInformation", kind: K::Alias { field: F::List(&F::Named(406), 1, 16), form: None, secret: false } },
/* 406 */ T { name: "ForbiddenAreaInformation-Item", kind: K::Sequence { names: &["pLMNIdentity", "forbiddenTACs", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "forbidden_tacs", ty: F::Named(407), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 407 */ T { name: "ForbiddenTACs", kind: K::Alias { field: F::List(&F::Named(1152), 1, 4096), form: None, secret: false } },
/* 408 */ T { name: "FromEUTRANtoNGRAN", kind: K::Sequence { names: &["sourceeNBID", "targetNGRANnodeID", "iE-Extensions"], members: &[M { field: "source_enb_id", ty: F::Named(499), optional: false }, M { field: "target_ngran_node_id", ty: F::Named(497), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: false } },
/* 409 */ T { name: "FromNGRANtoEUTRAN", kind: K::Sequence { names: &["sourceNGRANnodeID", "targeteNBID", "iE-Extensions"], members: &[M { field: "source_ngran_node_id", ty: F::Named(497), optional: false }, M { field: "target_enb_id", ty: F::Named(499), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: false } },
/* 410 */ T { name: "FurtherExtendedUEIdentityIndexValue", kind: K::Alias { field: F::Bits(20, 20), form: None, secret: false } },
/* 411 */ T { name: "GBR-QosInformation", kind: K::Sequence { names: &["maximumFlowBitRateDL", "maximumFlowBitRateUL", "guaranteedFlowBitRateDL", "guaranteedFlowBitRateUL", "notificationControl", "maximumPacketLossRateDL", "maximumPacketLossRateUL", "iE-Extensions"], members: &[M { field: "maximum_flow_bit_rate_dl", ty: F::Named(124), optional: false }, M { field: "maximum_flow_bit_rate_ul", ty: F::Named(124), optional: false }, M { field: "guaranteed_flow_bit_rate_dl", ty: F::Named(124), optional: false }, M { field: "guaranteed_flow_bit_rate_ul", ty: F::Named(124), optional: false }, M { field: "notification_control", ty: F::Named(756), optional: true }, M { field: "maximum_packet_loss_rate_dl", ty: F::Named(900), optional: true }, M { field: "maximum_packet_loss_rate_ul", ty: F::Named(900), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 412 */ T { name: "GNB-ID", kind: K::Choice { names: &["gNB-ID", "choice-Extensions"], alternatives: &[F::Bits(22, 32), F::Named(948)], root: 2, extensible: false } },
/* 413 */ T { name: "GNBSetID", kind: K::Alias { field: F::Bits(22, 22), form: None, secret: false } },
/* 414 */ T { name: "GTP-TEID", kind: K::Alias { field: F::OctetArray(4), form: Some(Form::Number(32)), secret: false } },
/* 415 */ T { name: "GTPTunnel", kind: K::Sequence { names: &["transportLayerAddress", "gTP-TEID", "iE-Extensions"], members: &[M { field: "transport_layer_address", ty: F::Named(1230), optional: false }, M { field: "gtp_teid", ty: F::Named(414), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 416 */ T { name: "GUAMI", kind: K::Sequence { names: &["pLMNIdentity", "aMFRegionID", "aMFSetID", "aMFPointer", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "amf_region_id", ty: F::Named(51), optional: false }, M { field: "amf_set_id", ty: F::Named(52), optional: false }, M { field: "amf_pointer", ty: F::Named(50), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 417 */ T { name: "GUAMIList", kind: K::Alias { field: F::List(&F::Named(418), 1, 1024), form: None, secret: false } },
/* 418 */ T { name: "GUAMIList-Item", kind: K::Sequence { names: &["gUAMI", "iE-Extensions"], members: &[M { field: "guami", ty: F::Named(416), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 419 */ T { name: "GUAMIType", kind: K::Enumerated { names: &["native", "mapped"], root: 2, extensible: true } },
/* 420 */ T { name: "GW-Context-Release-Indication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 421 */ T { name: "GeographyBasedMDT", kind: K::Sequence { names: &["nTN-Geographical-Area", "nTN-PLMN-List", "iE-Extensions"], members: &[M { field: "ntn_geographical_area", ty: F::Named(739), optional: false }, M { field: "ntn_plmn_list", ty: F::Named(625), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 422 */ T { name: "GlobalCable-ID", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 423 */ T { name: "GlobalCable-ID-new", kind: K::Sequence { names: &["globalCable-ID", "tAI", "iE-Extensions"], members: &[M { field: "global_cable_id", ty: F::Named(422), optional: false }, M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 424 */ T { name: "GlobalENB-ID", kind: K::Sequence { names: &["pLMNidentity", "eNB-ID", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "enb_id", ty: F::Named(311), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 425 */ T { name: "GlobalGNB-ID", kind: K::Sequence { names: &["pLMNIdentity", "gNB-ID", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "gnb_id", ty: F::Named(412), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 426 */ T { name: "GlobalLine-ID", kind: K::Sequence { names: &["globalLineIdentity", "lineType", "iE-Extensions"], members: &[M { field: "global_line_identity", ty: F::Named(427), optional: false }, M { field: "line_type", ty: F::Named(531), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 427 */ T { name: "GlobalLineIdentity", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 428 */ T { name: "GlobalN3IWF-ID", kind: K::Sequence { names: &["pLMNIdentity", "n3IWF-ID", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "n3iwf_id", ty: F::Named(666), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 429 */ T { name: "GlobalNgENB-ID", kind: K::Sequence { names: &["pLMNIdentity", "ngENB-ID", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "ng_enb_id", ty: F::Named(748), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 430 */ T { name: "GlobalRANNodeID", kind: K::Choice { names: &["globalGNB-ID", "globalNgENB-ID", "globalN3IWF-ID", "choice-Extensions"], alternatives: &[F::Named(425), F::Named(429), F::Named(428), F::Named(948)], root: 4, extensible: false } },
/* 431 */ T { name: "GlobalTNGF-ID", kind: K::Sequence { names: &["pLMNIdentity", "tNGF-ID", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "tngf_id", ty: F::Named(1182), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 432 */ T { name: "GlobalTWIF-ID", kind: K::Sequence { names: &["pLMNIdentity", "tWIF-ID", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "twif_id", ty: F::Named(1192), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 433 */ T { name: "GlobalW-AGF-ID", kind: K::Sequence { names: &["pLMNIdentity", "w-AGF-ID", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "w_agf_id", ty: F::Named(1331), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 434 */ T { name: "HFCNode-ID", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 435 */ T { name: "HFCNode-ID-new", kind: K::Sequence { names: &["hFCNode-ID", "tAI", "iE-Extensions"], members: &[M { field: "hfc_node_id", ty: F::Named(434), optional: false }, M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 436 */ T { name: "HLComActivate", kind: K::Sequence { names: &["nR-Paging-Long-eDRX-Information-for-RRC-INACTIVE", "iE-Extensions"], members: &[M { field: "nr_paging_long_edrx_information_for_rrc_inactive", ty: F::Named(716), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 437 */ T { name: "HLComDeactivate", kind: K::Sequence { names: &["uEReachabilityIndication", "iE-Extensions"], members: &[M { field: "ue_reachability_indication", ty: F::Named(438), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 438 */ T { name: "HLComDeactivateUEReachabilityIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 439 */ T { name: "HOReport", kind: K::Sequence { names: &["handoverReportType", "handoverCause", "sourcecellCGI", "targetcellCGI", "reestablishmentcellCGI", "sourcecellC-RNTI", "targetcellinE-UTRAN", "mobilityInformation", "uERLFReportContainer", "iE-Extensions"], members: &[M { field: "handover_report_type", ty: F::Named(440), optional: false }, M { field: "handover_cause", ty: F::Named(185), optional: false }, M { field: "sourcecell_cgi", ty: F::Named(684), optional: false }, M { field: "targetcell_cgi", ty: F::Named(684), optional: false }, M { field: "reestablishmentcell_cgi", ty: F::Named(684), optional: true }, M { field: "sourcecell_c_rnti", ty: F::Bits(16, 16), optional: true }, M { field: "targetcellin_e_utran", ty: F::Named(315), optional: true }, M { field: "mobility_information", ty: F::Named(647), optional: true }, M { field: "ue_rlf_report_container", ty: F::Named(1275), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 440 */ T { name: "HOReportHandoverReportType", kind: K::Enumerated { names: &["ho-too-early", "ho-to-wrong-cell", "intersystem-ping-pong"], root: 3, extensible: true } },
/* 441 */ T { name: "HandoverCancel", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 442 */ T { name: "HandoverCancelAcknowledge", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 443 */ T { name: "HandoverCommand", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 444 */ T { name: "HandoverCommandTransfer", kind: K::Sequence { names: &["dLForwardingUP-TNLInformation", "qosFlowToBeForwardedList", "dataForwardingResponseDRBList", "iE-Extensions"], members: &[M { field: "dl_forwarding_up_tnl_information", ty: F::Named(1298), optional: true }, M { field: "qos_flow_to_be_forwarded_list", ty: F::Named(995), optional: true }, M { field: "data_forwarding_response_drb_list", ty: F::Named(282), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 445 */ T { name: "HandoverFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 446 */ T { name: "HandoverFlag", kind: K::Enumerated { names: &["handover-preparation"], root: 1, extensible: true } },
/* 447 */ T { name: "HandoverNotify", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 448 */ T { name: "HandoverPreparationFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 449 */ T { name: "HandoverPreparationUnsuccessfulTransfer", kind: K::Sequence { names: &["cause", "iE-Extensions"], members: &[M { field: "cause", ty: F::Named(185), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 450 */ T { name: "HandoverRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 451 */ T { name: "HandoverRequestAcknowledge", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 452 */ T { name: "HandoverRequestAcknowledgeTransfer", kind: K::Sequence { names: &["dL-NGU-UP-TNLInformation", "dLForwardingUP-TNLInformation", "securityResult", "qosFlowSetupResponseList", "qosFlowFailedToSetupList", "dataForwardingResponseDRBList", "iE-Extensions"], members: &[M { field: "dl_ngu_up_tnl_information", ty: F::Named(1298), optional: false }, M { field: "dl_forwarding_up_tnl_information", ty: F::Named(1298), optional: true }, M { field: "security_result", ty: F::Named(1106), optional: true }, M { field: "qos_flow_setup_response_list", ty: F::Named(982), optional: false }, M { field: "qos_flow_failed_to_setup_list", ty: F::Named(981), optional: true }, M { field: "data_forwarding_response_drb_list", ty: F::Named(282), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 453 */ T { name: "HandoverRequired", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 454 */ T { name: "HandoverRequiredTransfer", kind: K::Sequence { names: &["directForwardingPathAvailability", "iE-Extensions"], members: &[M { field: "direct_forwarding_path_availability", ty: F::Named(287), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 455 */ T { name: "HandoverResourceAllocationUnsuccessfulTransfer", kind: K::Sequence { names: &["cause", "criticalityDiagnostics", "iE-Extensions"], members: &[M { field: "cause", ty: F::Named(185), optional: false }, M { field: "criticality_diagnostics", ty: F::Named(246), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 456 */ T { name: "HandoverSuccess", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 457 */ T { name: "HandoverType", kind: K::Enumerated { names: &["intra5gs", "fivegs-to-eps", "eps-to-5gs", "fivegs-to-utran"], root: 3, extensible: true } },
/* 458 */ T { name: "HandoverWindowDuration", kind: K::Alias { field: F::Unsigned(1, 6000, false), form: None, secret: false } },
/* 459 */ T { name: "HandoverWindowStart", kind: K::Alias { field: F::Unsigned(0, 549755813887, false), form: None, secret: false } },
/* 460 */ T { name: "HashedUEIdentityIndexValue", kind: K::Alias { field: F::BitString(13, 13, true), form: None, secret: false } },
/* 461 */ T { name: "Hysteresis", kind: K::Alias { field: F::Unsigned(0, 30, false), form: None, secret: false } },
/* 462 */ T { name: "IAB-Authorized", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 463 */ T { name: "IAB-Supported", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 464 */ T { name: "IABNodeIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 465 */ T { name: "IMSVoiceSupportIndicator", kind: K::Enumerated { names: &["supported", "not-supported"], root: 2, extensible: true } },
/* 466 */ T { name: "ImmediateMDTNr", kind: K::Sequence { names: &["measurementsToActivate", "m1Configuration", "m4Configuration", "m5Configuration", "m6Configuration", "m7Configuration", "bluetoothMeasurementConfiguration", "wLANMeasurementConfiguration", "mDT-Location-Info", "sensorMeasurementConfiguration", "iE-Extensions"], members: &[M { field: "measurements_to_activate", ty: F::Named(640), optional: false }, M { field: "m1_configuration", ty: F::Named(543), optional: true }, M { field: "m4_configuration", ty: F::Named(548), optional: true }, M { field: "m5_configuration", ty: F::Named(551), optional: true }, M { field: "m6_configuration", ty: F::Named(554), optional: true }, M { field: "m7_configuration", ty: F::Named(557), optional: true }, M { field: "bluetooth_measurement_configuration", ty: F::Named(128), optional: true }, M { field: "wlan_measurement_configuration", ty: F::Named(1335), optional: true }, M { field: "mdt_location_info", ty: F::Named(619), optional: true }, M { field: "sensor_measurement_configuration", ty: F::Named(1110), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 467 */ T { name: "IncludeBeamMeasurementsIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 468 */ T { name: "IndexToRFSP", kind: K::Alias { field: F::Unsigned(1, 256, true), form: None, secret: false } },
/* 469 */ T { name: "Indication-of-Bitrate-Adaptation", kind: K::Enumerated { names: &["uplink"], root: 1, extensible: true } },
/* 470 */ T { name: "InfoOnRecommendedCellsAndRANNodesForPaging", kind: K::Sequence { names: &["recommendedCellsForPaging", "recommendRANNodesForPaging", "iE-Extensions"], members: &[M { field: "recommended_cells_for_paging", ty: F::Named(1044), optional: false }, M { field: "recommend_ran_nodes_for_paging", ty: F::Named(1047), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 471 */ T { name: "InitialContextSetupFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 472 */ T { name: "InitialContextSetupRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 473 */ T { name: "InitialContextSetupResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 474 */ T { name: "InitialUEMessage", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 475 */ T { name: "InitiatingMessage", kind: K::Sequence { names: &["procedureCode", "criticality", "value"], members: &[M { field: "procedure_code", ty: F::Named(942), optional: false }, M { field: "criticality", ty: F::Named(245), optional: false }, M { field: "value", ty: F::Open, optional: false }], extensible: false } },
/* 476 */ T { name: "IntegrityProtectionIndication", kind: K::Enumerated { names: &["required", "preferred", "not-needed"], root: 3, extensible: true } },
/* 477 */ T { name: "IntegrityProtectionResult", kind: K::Enumerated { names: &["performed", "not-performed"], root: 2, extensible: true } },
/* 478 */ T { name: "IntendedNumberOfPagingAttempts", kind: K::Alias { field: F::Unsigned(1, 16, true), form: None, secret: false } },
/* 479 */ T { name: "IntendedServiceAreaCoordinates", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 480 */ T { name: "InterSystemFailureIndication", kind: K::Sequence { names: &["uERLFReportContainer", "iE-Extensions"], members: &[M { field: "ue_rlf_report_container", ty: F::Named(1275), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 481 */ T { name: "InterSystemHOReport", kind: K::Sequence { names: &["handoverReportType", "iE-Extensions"], members: &[M { field: "handover_report_type", ty: F::Named(482), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 482 */ T { name: "InterSystemHandoverReportType", kind: K::Choice { names: &["tooearlyIntersystemHO", "intersystemUnnecessaryHO", "choice-Extensions"], alternatives: &[F::Named(1224), F::Named(500), F::Named(948)], root: 3, extensible: false } },
/* 483 */ T { name: "InterfacesToTrace", kind: K::Alias { field: F::Bits(8, 8), form: None, secret: false } },
/* 484 */ T { name: "IntersystemCellActivationReply", kind: K::Sequence { names: &["activatedCellList", "activation-ID", "iE-Extensions"], members: &[M { field: "activated_cell_list", ty: F::Named(57), optional: false }, M { field: "activation_id", ty: F::Unsigned(0, 16384, true), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 485 */ T { name: "IntersystemCellActivationRequest", kind: K::Sequence { names: &["activationID", "cellsToActivateList", "iE-Extensions"], members: &[M { field: "activation_id", ty: F::Unsigned(0, 16384, true), optional: false }, M { field: "cells_to_activate_list", ty: F::Named(211), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 486 */ T { name: "IntersystemCellStateIndication", kind: K::Sequence { names: &["notificationCellList", "iE-Extensions"], members: &[M { field: "notification_cell_list", ty: F::Named(755), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 487 */ T { name: "IntersystemMobilityFailureforVoiceFallback", kind: K::Sequence { names: &["sourcecellID", "targetcellID", "reconnectCellID", "uERLFReportContainer", "iE-Extensions"], members: &[M { field: "sourcecell_id", ty: F::Named(684), optional: false }, M { field: "targetcell_id", ty: F::Named(315), optional: false }, M { field: "reconnect_cell_id", ty: F::Named(315), optional: true }, M { field: "ue_rlf_report_container", ty: F::Named(1275), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 488 */ T { name: "IntersystemResourceStatusReply", kind: K::Sequence { names: &["reportingsystem", "iE-Extensions"], members: &[M { field: "reportingsystem", ty: F::Named(1063), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 489 */ T { name: "IntersystemResourceStatusReport", kind: K::Sequence { names: &["reportingSystem", "iE-Extensions"], members: &[M { field: "reporting_system", ty: F::Named(1071), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 490 */ T { name: "IntersystemResourceStatusRequest", kind: K::Sequence { names: &["reportingSystem", "reportCharacteristics", "reportType", "iE-Extensions"], members: &[M { field: "reporting_system", ty: F::Named(1063), optional: false }, M { field: "report_characteristics", ty: F::Named(1059), optional: false }, M { field: "report_type", ty: F::Named(1061), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 491 */ T { name: "IntersystemResourceThreshold", kind: K::Alias { field: F::Unsigned(0, 100, false), form: None, secret: false } },
/* 492 */ T { name: "IntersystemSONConfigurationTransfer", kind: K::Sequence { names: &["transferType", "intersystemSONInformation", "iE-Extensions"], members: &[M { field: "transfer_type", ty: F::Named(498), optional: false }, M { field: "intersystem_son_information", ty: F::Named(493), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 493 */ T { name: "IntersystemSONInformation", kind: K::Choice { names: &["intersystemSONInformationReport", "choice-Extensions"], alternatives: &[F::Named(495), F::Named(948)], root: 2, extensible: false } },
/* 494 */ T { name: "IntersystemSONInformationReply", kind: K::Choice { names: &["nGRAN-CellActivation", "resourceStatus", "choice-Extensions"], alternatives: &[F::Named(484), F::Named(488), F::Named(948)], root: 3, extensible: false } },
/* 495 */ T { name: "IntersystemSONInformationReport", kind: K::Choice { names: &["hOReportInformation", "failureIndicationInformation", "choice-Extensions"], alternatives: &[F::Named(481), F::Named(480), F::Named(948)], root: 3, extensible: false } },
/* 496 */ T { name: "IntersystemSONInformationRequest", kind: K::Choice { names: &["nGRAN-CellActivation", "resourceStatus", "choice-Extensions"], alternatives: &[F::Named(485), F::Named(490), F::Named(948)], root: 3, extensible: false } },
/* 497 */ T { name: "IntersystemSONNGRANnodeID", kind: K::Sequence { names: &["globalRANNodeID", "selectedTAI", "iE-Extensions"], members: &[M { field: "global_ran_node_id", ty: F::Named(430), optional: false }, M { field: "selected_tai", ty: F::Named(1154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 498 */ T { name: "IntersystemSONTransferType", kind: K::Choice { names: &["fromEUTRANtoNGRAN", "fromNGRANtoEUTRAN", "choice-Extensions"], alternatives: &[F::Named(408), F::Named(409), F::Named(948)], root: 3, extensible: false } },
/* 499 */ T { name: "IntersystemSONeNBID", kind: K::Sequence { names: &["globaleNBID", "selectedEPSTAI", "iE-Extensions"], members: &[M { field: "global_enb_id", ty: F::Named(424), optional: false }, M { field: "selected_eps_tai", ty: F::Named(313), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 500 */ T { name: "IntersystemUnnecessaryHO", kind: K::Sequence { names: &["sourcecellID", "targetcellID", "earlyIRATHO", "candidateCellList", "iE-Extensions"], members: &[M { field: "sourcecell_id", ty: F::Named(684), optional: false }, M { field: "targetcell_id", ty: F::Named(315), optional: false }, M { field: "early_irat_ho", ty: F::Named(501), optional: false }, M { field: "candidate_cell_list", ty: F::Named(180), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 501 */ T { name: "IntersystemUnnecessaryHOEarlyIRATHO", kind: K::Enumerated { names: &["true", "false"], root: 2, extensible: true } },
/* 502 */ T { name: "InventoryFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 503 */ T { name: "InventoryFailureTransfer", kind: K::Sequence { names: &["correlationIdentifier", "cause", "criticalityDiagnostics", "iE-Extensions"], members: &[M { field: "correlation_identifier", ty: F::Named(16), optional: false }, M { field: "cause", ty: F::Named(185), optional: false }, M { field: "criticality_diagnostics", ty: F::Named(246), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 504 */ T { name: "InventoryReport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 505 */ T { name: "InventoryReportTransfer", kind: K::Sequence { names: &["correlationIdentifier", "globalgNB-ID", "readerReportList", "inventoryCompleteIndication", "iE-Extensions"], members: &[M { field: "correlation_identifier", ty: F::Named(16), optional: false }, M { field: "global_gnb_id", ty: F::Named(425), optional: false }, M { field: "reader_report_list", ty: F::Named(25), optional: true }, M { field: "inventory_complete_indication", ty: F::Named(506), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 506 */ T { name: "InventoryReportTransferInventoryCompleteIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 507 */ T { name: "InventoryRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 508 */ T { name: "InventoryRequestTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 509 */ T { name: "InventoryResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 510 */ T { name: "InventoryResponseTransfer", kind: K::Sequence { names: &["correlationIdentifier", "criticalityDiagnostics", "iE-Extensions"], members: &[M { field: "correlation_identifier", ty: F::Named(16), optional: false }, M { field: "criticality_diagnostics", ty: F::Named(246), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 511 */ T { name: "LAC", kind: K::Alias { field: F::OctetArray(2), form: Some(Form::Number(16)), secret: false } },
/* 512 */ T { name: "LAI", kind: K::Sequence { names: &["pLMNidentity", "lAC", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "lac", ty: F::Named(511), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 513 */ T { name: "LPWUSCNsubgroupID", kind: K::Alias { field: F::Unsigned(0, 30, true), form: None, secret: false } },
/* 514 */ T { name: "LPWUSDisableIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 515 */ T { name: "LPWUSPSAssistanceInformation", kind: K::Sequence { names: &["lPWUScNsubgroupID", "iE-Extensions"], members: &[M { field: "lpwus_cn_subgroup_id", ty: F::Named(513), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 516 */ T { name: "LTE-A2X-ServicesAuthorized", kind: K::Sequence { names: &["aerialUE", "aerialControllerUE", "iE-Extensions"], members: &[M { field: "aerial_ue", ty: F::Named(518), optional: true }, M { field: "aerial_controller_ue", ty: F::Named(517), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 517 */ T { name: "LTEA2XServicesAuthorizedAerialControllerUE", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 518 */ T { name: "LTEA2XServicesAuthorizedAerialUE", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 519 */ T { name: "LTEM-Indication", kind: K::Enumerated { names: &["lte-m"], root: 1, extensible: true } },
/* 520 */ T { name: "LTEUERLFReportContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 521 */ T { name: "LTEUESidelinkAggregateMaximumBitrate", kind: K::Sequence { names: &["uESidelinkAggregateMaximumBitRate", "iE-Extensions"], members: &[M { field: "ue_sidelink_aggregate_maximum_bit_rate", ty: F::Named(124), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 522 */ T { name: "LTEV2XServicesAuthorized", kind: K::Sequence { names: &["vehicleUE", "pedestrianUE", "iE-Extensions"], members: &[M { field: "vehicle_ue", ty: F::Named(1328), optional: true }, M { field: "pedestrian_ue", ty: F::Named(925), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 523 */ T { name: "LastVisitedCellInformation", kind: K::Choice { names: &["nGRANCell", "eUTRANCell", "uTRANCell", "gERANCell", "choice-Extensions"], alternatives: &[F::Named(527), F::Named(525), F::Named(530), F::Named(526), F::Named(948)], root: 5, extensible: false } },
/* 524 */ T { name: "LastVisitedCellItem", kind: K::Sequence { names: &["lastVisitedCellInformation", "iE-Extensions"], members: &[M { field: "last_visited_cell_information", ty: F::Named(523), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 525 */ T { name: "LastVisitedEUTRANCellInformation", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 526 */ T { name: "LastVisitedGERANCellInformation", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 527 */ T { name: "LastVisitedNGRANCellInformation", kind: K::Sequence { names: &["globalCellID", "cellType", "timeUEStayedInCell", "timeUEStayedInCellEnhancedGranularity", "hOCauseValue", "iE-Extensions"], members: &[M { field: "global_cell_id", ty: F::Named(684), optional: false }, M { field: "cell_type", ty: F::Named(210), optional: false }, M { field: "time_ue_stayed_in_cell", ty: F::Named(1217), optional: false }, M { field: "time_ue_stayed_in_cell_enhanced_granularity", ty: F::Named(1218), optional: true }, M { field: "ho_cause_value", ty: F::Named(185), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 528 */ T { name: "LastVisitedPSCellInformation", kind: K::Sequence { names: &["pSCellID", "timeStay", "iE-Extensions"], members: &[M { field: "ps_cell_id", ty: F::Named(684), optional: true }, M { field: "time_stay", ty: F::Unsigned(0, 40950, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 529 */ T { name: "LastVisitedPSCellList", kind: K::Alias { field: F::List(&F::Named(528), 1, 8), form: None, secret: false } },
/* 530 */ T { name: "LastVisitedUTRANCellInformation", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 531 */ T { name: "LineType", kind: K::Enumerated { names: &["dsl", "pon"], root: 2, extensible: true } },
/* 532 */ T { name: "Links-to-log", kind: K::Enumerated { names: &["uplink", "downlink", "both-uplink-and-downlink"], root: 3, extensible: true } },
/* 533 */ T { name: "LocationReport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 534 */ T { name: "LocationReportingAdditionalInfo", kind: K::Enumerated { names: &["includePSCell"], root: 1, extensible: true } },
/* 535 */ T { name: "LocationReportingControl", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 536 */ T { name: "LocationReportingFailureIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 537 */ T { name: "LocationReportingReferenceID", kind: K::Alias { field: F::Unsigned(1, 64, true), form: None, secret: false } },
/* 538 */ T { name: "LocationReportingRequestType", kind: K::Sequence { names: &["eventType", "reportArea", "areaOfInterestList", "locationReportingReferenceIDToBeCancelled", "iE-Extensions"], members: &[M { field: "event_type", ty: F::Named(360), optional: false }, M { field: "report_area", ty: F::Named(1058), optional: false }, M { field: "area_of_interest_list", ty: F::Named(91), optional: true }, M { field: "location_reporting_reference_id_to_be_cancelled", ty: F::Named(537), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 539 */ T { name: "LoggedMDTNr", kind: K::Sequence { names: &["loggingInterval", "loggingDuration", "loggedMDTTrigger", "bluetoothMeasurementConfiguration", "wLANMeasurementConfiguration", "sensorMeasurementConfiguration", "areaScopeOfNeighCellsList", "iE-Extensions"], members: &[M { field: "logging_interval", ty: F::Named(542), optional: false }, M { field: "logging_duration", ty: F::Named(541), optional: false }, M { field: "logged_mdt_trigger", ty: F::Named(540), optional: false }, M { field: "bluetooth_measurement_configuration", ty: F::Named(128), optional: true }, M { field: "wlan_measurement_configuration", ty: F::Named(1335), optional: true }, M { field: "sensor_measurement_configuration", ty: F::Named(1110), optional: true }, M { field: "area_scope_of_neigh_cells_list", ty: F::Named(99), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 540 */ T { name: "LoggedMDTTrigger", kind: K::Choice { names: &["periodical", "eventTrigger", "choice-Extensions"], alternatives: &[F::Null, F::Named(358), F::Named(948)], root: 3, extensible: false } },
/* 541 */ T { name: "LoggingDuration", kind: K::Enumerated { names: &["m10", "m20", "m40", "m60", "m90", "m120"], root: 6, extensible: true } },
/* 542 */ T { name: "LoggingInterval", kind: K::Enumerated { names: &["ms320", "ms640", "ms1280", "ms2560", "ms5120", "ms10240", "ms20480", "ms30720", "ms40960", "ms61440", "infinity"], root: 11, extensible: true } },
/* 543 */ T { name: "M1Configuration", kind: K::Sequence { names: &["m1reportingTrigger", "m1thresholdEventA2", "m1periodicReporting", "iE-Extensions"], members: &[M { field: "m1_reporting_trigger", ty: F::Named(545), optional: false }, M { field: "m1_threshold_event_a2", ty: F::Named(546), optional: true }, M { field: "m1_periodic_reporting", ty: F::Named(544), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 544 */ T { name: "M1PeriodicReporting", kind: K::Sequence { names: &["reportInterval", "reportAmount", "iE-Extensions"], members: &[M { field: "report_interval", ty: F::Named(1060), optional: false }, M { field: "report_amount", ty: F::Named(1057), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 545 */ T { name: "M1ReportingTrigger", kind: K::Enumerated { names: &["periodic", "a2eventtriggered", "a2eventtriggered-periodic"], root: 3, extensible: true } },
/* 546 */ T { name: "M1ThresholdEventA2", kind: K::Sequence { names: &["m1ThresholdType", "iE-Extensions"], members: &[M { field: "m1_threshold_type", ty: F::Named(547), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 547 */ T { name: "M1ThresholdType", kind: K::Choice { names: &["threshold-RSRP", "threshold-RSRQ", "threshold-SINR", "choice-Extensions"], alternatives: &[F::Named(1207), F::Named(1208), F::Named(1209), F::Named(948)], root: 4, extensible: false } },
/* 548 */ T { name: "M4Configuration", kind: K::Sequence { names: &["m4period", "m4-links-to-log", "iE-Extensions"], members: &[M { field: "m4_period", ty: F::Named(550), optional: false }, M { field: "m4_links_to_log", ty: F::Named(532), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 549 */ T { name: "M4ReportAmountMDT", kind: K::Enumerated { names: &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"], root: 8, extensible: true } },
/* 550 */ T { name: "M4period", kind: K::Enumerated { names: &["ms1024", "ms2048", "ms5120", "ms10240", "min1"], root: 5, extensible: true } },
/* 551 */ T { name: "M5Configuration", kind: K::Sequence { names: &["m5period", "m5-links-to-log", "iE-Extensions"], members: &[M { field: "m5_period", ty: F::Named(553), optional: false }, M { field: "m5_links_to_log", ty: F::Named(532), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 552 */ T { name: "M5ReportAmountMDT", kind: K::Enumerated { names: &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"], root: 8, extensible: true } },
/* 553 */ T { name: "M5period", kind: K::Enumerated { names: &["ms1024", "ms2048", "ms5120", "ms10240", "min1"], root: 5, extensible: true } },
/* 554 */ T { name: "M6Configuration", kind: K::Sequence { names: &["m6report-Interval", "m6-links-to-log", "iE-Extensions"], members: &[M { field: "m6_report_interval", ty: F::Named(556), optional: false }, M { field: "m6_links_to_log", ty: F::Named(532), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 555 */ T { name: "M6ReportAmountMDT", kind: K::Enumerated { names: &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"], root: 8, extensible: true } },
/* 556 */ T { name: "M6report-Interval", kind: K::Enumerated { names: &["ms120", "ms240", "ms480", "ms640", "ms1024", "ms2048", "ms5120", "ms10240", "ms20480", "ms40960", "min1", "min6", "min12", "min30"], root: 14, extensible: true } },
/* 557 */ T { name: "M7Configuration", kind: K::Sequence { names: &["m7period", "m7-links-to-log", "iE-Extensions"], members: &[M { field: "m7_period", ty: F::Named(559), optional: false }, M { field: "m7_links_to_log", ty: F::Named(532), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 558 */ T { name: "M7ReportAmountMDT", kind: K::Enumerated { names: &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"], root: 8, extensible: true } },
/* 559 */ T { name: "M7period", kind: K::Alias { field: F::Unsigned(1, 60, true), form: None, secret: false } },
/* 560 */ T { name: "MBS-ActiveSessionInformation-SourcetoTargetItem", kind: K::Sequence { names: &["mBS-SessionID", "mBS-AreaSessionID", "mBS-ServiceArea", "mBS-QoSFlowsToBeSetupList", "mBS-MappingandDataForwardingRequestList", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "mbs_area_session_id", ty: F::Named(564), optional: true }, M { field: "mbs_service_area", ty: F::Named(580), optional: true }, M { field: "mbs_qos_flows_to_be_setup_list", ty: F::Named(579), optional: false }, M { field: "mbs_mappingand_data_forwarding_request_list", ty: F::Named(575), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 561 */ T { name: "MBS-ActiveSessionInformation-SourcetoTargetList", kind: K::Alias { field: F::List(&F::Named(560), 1, 256), form: None, secret: false } },
/* 562 */ T { name: "MBS-ActiveSessionInformation-TargettoSourceItem", kind: K::Sequence { names: &["mBS-SessionID", "mBS-DataForwardingResponseMRBList", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "mbs_data_forwarding_response_mrb_list", ty: F::Named(568), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 563 */ T { name: "MBS-ActiveSessionInformation-TargettoSourceList", kind: K::Alias { field: F::List(&F::Named(562), 1, 256), form: None, secret: false } },
/* 564 */ T { name: "MBS-AreaSessionID", kind: K::Alias { field: F::Unsigned(0, 65535, true), form: None, secret: false } },
/* 565 */ T { name: "MBS-AreaTAIList", kind: K::Alias { field: F::List(&F::Named(1154), 1, 16), form: None, secret: false } },
/* 566 */ T { name: "MBS-AssistanceInformation", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 567 */ T { name: "MBS-DataForwardingResponseMRBItem", kind: K::Sequence { names: &["mRB-ID", "dL-Forwarding-UPTNLInformation", "mRB-ProgressInformation", "iE-Extensions"], members: &[M { field: "mrb_id", ty: F::Named(630), optional: false }, M { field: "dl_forwarding_up_tnl_information", ty: F::Named(1298), optional: false }, M { field: "mrb_progress_information", ty: F::Named(631), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 568 */ T { name: "MBS-DataForwardingResponseMRBList", kind: K::Alias { field: F::List(&F::Named(567), 1, 32), form: None, secret: false } },
/* 569 */ T { name: "MBS-DistributionReleaseRequestTransfer", kind: K::Sequence { names: &["mBS-SessionID", "mBS-AreaSessionID", "sharedNGU-UnicastTNLInformation", "cause", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "mbs_area_session_id", ty: F::Named(564), optional: true }, M { field: "shared_ngu_unicast_tnl_information", ty: F::Named(1298), optional: true }, M { field: "cause", ty: F::Named(185), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 570 */ T { name: "MBS-DistributionSetupRequestTransfer", kind: K::Sequence { names: &["mBS-SessionID", "mBS-AreaSessionID", "sharedNGU-UnicastTNLInformation", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "mbs_area_session_id", ty: F::Named(564), optional: true }, M { field: "shared_ngu_unicast_tnl_information", ty: F::Named(1298), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 571 */ T { name: "MBS-DistributionSetupResponseTransfer", kind: K::Sequence { names: &["mBS-SessionID", "mBS-AreaSessionID", "sharedNGU-MulticastTNLInformation", "mBS-QoSFlowsToBeSetupList", "mBSSessionStatus", "mBS-ServiceArea", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "mbs_area_session_id", ty: F::Named(564), optional: true }, M { field: "shared_ngu_multicast_tnl_information", ty: F::Named(1122), optional: true }, M { field: "mbs_qos_flows_to_be_setup_list", ty: F::Named(579), optional: false }, M { field: "mbs_session_status", ty: F::Named(611), optional: false }, M { field: "mbs_service_area", ty: F::Named(580), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 572 */ T { name: "MBS-DistributionSetupUnsuccessfulTransfer", kind: K::Sequence { names: &["mBS-SessionID", "mBS-AreaSessionID", "cause", "criticalityDiagnostics", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "mbs_area_session_id", ty: F::Named(564), optional: true }, M { field: "cause", ty: F::Named(185), optional: false }, M { field: "criticality_diagnostics", ty: F::Named(246), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 573 */ T { name: "MBS-IntendedServiceAreaList", kind: K::Alias { field: F::List(&F::Named(479), 1, 65536), form: None, secret: false } },
/* 574 */ T { name: "MBS-MappingandDataForwardingRequestItem", kind: K::Sequence { names: &["mRB-ID", "mBS-QoSFlowList", "mRB-ProgressInformation", "iE-Extensions"], members: &[M { field: "mrb_id", ty: F::Named(630), optional: false }, M { field: "mbs_qos_flow_list", ty: F::Named(577), optional: false }, M { field: "mrb_progress_information", ty: F::Named(631), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 575 */ T { name: "MBS-MappingandDataForwardingRequestList", kind: K::Alias { field: F::List(&F::Named(574), 1, 32), form: None, secret: false } },
/* 576 */ T { name: "MBS-NGUFailureIndication", kind: K::Choice { names: &["locationindependent", "locationdependent", "choice-Extensions"], alternatives: &[F::Named(596), F::Named(598), F::Named(948)], root: 3, extensible: false } },
/* 577 */ T { name: "MBS-QoSFlowList", kind: K::Alias { field: F::List(&F::Named(976), 1, 64), form: None, secret: false } },
/* 578 */ T { name: "MBS-QoSFlowsToBeSetupItem", kind: K::Sequence { names: &["mBSqosFlowIdentifier", "mBSqosFlowLevelQosParameters", "iE-Extensions"], members: &[M { field: "mbs_qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "mbs_qos_flow_level_qos_parameters", ty: F::Named(980), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 579 */ T { name: "MBS-QoSFlowsToBeSetupList", kind: K::Alias { field: F::List(&F::Named(578), 1, 64), form: None, secret: false } },
/* 580 */ T { name: "MBS-ServiceArea", kind: K::Choice { names: &["locationindependent", "locationdependent", "choice-Extensions"], alternatives: &[F::Named(582), F::Named(584), F::Named(948)], root: 3, extensible: false } },
/* 581 */ T { name: "MBS-ServiceAreaCellList", kind: K::Alias { field: F::List(&F::Named(711), 1, 8192), form: None, secret: false } },
/* 582 */ T { name: "MBS-ServiceAreaInformation", kind: K::Sequence { names: &["mBS-ServiceAreaCellList", "mBS-ServiceAreaTAIList", "iE-Extensions"], members: &[M { field: "mbs_service_area_cell_list", ty: F::Named(581), optional: true }, M { field: "mbs_service_area_tai_list", ty: F::Named(585), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 583 */ T { name: "MBS-ServiceAreaInformationItem", kind: K::Sequence { names: &["mBS-AreaSessionID", "mBS-ServiceAreaInformation", "iE-Extensions"], members: &[M { field: "mbs_area_session_id", ty: F::Named(564), optional: false }, M { field: "mbs_service_area_information", ty: F::Named(582), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 584 */ T { name: "MBS-ServiceAreaInformationList", kind: K::Alias { field: F::List(&F::Named(583), 1, 256), form: None, secret: false } },
/* 585 */ T { name: "MBS-ServiceAreaTAIList", kind: K::Alias { field: F::List(&F::Named(1154), 1, 1024), form: None, secret: false } },
/* 586 */ T { name: "MBS-SessionFSAID", kind: K::Alias { field: F::OctetArray(3), form: None, secret: false } },
/* 587 */ T { name: "MBS-SessionFSAIDList", kind: K::Alias { field: F::List(&F::Named(586), 1, 64), form: None, secret: false } },
/* 588 */ T { name: "MBS-SessionID", kind: K::Sequence { names: &["tMGI", "nID", "iE-Extensions"], members: &[M { field: "tmgi", ty: F::Named(1180), optional: false }, M { field: "nid", ty: F::Named(705), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 589 */ T { name: "MBS-SessionTNLInfo5GC", kind: K::Choice { names: &["locationindependent", "locationdependent", "choice-Extensions"], alternatives: &[F::Named(1122), F::Named(591), F::Named(948)], root: 3, extensible: false } },
/* 590 */ T { name: "MBS-SessionTNLInfo5GCItem", kind: K::Sequence { names: &["mBS-AreaSessionID", "sharedNGU-MulticastTNLInformation", "iE-Extensions"], members: &[M { field: "mbs_area_session_id", ty: F::Named(564), optional: false }, M { field: "shared_ngu_multicast_tnl_information", ty: F::Named(1122), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 591 */ T { name: "MBS-SessionTNLInfo5GCList", kind: K::Alias { field: F::List(&F::Named(590), 1, 256), form: None, secret: false } },
/* 592 */ T { name: "MBS-SessionTNLInfoNGRAN", kind: K::Choice { names: &["locationindependent", "locationdependent", "choice-Extensions"], alternatives: &[F::Named(1298), F::Named(594), F::Named(948)], root: 3, extensible: false } },
/* 593 */ T { name: "MBS-SessionTNLInfoNGRANItem", kind: K::Sequence { names: &["mBS-AreaSessionID", "sharedNGU-UnicastTNLInformation", "iE-Extensions"], members: &[M { field: "mbs_area_session_id", ty: F::Named(564), optional: false }, M { field: "shared_ngu_unicast_tnl_information", ty: F::Named(1298), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 594 */ T { name: "MBS-SessionTNLInfoNGRANList", kind: K::Alias { field: F::List(&F::Named(593), 1, 256), form: None, secret: false } },
/* 595 */ T { name: "MBS-SupportIndicator", kind: K::Enumerated { names: &["multicast-supported", "multicast-supported-with-reception-in-RRC-inactive"], root: 1, extensible: true } },
/* 596 */ T { name: "MBS-UP-FailureIndication", kind: K::Enumerated { names: &["ng-u-path-failure-detected"], root: 1, extensible: true } },
/* 597 */ T { name: "MBS-UP-FailureIndicationItem", kind: K::Sequence { names: &["mBS-AreaSessionID", "mBS-UP-FailureIndication", "iE-Extensions"], members: &[M { field: "mbs_area_session_id", ty: F::Named(564), optional: false }, M { field: "mbs_up_failure_indication", ty: F::Named(596), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 598 */ T { name: "MBS-UP-FailureIndicationList", kind: K::Alias { field: F::List(&F::Named(597), 1, 256), form: None, secret: false } },
/* 599 */ T { name: "MBSSessionFailedtoSetupItem", kind: K::Sequence { names: &["mBS-SessionID", "mBS-AreaSessionID", "cause", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "mbs_area_session_id", ty: F::Named(564), optional: true }, M { field: "cause", ty: F::Named(185), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 600 */ T { name: "MBSSessionFailedtoSetupList", kind: K::Alias { field: F::List(&F::Named(599), 1, 32), form: None, secret: false } },
/* 601 */ T { name: "MBSSessionReleaseResponseTransfer", kind: K::Sequence { names: &["mBS-SessionTNLInfoNGRAN", "iE-Extensions"], members: &[M { field: "mbs_session_tnl_info_ngran", ty: F::Named(592), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 602 */ T { name: "MBSSessionSetupOrModFailureTransfer", kind: K::Sequence { names: &["cause", "criticalityDiagnostics", "iE-Extensions"], members: &[M { field: "cause", ty: F::Named(185), optional: false }, M { field: "criticality_diagnostics", ty: F::Named(246), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 603 */ T { name: "MBSSessionSetupOrModRequestTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 604 */ T { name: "MBSSessionSetupOrModResponseTransfer", kind: K::Sequence { names: &["mBS-SessionTNLInfoNGRAN", "iE-Extensions"], members: &[M { field: "mbs_session_tnl_info_ngran", ty: F::Named(592), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 605 */ T { name: "MBSSessionSetupRequestItem", kind: K::Sequence { names: &["mBS-SessionID", "mBS-AreaSessionID", "associatedMBSQosFlowSetupRequestList", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "mbs_area_session_id", ty: F::Named(564), optional: true }, M { field: "associated_mbs_qos_flow_setup_request_list", ty: F::Named(105), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 606 */ T { name: "MBSSessionSetupRequestList", kind: K::Alias { field: F::List(&F::Named(605), 1, 32), form: None, secret: false } },
/* 607 */ T { name: "MBSSessionSetupResponseItem", kind: K::Sequence { names: &["mBS-SessionID", "mBS-AreaSessionID", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "mbs_area_session_id", ty: F::Named(564), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 608 */ T { name: "MBSSessionSetupResponseList", kind: K::Alias { field: F::List(&F::Named(607), 1, 32), form: None, secret: false } },
/* 609 */ T { name: "MBSSessionSetuporModifyRequestItem", kind: K::Sequence { names: &["mBS-SessionID", "mBS-AreaSessionID", "associatedMBSQosFlowSetuporModifyRequestList", "mBS-QosFlowToReleaseList", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "mbs_area_session_id", ty: F::Named(564), optional: true }, M { field: "associated_mbs_qos_flow_setupor_modify_request_list", ty: F::Named(107), optional: true }, M { field: "mbs_qos_flow_to_release_list", ty: F::Named(981), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 610 */ T { name: "MBSSessionSetuporModifyRequestList", kind: K::Alias { field: F::List(&F::Named(609), 1, 32), form: None, secret: false } },
/* 611 */ T { name: "MBSSessionStatus", kind: K::Enumerated { names: &["activated", "deactivated"], root: 2, extensible: true } },
/* 612 */ T { name: "MBSSessionToReleaseItem", kind: K::Sequence { names: &["mBS-SessionID", "cause", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "cause", ty: F::Named(185), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 613 */ T { name: "MBSSessionToReleaseList", kind: K::Alias { field: F::List(&F::Named(612), 1, 32), form: None, secret: false } },
/* 614 */ T { name: "MDT-Activation", kind: K::Enumerated { names: &["immediate-MDT-only", "logged-MDT-only", "immediate-MDT-and-Trace"], root: 3, extensible: true } },
/* 615 */ T { name: "MDT-AlignmentInfo", kind: K::Choice { names: &["s-basedMDT", "choice-Extensions"], alternatives: &[F::Named(696), F::Named(948)], root: 2, extensible: false } },
/* 616 */ T { name: "MDT-Configuration", kind: K::Sequence { names: &["mdt-Config-NR", "mdt-Config-EUTRA", "iE-Extensions"], members: &[M { field: "mdt_config_nr", ty: F::Named(618), optional: true }, M { field: "mdt_config_eutra", ty: F::Named(617), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 617 */ T { name: "MDT-Configuration-EUTRA", kind: K::Sequence { names: &["mdt-Activation", "areaScopeOfMDT", "mDTMode", "signallingBasedMDTPLMNList", "iE-Extensions"], members: &[M { field: "mdt_activation", ty: F::Named(614), optional: false }, M { field: "area_scope_of_mdt", ty: F::Named(96), optional: false }, M { field: "mdt_mode", ty: F::Named(623), optional: false }, M { field: "signalling_based_mdt_plmn_list", ty: F::Named(625), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 618 */ T { name: "MDT-Configuration-NR", kind: K::Sequence { names: &["mdt-Activation", "areaScopeOfMDT", "mDTModeNr", "signallingBasedMDTPLMNList", "iE-Extensions"], members: &[M { field: "mdt_activation", ty: F::Named(614), optional: false }, M { field: "area_scope_of_mdt", ty: F::Named(97), optional: false }, M { field: "mdt_mode_nr", ty: F::Named(624), optional: false }, M { field: "signalling_based_mdt_plmn_list", ty: F::Named(625), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 619 */ T { name: "MDT-Location-Info", kind: K::Sequence { names: &["mDT-Location-Information", "iE-Extensions"], members: &[M { field: "mdt_location_information", ty: F::Named(620), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 620 */ T { name: "MDT-Location-Information", kind: K::Alias { field: F::Bits(8, 8), form: None, secret: false } },
/* 621 */ T { name: "MDT-circle", kind: K::Sequence { names: &["reference-location", "distance-radius", "iE-Extensions"], members: &[M { field: "reference_location", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "distance_radius", ty: F::Unsigned(1, 65535, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 622 */ T { name: "MDT-polygon", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 623 */ T { name: "MDTModeEutra", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 624 */ T { name: "MDTModeNr", kind: K::Choice { names: &["immediateMDTNr", "loggedMDTNr", "choice-Extensions"], alternatives: &[F::Named(466), F::Named(539), F::Named(948)], root: 3, extensible: false } },
/* 625 */ T { name: "MDTPLMNList", kind: K::Alias { field: F::List(&F::Named(886), 1, 16), form: None, secret: false } },
/* 626 */ T { name: "MDTPLMNModificationList", kind: K::Alias { field: F::List(&F::Named(886), 0, 16), form: None, secret: false } },
/* 627 */ T { name: "MICOModeIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 628 */ T { name: "MMSID", kind: K::Alias { field: F::OctetArray(1), form: None, secret: false } },
/* 629 */ T { name: "MN-only-MDT-collection", kind: K::Enumerated { names: &["mn-only"], root: 1, extensible: true } },
/* 630 */ T { name: "MRB-ID", kind: K::Alias { field: F::Unsigned(1, 512, true), form: None, secret: false } },
/* 631 */ T { name: "MRB-ProgressInformation", kind: K::Choice { names: &["pDCP-SN-Length12", "pDCP-SN-Length18", "choice-Extensions"], alternatives: &[F::Unsigned(0, 4095, false), F::Unsigned(0, 262143, false), F::Named(948)], root: 3, extensible: false } },
/* 632 */ T { name: "MTCommunicationHandlingFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 633 */ T { name: "MTCommunicationHandlingRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 634 */ T { name: "MTCommunicationHandlingResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 635 */ T { name: "MaskedIMEISV", kind: K::Alias { field: F::Bits(64, 64), form: None, secret: false } },
/* 636 */ T { name: "MaxNrofRS-IndexesToReport", kind: K::Alias { field: F::Unsigned(1, 64, true), form: None, secret: false } },
/* 637 */ T { name: "MaximumDataBurstVolume", kind: K::Alias { field: F::Unsigned(0, 4095, true), form: None, secret: false } },
/* 638 */ T { name: "MaximumIntegrityProtectedDataRate", kind: K::Enumerated { names: &["bitrate64kbs", "maximum-UE-rate"], root: 2, extensible: true } },
/* 639 */ T { name: "MeasurementThresholdL1LoggedMDT", kind: K::Choice { names: &["threshold-RSRP", "threshold-RSRQ", "choice-Extensions"], alternatives: &[F::Named(1207), F::Named(1208), F::Named(948)], root: 3, extensible: false } },
/* 640 */ T { name: "MeasurementsToActivate", kind: K::Alias { field: F::Bits(8, 8), form: None, secret: false } },
/* 641 */ T { name: "MessageIdentifier", kind: K::Alias { field: F::Bits(16, 16), form: None, secret: false } },
/* 642 */ T { name: "MicoAllPLMN", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 643 */ T { name: "MobileIAB-Authorized", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 644 */ T { name: "MobileIAB-MTUserLocationInformation", kind: K::Sequence { names: &["nRCGI", "tAI", "iE-Extensions"], members: &[M { field: "nr_cgi", ty: F::Named(711), optional: false }, M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 645 */ T { name: "MobileIAB-Supported", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 646 */ T { name: "MobileIABNodeIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 647 */ T { name: "MobilityInformation", kind: K::Alias { field: F::Bits(16, 16), form: None, secret: false } },
/* 648 */ T { name: "MobilityRestrictionList", kind: K::Sequence { names: &["servingPLMN", "equivalentPLMNs", "rATRestrictions", "forbiddenAreaInformation", "serviceAreaInformation", "iE-Extensions"], members: &[M { field: "serving_plmn", ty: F::Named(886), optional: false }, M { field: "equivalent_plmns", ty: F::Named(352), optional: true }, M { field: "rat_restrictions", ty: F::Named(1025), optional: true }, M { field: "forbidden_area_information", ty: F::Named(405), optional: true }, M { field: "service_area_information", ty: F::Named(1118), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 649 */ T { name: "MonitoringRequest", kind: K::Enumerated { names: &["ul", "dl", "both", "stop"], root: 4, extensible: true } },
/* 650 */ T { name: "MonitoringRequestonAvailableBitrate", kind: K::Sequence { names: &["monitoringRequest", "dlAvailableBitrateReportThresholds", "ulAvailableBitrateReportThresholds", "iE-Extensions"], members: &[M { field: "monitoring_request", ty: F::Named(649), optional: false }, M { field: "dl_available_bitrate_report_thresholds", ty: F::Named(114), optional: true }, M { field: "ul_available_bitrate_report_thresholds", ty: F::Named(114), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 651 */ T { name: "MulticastGroupPaging", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 652 */ T { name: "MulticastGroupPagingArea", kind: K::Sequence { names: &["mBS-AreaTAIList", "iE-Extensions"], members: &[M { field: "mbs_area_tai_list", ty: F::Named(565), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 653 */ T { name: "MulticastGroupPagingAreaItem", kind: K::Sequence { names: &["multicastGroupPagingArea", "uE-PagingList", "iE-Extensions"], members: &[M { field: "multicast_group_paging_area", ty: F::Named(652), optional: false }, M { field: "ue_paging_list", ty: F::Named(1237), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 654 */ T { name: "MulticastGroupPagingAreaList", kind: K::Alias { field: F::List(&F::Named(653), 1, 64), form: None, secret: false } },
/* 655 */ T { name: "MulticastSessionActivationFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 656 */ T { name: "MulticastSessionActivationRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 657 */ T { name: "MulticastSessionActivationRequestTransfer", kind: K::Sequence { names: &["mBS-SessionID", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 658 */ T { name: "MulticastSessionActivationResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 659 */ T { name: "MulticastSessionDeactivationRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 660 */ T { name: "MulticastSessionDeactivationRequestTransfer", kind: K::Sequence { names: &["mBS-SessionID", "iE-Extensions"], members: &[M { field: "mbs_session_id", ty: F::Named(588), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 661 */ T { name: "MulticastSessionDeactivationResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 662 */ T { name: "MulticastSessionUpdateFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 663 */ T { name: "MulticastSessionUpdateRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 664 */ T { name: "MulticastSessionUpdateRequestTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 665 */ T { name: "MulticastSessionUpdateResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 666 */ T { name: "N3IWF-ID", kind: K::Choice { names: &["n3IWF-ID", "choice-Extensions"], alternatives: &[F::Bits(16, 16), F::Named(948)], root: 2, extensible: false } },
/* 667 */ T { name: "N6JitterInformation", kind: K::Sequence { names: &["n6JitterLowerBound", "n6JitterUpperBound", "iE-Extensions"], members: &[M { field: "n6_jitter_lower_bound", ty: F::Signed(-127, 127, false), optional: false }, M { field: "n6_jitter_upper_bound", ty: F::Signed(-127, 127, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 668 */ T { name: "NAS-PDU", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 669 */ T { name: "NASNonDeliveryIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 670 */ T { name: "NASSecurityParametersFromNGRAN", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 671 */ T { name: "NB-IoT-DefaultPagingDRX", kind: K::Enumerated { names: &["rf128", "rf256", "rf512", "rf1024"], root: 4, extensible: true } },
/* 672 */ T { name: "NB-IoT-Paging-TimeWindow", kind: K::Enumerated { names: &["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13", "s14", "s15", "s16"], root: 16, extensible: true } },
/* 673 */ T { name: "NB-IoT-Paging-eDRXCycle", kind: K::Enumerated { names: &["hf2", "hf4", "hf6", "hf8", "hf10", "hf12", "hf14", "hf16", "hf32", "hf64", "hf128", "hf256", "hf512", "hf1024"], root: 14, extensible: true } },
/* 674 */ T { name: "NB-IoT-Paging-eDRXInfo", kind: K::Sequence { names: &["nB-IoT-Paging-eDRXCycle", "nB-IoT-Paging-TimeWindow", "iE-Extensions"], members: &[M { field: "nb_iot_paging_edrx_cycle", ty: F::Named(673), optional: false }, M { field: "nb_iot_paging_time_window", ty: F::Named(672), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 675 */ T { name: "NB-IoT-PagingDRX", kind: K::Enumerated { names: &["rf32", "rf64", "rf128", "rf256", "rf512", "rf1024"], root: 6, extensible: true } },
/* 676 */ T { name: "NB-IoT-UEPriority", kind: K::Alias { field: F::Unsigned(0, 255, true), form: None, secret: false } },
/* 677 */ T { name: "NGAP-PDU", kind: K::Choice { names: &["initiatingMessage", "successfulOutcome", "unsuccessfulOutcome"], alternatives: &[F::Named(475), F::Named(1139), F::Named(1306)], root: 3, extensible: true } },
/* 678 */ T { name: "NGAPIESupportInformationRequestItem", kind: K::Sequence { names: &["ngap-ProtocolIE-Id", "iE-Extensions"], members: &[M { field: "ngap_protocol_ie_id", ty: F::Named(949), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 679 */ T { name: "NGAPIESupportInformationRequestList", kind: K::Alias { field: F::List(&F::Named(678), 1, 32), form: None, secret: false } },
/* 680 */ T { name: "NGAPIESupportInformationResponseItem", kind: K::Sequence { names: &["ngap-ProtocolIE-Id", "ngap-ProtocolIESupportInfo", "ngap-ProtocolIEPresenceInfo", "iE-Extensions"], members: &[M { field: "ngap_protocol_ie_id", ty: F::Named(949), optional: false }, M { field: "ngap_protocol_ie_support_info", ty: F::Named(682), optional: false }, M { field: "ngap_protocol_ie_presence_info", ty: F::Named(681), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 681 */ T { name: "NGAPIESupportInformationResponseItemNgapProtocolIEPresenceInfo", kind: K::Enumerated { names: &["present", "not-present"], root: 2, extensible: true } },
/* 682 */ T { name: "NGAPIESupportInformationResponseItemNgapProtocolIESupportInfo", kind: K::Enumerated { names: &["supported", "not-supported"], root: 2, extensible: true } },
/* 683 */ T { name: "NGAPIESupportInformationResponseList", kind: K::Alias { field: F::List(&F::Named(680), 1, 32), form: None, secret: false } },
/* 684 */ T { name: "NGRAN-CGI", kind: K::Choice { names: &["nR-CGI", "eUTRA-CGI", "choice-Extensions"], alternatives: &[F::Named(711), F::Named(315), F::Named(948)], root: 3, extensible: false } },
/* 685 */ T { name: "NGRAN-CellReportItem", kind: K::Sequence { names: &["nGRAN-CGI", "nGRAN-CompositeAvailableCapacityGroup", "nGRAN-NumberOfActiveUEs", "nGRAN-NoofRRCConnections", "nGRAN-RadioResourceStatus", "iE-Extensions"], members: &[M { field: "ngran_cgi", ty: F::Named(684), optional: false }, M { field: "ngran_composite_available_capacity_group", ty: F::Named(326), optional: false }, M { field: "ngran_number_of_active_ues", ty: F::Named(690), optional: true }, M { field: "ngran_noof_rrc_connections", ty: F::Named(689), optional: true }, M { field: "ngran_radio_resource_status", ty: F::Named(691), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 686 */ T { name: "NGRAN-CellReportList", kind: K::Alias { field: F::List(&F::Named(685), 1, 256), form: None, secret: false } },
/* 687 */ T { name: "NGRAN-CellToReportItem", kind: K::Sequence { names: &["nGRAN-CGI", "iE-Extensions"], members: &[M { field: "ngran_cgi", ty: F::Named(684), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 688 */ T { name: "NGRAN-CellToReportList", kind: K::Alias { field: F::List(&F::Named(687), 1, 256), form: None, secret: false } },
/* 689 */ T { name: "NGRAN-NoofRRCConnections", kind: K::Alias { field: F::Unsigned(1, 65536, true), form: None, secret: false } },
/* 690 */ T { name: "NGRAN-NumberOfActiveUEs", kind: K::Alias { field: F::Unsigned(0, 16777215, true), form: None, secret: false } },
/* 691 */ T { name: "NGRAN-RadioResourceStatus", kind: K::Sequence { names: &["dL-GBR-PRB-usage-for-MIMO", "uL-GBR-PRB-usage-for-MIMO", "dL-non-GBR-PRB-usage-for-MIMO", "uL-non-GBR-PRB-usage-for-MIMO", "dL-Total-PRB-usage-for-MIMO", "uL-Total-PRB-usage-for-MIMO", "iE-Extensions"], members: &[M { field: "dl_gbr_prb_usage_for_mimo", ty: F::Unsigned(0, 100, false), optional: false }, M { field: "ul_gbr_prb_usage_for_mimo", ty: F::Unsigned(0, 100, false), optional: false }, M { field: "dl_non_gbr_prb_usage_for_mimo", ty: F::Unsigned(0, 100, false), optional: false }, M { field: "ul_non_gbr_prb_usage_for_mimo", ty: F::Unsigned(0, 100, false), optional: false }, M { field: "dl_total_prb_usage_for_mimo", ty: F::Unsigned(0, 100, false), optional: false }, M { field: "ul_total_prb_usage_for_mimo", ty: F::Unsigned(0, 100, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 692 */ T { name: "NGRAN-ReportingStatusIEs", kind: K::Sequence { names: &["nGRAN-CellReportList", "iE-Extensions"], members: &[M { field: "ngran_cell_report_list", ty: F::Named(686), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 693 */ T { name: "NGRAN-ReportingSystemIEs", kind: K::Sequence { names: &["nGRAN-CellToReportList", "iE-Extensions"], members: &[M { field: "ngran_cell_to_report_list", ty: F::Named(688), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 694 */ T { name: "NGRAN-TNLAssociationToRemoveItem", kind: K::Sequence { names: &["tNLAssociationTransportLayerAddress", "tNLAssociationTransportLayerAddressAMF", "iE-Extensions"], members: &[M { field: "tnl_association_transport_layer_address", ty: F::Named(167), optional: false }, M { field: "tnl_association_transport_layer_address_amf", ty: F::Named(167), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: false } },
/* 695 */ T { name: "NGRAN-TNLAssociationToRemoveList", kind: K::Alias { field: F::List(&F::Named(694), 1, 32), form: None, secret: false } },
/* 696 */ T { name: "NGRANTraceID", kind: K::Alias { field: F::OctetArray(8), form: None, secret: false } },
/* 697 */ T { name: "NGRemovalFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 698 */ T { name: "NGRemovalRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 699 */ T { name: "NGRemovalResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 700 */ T { name: "NGReset", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 701 */ T { name: "NGResetAcknowledge", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 702 */ T { name: "NGSetupFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 703 */ T { name: "NGSetupRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 704 */ T { name: "NGSetupResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 705 */ T { name: "NID", kind: K::Alias { field: F::Bits(44, 44), form: None, secret: false } },
/* 706 */ T { name: "NPN-AccessInformation", kind: K::Choice { names: &["pNI-NPN-Access-Information", "choice-Extensions"], alternatives: &[F::Named(195), F::Named(948)], root: 2, extensible: false } },
/* 707 */ T { name: "NPN-MobilityInformation", kind: K::Choice { names: &["sNPN-MobilityInformation", "pNI-NPN-MobilityInformation", "choice-Extensions"], alternatives: &[F::Named(1084), F::Named(891), F::Named(948)], root: 3, extensible: false } },
/* 708 */ T { name: "NPN-PagingAssistanceInformation", kind: K::Choice { names: &["pNI-NPN-PagingAssistance", "choice-Extensions"], alternatives: &[F::Named(75), F::Named(948)], root: 2, extensible: false } },
/* 709 */ T { name: "NPN-Support", kind: K::Choice { names: &["sNPN", "choice-Extensions"], alternatives: &[F::Named(705), F::Named(948)], root: 2, extensible: false } },
/* 710 */ T { name: "NR-A2X-ServicesAuthorized", kind: K::Sequence { names: &["aerialUE", "aerialControllerUE", "iE-Extensions"], members: &[M { field: "aerial_ue", ty: F::Named(722), optional: true }, M { field: "aerial_controller_ue", ty: F::Named(721), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 711 */ T { name: "NR-CGI", kind: K::Sequence { names: &["pLMNIdentity", "nRCellIdentity", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "nr_cell_identity", ty: F::Named(724), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 712 */ T { name: "NR-CGIList", kind: K::Alias { field: F::List(&F::Named(711), 1, 16384), form: None, secret: false } },
/* 713 */ T { name: "NR-CGIListForWarning", kind: K::Alias { field: F::List(&F::Named(711), 1, 65535), form: None, secret: false } },
/* 714 */ T { name: "NR-PCI", kind: K::Alias { field: F::Unsigned(0, 1007, true), form: None, secret: false } },
/* 715 */ T { name: "NR-Paging-Long-eDRX-Cycle-for-RRC-INACTIVE", kind: K::Enumerated { names: &["hf2", "hf4", "hf8", "hf16", "hf32", "hf64", "hf128", "hf256", "hf512", "hf1024"], root: 10, extensible: true } },
/* 716 */ T { name: "NR-Paging-Long-eDRX-Information-for-RRC-INACTIVE", kind: K::Sequence { names: &["nR-paging-Long-eDRX-Cycle-for-RRC-INACTIVE", "nR-paging-Time-Window-for-RRC-INACTIVE", "iE-Extensions"], members: &[M { field: "nr_paging_long_edrx_cycle_for_rrc_inactive", ty: F::Named(715), optional: false }, M { field: "nr_paging_time_window_for_rrc_inactive", ty: F::Named(718), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 717 */ T { name: "NR-Paging-Time-Window", kind: K::Enumerated { names: &["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13", "s14", "s15", "s16", "s17", "s18", "s19", "s20", "s21", "s22", "s23", "s24", "s25", "s26", "s27", "s28", "s29", "s30", "s31", "s32"], root: 16, extensible: true } },
/* 718 */ T { name: "NR-Paging-Time-Window-for-RRC-INACTIVE", kind: K::Enumerated { names: &["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13", "s14", "s15", "s16", "s17", "s18", "s19", "s20", "s21", "s22", "s23", "s24", "s25", "s26", "s27", "s28", "s29", "s30", "s31", "s32"], root: 32, extensible: true } },
/* 719 */ T { name: "NR-Paging-eDRX-Cycle", kind: K::Enumerated { names: &["hfquarter", "hfhalf", "hf1", "hf2", "hf4", "hf8", "hf16", "hf32", "hf64", "hf128", "hf256", "hf512", "hf1024"], root: 13, extensible: true } },
/* 720 */ T { name: "NR-PagingeDRXInformation", kind: K::Sequence { names: &["nR-paging-eDRX-Cycle", "nR-paging-Time-Window", "iE-Extensions"], members: &[M { field: "nr_paging_edrx_cycle", ty: F::Named(719), optional: false }, M { field: "nr_paging_time_window", ty: F::Named(717), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 721 */ T { name: "NRA2XServicesAuthorizedAerialControllerUE", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 722 */ T { name: "NRA2XServicesAuthorizedAerialUE", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 723 */ T { name: "NRARFCN", kind: K::Alias { field: F::Unsigned(0, 3279165, false), form: None, secret: false } },
/* 724 */ T { name: "NRCellIdentity", kind: K::Alias { field: F::Bits(36, 36), form: Some(Form::Number(36)), secret: false } },
/* 725 */ T { name: "NRFrequencyBand", kind: K::Alias { field: F::Unsigned(1, 1024, true), form: None, secret: false } },
/* 726 */ T { name: "NRFrequencyBand-List", kind: K::Alias { field: F::List(&F::Named(727), 1, 32), form: None, secret: false } },
/* 727 */ T { name: "NRFrequencyBandItem", kind: K::Sequence { names: &["nr-frequency-band", "iE-Extension"], members: &[M { field: "nr_frequency_band", ty: F::Named(725), optional: false }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 728 */ T { name: "NRFrequencyInfo", kind: K::Sequence { names: &["nrARFCN", "frequencyBand-List", "iE-Extension"], members: &[M { field: "nr_arfcn", ty: F::Named(723), optional: false }, M { field: "frequency_band_list", ty: F::Named(726), optional: false }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 729 */ T { name: "NRMobilityHistoryReport", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 730 */ T { name: "NRNTNTAIInformation", kind: K::Sequence { names: &["servingPLMN", "tACListInNRNTN", "uELocationDerivedTACInNRNTN", "iE-Extensions"], members: &[M { field: "serving_plmn", ty: F::Named(886), optional: false }, M { field: "tac_list_in_nr_ntn", ty: F::Named(1153), optional: false }, M { field: "ue_location_derived_tac_in_nr_ntn", ty: F::Named(1152), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 731 */ T { name: "NRPPa-PDU", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 732 */ T { name: "NRUERLFReportContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 733 */ T { name: "NRUESidelinkAggregateMaximumBitrate", kind: K::Sequence { names: &["uESidelinkAggregateMaximumBitRate", "iE-Extensions"], members: &[M { field: "ue_sidelink_aggregate_maximum_bit_rate", ty: F::Named(124), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 734 */ T { name: "NRV2XServicesAuthorized", kind: K::Sequence { names: &["vehicleUE", "pedestrianUE", "iE-Extensions"], members: &[M { field: "vehicle_ue", ty: F::Named(1328), optional: true }, M { field: "pedestrian_ue", ty: F::Named(925), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 735 */ T { name: "NRencryptionAlgorithms", kind: K::Alias { field: F::BitString(16, 16, true), form: None, secret: false } },
/* 736 */ T { name: "NRintegrityProtectionAlgorithms", kind: K::Alias { field: F::BitString(16, 16, true), form: None, secret: false } },
/* 737 */ T { name: "NSAG-ID", kind: K::Alias { field: F::Unsigned(0, 255, true), form: None, secret: false } },
/* 738 */ T { name: "NTN-Geographical-Area-Item", kind: K::Choice { names: &["circle", "polygon", "choice-Extensions"], alternatives: &[F::Named(621), F::Named(622), F::Named(948)], root: 3, extensible: false } },
/* 739 */ T { name: "NTN-Geographical-Area-List", kind: K::Alias { field: F::List(&F::Named(738), 1, 8), form: None, secret: false } },
/* 740 */ T { name: "NetworkControlledRepeaterAuthorized", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 741 */ T { name: "NetworkInstance", kind: K::Alias { field: F::Unsigned(1, 256, true), form: None, secret: false } },
/* 742 */ T { name: "NetworkSliceAreaScopeofMDT", kind: K::Sequence { names: &["networkSliceListforMDT", "iE-Extensions"], members: &[M { field: "network_slice_listfor_mdt", ty: F::Named(744), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 743 */ T { name: "NetworkSliceItemforMDT", kind: K::Sequence { names: &["plmnID", "sliceMDTList", "iE-Extensions"], members: &[M { field: "plmn_id", ty: F::Named(886), optional: false }, M { field: "slice_mdt_list", ty: F::Named(1124), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 744 */ T { name: "NetworkSliceListforMDT", kind: K::Alias { field: F::List(&F::Named(743), 1, 16), form: None, secret: false } },
/* 745 */ T { name: "NewSecurityContextInd", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 746 */ T { name: "NextHopChainingCount", kind: K::Alias { field: F::Unsigned(0, 7, false), form: None, secret: false } },
/* 747 */ T { name: "NextPagingAreaScope", kind: K::Enumerated { names: &["same", "changed"], root: 2, extensible: true } },
/* 748 */ T { name: "NgENB-ID", kind: K::Choice { names: &["macroNgENB-ID", "shortMacroNgENB-ID", "longMacroNgENB-ID", "choice-Extensions"], alternatives: &[F::Bits(20, 20), F::Bits(18, 18), F::Bits(21, 21), F::Named(948)], root: 4, extensible: false } },
/* 749 */ T { name: "NoPDUSessionIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 750 */ T { name: "NonDynamic5QIDescriptor", kind: K::Sequence { names: &["fiveQI", "priorityLevelQos", "averagingWindow", "maximumDataBurstVolume", "iE-Extensions"], members: &[M { field: "five_qi", ty: F::Named(404), optional: false }, M { field: "priority_level_qos", ty: F::Named(937), optional: true }, M { field: "averaging_window", ty: F::Named(118), optional: true }, M { field: "maximum_data_burst_volume", ty: F::Named(637), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 751 */ T { name: "NotAllowedTACs", kind: K::Alias { field: F::List(&F::Named(1152), 1, 16), form: None, secret: false } },
/* 752 */ T { name: "NotificationCause", kind: K::Enumerated { names: &["fulfilled", "not-fulfilled", "not-fulfilled-DL", "not-fulfilled-UL"], root: 2, extensible: true } },
/* 753 */ T { name: "NotificationCell-Item", kind: K::Sequence { names: &["nGRAN-CGI", "notifyFlag", "iE-Extensions"], members: &[M { field: "ngran_cgi", ty: F::Named(684), optional: false }, M { field: "notify_flag", ty: F::Named(754), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 754 */ T { name: "NotificationCellItemNotifyFlag", kind: K::Enumerated { names: &["activated", "deactivated"], root: 2, extensible: true } },
/* 755 */ T { name: "NotificationCellList", kind: K::Alias { field: F::List(&F::Named(753), 1, 16384), form: None, secret: false } },
/* 756 */ T { name: "NotificationControl", kind: K::Enumerated { names: &["notification-requested"], root: 1, extensible: true } },
/* 757 */ T { name: "NotifySourceNGRANNode", kind: K::Enumerated { names: &["notifySource"], root: 1, extensible: true } },
/* 758 */ T { name: "NumberOfBroadcasts", kind: K::Alias { field: F::Unsigned(0, 65535, false), form: None, secret: false } },
/* 759 */ T { name: "NumberOfBroadcastsRequested", kind: K::Alias { field: F::Unsigned(0, 65535, false), form: None, secret: false } },
/* 760 */ T { name: "NumberOfMeasurementReportingLevels", kind: K::Enumerated { names: &["n2", "n3", "n4", "n5", "n10", "n0"], root: 5, extensible: true } },
/* 761 */ T { name: "OCTET STRING", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 762 */ T { name: "OnboardingSupport", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 763 */ T { name: "OverloadAction", kind: K::Enumerated { names: &["reject-non-emergency-mo-dt", "reject-rrc-cr-signalling", "permit-emergency-sessions-and-mobile-terminated-services-only", "permit-high-priority-sessions-and-mobile-terminated-services-only"], root: 4, extensible: true } },
/* 764 */ T { name: "OverloadResponse", kind: K::Choice { names: &["overloadAction", "choice-Extensions"], alternatives: &[F::Named(763), F::Named(948)], root: 2, extensible: false } },
/* 765 */ T { name: "OverloadStart", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 766 */ T { name: "OverloadStartNSSAIItem", kind: K::Sequence { names: &["sliceOverloadList", "sliceOverloadResponse", "sliceTrafficLoadReductionIndication", "iE-Extensions"], members: &[M { field: "slice_overload_list", ty: F::Named(1126), optional: false }, M { field: "slice_overload_response", ty: F::Named(764), optional: true }, M { field: "slice_traffic_load_reduction_indication", ty: F::Named(1229), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 767 */ T { name: "OverloadStartNSSAIList", kind: K::Alias { field: F::List(&F::Named(766), 1, 1024), form: None, secret: false } },
/* 768 */ T { name: "OverloadStop", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 769 */ T { name: "PC5FlowBitRates", kind: K::Sequence { names: &["guaranteedFlowBitRate", "maximumFlowBitRate", "iE-Extensions"], members: &[M { field: "guaranteed_flow_bit_rate", ty: F::Named(124), optional: false }, M { field: "maximum_flow_bit_rate", ty: F::Named(124), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 770 */ T { name: "PC5QoSFlowItem", kind: K::Sequence { names: &["pQI", "pc5FlowBitRates", "range", "iE-Extensions"], members: &[M { field: "pqi", ty: F::Named(404), optional: false }, M { field: "pc5_flow_bit_rates", ty: F::Named(769), optional: true }, M { field: "range", ty: F::Named(1041), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 771 */ T { name: "PC5QoSFlowList", kind: K::Alias { field: F::List(&F::Named(770), 1, 2048), form: None, secret: false } },
/* 772 */ T { name: "PC5QoSParameters", kind: K::Sequence { names: &["pc5QoSFlowList", "pc5LinkAggregateBitRates", "iE-Extensions"], members: &[M { field: "pc5_qos_flow_list", ty: F::Named(771), optional: false }, M { field: "pc5_link_aggregate_bit_rates", ty: F::Named(124), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 773 */ T { name: "PCIListForMDT", kind: K::Alias { field: F::List(&F::Named(714), 1, 32), form: None, secret: false } },
/* 774 */ T { name: "PDUSessionAggregateMaximumBitRate", kind: K::Sequence { names: &["pDUSessionAggregateMaximumBitRateDL", "pDUSessionAggregateMaximumBitRateUL", "iE-Extensions"], members: &[M { field: "pdu_session_aggregate_maximum_bit_rate_dl", ty: F::Named(124), optional: false }, M { field: "pdu_session_aggregate_maximum_bit_rate_ul", ty: F::Named(124), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 775 */ T { name: "PDUSessionForPagingItem", kind: K::Sequence { names: &["pDUSessionID", "pagingPolicyDifferentiationList", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "paging_policy_differentiation_list", ty: F::Named(911), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 776 */ T { name: "PDUSessionForPagingList", kind: K::Alias { field: F::List(&F::Named(775), 1, 256), form: None, secret: false } },
/* 777 */ T { name: "PDUSessionID", kind: K::Alias { field: F::Unsigned(0, 255, false), form: None, secret: false } },
/* 778 */ T { name: "PDUSessionListMTCommHReq", kind: K::Alias { field: F::List(&F::Named(779), 1, 256), form: None, secret: false } },
/* 779 */ T { name: "PDUSessionListMTCommHReq-Item", kind: K::Sequence { names: &["pDUSessionID", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 780 */ T { name: "PDUSessionPairID", kind: K::Alias { field: F::Unsigned(0, 255, true), form: None, secret: false } },
/* 781 */ T { name: "PDUSessionResourceAdmittedItem", kind: K::Sequence { names: &["pDUSessionID", "handoverRequestAcknowledgeTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "handover_request_acknowledge_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 782 */ T { name: "PDUSessionResourceAdmittedList", kind: K::Alias { field: F::List(&F::Named(781), 1, 256), form: None, secret: false } },
/* 783 */ T { name: "PDUSessionResourceFailedToModifyItemModCfm", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionResourceModifyIndicationUnsuccessfulTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_resource_modify_indication_unsuccessful_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 784 */ T { name: "PDUSessionResourceFailedToModifyItemModRes", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionResourceModifyUnsuccessfulTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_resource_modify_unsuccessful_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 785 */ T { name: "PDUSessionResourceFailedToModifyListModCfm", kind: K::Alias { field: F::List(&F::Named(783), 1, 256), form: None, secret: false } },
/* 786 */ T { name: "PDUSessionResourceFailedToModifyListModRes", kind: K::Alias { field: F::List(&F::Named(784), 1, 256), form: None, secret: false } },
/* 787 */ T { name: "PDUSessionResourceFailedToResumeItemRESReq", kind: K::Sequence { names: &["pDUSessionID", "cause", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "cause", ty: F::Named(185), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 788 */ T { name: "PDUSessionResourceFailedToResumeItemRESRes", kind: K::Sequence { names: &["pDUSessionID", "cause", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "cause", ty: F::Named(185), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 789 */ T { name: "PDUSessionResourceFailedToResumeListRESReq", kind: K::Alias { field: F::List(&F::Named(787), 1, 256), form: None, secret: false } },
/* 790 */ T { name: "PDUSessionResourceFailedToResumeListRESRes", kind: K::Alias { field: F::List(&F::Named(788), 1, 256), form: None, secret: false } },
/* 791 */ T { name: "PDUSessionResourceFailedToSetupItemCxtFail", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionResourceSetupUnsuccessfulTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_resource_setup_unsuccessful_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 792 */ T { name: "PDUSessionResourceFailedToSetupItemCxtRes", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionResourceSetupUnsuccessfulTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_resource_setup_unsuccessful_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 793 */ T { name: "PDUSessionResourceFailedToSetupItemHOAck", kind: K::Sequence { names: &["pDUSessionID", "handoverResourceAllocationUnsuccessfulTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "handover_resource_allocation_unsuccessful_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 794 */ T { name: "PDUSessionResourceFailedToSetupItemPSReq", kind: K::Sequence { names: &["pDUSessionID", "pathSwitchRequestSetupFailedTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "path_switch_request_setup_failed_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 795 */ T { name: "PDUSessionResourceFailedToSetupItemSURes", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionResourceSetupUnsuccessfulTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_resource_setup_unsuccessful_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 796 */ T { name: "PDUSessionResourceFailedToSetupListCxtFail", kind: K::Alias { field: F::List(&F::Named(791), 1, 256), form: None, secret: false } },
/* 797 */ T { name: "PDUSessionResourceFailedToSetupListCxtRes", kind: K::Alias { field: F::List(&F::Named(792), 1, 256), form: None, secret: false } },
/* 798 */ T { name: "PDUSessionResourceFailedToSetupListHOAck", kind: K::Alias { field: F::List(&F::Named(793), 1, 256), form: None, secret: false } },
/* 799 */ T { name: "PDUSessionResourceFailedToSetupListPSReq", kind: K::Alias { field: F::List(&F::Named(794), 1, 256), form: None, secret: false } },
/* 800 */ T { name: "PDUSessionResourceFailedToSetupListSURes", kind: K::Alias { field: F::List(&F::Named(795), 1, 256), form: None, secret: false } },
/* 801 */ T { name: "PDUSessionResourceHandoverItem", kind: K::Sequence { names: &["pDUSessionID", "handoverCommandTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "handover_command_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 802 */ T { name: "PDUSessionResourceHandoverList", kind: K::Alias { field: F::List(&F::Named(801), 1, 256), form: None, secret: false } },
/* 803 */ T { name: "PDUSessionResourceInformationItem", kind: K::Sequence { names: &["pDUSessionID", "qosFlowInformationList", "dRBsToQosFlowsMappingList", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "qos_flow_information_list", ty: F::Named(978), optional: false }, M { field: "drbs_to_qos_flows_mapping_list", ty: F::Named(277), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 804 */ T { name: "PDUSessionResourceInformationList", kind: K::Alias { field: F::List(&F::Named(803), 1, 256), form: None, secret: false } },
/* 805 */ T { name: "PDUSessionResourceItemCxtRelCpl", kind: K::Sequence { names: &["pDUSessionID", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 806 */ T { name: "PDUSessionResourceItemCxtRelReq", kind: K::Sequence { names: &["pDUSessionID", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 807 */ T { name: "PDUSessionResourceItemHORqd", kind: K::Sequence { names: &["pDUSessionID", "handoverRequiredTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "handover_required_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 808 */ T { name: "PDUSessionResourceListCxtRelCpl", kind: K::Alias { field: F::List(&F::Named(805), 1, 256), form: None, secret: false } },
/* 809 */ T { name: "PDUSessionResourceListCxtRelReq", kind: K::Alias { field: F::List(&F::Named(806), 1, 256), form: None, secret: false } },
/* 810 */ T { name: "PDUSessionResourceListHORqd", kind: K::Alias { field: F::List(&F::Named(807), 1, 256), form: None, secret: false } },
/* 811 */ T { name: "PDUSessionResourceModifyConfirm", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 812 */ T { name: "PDUSessionResourceModifyConfirmTransfer", kind: K::Sequence { names: &["qosFlowModifyConfirmList", "uLNGU-UP-TNLInformation", "additionalNG-UUPTNLInformation", "qosFlowFailedToModifyList", "iE-Extensions"], members: &[M { field: "qos_flow_modify_confirm_list", ty: F::Named(984), optional: false }, M { field: "ul_ngu_up_tnl_information", ty: F::Named(1298), optional: false }, M { field: "additional_ng_u_up_tnl_information", ty: F::Named(1302), optional: true }, M { field: "qos_flow_failed_to_modify_list", ty: F::Named(981), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 813 */ T { name: "PDUSessionResourceModifyIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 814 */ T { name: "PDUSessionResourceModifyIndicationTransfer", kind: K::Sequence { names: &["dLQosFlowPerTNLInformation", "additionalDLQosFlowPerTNLInformation", "iE-Extensions"], members: &[M { field: "dl_qos_flow_per_tnl_information", ty: F::Named(989), optional: false }, M { field: "additional_dl_qos_flow_per_tnl_information", ty: F::Named(991), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 815 */ T { name: "PDUSessionResourceModifyIndicationUnsuccessfulTransfer", kind: K::Sequence { names: &["cause", "iE-Extensions"], members: &[M { field: "cause", ty: F::Named(185), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 816 */ T { name: "PDUSessionResourceModifyItemModCfm", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionResourceModifyConfirmTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_resource_modify_confirm_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 817 */ T { name: "PDUSessionResourceModifyItemModInd", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionResourceModifyIndicationTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_resource_modify_indication_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 818 */ T { name: "PDUSessionResourceModifyItemModReq", kind: K::Sequence { names: &["pDUSessionID", "nAS-PDU", "pDUSessionResourceModifyRequestTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "nas_pdu", ty: F::Named(668), optional: true }, M { field: "pdu_session_resource_modify_request_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 819 */ T { name: "PDUSessionResourceModifyItemModRes", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionResourceModifyResponseTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_resource_modify_response_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 820 */ T { name: "PDUSessionResourceModifyListModCfm", kind: K::Alias { field: F::List(&F::Named(816), 1, 256), form: None, secret: false } },
/* 821 */ T { name: "PDUSessionResourceModifyListModInd", kind: K::Alias { field: F::List(&F::Named(817), 1, 256), form: None, secret: false } },
/* 822 */ T { name: "PDUSessionResourceModifyListModReq", kind: K::Alias { field: F::List(&F::Named(818), 1, 256), form: None, secret: false } },
/* 823 */ T { name: "PDUSessionResourceModifyListModRes", kind: K::Alias { field: F::List(&F::Named(819), 1, 256), form: None, secret: false } },
/* 824 */ T { name: "PDUSessionResourceModifyRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 825 */ T { name: "PDUSessionResourceModifyRequestTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 826 */ T { name: "PDUSessionResourceModifyResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 827 */ T { name: "PDUSessionResourceModifyResponseTransfer", kind: K::Sequence { names: &["dL-NGU-UP-TNLInformation", "uL-NGU-UP-TNLInformation", "qosFlowAddOrModifyResponseList", "additionalDLQosFlowPerTNLInformation", "qosFlowFailedToAddOrModifyList", "iE-Extensions"], members: &[M { field: "dl_ngu_up_tnl_information", ty: F::Named(1298), optional: true }, M { field: "ul_ngu_up_tnl_information", ty: F::Named(1298), optional: true }, M { field: "qos_flow_add_or_modify_response_list", ty: F::Named(969), optional: true }, M { field: "additional_dl_qos_flow_per_tnl_information", ty: F::Named(991), optional: true }, M { field: "qos_flow_failed_to_add_or_modify_list", ty: F::Named(981), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 828 */ T { name: "PDUSessionResourceModifyUnsuccessfulTransfer", kind: K::Sequence { names: &["cause", "criticalityDiagnostics", "iE-Extensions"], members: &[M { field: "cause", ty: F::Named(185), optional: false }, M { field: "criticality_diagnostics", ty: F::Named(246), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 829 */ T { name: "PDUSessionResourceNotify", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 830 */ T { name: "PDUSessionResourceNotifyItem", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionResourceNotifyTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_resource_notify_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 831 */ T { name: "PDUSessionResourceNotifyList", kind: K::Alias { field: F::List(&F::Named(830), 1, 256), form: None, secret: false } },
/* 832 */ T { name: "PDUSessionResourceNotifyReleasedTransfer", kind: K::Sequence { names: &["cause", "iE-Extensions"], members: &[M { field: "cause", ty: F::Named(185), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 833 */ T { name: "PDUSessionResourceNotifyTransfer", kind: K::Sequence { names: &["qosFlowNotifyList", "qosFlowReleasedList", "iE-Extensions"], members: &[M { field: "qos_flow_notify_list", ty: F::Named(986), optional: true }, M { field: "qos_flow_released_list", ty: F::Named(981), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 834 */ T { name: "PDUSessionResourceReleaseCommand", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 835 */ T { name: "PDUSessionResourceReleaseCommandTransfer", kind: K::Sequence { names: &["cause", "iE-Extensions"], members: &[M { field: "cause", ty: F::Named(185), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 836 */ T { name: "PDUSessionResourceReleaseResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 837 */ T { name: "PDUSessionResourceReleaseResponseTransfer", kind: K::Sequence { names: &["iE-Extensions"], members: &[M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 838 */ T { name: "PDUSessionResourceReleasedItemNot", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionResourceNotifyReleasedTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_resource_notify_released_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 839 */ T { name: "PDUSessionResourceReleasedItemPSAck", kind: K::Sequence { names: &["pDUSessionID", "pathSwitchRequestUnsuccessfulTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "path_switch_request_unsuccessful_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 840 */ T { name: "PDUSessionResourceReleasedItemPSFail", kind: K::Sequence { names: &["pDUSessionID", "pathSwitchRequestUnsuccessfulTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "path_switch_request_unsuccessful_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 841 */ T { name: "PDUSessionResourceReleasedItemRelRes", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionResourceReleaseResponseTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_resource_release_response_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 842 */ T { name: "PDUSessionResourceReleasedListNot", kind: K::Alias { field: F::List(&F::Named(838), 1, 256), form: None, secret: false } },
/* 843 */ T { name: "PDUSessionResourceReleasedListPSAck", kind: K::Alias { field: F::List(&F::Named(839), 1, 256), form: None, secret: false } },
/* 844 */ T { name: "PDUSessionResourceReleasedListPSFail", kind: K::Alias { field: F::List(&F::Named(840), 1, 256), form: None, secret: false } },
/* 845 */ T { name: "PDUSessionResourceReleasedListRelRes", kind: K::Alias { field: F::List(&F::Named(841), 1, 256), form: None, secret: false } },
/* 846 */ T { name: "PDUSessionResourceResumeItemRESReq", kind: K::Sequence { names: &["pDUSessionID", "uEContextResumeRequestTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "ue_context_resume_request_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 847 */ T { name: "PDUSessionResourceResumeItemRESRes", kind: K::Sequence { names: &["pDUSessionID", "uEContextResumeResponseTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "ue_context_resume_response_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 848 */ T { name: "PDUSessionResourceResumeListRESReq", kind: K::Alias { field: F::List(&F::Named(846), 1, 256), form: None, secret: false } },
/* 849 */ T { name: "PDUSessionResourceResumeListRESRes", kind: K::Alias { field: F::List(&F::Named(847), 1, 256), form: None, secret: false } },
/* 850 */ T { name: "PDUSessionResourceSecondaryRATUsageItem", kind: K::Sequence { names: &["pDUSessionID", "secondaryRATDataUsageReportTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "secondary_rat_data_usage_report_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 851 */ T { name: "PDUSessionResourceSecondaryRATUsageList", kind: K::Alias { field: F::List(&F::Named(850), 1, 256), form: None, secret: false } },
/* 852 */ T { name: "PDUSessionResourceSetupItemCxtReq", kind: K::Sequence { names: &["pDUSessionID", "nAS-PDU", "s-NSSAI", "pDUSessionResourceSetupRequestTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "nas_pdu", ty: F::Named(668), optional: true }, M { field: "s_nssai", ty: F::Named(1074), optional: false }, M { field: "pdu_session_resource_setup_request_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 853 */ T { name: "PDUSessionResourceSetupItemCxtRes", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionResourceSetupResponseTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_resource_setup_response_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 854 */ T { name: "PDUSessionResourceSetupItemHOReq", kind: K::Sequence { names: &["pDUSessionID", "s-NSSAI", "handoverRequestTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "s_nssai", ty: F::Named(1074), optional: false }, M { field: "handover_request_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 855 */ T { name: "PDUSessionResourceSetupItemSUReq", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionNAS-PDU", "s-NSSAI", "pDUSessionResourceSetupRequestTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_nas_pdu", ty: F::Named(668), optional: true }, M { field: "s_nssai", ty: F::Named(1074), optional: false }, M { field: "pdu_session_resource_setup_request_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 856 */ T { name: "PDUSessionResourceSetupItemSURes", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionResourceSetupResponseTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_resource_setup_response_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 857 */ T { name: "PDUSessionResourceSetupListCxtReq", kind: K::Alias { field: F::List(&F::Named(852), 1, 256), form: None, secret: false } },
/* 858 */ T { name: "PDUSessionResourceSetupListCxtRes", kind: K::Alias { field: F::List(&F::Named(853), 1, 256), form: None, secret: false } },
/* 859 */ T { name: "PDUSessionResourceSetupListHOReq", kind: K::Alias { field: F::List(&F::Named(854), 1, 256), form: None, secret: false } },
/* 860 */ T { name: "PDUSessionResourceSetupListSUReq", kind: K::Alias { field: F::List(&F::Named(855), 1, 256), form: None, secret: false } },
/* 861 */ T { name: "PDUSessionResourceSetupListSURes", kind: K::Alias { field: F::List(&F::Named(856), 1, 256), form: None, secret: false } },
/* 862 */ T { name: "PDUSessionResourceSetupRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 863 */ T { name: "PDUSessionResourceSetupRequestTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 864 */ T { name: "PDUSessionResourceSetupResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 865 */ T { name: "PDUSessionResourceSetupResponseTransfer", kind: K::Sequence { names: &["dLQosFlowPerTNLInformation", "additionalDLQosFlowPerTNLInformation", "securityResult", "qosFlowFailedToSetupList", "iE-Extensions"], members: &[M { field: "dl_qos_flow_per_tnl_information", ty: F::Named(989), optional: false }, M { field: "additional_dl_qos_flow_per_tnl_information", ty: F::Named(991), optional: true }, M { field: "security_result", ty: F::Named(1106), optional: true }, M { field: "qos_flow_failed_to_setup_list", ty: F::Named(981), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 866 */ T { name: "PDUSessionResourceSetupUnsuccessfulTransfer", kind: K::Sequence { names: &["cause", "criticalityDiagnostics", "iE-Extensions"], members: &[M { field: "cause", ty: F::Named(185), optional: false }, M { field: "criticality_diagnostics", ty: F::Named(246), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 867 */ T { name: "PDUSessionResourceSuspendItemSUSReq", kind: K::Sequence { names: &["pDUSessionID", "uEContextSuspendRequestTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "ue_context_suspend_request_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 868 */ T { name: "PDUSessionResourceSuspendListSUSReq", kind: K::Alias { field: F::List(&F::Named(867), 1, 256), form: None, secret: false } },
/* 869 */ T { name: "PDUSessionResourceSwitchedItem", kind: K::Sequence { names: &["pDUSessionID", "pathSwitchRequestAcknowledgeTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "path_switch_request_acknowledge_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 870 */ T { name: "PDUSessionResourceSwitchedList", kind: K::Alias { field: F::List(&F::Named(869), 1, 256), form: None, secret: false } },
/* 871 */ T { name: "PDUSessionResourceToBeSwitchedDLItem", kind: K::Sequence { names: &["pDUSessionID", "pathSwitchRequestTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "path_switch_request_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 872 */ T { name: "PDUSessionResourceToBeSwitchedDLList", kind: K::Alias { field: F::List(&F::Named(871), 1, 256), form: None, secret: false } },
/* 873 */ T { name: "PDUSessionResourceToReleaseItemHOCmd", kind: K::Sequence { names: &["pDUSessionID", "handoverPreparationUnsuccessfulTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "handover_preparation_unsuccessful_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 874 */ T { name: "PDUSessionResourceToReleaseItemRelCmd", kind: K::Sequence { names: &["pDUSessionID", "pDUSessionResourceReleaseCommandTransfer", "iE-Extensions"], members: &[M { field: "pdu_session_id", ty: F::Named(777), optional: false }, M { field: "pdu_session_resource_release_command_transfer", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 875 */ T { name: "PDUSessionResourceToReleaseListHOCmd", kind: K::Alias { field: F::List(&F::Named(873), 1, 256), form: None, secret: false } },
/* 876 */ T { name: "PDUSessionResourceToReleaseListRelCmd", kind: K::Alias { field: F::List(&F::Named(874), 1, 256), form: None, secret: false } },
/* 877 */ T { name: "PDUSessionType", kind: K::Enumerated { names: &["ipv4", "ipv6", "ipv4v6", "ethernet", "unstructured"], root: 5, extensible: true } },
/* 878 */ T { name: "PDUSessionUsageReport", kind: K::Sequence { names: &["rATType", "pDUSessionTimedReportList", "iE-Extensions"], members: &[M { field: "rat_type", ty: F::Named(879), optional: false }, M { field: "pdu_session_timed_report_list", ty: F::Named(1330), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 879 */ T { name: "PDUSessionUsageReportRATType", kind: K::Enumerated { names: &["nr", "eutra", "nr-unlicensed", "e-utra-unlicensed"], root: 2, extensible: true } },
/* 880 */ T { name: "PDUSetbasedHandlingIndicator", kind: K::Enumerated { names: &["supported"], root: 1, extensible: true } },
/* 881 */ T { name: "PDUsetQoSInformation", kind: K::Sequence { names: &["pduSetDelayBudget", "pduSetErrorRate", "pduSetIntegratedHandlingInformation", "iE-Extensions"], members: &[M { field: "pdu_set_delay_budget", ty: F::Named(376), optional: true }, M { field: "pdu_set_error_rate", ty: F::Named(899), optional: true }, M { field: "pdu_set_integrated_handling_information", ty: F::Named(882), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 882 */ T { name: "PDUsetQoSInformationPduSetIntegratedHandlingInformation", kind: K::Enumerated { names: &["true", "false"], root: 2, extensible: true } },
/* 883 */ T { name: "PDUsetQoSParameters", kind: K::Sequence { names: &["ulPDUSetQoSInformation", "dlPDUSetQoSInformation", "iE-Extensions"], members: &[M { field: "ul_pdu_set_qos_information", ty: F::Named(881), optional: true }, M { field: "dl_pdu_set_qos_information", ty: F::Named(881), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 884 */ T { name: "PEIPSassistanceInformation", kind: K::Sequence { names: &["cNsubgroupID", "iE-Extensions"], members: &[M { field: "cn_subgroup_id", ty: F::Named(164), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 885 */ T { name: "PLMNAreaBasedQMC", kind: K::Sequence { names: &["plmnListforQMC", "iE-Extensions"], members: &[M { field: "plmn_listfor_qmc", ty: F::Named(887), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 886 */ T { name: "PLMNIdentity", kind: K::Alias { field: F::OctetArray(3), form: Some(Form::Plmn), secret: false } },
/* 887 */ T { name: "PLMNListforQMC", kind: K::Alias { field: F::List(&F::Named(886), 1, 16), form: None, secret: false } },
/* 888 */ T { name: "PLMNSupportItem", kind: K::Sequence { names: &["pLMNIdentity", "sliceSupportList", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "slice_support_list", ty: F::Named(1128), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 889 */ T { name: "PLMNSupportList", kind: K::Alias { field: F::List(&F::Named(888), 1, 12), form: None, secret: false } },
/* 890 */ T { name: "PNI-NPN-AreaScopeofMDT", kind: K::Sequence { names: &["cAGListforMDT", "iE-Extensions"], members: &[M { field: "cag_listfor_mdt", ty: F::Named(154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 891 */ T { name: "PNI-NPN-MobilityInformation", kind: K::Sequence { names: &["allowed-PNI-NPI-List", "iE-Extensions"], members: &[M { field: "allowed_pni_npi_list", ty: F::Named(75), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 892 */ T { name: "PNI-NPNBasedMDT", kind: K::Sequence { names: &["cAGListforMDT", "iE-Extensions"], members: &[M { field: "cag_listfor_mdt", ty: F::Named(154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 893 */ T { name: "PWSCancelRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 894 */ T { name: "PWSCancelResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 895 */ T { name: "PWSFailedCellIDList", kind: K::Choice { names: &["eUTRA-CGI-PWSFailedList", "nR-CGI-PWSFailedList", "choice-Extensions"], alternatives: &[F::Named(316), F::Named(712), F::Named(948)], root: 3, extensible: false } },
/* 896 */ T { name: "PWSFailureIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 897 */ T { name: "PWSRestartIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 898 */ T { name: "PacketDelayBudget", kind: K::Alias { field: F::Unsigned(0, 1023, true), form: None, secret: false } },
/* 899 */ T { name: "PacketErrorRate", kind: K::Sequence { names: &["pERScalar", "pERExponent", "iE-Extensions"], members: &[M { field: "per_scalar", ty: F::Unsigned(0, 9, true), optional: false }, M { field: "per_exponent", ty: F::Unsigned(0, 9, true), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 900 */ T { name: "PacketLossRate", kind: K::Alias { field: F::Unsigned(0, 1000, true), form: None, secret: false } },
/* 901 */ T { name: "Paging", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 902 */ T { name: "PagingAssisDataforCEcapabUE", kind: K::Sequence { names: &["eUTRA-CGI", "coverageEnhancementLevel", "iE-Extensions"], members: &[M { field: "eutra_cgi", ty: F::Named(315), optional: false }, M { field: "coverage_enhancement_level", ty: F::Named(244), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 903 */ T { name: "PagingAttemptCount", kind: K::Alias { field: F::Unsigned(1, 16, true), form: None, secret: false } },
/* 904 */ T { name: "PagingAttemptInformation", kind: K::Sequence { names: &["pagingAttemptCount", "intendedNumberOfPagingAttempts", "nextPagingAreaScope", "iE-Extensions"], members: &[M { field: "paging_attempt_count", ty: F::Named(903), optional: false }, M { field: "intended_number_of_paging_attempts", ty: F::Named(478), optional: false }, M { field: "next_paging_area_scope", ty: F::Named(747), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 905 */ T { name: "PagingCause", kind: K::Enumerated { names: &["voice"], root: 1, extensible: true } },
/* 906 */ T { name: "PagingCauseIndicationForVoiceService", kind: K::Enumerated { names: &["supported"], root: 1, extensible: true } },
/* 907 */ T { name: "PagingDRX", kind: K::Enumerated { names: &["v32", "v64", "v128", "v256"], root: 4, extensible: true } },
/* 908 */ T { name: "PagingOrigin", kind: K::Enumerated { names: &["non-3gpp"], root: 1, extensible: true } },
/* 909 */ T { name: "PagingPolicyDifferentiation", kind: K::Sequence { names: &["pDUSessionForPagingList", "iE-Extensions"], members: &[M { field: "pdu_session_for_paging_list", ty: F::Named(776), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 910 */ T { name: "PagingPolicyDifferentiationItem", kind: K::Sequence { names: &["qosFlowIdentifier", "pagingPolicyIndicator", "allocationAndRetentionPriority", "fiveQI", "dl-DataSize", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: true }, M { field: "paging_policy_indicator", ty: F::Named(912), optional: true }, M { field: "allocation_and_retention_priority", ty: F::Named(72), optional: true }, M { field: "five_qi", ty: F::Named(404), optional: true }, M { field: "dl_data_size", ty: F::Unsigned(0, 96000, true), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 911 */ T { name: "PagingPolicyDifferentiationList", kind: K::Alias { field: F::List(&F::Named(910), 1, 64), form: None, secret: false } },
/* 912 */ T { name: "PagingPolicyIndicator", kind: K::Alias { field: F::Unsigned(0, 7, true), form: None, secret: false } },
/* 913 */ T { name: "PagingPriority", kind: K::Enumerated { names: &["priolevel1", "priolevel2", "priolevel3", "priolevel4", "priolevel5", "priolevel6", "priolevel7", "priolevel8"], root: 8, extensible: true } },
/* 914 */ T { name: "PagingProbabilityInformation", kind: K::Enumerated { names: &["p00", "p05", "p10", "p15", "p20", "p25", "p30", "p35", "p40", "p45", "p50", "p55", "p60", "p65", "p70", "p75", "p80", "p85", "p90", "p95", "p100"], root: 21, extensible: true } },
/* 915 */ T { name: "ParentTImeSource", kind: K::Enumerated { names: &["synce", "ptp", "gnss", "atomicclock", "terrestrialradio", "serialtimecode", "ntp", "handset", "other"], root: 9, extensible: true } },
/* 916 */ T { name: "Partially-Allowed-NSSAI", kind: K::Alias { field: F::List(&F::Named(917), 1, 8), form: None, secret: false } },
/* 917 */ T { name: "PartiallyAllowedNSSAI-Item", kind: K::Sequence { names: &["s-NSSAI", "iE-Extensions"], members: &[M { field: "s_nssai", ty: F::Named(1074), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 918 */ T { name: "PathSwitchRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 919 */ T { name: "PathSwitchRequestAcknowledge", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 920 */ T { name: "PathSwitchRequestAcknowledgeTransfer", kind: K::Sequence { names: &["uL-NGU-UP-TNLInformation", "securityIndication", "iE-Extensions"], members: &[M { field: "ul_ngu_up_tnl_information", ty: F::Named(1298), optional: true }, M { field: "security_indication", ty: F::Named(1104), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 921 */ T { name: "PathSwitchRequestFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 922 */ T { name: "PathSwitchRequestSetupFailedTransfer", kind: K::Sequence { names: &["cause", "iE-Extensions"], members: &[M { field: "cause", ty: F::Named(185), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 923 */ T { name: "PathSwitchRequestTransfer", kind: K::Sequence { names: &["dL-NGU-UP-TNLInformation", "dL-NGU-TNLInformationReused", "userPlaneSecurityInformation", "qosFlowAcceptedList", "iE-Extensions"], members: &[M { field: "dl_ngu_up_tnl_information", ty: F::Named(1298), optional: false }, M { field: "dl_ngu_tnl_information_reused", ty: F::Named(257), optional: true }, M { field: "user_plane_security_information", ty: F::Named(1327), optional: true }, M { field: "qos_flow_accepted_list", ty: F::Named(965), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 924 */ T { name: "PathSwitchRequestUnsuccessfulTransfer", kind: K::Sequence { names: &["cause", "iE-Extensions"], members: &[M { field: "cause", ty: F::Named(185), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 925 */ T { name: "PedestrianUE", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 926 */ T { name: "PeriodicRegistrationUpdateTimer", kind: K::Alias { field: F::Bits(8, 8), form: None, secret: false } },
/* 927 */ T { name: "PeriodicReportingIEs", kind: K::Sequence { names: &["reportingPeriodicity", "iE-Extensions"], members: &[M { field: "reporting_periodicity", ty: F::Named(1062), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 928 */ T { name: "Periodicity", kind: K::Alias { field: F::Unsigned(0, 640000, true), form: None, secret: false } },
/* 929 */ T { name: "PeriodicityBound", kind: K::Sequence { names: &["periodicityLowerBound", "periodicityUpperBound", "iE-Extensions"], members: &[M { field: "periodicity_lower_bound", ty: F::Named(928), optional: false }, M { field: "periodicity_upper_bound", ty: F::Named(928), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 930 */ T { name: "PeriodicityList", kind: K::Sequence { names: &["allowedPeriodicityList", "iE-Extensions"], members: &[M { field: "allowed_periodicity_list", ty: F::Named(79), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 931 */ T { name: "PeriodicityRange", kind: K::Choice { names: &["periodicityBound", "periodicityList", "choice-extensions"], alternatives: &[F::Named(929), F::Named(79), F::Named(948)], root: 3, extensible: false } },
/* 932 */ T { name: "PortNumber", kind: K::Alias { field: F::OctetArray(2), form: Some(Form::Number(16)), secret: false } },
/* 933 */ T { name: "Pre-emptionCapability", kind: K::Enumerated { names: &["shall-not-trigger-pre-emption", "may-trigger-pre-emption"], root: 2, extensible: true } },
/* 934 */ T { name: "Pre-emptionVulnerability", kind: K::Enumerated { names: &["not-pre-emptable", "pre-emptable"], root: 2, extensible: true } },
/* 935 */ T { name: "Presence", kind: K::Enumerated { names: &["optional", "conditional", "mandatory"], root: 3, extensible: false } },
/* 936 */ T { name: "PriorityLevelARP", kind: K::Alias { field: F::Unsigned(1, 15, false), form: None, secret: false } },
/* 937 */ T { name: "PriorityLevelQos", kind: K::Alias { field: F::Unsigned(1, 127, true), form: None, secret: false } },
/* 938 */ T { name: "PrivacyIndicator", kind: K::Enumerated { names: &["immediate-MDT", "logged-MDT"], root: 2, extensible: true } },
/* 939 */ T { name: "PrivateIE-ID", kind: K::Choice { names: &["local", "global"], alternatives: &[F::Unsigned(0, 65535, false), F::ObjectIdentifier], root: 2, extensible: false } },
/* 940 */ T { name: "PrivateMessage", kind: K::Sequence { names: &["privateIEs"], members: &[M { field: "private_ies", ty: F::Named(941), optional: false }], extensible: true } },
/* 941 */ T { name: "PrivateMessagePrivateIEs", kind: K::Alias { field: F::List(&F::Named(1078), 1, 65535), form: None, secret: false } },
/* 942 */ T { name: "ProcedureCode", kind: K::Alias { field: F::Unsigned(0, 255, false), form: None, secret: false } },
/* 943 */ T { name: "ProcedureStageChoice", kind: K::Choice { names: &["first-dl-count", "choice-Extensions"], alternatives: &[F::Named(383), F::Named(948)], root: 2, extensible: false } },
/* 944 */ T { name: "ProtocolExtensionContainer", kind: K::Alias { field: F::List(&F::Named(945), 1, 65535), form: None, secret: false } },
/* 945 */ T { name: "ProtocolExtensionField", kind: K::Sequence { names: &["id", "criticality", "extensionValue"], members: &[M { field: "id", ty: F::Named(946), optional: false }, M { field: "criticality", ty: F::Named(245), optional: false }, M { field: "extension_value", ty: F::Open, optional: false }], extensible: false } },
/* 946 */ T { name: "ProtocolExtensionID", kind: K::Alias { field: F::Unsigned(0, 65535, false), form: None, secret: false } },
/* 947 */ T { name: "ProtocolIE-Container", kind: K::Alias { field: F::List(&F::Named(948), 0, 65535), form: None, secret: false } },
/* 948 */ T { name: "ProtocolIE-Field", kind: K::Sequence { names: &["id", "criticality", "value"], members: &[M { field: "id", ty: F::Named(949), optional: false }, M { field: "criticality", ty: F::Named(245), optional: false }, M { field: "value", ty: F::Open, optional: false }], extensible: false } },
/* 949 */ T { name: "ProtocolIE-ID", kind: K::Alias { field: F::Unsigned(0, 65535, false), form: None, secret: false } },
/* 950 */ T { name: "QMCConfigInfo", kind: K::Sequence { names: &["uEAppLayerMeasInfoList", "iE-Extensions"], members: &[M { field: "ue_app_layer_meas_info_list", ty: F::Named(1245), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 951 */ T { name: "QMCDeactivation", kind: K::Sequence { names: &["qoEReferenceList", "iE-Extensions"], members: &[M { field: "qoe_reference_list", ty: F::Named(956), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 952 */ T { name: "QoERVQoEReportingPaths", kind: K::Sequence { names: &["qoEReportingPath", "rVQoEReportingPath", "iE-Extensions"], members: &[M { field: "qoe_reporting_path", ty: F::Named(953), optional: true }, M { field: "rv_qoe_reporting_path", ty: F::Named(954), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 953 */ T { name: "QoERVQoEReportingPathsQoEReportingPath", kind: K::Enumerated { names: &["srb4", "srb5"], root: 2, extensible: true } },
/* 954 */ T { name: "QoERVQoEReportingPathsRVQoEReportingPath", kind: K::Enumerated { names: &["srb4", "srb5"], root: 2, extensible: true } },
/* 955 */ T { name: "QoEReference", kind: K::Alias { field: F::OctetArray(6), form: None, secret: false } },
/* 956 */ T { name: "QoEReferenceList", kind: K::Alias { field: F::List(&F::Named(955), 1, 16), form: None, secret: false } },
/* 957 */ T { name: "QoSFlowList", kind: K::Alias { field: F::List(&F::Named(976), 1, 64), form: None, secret: false } },
/* 958 */ T { name: "QoSFlowTSCItem", kind: K::Sequence { names: &["qosFlowIdentifier", "tSCTrafficCharacteristicsFeedback", "aNPacketDelayBudgetUL", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "tsc_traffic_characteristics_feedback", ty: F::Named(1190), optional: true }, M { field: "an_packet_delay_budget_ul", ty: F::Named(376), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 959 */ T { name: "QoSFlowTSCList", kind: K::Alias { field: F::List(&F::Named(958), 1, 64), form: None, secret: false } },
/* 960 */ T { name: "QoSFlowsUsageReport-Item", kind: K::Sequence { names: &["qosFlowIdentifier", "rATType", "qoSFlowsTimedReportList", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "rat_type", ty: F::Named(961), optional: false }, M { field: "qos_flows_timed_report_list", ty: F::Named(1330), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 961 */ T { name: "QoSFlowsUsageReportItemRATType", kind: K::Enumerated { names: &["nr", "eutra", "nr-unlicensed", "e-utra-unlicensed"], root: 2, extensible: true } },
/* 962 */ T { name: "QoSFlowsUsageReportList", kind: K::Alias { field: F::List(&F::Named(960), 1, 64), form: None, secret: false } },
/* 963 */ T { name: "QosCharacteristics", kind: K::Choice { names: &["nonDynamic5QI", "dynamic5QI", "choice-Extensions"], alternatives: &[F::Named(750), F::Named(300), F::Named(948)], root: 3, extensible: false } },
/* 964 */ T { name: "QosFlowAcceptedItem", kind: K::Sequence { names: &["qosFlowIdentifier", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 965 */ T { name: "QosFlowAcceptedList", kind: K::Alias { field: F::List(&F::Named(964), 1, 64), form: None, secret: false } },
/* 966 */ T { name: "QosFlowAddOrModifyRequestItem", kind: K::Sequence { names: &["qosFlowIdentifier", "qosFlowLevelQosParameters", "e-RAB-ID", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "qos_flow_level_qos_parameters", ty: F::Named(980), optional: true }, M { field: "e_rab_id", ty: F::Named(301), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 967 */ T { name: "QosFlowAddOrModifyRequestList", kind: K::Alias { field: F::List(&F::Named(966), 1, 64), form: None, secret: false } },
/* 968 */ T { name: "QosFlowAddOrModifyResponseItem", kind: K::Sequence { names: &["qosFlowIdentifier", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 969 */ T { name: "QosFlowAddOrModifyResponseList", kind: K::Alias { field: F::List(&F::Named(968), 1, 64), form: None, secret: false } },
/* 970 */ T { name: "QosFlowAdditionalInfoItemRelCom", kind: K::Sequence { names: &["qosFlowIdentifier", "downlinkTLContainer", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "downlink_tl_container", ty: F::Named(1179), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 971 */ T { name: "QosFlowAdditionalInfoItemRelRes", kind: K::Sequence { names: &["qosFlowIdentifier", "uplinkTLContainer", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "uplink_tl_container", ty: F::Named(1179), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 972 */ T { name: "QosFlowAdditionalInfoListRelCom", kind: K::Alias { field: F::List(&F::Named(970), 1, 64), form: None, secret: false } },
/* 973 */ T { name: "QosFlowAdditionalInfoListRelRes", kind: K::Alias { field: F::List(&F::Named(971), 1, 64), form: None, secret: false } },
/* 974 */ T { name: "QosFlowFeedbackItem", kind: K::Sequence { names: &["qosFlowIdentifier", "updateFeedback", "cNpacketDelayBudgetDL", "cNpacketDelayBudgetUL", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "update_feedback", ty: F::Named(1307), optional: true }, M { field: "cn_packet_delay_budget_dl", ty: F::Named(376), optional: true }, M { field: "cn_packet_delay_budget_ul", ty: F::Named(376), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 975 */ T { name: "QosFlowFeedbackList", kind: K::Alias { field: F::List(&F::Named(974), 1, 64), form: None, secret: false } },
/* 976 */ T { name: "QosFlowIdentifier", kind: K::Alias { field: F::Unsigned(0, 63, true), form: None, secret: false } },
/* 977 */ T { name: "QosFlowInformationItem", kind: K::Sequence { names: &["qosFlowIdentifier", "dLForwarding", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "dl_forwarding", ty: F::Named(261), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 978 */ T { name: "QosFlowInformationList", kind: K::Alias { field: F::List(&F::Named(977), 1, 64), form: None, secret: false } },
/* 979 */ T { name: "QosFlowItemWithDataForwarding", kind: K::Sequence { names: &["qosFlowIdentifier", "dataForwardingAccepted", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "data_forwarding_accepted", ty: F::Named(279), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 980 */ T { name: "QosFlowLevelQosParameters", kind: K::Sequence { names: &["qosCharacteristics", "allocationAndRetentionPriority", "gBR-QosInformation", "reflectiveQosAttribute", "additionalQosFlowInformation", "iE-Extensions"], members: &[M { field: "qos_characteristics", ty: F::Named(963), optional: false }, M { field: "allocation_and_retention_priority", ty: F::Named(72), optional: false }, M { field: "gbr_qos_information", ty: F::Named(411), optional: true }, M { field: "reflective_qos_attribute", ty: F::Named(1052), optional: true }, M { field: "additional_qos_flow_information", ty: F::Named(63), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 981 */ T { name: "QosFlowListWithCause", kind: K::Alias { field: F::List(&F::Named(996), 1, 64), form: None, secret: false } },
/* 982 */ T { name: "QosFlowListWithDataForwarding", kind: K::Alias { field: F::List(&F::Named(979), 1, 64), form: None, secret: false } },
/* 983 */ T { name: "QosFlowModifyConfirmItem", kind: K::Sequence { names: &["qosFlowIdentifier", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 984 */ T { name: "QosFlowModifyConfirmList", kind: K::Alias { field: F::List(&F::Named(983), 1, 64), form: None, secret: false } },
/* 985 */ T { name: "QosFlowNotifyItem", kind: K::Sequence { names: &["qosFlowIdentifier", "notificationCause", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "notification_cause", ty: F::Named(752), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 986 */ T { name: "QosFlowNotifyList", kind: K::Alias { field: F::List(&F::Named(985), 1, 64), form: None, secret: false } },
/* 987 */ T { name: "QosFlowParametersItem", kind: K::Sequence { names: &["qosFlowIdentifier", "alternativeQoSParaSetList", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "alternative_qos_para_set_list", ty: F::Named(83), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 988 */ T { name: "QosFlowParametersList", kind: K::Alias { field: F::List(&F::Named(987), 1, 64), form: None, secret: false } },
/* 989 */ T { name: "QosFlowPerTNLInformation", kind: K::Sequence { names: &["uPTransportLayerInformation", "associatedQosFlowList", "iE-Extensions"], members: &[M { field: "up_transport_layer_information", ty: F::Named(1298), optional: false }, M { field: "associated_qos_flow_list", ty: F::Named(110), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 990 */ T { name: "QosFlowPerTNLInformationItem", kind: K::Sequence { names: &["qosFlowPerTNLInformation", "iE-Extensions"], members: &[M { field: "qos_flow_per_tnl_information", ty: F::Named(989), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 991 */ T { name: "QosFlowPerTNLInformationList", kind: K::Alias { field: F::List(&F::Named(990), 1, 3), form: None, secret: false } },
/* 992 */ T { name: "QosFlowSetupRequestItem", kind: K::Sequence { names: &["qosFlowIdentifier", "qosFlowLevelQosParameters", "e-RAB-ID", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "qos_flow_level_qos_parameters", ty: F::Named(980), optional: false }, M { field: "e_rab_id", ty: F::Named(301), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 993 */ T { name: "QosFlowSetupRequestList", kind: K::Alias { field: F::List(&F::Named(992), 1, 64), form: None, secret: false } },
/* 994 */ T { name: "QosFlowToBeForwardedItem", kind: K::Sequence { names: &["qosFlowIdentifier", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 995 */ T { name: "QosFlowToBeForwardedList", kind: K::Alias { field: F::List(&F::Named(994), 1, 64), form: None, secret: false } },
/* 996 */ T { name: "QosFlowWithCauseItem", kind: K::Sequence { names: &["qosFlowIdentifier", "cause", "iE-Extensions"], members: &[M { field: "qos_flow_identifier", ty: F::Named(976), optional: false }, M { field: "cause", ty: F::Named(185), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 997 */ T { name: "QosMonitoringReportingFrequency", kind: K::Alias { field: F::Unsigned(1, 1800, true), form: None, secret: false } },
/* 998 */ T { name: "QosMonitoringRequest", kind: K::Enumerated { names: &["ul", "dl", "both", "stop"], root: 3, extensible: true } },
/* 999 */ T { name: "RAN-AIOT-Device-NGAP-ID", kind: K::Alias { field: F::Unsigned(0, 4294967295, false), form: None, secret: false } },
/* 1000 */ T { name: "RAN-TSSRequestType", kind: K::Enumerated { names: &["start", "stop"], root: 2, extensible: true } },
/* 1001 */ T { name: "RAN-TSSScope", kind: K::Choice { names: &["rANNodeLevel", "cellListLevel", "choice-Extensions"], alternatives: &[F::Named(425), F::Named(1014), F::Named(948)], root: 3, extensible: false } },
/* 1002 */ T { name: "RAN-UE-NGAP-ID", kind: K::Alias { field: F::Unsigned(0, 4294967295, false), form: None, secret: false } },
/* 1003 */ T { name: "RANCPRelocationIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1004 */ T { name: "RANConfigurationUpdate", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1005 */ T { name: "RANConfigurationUpdateAcknowledge", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1006 */ T { name: "RANConfigurationUpdateFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1007 */ T { name: "RANNodeName", kind: K::Alias { field: F::Text(Text::Printable, 1, 150, true), form: None, secret: false } },
/* 1008 */ T { name: "RANNodeNameUTF8String", kind: K::Alias { field: F::Text(Text::Utf8, 1, 150, true), form: None, secret: false } },
/* 1009 */ T { name: "RANNodeNameVisibleString", kind: K::Alias { field: F::Text(Text::Visible, 1, 150, true), form: None, secret: false } },
/* 1010 */ T { name: "RANPagingPriority", kind: K::Alias { field: F::Unsigned(1, 256, false), form: None, secret: false } },
/* 1011 */ T { name: "RANPagingRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1012 */ T { name: "RANStatusTransfer-TransparentContainer", kind: K::Sequence { names: &["dRBsSubjectToStatusTransferList", "iE-Extensions"], members: &[M { field: "drbs_subject_to_status_transfer_list", ty: F::Named(275), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1013 */ T { name: "RANTSSCellItem", kind: K::Sequence { names: &["nRCGI", "iE-Extensions"], members: &[M { field: "nr_cgi", ty: F::Named(711), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1014 */ T { name: "RANTSSCellList", kind: K::Alias { field: F::List(&F::Named(1013), 1, 16384), form: None, secret: false } },
/* 1015 */ T { name: "RANTimingSynchronisationStatusInfo", kind: K::Sequence { names: &["synchronisationState", "traceabletoUTC", "traceabletoGNSS", "clockFrequencyStability", "clockAccuracy", "parentTImeSource", "iE-Extensions"], members: &[M { field: "synchronisation_state", ty: F::Named(1016), optional: true }, M { field: "traceableto_utc", ty: F::Named(1018), optional: true }, M { field: "traceableto_gnss", ty: F::Named(1017), optional: true }, M { field: "clock_frequency_stability", ty: F::Bits(16, 16), optional: true }, M { field: "clock_accuracy", ty: F::Named(212), optional: true }, M { field: "parent_time_source", ty: F::Named(915), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1016 */ T { name: "RANTimingSynchronisationStatusInfoSynchronisationState", kind: K::Enumerated { names: &["locked", "holdover", "freerun"], root: 3, extensible: true } },
/* 1017 */ T { name: "RANTimingSynchronisationStatusInfoTraceabletoGNSS", kind: K::Enumerated { names: &["true", "false"], root: 2, extensible: true } },
/* 1018 */ T { name: "RANTimingSynchronisationStatusInfoTraceabletoUTC", kind: K::Enumerated { names: &["true", "false"], root: 2, extensible: true } },
/* 1019 */ T { name: "RANfeedbacktype", kind: K::Choice { names: &["proactive", "reactive", "choice-extensions"], alternatives: &[F::Named(1020), F::Named(1021), F::Named(948)], root: 3, extensible: false } },
/* 1020 */ T { name: "RANfeedbacktype-proactive", kind: K::Sequence { names: &["burstArrivalTimeWindow", "periodicityRange", "iE-Extension"], members: &[M { field: "burst_arrival_time_window", ty: F::Named(151), optional: false }, M { field: "periodicity_range", ty: F::Named(931), optional: true }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 1021 */ T { name: "RANfeedbacktype-reactive", kind: K::Sequence { names: &["capabilityForBATAdaptation", "iE-Extension"], members: &[M { field: "capability_for_bat_adaptation", ty: F::Named(1022), optional: false }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 1022 */ T { name: "RANfeedbacktypeReactiveCapabilityForBATAdaptation", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 1023 */ T { name: "RAT-Information", kind: K::Enumerated { names: &["unlicensed", "nb-IoT", "nR-LEO", "nR-MEO", "nR-GEO", "nR-OTHERSAT"], root: 2, extensible: true } },
/* 1024 */ T { name: "RATRestrictionInformation", kind: K::Alias { field: F::BitString(8, 8, true), form: None, secret: false } },
/* 1025 */ T { name: "RATRestrictions", kind: K::Alias { field: F::List(&F::Named(1026), 1, 16), form: None, secret: false } },
/* 1026 */ T { name: "RATRestrictions-Item", kind: K::Sequence { names: &["pLMNIdentity", "rATRestrictionInformation", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "rat_restriction_information", ty: F::Named(1024), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1027 */ T { name: "RGLevelWirelineAccessCharacteristics", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 1028 */ T { name: "RIMInformation", kind: K::Sequence { names: &["targetgNBSetID", "rIM-RSDetection", "iE-Extensions"], members: &[M { field: "target_gnb_set_id", ty: F::Named(413), optional: false }, M { field: "rim_rs_detection", ty: F::Named(1029), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1029 */ T { name: "RIMInformationRIMRSDetection", kind: K::Enumerated { names: &["rs-detected", "rs-disappeared"], root: 2, extensible: true } },
/* 1030 */ T { name: "RIMInformationTransfer", kind: K::Sequence { names: &["targetRANNodeID-RIM", "sourceRANNodeID", "rIMInformation", "iE-Extensions"], members: &[M { field: "target_ran_node_id_rim", ty: F::Named(1201), optional: false }, M { field: "source_ran_node_id", ty: F::Named(1134), optional: false }, M { field: "rim_information", ty: F::Named(1028), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1031 */ T { name: "RNC-ID", kind: K::Alias { field: F::Unsigned(0, 4095, false), form: None, secret: false } },
/* 1032 */ T { name: "RRCContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 1033 */ T { name: "RRCEstablishmentCause", kind: K::Enumerated { names: &["emergency", "highPriorityAccess", "mt-Access", "mo-Signalling", "mo-Data", "mo-VoiceCall", "mo-VideoCall", "mo-SMS", "mps-PriorityAccess", "mcs-PriorityAccess", "notAvailable", "mo-ExceptionData"], root: 10, extensible: true } },
/* 1034 */ T { name: "RRCInactiveTransitionReport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1035 */ T { name: "RRCInactiveTransitionReportRequest", kind: K::Enumerated { names: &["subsequent-state-transition-report", "single-rrc-connected-state-report", "cancel-report"], root: 3, extensible: true } },
/* 1036 */ T { name: "RRCState", kind: K::Enumerated { names: &["inactive", "connected"], root: 2, extensible: true } },
/* 1037 */ T { name: "RSN", kind: K::Enumerated { names: &["v1", "v2"], root: 2, extensible: true } },
/* 1038 */ T { name: "RSPPFlowBitRates", kind: K::Sequence { names: &["guaranteedFlowBitRate", "maximumFlowBitRate", "iE-Extensions"], members: &[M { field: "guaranteed_flow_bit_rate", ty: F::Named(124), optional: false }, M { field: "maximum_flow_bit_rate", ty: F::Named(124), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1039 */ T { name: "RSPPQoSFlowItem", kind: K::Sequence { names: &["pQI", "rSPPFlowBitRates", "range", "iE-Extensions"], members: &[M { field: "pqi", ty: F::Named(404), optional: false }, M { field: "rspp_flow_bit_rates", ty: F::Named(1038), optional: true }, M { field: "range", ty: F::Named(1041), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1040 */ T { name: "RSPPQoSFlowList", kind: K::Alias { field: F::List(&F::Named(1039), 1, 2048), form: None, secret: false } },
/* 1041 */ T { name: "Range", kind: K::Enumerated { names: &["m50", "m80", "m180", "m200", "m350", "m400", "m500", "m700", "m1000"], root: 9, extensible: true } },
/* 1042 */ T { name: "RecommendedCellItem", kind: K::Sequence { names: &["nGRAN-CGI", "timeStayedInCell", "iE-Extensions"], members: &[M { field: "ngran_cgi", ty: F::Named(684), optional: false }, M { field: "time_stayed_in_cell", ty: F::Unsigned(0, 4095, false), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1043 */ T { name: "RecommendedCellList", kind: K::Alias { field: F::List(&F::Named(1042), 1, 16), form: None, secret: false } },
/* 1044 */ T { name: "RecommendedCellsForPaging", kind: K::Sequence { names: &["recommendedCellList", "iE-Extensions"], members: &[M { field: "recommended_cell_list", ty: F::Named(1043), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1045 */ T { name: "RecommendedRANNodeItem", kind: K::Sequence { names: &["aMFPagingTarget", "iE-Extensions"], members: &[M { field: "amf_paging_target", ty: F::Named(49), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1046 */ T { name: "RecommendedRANNodeList", kind: K::Alias { field: F::List(&F::Named(1045), 1, 16), form: None, secret: false } },
/* 1047 */ T { name: "RecommendedRANNodesForPaging", kind: K::Sequence { names: &["recommendedRANNodeList", "iE-Extensions"], members: &[M { field: "recommended_ran_node_list", ty: F::Named(1046), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1048 */ T { name: "RedCapIndication", kind: K::Enumerated { names: &["redcap"], root: 1, extensible: true } },
/* 1049 */ T { name: "RedirectionVoiceFallback", kind: K::Enumerated { names: &["possible", "not-possible"], root: 2, extensible: true } },
/* 1050 */ T { name: "RedundantPDUSessionInformation", kind: K::Sequence { names: &["rSN", "iE-Extensions"], members: &[M { field: "rsn", ty: F::Named(1037), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1051 */ T { name: "RedundantQosFlowIndicator", kind: K::Enumerated { names: &["true", "false"], root: 2, extensible: false } },
/* 1052 */ T { name: "ReflectiveQosAttribute", kind: K::Enumerated { names: &["subject-to"], root: 1, extensible: true } },
/* 1053 */ T { name: "RejectedNSSAIinPLMN", kind: K::Alias { field: F::OctetArray(32), form: None, secret: false } },
/* 1054 */ T { name: "RejectedNSSAIinTA", kind: K::Alias { field: F::OctetArray(32), form: None, secret: false } },
/* 1055 */ T { name: "RelativeAMFCapacity", kind: K::Alias { field: F::Unsigned(0, 255, false), form: None, secret: false } },
/* 1056 */ T { name: "RepetitionPeriod", kind: K::Alias { field: F::Unsigned(0, 131071, false), form: None, secret: false } },
/* 1057 */ T { name: "ReportAmountMDT", kind: K::Enumerated { names: &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "rinfinity"], root: 8, extensible: false } },
/* 1058 */ T { name: "ReportArea", kind: K::Enumerated { names: &["cell"], root: 1, extensible: true } },
/* 1059 */ T { name: "ReportCharacteristics", kind: K::Alias { field: F::Bits(32, 32), form: None, secret: false } },
/* 1060 */ T { name: "ReportIntervalMDT", kind: K::Enumerated { names: &["ms120", "ms240", "ms480", "ms640", "ms1024", "ms2048", "ms5120", "ms10240", "min1", "min6", "min12", "min30", "min60"], root: 13, extensible: false } },
/* 1061 */ T { name: "ReportType", kind: K::Choice { names: &["eventBasedReporting", "periodicReporting", "choice-Extensions"], alternatives: &[F::Named(356), F::Named(927), F::Named(948)], root: 3, extensible: false } },
/* 1062 */ T { name: "ReportingPeriodicity", kind: K::Enumerated { names: &["stop", "single", "ms1000", "ms2000", "ms5000", "ms10000"], root: 6, extensible: true } },
/* 1063 */ T { name: "ReportingSystem", kind: K::Choice { names: &["eUTRAN", "nGRAN", "noReporting", "choice-Extensions"], alternatives: &[F::Named(330), F::Named(693), F::Null, F::Named(948)], root: 4, extensible: false } },
/* 1064 */ T { name: "ReportingThreshold", kind: K::Alias { field: F::Unsigned(0, 4000000000, true), form: None, secret: false } },
/* 1065 */ T { name: "RequestedNSSAI", kind: K::Alias { field: F::List(&F::Named(1066), 1, 8), form: None, secret: false } },
/* 1066 */ T { name: "RequestedNSSAIItem", kind: K::Sequence { names: &["s-NSSAI", "iE-Extensions"], members: &[M { field: "s_nssai", ty: F::Named(1074), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1067 */ T { name: "RequestedTNLInfo", kind: K::Sequence { names: &["pLMNIdentity", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1068 */ T { name: "RerouteNASRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1069 */ T { name: "ResetAll", kind: K::Enumerated { names: &["reset-all"], root: 1, extensible: true } },
/* 1070 */ T { name: "ResetType", kind: K::Choice { names: &["nG-Interface", "partOfNG-Interface", "choice-Extensions"], alternatives: &[F::Named(1069), F::Named(1240), F::Named(948)], root: 3, extensible: false } },
/* 1071 */ T { name: "ResourceStatusReportingSystem", kind: K::Choice { names: &["eUTRAN-ReportingStatus", "nGRAN-ReportingStatus", "choice-Extensions"], alternatives: &[F::Named(329), F::Named(692), F::Named(948)], root: 3, extensible: false } },
/* 1072 */ T { name: "RetrieveUEInformation", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1073 */ T { name: "RoutingID", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 1074 */ T { name: "S-NSSAI", kind: K::Sequence { names: &["sST", "sD", "iE-Extensions"], members: &[M { field: "sst", ty: F::Named(1098), optional: false }, M { field: "sd", ty: F::Named(1077), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1075 */ T { name: "SCGActivationTime", kind: K::Alias { field: F::Unsigned(0, 40950, false), form: None, secret: false } },
/* 1076 */ T { name: "SCTP-TLAs", kind: K::Alias { field: F::List(&F::Named(1230), 1, 2), form: None, secret: false } },
/* 1077 */ T { name: "SD", kind: K::Alias { field: F::OctetArray(3), form: None, secret: false } },
/* 1078 */ T { name: "SEQUENCE", kind: K::Sequence { names: &["id", "criticality", "value"], members: &[M { field: "id", ty: F::Named(939), optional: false }, M { field: "criticality", ty: F::Named(245), optional: false }, M { field: "value", ty: F::Open, optional: false }], extensible: false } },
/* 1079 */ T { name: "SLPositioningRangingAuthorized", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 1080 */ T { name: "SLPositioningRangingQoSParameters", kind: K::Sequence { names: &["rSPPQoSFlowList", "rSPPLinkAggregateBitRates", "iE-Extensions"], members: &[M { field: "rspp_qos_flow_list", ty: F::Named(1040), optional: false }, M { field: "rspp_link_aggregate_bit_rates", ty: F::Named(124), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1081 */ T { name: "SLPositioningRangingServiceInfo", kind: K::Sequence { names: &["sLPositioningRangingAuthorized", "sLPositioningRangingQoSParameters", "iE-Extensions"], members: &[M { field: "sl_positioning_ranging_authorized", ty: F::Named(1079), optional: false }, M { field: "sl_positioning_ranging_qos_parameters", ty: F::Named(1080), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1082 */ T { name: "SNPN-BasedMDT", kind: K::Sequence { names: &["sNPNListforMDT", "iE-Extensions"], members: &[M { field: "snpn_listfor_mdt", ty: F::Named(1088), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1083 */ T { name: "SNPN-CellBasedMDT", kind: K::Sequence { names: &["sNPNcellIdListforMDT", "iE-Extensions"], members: &[M { field: "snpn_cell_id_listfor_mdt", ty: F::Named(1086), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1084 */ T { name: "SNPN-MobilityInformation", kind: K::Sequence { names: &["serving-NID", "iE-Extensions"], members: &[M { field: "serving_nid", ty: F::Named(705), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1085 */ T { name: "SNPN-TAIBasedMDT", kind: K::Sequence { names: &["sNPNTAIListforMDT", "iE-Extensions"], members: &[M { field: "snpn_tai_listfor_mdt", ty: F::Named(1090), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1086 */ T { name: "SNPNCellIdListforMDT", kind: K::Alias { field: F::List(&F::Named(1087), 1, 32), form: None, secret: false } },
/* 1087 */ T { name: "SNPNCellIdListforMDTItem", kind: K::Sequence { names: &["nR-CGI", "nID", "iE-Extensions"], members: &[M { field: "nr_cgi", ty: F::Named(711), optional: false }, M { field: "nid", ty: F::Named(705), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1088 */ T { name: "SNPNListforMDT", kind: K::Alias { field: F::List(&F::Named(1089), 1, 16), form: None, secret: false } },
/* 1089 */ T { name: "SNPNListforMDTItem", kind: K::Sequence { names: &["pLMNIdentity", "nID", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "nid", ty: F::Named(705), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1090 */ T { name: "SNPNTAIListforMDT", kind: K::Alias { field: F::List(&F::Named(1091), 1, 8), form: None, secret: false } },
/* 1091 */ T { name: "SNPNTAIListforMDTItem", kind: K::Sequence { names: &["tAI", "nID", "iE-Extensions"], members: &[M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "nid", ty: F::Named(705), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1092 */ T { name: "SONConfigurationTransfer", kind: K::Sequence { names: &["targetRANNodeID-SON", "sourceRANNodeID", "sONInformation", "xnTNLConfigurationInfo", "iE-Extensions"], members: &[M { field: "target_ran_node_id_son", ty: F::Named(1202), optional: false }, M { field: "source_ran_node_id", ty: F::Named(1134), optional: false }, M { field: "son_information", ty: F::Named(1093), optional: false }, M { field: "xn_tnl_configuration_info", ty: F::Named(1351), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1093 */ T { name: "SONInformation", kind: K::Choice { names: &["sONInformationRequest", "sONInformationReply", "choice-Extensions"], alternatives: &[F::Named(1096), F::Named(1094), F::Named(948)], root: 3, extensible: false } },
/* 1094 */ T { name: "SONInformationReply", kind: K::Sequence { names: &["xnTNLConfigurationInfo", "iE-Extensions"], members: &[M { field: "xn_tnl_configuration_info", ty: F::Named(1351), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1095 */ T { name: "SONInformationReport", kind: K::Choice { names: &["failureIndicationInformation", "hOReportInformation", "choice-Extensions"], alternatives: &[F::Named(382), F::Named(439), F::Named(948)], root: 3, extensible: false } },
/* 1096 */ T { name: "SONInformationRequest", kind: K::Enumerated { names: &["xn-TNL-configuration-info"], root: 1, extensible: true } },
/* 1097 */ T { name: "SRVCCOperationPossible", kind: K::Enumerated { names: &["possible", "notPossible"], root: 2, extensible: true } },
/* 1098 */ T { name: "SST", kind: K::Alias { field: F::OctetArray(1), form: Some(Form::Number(8)), secret: false } },
/* 1099 */ T { name: "ScheduledCommunicationTime", kind: K::Sequence { names: &["dayofWeek", "timeofDayStart", "timeofDayEnd", "iE-Extensions"], members: &[M { field: "dayof_week", ty: F::Bits(7, 7), optional: true }, M { field: "timeof_day_start", ty: F::Unsigned(0, 86399, true), optional: true }, M { field: "timeof_day_end", ty: F::Unsigned(0, 86399, true), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1100 */ T { name: "SecondaryRATDataUsageReport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1101 */ T { name: "SecondaryRATDataUsageReportTransfer", kind: K::Sequence { names: &["secondaryRATUsageInformation", "iE-Extensions"], members: &[M { field: "secondary_rat_usage_information", ty: F::Named(1102), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1102 */ T { name: "SecondaryRATUsageInformation", kind: K::Sequence { names: &["pDUSessionUsageReport", "qosFlowsUsageReportList", "iE-Extension"], members: &[M { field: "pdu_session_usage_report", ty: F::Named(878), optional: true }, M { field: "qos_flows_usage_report_list", ty: F::Named(962), optional: true }, M { field: "ie_extension", ty: F::Named(944), optional: true }], extensible: true } },
/* 1103 */ T { name: "SecurityContext", kind: K::Sequence { names: &["nextHopChainingCount", "nextHopNH", "iE-Extensions"], members: &[M { field: "next_hop_chaining_count", ty: F::Named(746), optional: false }, M { field: "next_hop_nh", ty: F::Named(1105), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1104 */ T { name: "SecurityIndication", kind: K::Sequence { names: &["integrityProtectionIndication", "confidentialityProtectionIndication", "maximumIntegrityProtectedDataRate-UL", "iE-Extensions"], members: &[M { field: "integrity_protection_indication", ty: F::Named(476), optional: false }, M { field: "confidentiality_protection_indication", ty: F::Named(237), optional: false }, M { field: "maximum_integrity_protected_data_rate_ul", ty: F::Named(638), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1105 */ T { name: "SecurityKey", kind: K::Alias { field: F::BitString(256, 256, false), form: None, secret: true } },
/* 1106 */ T { name: "SecurityResult", kind: K::Sequence { names: &["integrityProtectionResult", "confidentialityProtectionResult", "iE-Extensions"], members: &[M { field: "integrity_protection_result", ty: F::Named(477), optional: false }, M { field: "confidentiality_protection_result", ty: F::Named(238), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1107 */ T { name: "SensorMeasConfig", kind: K::Enumerated { names: &["setup"], root: 1, extensible: true } },
/* 1108 */ T { name: "SensorMeasConfigNameItem", kind: K::Sequence { names: &["sensorNameConfig", "iE-Extensions"], members: &[M { field: "sensor_name_config", ty: F::Named(1111), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1109 */ T { name: "SensorMeasConfigNameList", kind: K::Alias { field: F::List(&F::Named(1108), 1, 3), form: None, secret: false } },
/* 1110 */ T { name: "SensorMeasurementConfiguration", kind: K::Sequence { names: &["sensorMeasConfig", "sensorMeasConfigNameList", "iE-Extensions"], members: &[M { field: "sensor_meas_config", ty: F::Named(1107), optional: false }, M { field: "sensor_meas_config_name_list", ty: F::Named(1109), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1111 */ T { name: "SensorNameConfig", kind: K::Choice { names: &["uncompensatedBarometricConfig", "ueSpeedConfig", "ueOrientationConfig", "choice-Extensions"], alternatives: &[F::Named(1114), F::Named(1113), F::Named(1112), F::Named(948)], root: 4, extensible: false } },
/* 1112 */ T { name: "SensorNameConfigUeOrientationConfig", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 1113 */ T { name: "SensorNameConfigUeSpeedConfig", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 1114 */ T { name: "SensorNameConfigUncompensatedBarometricConfig", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 1115 */ T { name: "SerialNumber", kind: K::Alias { field: F::Bits(16, 16), form: None, secret: false } },
/* 1116 */ T { name: "ServedGUAMIItem", kind: K::Sequence { names: &["gUAMI", "backupAMFName", "iE-Extensions"], members: &[M { field: "guami", ty: F::Named(416), optional: false }, M { field: "backup_amf_name", ty: F::Named(46), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1117 */ T { name: "ServedGUAMIList", kind: K::Alias { field: F::List(&F::Named(1116), 1, 256), form: None, secret: false } },
/* 1118 */ T { name: "ServiceAreaInformation", kind: K::Alias { field: F::List(&F::Named(1119), 1, 16), form: None, secret: false } },
/* 1119 */ T { name: "ServiceAreaInformation-Item", kind: K::Sequence { names: &["pLMNIdentity", "allowedTACs", "notAllowedTACs", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "allowed_tacs", ty: F::Named(80), optional: true }, M { field: "not_allowed_tacs", ty: F::Named(751), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1120 */ T { name: "ServiceType", kind: K::Enumerated { names: &["streaming", "mTSI", "vR"], root: 3, extensible: true } },
/* 1121 */ T { name: "SgNB-UE-X2AP-ID", kind: K::Alias { field: F::Unsigned(0, 4294967295, false), form: None, secret: false } },
/* 1122 */ T { name: "SharedNGU-MulticastTNLInformation", kind: K::Sequence { names: &["iP-MulticastAddress", "iP-SourceAddress", "gTP-TEID", "iE-Extensions"], members: &[M { field: "ip_multicast_address", ty: F::Named(1230), optional: false }, M { field: "ip_source_address", ty: F::Named(1230), optional: false }, M { field: "gtp_teid", ty: F::Named(414), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1123 */ T { name: "SliceMDTItem", kind: K::Sequence { names: &["sNSSAI", "iE-Extensions"], members: &[M { field: "s_nssai", ty: F::Named(1074), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1124 */ T { name: "SliceMDTList", kind: K::Alias { field: F::List(&F::Named(1123), 1, 1024), form: None, secret: false } },
/* 1125 */ T { name: "SliceOverloadItem", kind: K::Sequence { names: &["s-NSSAI", "iE-Extensions"], members: &[M { field: "s_nssai", ty: F::Named(1074), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1126 */ T { name: "SliceOverloadList", kind: K::Alias { field: F::List(&F::Named(1125), 1, 1024), form: None, secret: false } },
/* 1127 */ T { name: "SliceSupportItem", kind: K::Sequence { names: &["s-NSSAI", "iE-Extensions"], members: &[M { field: "s_nssai", ty: F::Named(1074), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1128 */ T { name: "SliceSupportList", kind: K::Alias { field: F::List(&F::Named(1127), 1, 1024), form: None, secret: false } },
/* 1129 */ T { name: "SliceSupportListQMC", kind: K::Alias { field: F::List(&F::Named(1130), 1, 16), form: None, secret: false } },
/* 1130 */ T { name: "SliceSupportQMC-Item", kind: K::Sequence { names: &["s-NSSAI", "iE-Extensions"], members: &[M { field: "s_nssai", ty: F::Named(1074), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1131 */ T { name: "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer", kind: K::Sequence { names: &["rRCContainer", "pDUSessionResourceInformationList", "e-RABInformationList", "targetCell-ID", "indexToRFSP", "uEHistoryInformation", "iE-Extensions"], members: &[M { field: "rrc_container", ty: F::Named(1032), optional: false }, M { field: "pdu_session_resource_information_list", ty: F::Named(804), optional: true }, M { field: "e_rab_information_list", ty: F::Named(303), optional: true }, M { field: "target_cell_id", ty: F::Named(684), optional: false }, M { field: "index_to_rfsp", ty: F::Named(468), optional: true }, M { field: "ue_history_information", ty: F::Named(1267), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1132 */ T { name: "SourceNodeID", kind: K::Choice { names: &["sourceengNB-ID", "choice-Extensions"], alternatives: &[F::Named(425), F::Named(948)], root: 2, extensible: false } },
/* 1133 */ T { name: "SourceOfUEActivityBehaviourInformation", kind: K::Enumerated { names: &["subscription-information", "statistics"], root: 2, extensible: true } },
/* 1134 */ T { name: "SourceRANNodeID", kind: K::Sequence { names: &["globalRANNodeID", "selectedTAI", "iE-Extensions"], members: &[M { field: "global_ran_node_id", ty: F::Named(430), optional: false }, M { field: "selected_tai", ty: F::Named(1154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1135 */ T { name: "SourceToTarget-AMFInformationReroute", kind: K::Sequence { names: &["configuredNSSAI", "rejectedNSSAIinPLMN", "rejectedNSSAIinTA", "iE-Extensions"], members: &[M { field: "configured_nssai", ty: F::Named(239), optional: true }, M { field: "rejected_nssai_in_plmn", ty: F::Named(1053), optional: true }, M { field: "rejected_nssai_in_ta", ty: F::Named(1054), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1136 */ T { name: "SourceToTarget-TransparentContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 1137 */ T { name: "SuccessfulHandoverReport-Item", kind: K::Sequence { names: &["successfulHOReportContainer", "iE-Extensions"], members: &[M { field: "successful_ho_report_container", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1138 */ T { name: "SuccessfulHandoverReportList", kind: K::Alias { field: F::List(&F::Named(1137), 1, 64), form: None, secret: false } },
/* 1139 */ T { name: "SuccessfulOutcome", kind: K::Sequence { names: &["procedureCode", "criticality", "value"], members: &[M { field: "procedure_code", ty: F::Named(942), optional: false }, M { field: "criticality", ty: F::Named(245), optional: false }, M { field: "value", ty: F::Open, optional: false }], extensible: false } },
/* 1140 */ T { name: "SuccessfulPSCellChangeReport-Item", kind: K::Sequence { names: &["successfulPSCellChangeReportContainer", "iE-Extensions"], members: &[M { field: "successful_ps_cell_change_report_container", ty: F::Octets(0, UNBOUNDED, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1141 */ T { name: "SuccessfulPSCellChangeReportList", kind: K::Alias { field: F::List(&F::Named(1140), 1, 64), form: None, secret: false } },
/* 1142 */ T { name: "SupportedTAItem", kind: K::Sequence { names: &["tAC", "broadcastPLMNList", "iE-Extensions"], members: &[M { field: "tac", ty: F::Named(1152), optional: false }, M { field: "broadcast_plmn_list", ty: F::Named(134), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1143 */ T { name: "SupportedTAList", kind: K::Alias { field: F::List(&F::Named(1142), 1, 256), form: None, secret: false } },
/* 1144 */ T { name: "SupportedUEType", kind: K::Enumerated { names: &["non-RedCap-eRedCap-UE", "redCap-eRedCap-UE"], root: 2, extensible: true } },
/* 1145 */ T { name: "SupportedUETypeList", kind: K::Alias { field: F::List(&F::Named(1144), 1, 8), form: None, secret: false } },
/* 1146 */ T { name: "SurvivalTime", kind: K::Alias { field: F::Unsigned(0, 1920000, true), form: None, secret: false } },
/* 1147 */ T { name: "Suspend-Request-Indication", kind: K::Enumerated { names: &["suspend-requested"], root: 1, extensible: true } },
/* 1148 */ T { name: "Suspend-Response-Indication", kind: K::Enumerated { names: &["suspend-indicated"], root: 1, extensible: true } },
/* 1149 */ T { name: "SuspendIndicator", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 1150 */ T { name: "TABasedMDT", kind: K::Sequence { names: &["tAListforMDT", "iE-Extensions"], members: &[M { field: "ta_listfor_mdt", ty: F::Named(1177), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1151 */ T { name: "TABasedQMC", kind: K::Sequence { names: &["tAListforQMC", "iE-Extensions"], members: &[M { field: "ta_listfor_qmc", ty: F::Named(1178), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1152 */ T { name: "TAC", kind: K::Alias { field: F::OctetArray(3), form: Some(Form::Number(24)), secret: false } },
/* 1153 */ T { name: "TACListInNRNTN", kind: K::Alias { field: F::List(&F::Named(1152), 1, 12), form: None, secret: false } },
/* 1154 */ T { name: "TAI", kind: K::Sequence { names: &["pLMNIdentity", "tAC", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "tac", ty: F::Named(1152), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1155 */ T { name: "TAIBasedMDT", kind: K::Sequence { names: &["tAIListforMDT", "iE-Extensions"], members: &[M { field: "tai_listfor_mdt", ty: F::Named(1171), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1156 */ T { name: "TAIBasedQMC", kind: K::Sequence { names: &["tAIListforQMC", "iE-Extensions"], members: &[M { field: "tai_listfor_qmc", ty: F::Named(1172), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1157 */ T { name: "TAIBroadcastEUTRA", kind: K::Alias { field: F::List(&F::Named(1158), 1, 65535), form: None, secret: false } },
/* 1158 */ T { name: "TAIBroadcastEUTRA-Item", kind: K::Sequence { names: &["tAI", "completedCellsInTAI-EUTRA", "iE-Extensions"], members: &[M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "completed_cells_in_tai_eutra", ty: F::Named(231), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1159 */ T { name: "TAIBroadcastNR", kind: K::Alias { field: F::List(&F::Named(1160), 1, 65535), form: None, secret: false } },
/* 1160 */ T { name: "TAIBroadcastNR-Item", kind: K::Sequence { names: &["tAI", "completedCellsInTAI-NR", "iE-Extensions"], members: &[M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "completed_cells_in_tai_nr", ty: F::Named(233), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1161 */ T { name: "TAICancelledEUTRA", kind: K::Alias { field: F::List(&F::Named(1162), 1, 65535), form: None, secret: false } },
/* 1162 */ T { name: "TAICancelledEUTRA-Item", kind: K::Sequence { names: &["tAI", "cancelledCellsInTAI-EUTRA", "iE-Extensions"], members: &[M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "cancelled_cells_in_tai_eutra", ty: F::Named(173), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1163 */ T { name: "TAICancelledNR", kind: K::Alias { field: F::List(&F::Named(1164), 1, 65535), form: None, secret: false } },
/* 1164 */ T { name: "TAICancelledNR-Item", kind: K::Sequence { names: &["tAI", "cancelledCellsInTAI-NR", "iE-Extensions"], members: &[M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "cancelled_cells_in_tai_nr", ty: F::Named(175), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1165 */ T { name: "TAIListForInactive", kind: K::Alias { field: F::List(&F::Named(1166), 1, 16), form: None, secret: false } },
/* 1166 */ T { name: "TAIListForInactiveItem", kind: K::Sequence { names: &["tAI", "iE-Extensions"], members: &[M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1167 */ T { name: "TAIListForPaging", kind: K::Alias { field: F::List(&F::Named(1168), 1, 16), form: None, secret: false } },
/* 1168 */ T { name: "TAIListForPagingItem", kind: K::Sequence { names: &["tAI", "iE-Extensions"], members: &[M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1169 */ T { name: "TAIListForRestart", kind: K::Alias { field: F::List(&F::Named(1154), 1, 2048), form: None, secret: false } },
/* 1170 */ T { name: "TAIListForWarning", kind: K::Alias { field: F::List(&F::Named(1154), 1, 65535), form: None, secret: false } },
/* 1171 */ T { name: "TAIListforMDT", kind: K::Alias { field: F::List(&F::Named(1154), 1, 8), form: None, secret: false } },
/* 1172 */ T { name: "TAIListforQMC", kind: K::Alias { field: F::List(&F::Named(1154), 1, 8), form: None, secret: false } },
/* 1173 */ T { name: "TAIMBSSupportItem", kind: K::Sequence { names: &["tAI", "iE-Extensions"], members: &[M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1174 */ T { name: "TAIMBSSupportList", kind: K::Alias { field: F::List(&F::Named(1173), 1, 256), form: None, secret: false } },
/* 1175 */ T { name: "TAINSAGSupportItem", kind: K::Sequence { names: &["nSAG-ID", "nSAGSliceSupportList", "iE-Extensions"], members: &[M { field: "nsag_id", ty: F::Named(737), optional: false }, M { field: "nsag_slice_support_list", ty: F::Named(380), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1176 */ T { name: "TAINSAGSupportList", kind: K::Alias { field: F::List(&F::Named(1175), 1, 256), form: None, secret: false } },
/* 1177 */ T { name: "TAListforMDT", kind: K::Alias { field: F::List(&F::Named(1152), 1, 8), form: None, secret: false } },
/* 1178 */ T { name: "TAListforQMC", kind: K::Alias { field: F::List(&F::Named(1152), 1, 8), form: None, secret: false } },
/* 1179 */ T { name: "TLContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 1180 */ T { name: "TMGI", kind: K::Alias { field: F::OctetArray(6), form: None, secret: false } },
/* 1181 */ T { name: "TNAP-ID", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 1182 */ T { name: "TNGF-ID", kind: K::Choice { names: &["tNGF-ID", "choice-Extensions"], alternatives: &[F::BitString(32, 32, true), F::Named(948)], root: 2, extensible: false } },
/* 1183 */ T { name: "TNLAddressWeightFactor", kind: K::Alias { field: F::Unsigned(0, 255, false), form: None, secret: false } },
/* 1184 */ T { name: "TNLAssociationItem", kind: K::Sequence { names: &["tNLAssociationAddress", "cause", "iE-Extensions"], members: &[M { field: "tnl_association_address", ty: F::Named(167), optional: false }, M { field: "cause", ty: F::Named(185), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1185 */ T { name: "TNLAssociationList", kind: K::Alias { field: F::List(&F::Named(1184), 1, 32), form: None, secret: false } },
/* 1186 */ T { name: "TNLAssociationUsage", kind: K::Enumerated { names: &["ue", "non-ue", "both"], root: 3, extensible: true } },
/* 1187 */ T { name: "TSCAssistanceInformation", kind: K::Sequence { names: &["periodicity", "burstArrivalTime", "iE-Extensions"], members: &[M { field: "periodicity", ty: F::Named(928), optional: false }, M { field: "burst_arrival_time", ty: F::Named(150), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1188 */ T { name: "TSCFeedbackInformation", kind: K::Sequence { names: &["burstArrivalTimeOffset", "adjustedPeriodicity", "iE-Extensions"], members: &[M { field: "burst_arrival_time_offset", ty: F::Signed(-640000, 640000, true), optional: false }, M { field: "adjusted_periodicity", ty: F::Named(928), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1189 */ T { name: "TSCTrafficCharacteristics", kind: K::Sequence { names: &["tSCAssistanceInformationDL", "tSCAssistanceInformationUL", "iE-Extensions"], members: &[M { field: "tsc_assistance_information_dl", ty: F::Named(1187), optional: true }, M { field: "tsc_assistance_information_ul", ty: F::Named(1187), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1190 */ T { name: "TSCTrafficCharacteristicsFeedback", kind: K::Sequence { names: &["tSCFeedbackInformationDL", "tSCFeedbackInformationUL", "iE-Extensions"], members: &[M { field: "tsc_feedback_information_dl", ty: F::Named(1188), optional: true }, M { field: "tsc_feedback_information_ul", ty: F::Named(1188), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1191 */ T { name: "TWAP-ID", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 1192 */ T { name: "TWIF-ID", kind: K::Choice { names: &["tWIF-ID", "choice-Extensions"], alternatives: &[F::BitString(32, 32, true), F::Named(948)], root: 2, extensible: false } },
/* 1193 */ T { name: "TargetHomeENB-ID", kind: K::Sequence { names: &["pLMNidentity", "homeENB-ID", "selected-EPS-TAI", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(886), optional: false }, M { field: "home_enb_id", ty: F::Bits(28, 28), optional: false }, M { field: "selected_eps_tai", ty: F::Named(313), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1194 */ T { name: "TargetID", kind: K::Choice { names: &["targetRANNodeID", "targeteNB-ID", "choice-Extensions"], alternatives: &[F::Named(1200), F::Named(1205), F::Named(948)], root: 3, extensible: false } },
/* 1195 */ T { name: "TargetNGRANNode-ToSourceNGRANNode-FailureTransparentContainer", kind: K::Sequence { names: &["cell-CAGInformation", "iE-Extensions"], members: &[M { field: "cell_cag_information", ty: F::Named(191), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1196 */ T { name: "TargetNGRANNode-ToSourceNGRANNode-TransparentContainer", kind: K::Sequence { names: &["rRCContainer", "iE-Extensions"], members: &[M { field: "rrc_container", ty: F::Named(1032), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1197 */ T { name: "TargetNSSAI", kind: K::Alias { field: F::List(&F::Named(1198), 1, 8), form: None, secret: false } },
/* 1198 */ T { name: "TargetNSSAI-Item", kind: K::Sequence { names: &["s-NSSAI", "iE-Extensions"], members: &[M { field: "s_nssai", ty: F::Named(1074), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1199 */ T { name: "TargetNSSAIInformation", kind: K::Sequence { names: &["targetNSSAI", "indexToRFSP", "iE-Extensions"], members: &[M { field: "target_nssai", ty: F::Named(1197), optional: false }, M { field: "index_to_rfsp", ty: F::Named(468), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1200 */ T { name: "TargetRANNodeID", kind: K::Sequence { names: &["globalRANNodeID", "selectedTAI", "iE-Extensions"], members: &[M { field: "global_ran_node_id", ty: F::Named(430), optional: false }, M { field: "selected_tai", ty: F::Named(1154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1201 */ T { name: "TargetRANNodeID-RIM", kind: K::Sequence { names: &["globalRANNodeID", "selectedTAI", "iE-Extensions"], members: &[M { field: "global_ran_node_id", ty: F::Named(430), optional: false }, M { field: "selected_tai", ty: F::Named(1154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1202 */ T { name: "TargetRANNodeID-SON", kind: K::Sequence { names: &["globalRANNodeID", "selectedTAI", "iE-Extensions"], members: &[M { field: "global_ran_node_id", ty: F::Named(430), optional: false }, M { field: "selected_tai", ty: F::Named(1154), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1203 */ T { name: "TargetRNC-ID", kind: K::Sequence { names: &["lAI", "rNC-ID", "extendedRNC-ID", "iE-Extensions"], members: &[M { field: "lai", ty: F::Named(512), optional: false }, M { field: "rnc_id", ty: F::Named(1031), optional: false }, M { field: "extended_rnc_id", ty: F::Named(378), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1204 */ T { name: "TargetToSource-TransparentContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 1205 */ T { name: "TargeteNB-ID", kind: K::Sequence { names: &["globalENB-ID", "selected-EPS-TAI", "iE-Extensions"], members: &[M { field: "global_enb_id", ty: F::Named(429), optional: false }, M { field: "selected_eps_tai", ty: F::Named(313), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1206 */ T { name: "TargettoSource-Failure-TransparentContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 1207 */ T { name: "Threshold-RSRP", kind: K::Alias { field: F::Unsigned(0, 127, false), form: None, secret: false } },
/* 1208 */ T { name: "Threshold-RSRQ", kind: K::Alias { field: F::Unsigned(0, 127, false), form: None, secret: false } },
/* 1209 */ T { name: "Threshold-SINR", kind: K::Alias { field: F::Unsigned(0, 127, false), form: None, secret: false } },
/* 1210 */ T { name: "TimeBasedHandoverInformation", kind: K::Sequence { names: &["hOWindowStart", "hOWindowDuration", "iE-Extensions"], members: &[M { field: "ho_window_start", ty: F::Named(459), optional: false }, M { field: "ho_window_duration", ty: F::Named(458), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1211 */ T { name: "TimeSinceFailure", kind: K::Alias { field: F::Unsigned(0, 172800, true), form: None, secret: false } },
/* 1212 */ T { name: "TimeStamp", kind: K::Alias { field: F::OctetArray(4), form: None, secret: false } },
/* 1213 */ T { name: "TimeSyncAssistanceInfo", kind: K::Sequence { names: &["timeDistributionIndication", "uUTimeSyncErrorBudget", "iE-Extensions"], members: &[M { field: "time_distribution_indication", ty: F::Named(1214), optional: false }, M { field: "uu_time_sync_error_budget", ty: F::Unsigned(1, 1000000, true), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1214 */ T { name: "TimeSyncAssistanceInfoTimeDistributionIndication", kind: K::Enumerated { names: &["enabled", "disabled"], root: 2, extensible: true } },
/* 1215 */ T { name: "TimeToTrigger", kind: K::Enumerated { names: &["ms0", "ms40", "ms64", "ms80", "ms100", "ms128", "ms160", "ms256", "ms320", "ms480", "ms512", "ms640", "ms1024", "ms1280", "ms2560", "ms5120"], root: 16, extensible: false } },
/* 1216 */ T { name: "TimeToWait", kind: K::Enumerated { names: &["v1s", "v2s", "v5s", "v10s", "v20s", "v60s"], root: 6, extensible: true } },
/* 1217 */ T { name: "TimeUEStayedInCell", kind: K::Alias { field: F::Unsigned(0, 4095, false), form: None, secret: false } },
/* 1218 */ T { name: "TimeUEStayedInCellEnhancedGranularity", kind: K::Alias { field: F::Unsigned(0, 40950, false), form: None, secret: false } },
/* 1219 */ T { name: "TimerApproachForGUAMIRemoval", kind: K::Enumerated { names: &["apply-timer"], root: 1, extensible: true } },
/* 1220 */ T { name: "TimingSynchronisationStatusFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1221 */ T { name: "TimingSynchronisationStatusReport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1222 */ T { name: "TimingSynchronisationStatusRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1223 */ T { name: "TimingSynchronisationStatusResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1224 */ T { name: "TooearlyIntersystemHO", kind: K::Sequence { names: &["sourcecellID", "failurecellID", "uERLFReportContainer", "iE-Extensions"], members: &[M { field: "sourcecell_id", ty: F::Named(315), optional: false }, M { field: "failurecell_id", ty: F::Named(684), optional: false }, M { field: "ue_rlf_report_container", ty: F::Named(1275), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1225 */ T { name: "TraceActivation", kind: K::Sequence { names: &["nGRANTraceID", "interfacesToTrace", "traceDepth", "traceCollectionEntityIPAddress", "iE-Extensions"], members: &[M { field: "ngran_trace_id", ty: F::Named(696), optional: false }, M { field: "interfaces_to_trace", ty: F::Named(483), optional: false }, M { field: "trace_depth", ty: F::Named(1226), optional: false }, M { field: "trace_collection_entity_ip_address", ty: F::Named(1230), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1226 */ T { name: "TraceDepth", kind: K::Enumerated { names: &["minimum", "medium", "maximum", "minimumWithoutVendorSpecificExtension", "mediumWithoutVendorSpecificExtension", "maximumWithoutVendorSpecificExtension", "minimumOnlyVendorSpecificTraceRecord", "mediumOnlyVendorSpecificTraceRecord", "maximumOnlyVendorSpecificTraceRecord"], root: 6, extensible: true } },
/* 1227 */ T { name: "TraceFailureIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1228 */ T { name: "TraceStart", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1229 */ T { name: "TrafficLoadReductionIndication", kind: K::Alias { field: F::Unsigned(1, 99, false), form: None, secret: false } },
/* 1230 */ T { name: "TransportLayerAddress", kind: K::Alias { field: F::BitString(1, 160, true), form: Some(Form::Address), secret: false } },
/* 1231 */ T { name: "TriggeringMessage", kind: K::Enumerated { names: &["initiating-message", "successful-outcome", "unsuccessful-outcome"], root: 3, extensible: false } },
/* 1232 */ T { name: "TypeOfError", kind: K::Enumerated { names: &["not-understood", "missing"], root: 2, extensible: true } },
/* 1233 */ T { name: "UE-DifferentiationInfo", kind: K::Sequence { names: &["periodicCommunicationIndicator", "periodicTime", "scheduledCommunicationTime", "stationaryIndication", "trafficProfile", "batteryIndication", "iE-Extensions"], members: &[M { field: "periodic_communication_indicator", ty: F::Named(1264), optional: true }, M { field: "periodic_time", ty: F::Unsigned(1, 3600, true), optional: true }, M { field: "scheduled_communication_time", ty: F::Named(1099), optional: true }, M { field: "stationary_indication", ty: F::Named(1265), optional: true }, M { field: "traffic_profile", ty: F::Named(1266), optional: true }, M { field: "battery_indication", ty: F::Named(1263), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1234 */ T { name: "UE-NGAP-ID-pair", kind: K::Sequence { names: &["aMF-UE-NGAP-ID", "rAN-UE-NGAP-ID", "iE-Extensions"], members: &[M { field: "amf_ue_ngap_id", ty: F::Named(41), optional: false }, M { field: "ran_ue_ngap_id", ty: F::Named(1002), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1235 */ T { name: "UE-NGAP-IDs", kind: K::Choice { names: &["uE-NGAP-ID-pair", "aMF-UE-NGAP-ID", "choice-Extensions"], alternatives: &[F::Named(1234), F::Named(41), F::Named(948)], root: 3, extensible: false } },
/* 1236 */ T { name: "UE-PagingItem", kind: K::Sequence { names: &["uEIdentityIndexValue", "pagingDRX", "iE-Extensions"], members: &[M { field: "ue_identity_index_value", ty: F::Named(1269), optional: false }, M { field: "paging_drx", ty: F::Named(907), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1237 */ T { name: "UE-PagingList", kind: K::Alias { field: F::List(&F::Named(1236), 1, 4096), form: None, secret: false } },
/* 1238 */ T { name: "UE-UP-CIoT-Support", kind: K::Enumerated { names: &["supported"], root: 1, extensible: true } },
/* 1239 */ T { name: "UE-associatedLogicalNG-connectionItem", kind: K::Sequence { names: &["aMF-UE-NGAP-ID", "rAN-UE-NGAP-ID", "iE-Extensions"], members: &[M { field: "amf_ue_ngap_id", ty: F::Named(41), optional: true }, M { field: "ran_ue_ngap_id", ty: F::Named(1002), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1240 */ T { name: "UE-associatedLogicalNG-connectionList", kind: K::Alias { field: F::List(&F::Named(1239), 1, 65536), form: None, secret: false } },
/* 1241 */ T { name: "UEAggregateMaximumBitRate", kind: K::Sequence { names: &["uEAggregateMaximumBitRateDL", "uEAggregateMaximumBitRateUL", "iE-Extensions"], members: &[M { field: "ue_aggregate_maximum_bit_rate_dl", ty: F::Named(124), optional: false }, M { field: "ue_aggregate_maximum_bit_rate_ul", ty: F::Named(124), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1242 */ T { name: "UEAppLayerMeasConfigInfo", kind: K::Sequence { names: &["qoEReference", "serviceType", "areaScopeOfQMC", "measCollEntityIPAddress", "qoEMeasurementStatus", "containerForAppLayerMeasConfig", "measConfigAppLayerID", "sliceSupportListQMC", "mDT-AlignmentInfo", "availableRANVisibleQoEMetrics", "iE-Extensions"], members: &[M { field: "qoe_reference", ty: F::Named(955), optional: false }, M { field: "service_type", ty: F::Named(1120), optional: false }, M { field: "area_scope_of_qmc", ty: F::Named(100), optional: true }, M { field: "meas_coll_entity_ip_address", ty: F::Named(1230), optional: false }, M { field: "qoe_measurement_status", ty: F::Named(1243), optional: true }, M { field: "container_for_app_layer_meas_config", ty: F::Octets(1, 8000, false), optional: true }, M { field: "meas_config_app_layer_id", ty: F::Unsigned(0, 15, true), optional: true }, M { field: "slice_support_list_qmc", ty: F::Named(1129), optional: true }, M { field: "mdt_alignment_info", ty: F::Named(615), optional: true }, M { field: "available_ran_visible_qoe_metrics", ty: F::Named(115), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1243 */ T { name: "UEAppLayerMeasConfigInfoQoEMeasurementStatus", kind: K::Enumerated { names: &["ongoing"], root: 1, extensible: true } },
/* 1244 */ T { name: "UEAppLayerMeasInfoItem", kind: K::Sequence { names: &["uEAppLayerMeasConfigInfo", "iE-Extensions"], members: &[M { field: "ue_app_layer_meas_config_info", ty: F::Named(1242), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1245 */ T { name: "UEAppLayerMeasInfoList", kind: K::Alias { field: F::List(&F::Named(1244), 1, 16), form: None, secret: false } },
/* 1246 */ T { name: "UECapabilityInfoRequest", kind: K::Enumerated { names: &["requested"], root: 1, extensible: true } },
/* 1247 */ T { name: "UEContextModificationFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1248 */ T { name: "UEContextModificationRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1249 */ T { name: "UEContextModificationResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1250 */ T { name: "UEContextReleaseCommand", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1251 */ T { name: "UEContextReleaseComplete", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1252 */ T { name: "UEContextReleaseRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1253 */ T { name: "UEContextRequest", kind: K::Enumerated { names: &["requested"], root: 1, extensible: true } },
/* 1254 */ T { name: "UEContextResumeFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1255 */ T { name: "UEContextResumeRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1256 */ T { name: "UEContextResumeRequestTransfer", kind: K::Sequence { names: &["qosFlowFailedToResumeList", "iE-Extensions"], members: &[M { field: "qos_flow_failed_to_resume_list", ty: F::Named(981), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1257 */ T { name: "UEContextResumeResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1258 */ T { name: "UEContextResumeResponseTransfer", kind: K::Sequence { names: &["qosFlowFailedToResumeList", "iE-Extensions"], members: &[M { field: "qos_flow_failed_to_resume_list", ty: F::Named(981), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1259 */ T { name: "UEContextSuspendFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1260 */ T { name: "UEContextSuspendRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1261 */ T { name: "UEContextSuspendRequestTransfer", kind: K::Sequence { names: &["suspendIndicator", "iE-Extensions"], members: &[M { field: "suspend_indicator", ty: F::Named(1149), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1262 */ T { name: "UEContextSuspendResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1263 */ T { name: "UEDifferentiationInfoBatteryIndication", kind: K::Enumerated { names: &["battery-powered", "battery-powered-not-rechargeable-or-replaceable", "not-battery-powered"], root: 3, extensible: true } },
/* 1264 */ T { name: "UEDifferentiationInfoPeriodicCommunicationIndicator", kind: K::Enumerated { names: &["periodically", "ondemand"], root: 2, extensible: true } },
/* 1265 */ T { name: "UEDifferentiationInfoStationaryIndication", kind: K::Enumerated { names: &["stationary", "mobile"], root: 2, extensible: true } },
/* 1266 */ T { name: "UEDifferentiationInfoTrafficProfile", kind: K::Enumerated { names: &["single-packet", "dual-packets", "multiple-packets"], root: 3, extensible: true } },
/* 1267 */ T { name: "UEHistoryInformation", kind: K::Alias { field: F::List(&F::Named(524), 1, 16), form: None, secret: false } },
/* 1268 */ T { name: "UEHistoryInformationFromTheUE", kind: K::Choice { names: &["nR", "choice-Extensions"], alternatives: &[F::Named(729), F::Named(948)], root: 2, extensible: false } },
/* 1269 */ T { name: "UEIdentityIndexValue", kind: K::Choice { names: &["indexLength10", "choice-Extensions"], alternatives: &[F::Bits(10, 10), F::Named(948)], root: 2, extensible: false } },
/* 1270 */ T { name: "UEInformationTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1271 */ T { name: "UEPagingIdentity", kind: K::Choice { names: &["fiveG-S-TMSI", "choice-Extensions"], alternatives: &[F::Named(386), F::Named(948)], root: 2, extensible: false } },
/* 1272 */ T { name: "UEPresence", kind: K::Enumerated { names: &["in", "out", "unknown"], root: 3, extensible: true } },
/* 1273 */ T { name: "UEPresenceInAreaOfInterestItem", kind: K::Sequence { names: &["locationReportingReferenceID", "uEPresence", "iE-Extensions"], members: &[M { field: "location_reporting_reference_id", ty: F::Named(537), optional: false }, M { field: "ue_presence", ty: F::Named(1272), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1274 */ T { name: "UEPresenceInAreaOfInterestList", kind: K::Alias { field: F::List(&F::Named(1273), 1, 64), form: None, secret: false } },
/* 1275 */ T { name: "UERLFReportContainer", kind: K::Choice { names: &["nR", "lTE", "choice-Extensions"], alternatives: &[F::Named(732), F::Named(520), F::Named(948)], root: 3, extensible: false } },
/* 1276 */ T { name: "UERadioCapability", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 1277 */ T { name: "UERadioCapabilityCheckRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1278 */ T { name: "UERadioCapabilityCheckResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1279 */ T { name: "UERadioCapabilityForPaging", kind: K::Sequence { names: &["uERadioCapabilityForPagingOfNR", "uERadioCapabilityForPagingOfEUTRA", "iE-Extensions"], members: &[M { field: "ue_radio_capability_for_paging_of_nr", ty: F::Named(1282), optional: true }, M { field: "ue_radio_capability_for_paging_of_eutra", ty: F::Named(1280), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1280 */ T { name: "UERadioCapabilityForPagingOfEUTRA", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 1281 */ T { name: "UERadioCapabilityForPagingOfNB-IoT", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 1282 */ T { name: "UERadioCapabilityForPagingOfNR", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 1283 */ T { name: "UERadioCapabilityID", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 1284 */ T { name: "UERadioCapabilityIDMappingRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1285 */ T { name: "UERadioCapabilityIDMappingResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1286 */ T { name: "UERadioCapabilityInfoIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1287 */ T { name: "UERetentionInformation", kind: K::Enumerated { names: &["ues-retained"], root: 1, extensible: true } },
/* 1288 */ T { name: "UESecurityCapabilities", kind: K::Sequence { names: &["nRencryptionAlgorithms", "nRintegrityProtectionAlgorithms", "eUTRAencryptionAlgorithms", "eUTRAintegrityProtectionAlgorithms", "iE-Extensions"], members: &[M { field: "nr_encryption_algorithms", ty: F::Named(735), optional: false }, M { field: "nr_integrity_protection_algorithms", ty: F::Named(736), optional: false }, M { field: "eutra_encryption_algorithms", ty: F::Named(331), optional: false }, M { field: "eutra_integrity_protection_algorithms", ty: F::Named(332), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1289 */ T { name: "UESliceMaximumBitRateItem", kind: K::Sequence { names: &["s-NSSAI", "uESliceMaximumBitRateDL", "uESliceMaximumBitRateUL", "iE-Extensions"], members: &[M { field: "s_nssai", ty: F::Named(1074), optional: false }, M { field: "ue_slice_maximum_bit_rate_dl", ty: F::Named(124), optional: false }, M { field: "ue_slice_maximum_bit_rate_ul", ty: F::Named(124), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1290 */ T { name: "UESliceMaximumBitRateList", kind: K::Alias { field: F::List(&F::Named(1289), 1, 8), form: None, secret: false } },
/* 1291 */ T { name: "UETNLABindingReleaseRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1292 */ T { name: "UL-CP-SecurityInformation", kind: K::Sequence { names: &["ul-NAS-MAC", "ul-NAS-Count", "iE-Extensions"], members: &[M { field: "ul_nas_mac", ty: F::Named(1294), optional: false }, M { field: "ul_nas_count", ty: F::Named(1293), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1293 */ T { name: "UL-NAS-Count", kind: K::Alias { field: F::Bits(5, 5), form: Some(Form::Number(5)), secret: false } },
/* 1294 */ T { name: "UL-NAS-MAC", kind: K::Alias { field: F::Bits(16, 16), form: None, secret: false } },
/* 1295 */ T { name: "UL-NGU-UP-TNLModifyItem", kind: K::Sequence { names: &["uL-NGU-UP-TNLInformation", "dL-NGU-UP-TNLInformation", "iE-Extensions"], members: &[M { field: "ul_ngu_up_tnl_information", ty: F::Named(1298), optional: false }, M { field: "dl_ngu_up_tnl_information", ty: F::Named(1298), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1296 */ T { name: "UL-NGU-UP-TNLModifyList", kind: K::Alias { field: F::List(&F::Named(1295), 1, 4), form: None, secret: false } },
/* 1297 */ T { name: "ULForwarding", kind: K::Enumerated { names: &["ul-forwarding-proposed"], root: 1, extensible: true } },
/* 1298 */ T { name: "UPTransportLayerInformation", kind: K::Choice { names: &["gTPTunnel", "choice-Extensions"], alternatives: &[F::Named(415), F::Named(948)], root: 2, extensible: false } },
/* 1299 */ T { name: "UPTransportLayerInformationItem", kind: K::Sequence { names: &["nGU-UP-TNLInformation", "iE-Extensions"], members: &[M { field: "ngu_up_tnl_information", ty: F::Named(1298), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1300 */ T { name: "UPTransportLayerInformationList", kind: K::Alias { field: F::List(&F::Named(1299), 1, 3), form: None, secret: false } },
/* 1301 */ T { name: "UPTransportLayerInformationPairItem", kind: K::Sequence { names: &["uL-NGU-UP-TNLInformation", "dL-NGU-UP-TNLInformation", "iE-Extensions"], members: &[M { field: "ul_ngu_up_tnl_information", ty: F::Named(1298), optional: false }, M { field: "dl_ngu_up_tnl_information", ty: F::Named(1298), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1302 */ T { name: "UPTransportLayerInformationPairList", kind: K::Alias { field: F::List(&F::Named(1301), 1, 3), form: None, secret: false } },
/* 1303 */ T { name: "URI-address", kind: K::Alias { field: F::Text(Text::Visible, 0, UNBOUNDED, false), form: None, secret: false } },
/* 1304 */ T { name: "UnavailableGUAMIItem", kind: K::Sequence { names: &["gUAMI", "timerApproachForGUAMIRemoval", "backupAMFName", "iE-Extensions"], members: &[M { field: "guami", ty: F::Named(416), optional: false }, M { field: "timer_approach_for_guami_removal", ty: F::Named(1219), optional: true }, M { field: "backup_amf_name", ty: F::Named(46), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1305 */ T { name: "UnavailableGUAMIList", kind: K::Alias { field: F::List(&F::Named(1304), 1, 256), form: None, secret: false } },
/* 1306 */ T { name: "UnsuccessfulOutcome", kind: K::Sequence { names: &["procedureCode", "criticality", "value"], members: &[M { field: "procedure_code", ty: F::Named(942), optional: false }, M { field: "criticality", ty: F::Named(245), optional: false }, M { field: "value", ty: F::Open, optional: false }], extensible: false } },
/* 1307 */ T { name: "UpdateFeedback", kind: K::Alias { field: F::BitString(8, 8, true), form: None, secret: false } },
/* 1308 */ T { name: "UplinkNASTransport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1309 */ T { name: "UplinkNonUEAssociatedNRPPaTransport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1310 */ T { name: "UplinkRANConfigurationTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1311 */ T { name: "UplinkRANEarlyStatusTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1312 */ T { name: "UplinkRANStatusTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1313 */ T { name: "UplinkRIMInformationTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1314 */ T { name: "UplinkUEAssociatedNRPPaTransport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1315 */ T { name: "UserLocationInformation", kind: K::Choice { names: &["userLocationInformationEUTRA", "userLocationInformationNR", "userLocationInformationN3IWF-with-PortNumber", "choice-Extensions"], alternatives: &[F::Named(1316), F::Named(1319), F::Named(1317), F::Named(948)], root: 4, extensible: false } },
/* 1316 */ T { name: "UserLocationInformationEUTRA", kind: K::Sequence { names: &["eUTRA-CGI", "tAI", "timeStamp", "iE-Extensions"], members: &[M { field: "eutra_cgi", ty: F::Named(315), optional: false }, M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "time_stamp", ty: F::Named(1212), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1317 */ T { name: "UserLocationInformationN3IWF-with-PortNumber", kind: K::Sequence { names: &["iPAddress", "portNumber", "iE-Extensions"], members: &[M { field: "ip_address", ty: F::Named(1230), optional: false }, M { field: "port_number", ty: F::Named(932), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1318 */ T { name: "UserLocationInformationN3IWF-without-PortNumber", kind: K::Sequence { names: &["iPAddress", "tAI", "iE-Extensions"], members: &[M { field: "ip_address", ty: F::Named(1230), optional: false }, M { field: "tai", ty: F::Named(1154), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1319 */ T { name: "UserLocationInformationNR", kind: K::Sequence { names: &["nR-CGI", "tAI", "timeStamp", "iE-Extensions"], members: &[M { field: "nr_cgi", ty: F::Named(711), optional: false }, M { field: "tai", ty: F::Named(1154), optional: false }, M { field: "time_stamp", ty: F::Named(1212), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1320 */ T { name: "UserLocationInformationTNGF", kind: K::Sequence { names: &["tNAP-ID", "iPAddress", "portNumber", "iE-Extensions"], members: &[M { field: "tnap_id", ty: F::Named(1181), optional: false }, M { field: "ip_address", ty: F::Named(1230), optional: false }, M { field: "port_number", ty: F::Named(932), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1321 */ T { name: "UserLocationInformationTWIF", kind: K::Sequence { names: &["tWAP-ID", "iPAddress", "portNumber", "iE-Extensions"], members: &[M { field: "twap_id", ty: F::Named(1191), optional: false }, M { field: "ip_address", ty: F::Named(1230), optional: false }, M { field: "port_number", ty: F::Named(932), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1322 */ T { name: "UserLocationInformationW-AGF", kind: K::Choice { names: &["globalLine-ID", "hFCNode-ID", "choice-Extensions"], alternatives: &[F::Named(426), F::Named(434), F::Named(948)], root: 3, extensible: false } },
/* 1323 */ T { name: "UserPlaneErrorIndicator", kind: K::Enumerated { names: &["gTP-U-error-indication-received"], root: 1, extensible: true } },
/* 1324 */ T { name: "UserPlaneFailureIndication", kind: K::Sequence { names: &["userPlaneFailureType", "uL-NGU-UP-TNLInformation", "dL-NGU-UP-TNLInformation", "iE-Extensions"], members: &[M { field: "user_plane_failure_type", ty: F::Named(1326), optional: false }, M { field: "ul_ngu_up_tnl_information", ty: F::Named(1298), optional: false }, M { field: "dl_ngu_up_tnl_information", ty: F::Named(1298), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1325 */ T { name: "UserPlaneFailureIndicationReport", kind: K::Enumerated { names: &["qos-flows-and-tunnel-to-be-released", "qoS-flows-to-be-moved", "new-transport-address-allocated"], root: 3, extensible: true } },
/* 1326 */ T { name: "UserPlaneFailureType", kind: K::Enumerated { names: &["gtp-u-error-indication-received", "up-path-failure"], root: 2, extensible: true } },
/* 1327 */ T { name: "UserPlaneSecurityInformation", kind: K::Sequence { names: &["securityResult", "securityIndication", "iE-Extensions"], members: &[M { field: "security_result", ty: F::Named(1106), optional: false }, M { field: "security_indication", ty: F::Named(1104), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1328 */ T { name: "VehicleUE", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 1329 */ T { name: "VolumeTimedReport-Item", kind: K::Sequence { names: &["startTimeStamp", "endTimeStamp", "usageCountUL", "usageCountDL", "iE-Extensions"], members: &[M { field: "start_time_stamp", ty: F::OctetArray(4), optional: false }, M { field: "end_time_stamp", ty: F::OctetArray(4), optional: false }, M { field: "usage_count_ul", ty: F::Unsigned(0, 18446744073709551615, false), optional: false }, M { field: "usage_count_dl", ty: F::Unsigned(0, 18446744073709551615, false), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1330 */ T { name: "VolumeTimedReportList", kind: K::Alias { field: F::List(&F::Named(1329), 1, 2), form: None, secret: false } },
/* 1331 */ T { name: "W-AGF-ID", kind: K::Choice { names: &["w-AGF-ID", "choice-Extensions"], alternatives: &[F::BitString(16, 16, true), F::Named(948)], root: 2, extensible: false } },
/* 1332 */ T { name: "WLANMeasConfig", kind: K::Enumerated { names: &["setup"], root: 1, extensible: true } },
/* 1333 */ T { name: "WLANMeasConfigNameItem", kind: K::Sequence { names: &["wLANName", "iE-Extensions"], members: &[M { field: "wlan_name", ty: F::Named(1338), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1334 */ T { name: "WLANMeasConfigNameList", kind: K::Alias { field: F::List(&F::Named(1333), 1, 4), form: None, secret: false } },
/* 1335 */ T { name: "WLANMeasurementConfiguration", kind: K::Sequence { names: &["wlanMeasConfig", "wlanMeasConfigNameList", "wlan-rssi", "wlan-rtt", "iE-Extensions"], members: &[M { field: "wlan_meas_config", ty: F::Named(1332), optional: false }, M { field: "wlan_meas_config_name_list", ty: F::Named(1334), optional: true }, M { field: "wlan_rssi", ty: F::Named(1336), optional: true }, M { field: "wlan_rtt", ty: F::Named(1337), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1336 */ T { name: "WLANMeasurementConfigurationWlanRssi", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 1337 */ T { name: "WLANMeasurementConfigurationWlanRtt", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 1338 */ T { name: "WLANName", kind: K::Alias { field: F::Octets(1, 32, false), form: None, secret: false } },
/* 1339 */ T { name: "WUS-Assistance-Information", kind: K::Sequence { names: &["pagingProbabilityInformation", "iE-Extensions"], members: &[M { field: "paging_probability_information", ty: F::Named(914), optional: false }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1340 */ T { name: "WarningAreaCoordinates", kind: K::Alias { field: F::Octets(1, 1024, false), form: None, secret: false } },
/* 1341 */ T { name: "WarningAreaList", kind: K::Choice { names: &["eUTRA-CGIListForWarning", "nR-CGIListForWarning", "tAIListForWarning", "emergencyAreaIDList", "choice-Extensions"], alternatives: &[F::Named(317), F::Named(713), F::Named(1170), F::Named(344), F::Named(948)], root: 5, extensible: false } },
/* 1342 */ T { name: "WarningMessageContents", kind: K::Alias { field: F::Octets(1, 9600, false), form: None, secret: false } },
/* 1343 */ T { name: "WarningSecurityInfo", kind: K::Alias { field: F::OctetArray(50), form: None, secret: false } },
/* 1344 */ T { name: "WarningType", kind: K::Alias { field: F::OctetArray(2), form: None, secret: false } },
/* 1345 */ T { name: "WriteReplaceWarningRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1346 */ T { name: "WriteReplaceWarningResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(947), optional: false }], extensible: true } },
/* 1347 */ T { name: "XnExtTLA-Item", kind: K::Sequence { names: &["iPsecTLA", "gTP-TLAs", "iE-Extensions"], members: &[M { field: "ipsec_tla", ty: F::Named(1230), optional: true }, M { field: "gtp_tlas", ty: F::Named(1349), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1348 */ T { name: "XnExtTLAs", kind: K::Alias { field: F::List(&F::Named(1347), 1, 16), form: None, secret: false } },
/* 1349 */ T { name: "XnGTP-TLAs", kind: K::Alias { field: F::List(&F::Named(1230), 1, 16), form: None, secret: false } },
/* 1350 */ T { name: "XnTLAs", kind: K::Alias { field: F::List(&F::Named(1230), 1, 2), form: None, secret: false } },
/* 1351 */ T { name: "XnTNLConfigurationInfo", kind: K::Sequence { names: &["xnTransportLayerAddresses", "xnExtendedTransportLayerAddresses", "iE-Extensions"], members: &[M { field: "xn_transport_layer_addresses", ty: F::Named(1350), optional: false }, M { field: "xn_extended_transport_layer_addresses", ty: F::Named(1348), optional: true }, M { field: "ie_extensions", ty: F::Named(944), optional: true }], extensible: true } },
/* 1352 */ T { name: "XrDeviceWith2Rx", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
];
