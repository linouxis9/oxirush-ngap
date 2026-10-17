/*
   OxiRush
   Copyright 2025-2026 Valentin D'Emmanuele

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

// The messages of `readers`, whose program oxirush-aper has (`src/bin/readers.rs` of its
// `protocols/benches/`) and includes this file: the four that `protocol.rs` writes, each
// with its IEs, the first first.

readers!(initial_context_setup_request InitialContextSetupRequest {
    AMF_UE_NGAP_ID: AMFUENGAPID,
    RAN_UE_NGAP_ID: RANUENGAPID,
    UEAggregateMaximumBitRate: UEAggregateMaximumBitRate,
    GUAMI: GUAMI,
    PDUSessionResourceSetupListCxtReq: PDUSessionResourceSetupListCxtReq<'a>,
    AllowedNSSAI: AllowedNSSAI,
    UESecurityCapabilities: UESecurityCapabilities<'a>,
    SecurityKey: SecurityKey<'a>,
    MobilityRestrictionList: MobilityRestrictionList<'a>,
    UERadioCapability: UERadioCapability<'a>,
    MaskedIMEISV: MaskedIMEISV,
    NAS_PDU: NASPDU<'a>,
});

readers!(pdu_session_resource_setup_request PDUSessionResourceSetupRequest {
    AMF_UE_NGAP_ID: AMFUENGAPID,
    RAN_UE_NGAP_ID: RANUENGAPID,
    PDUSessionResourceSetupListSUReq: PDUSessionResourceSetupListSUReq<'a>,
    UEAggregateMaximumBitRate: UEAggregateMaximumBitRate,
});

readers!(handover_required HandoverRequired {
    AMF_UE_NGAP_ID: AMFUENGAPID,
    RAN_UE_NGAP_ID: RANUENGAPID,
    HandoverType: HandoverType,
    Cause: Cause,
    TargetID: TargetID,
    DirectForwardingPathAvailability: DirectForwardingPathAvailability,
    PDUSessionResourceListHORqd: PDUSessionResourceListHORqd<'a>,
    SourceToTarget_TransparentContainer: SourceToTargetTransparentContainer<'a>,
});

readers!(paging Paging {
    UEPagingIdentity: UEPagingIdentity,
    PagingDRX: PagingDRX,
    TAIListForPaging: TAIListForPaging,
    PagingPriority: PagingPriority,
});

/// The operations on each message, by its name in `written()`, and the check that the
/// readers read what the macro reads.
const MESSAGES: &[Message] = &[
    (
        "initial-context-setup-request",
        initial_context_setup_request::OPERATIONS,
        initial_context_setup_request::same,
    ),
    (
        "pdu-session-resource-setup-request",
        pdu_session_resource_setup_request::OPERATIONS,
        pdu_session_resource_setup_request::same,
    ),
    (
        "handover-required",
        handover_required::OPERATIONS,
        handover_required::same,
    ),
    ("paging", paging::OPERATIONS, paging::same),
];
