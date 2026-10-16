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

//! What the benchmarks have of NGAP: the type of a PDU, four messages of the
//! size that a network sends, and the operations that take the type of a
//! message. `lib.rs`, which is the same for oxirush-s1ap, has the rest.

pub use oxirush_ngap as codec;

use crate::{Typed, Written, done};
use codec::ngap;
use codec::{Error, build_ngap, extract_ngap_ies};
use serde_json::{Value, json};

/// A PDU of the protocol, which owns its octets.
pub type Pdu = ngap::NGAP_PDU;

/// A PDU that borrows its message from the octets that it is decoded from.
pub type Borrowed<'a> = ngap::NGAPPDU<'a>;

/// The fixtures of the crate: a line for each message, with its name and its
/// octets in hexadecimal.
pub const FIXTURES: &str = include_str!("../../tests/fixtures/messages.tsv");

/// The root of the paths of the IEs of a message.
pub const ROOT: &str = "/ngap";

/// The operations that take the type of a message: its direction, its
/// procedure, the criticality of its PDU and its type, then each of its IEs
/// as the macros name it, with its criticality and the type of its value.
macro_rules! typed {
    ($direction:ident $procedure:ident $criticality:ident $message:ident {
        $($ie_criticality:ident $ie:ident: $type:ident $(<$borrows:lifetime>)?,)*
    }) => {{
        /// The values of the IEs of the message, in their order. Those that hold
        /// octets borrow them.
        type Values<'a> = ($(ngap::$type $(<$borrows>)?,)*);

        #[allow(non_snake_case)]
        fn read<'a>(message: &'a ngap::$message<'a>) -> Result<Values<'a>, Error> {
            extract_ngap_ies!(message, $message,
                $(req $ie: ngap::$type $(<$borrows>)? = $ie(value) => value,)*
            );
            Ok(($($ie,)*))
        }

        /// The same values, which borrow nothing.
        #[allow(non_snake_case)]
        fn owned(($($ie,)*): Values<'_>) -> Values<'static> {
            ($(typed!(@owned $ie $(, $borrows)?),)*)
        }

        #[allow(non_snake_case)]
        fn build(($($ie,)*): Values<'_>) -> Result<Pdu, Error> {
            build_ngap!($direction, $procedure, $criticality, $message,
                $($ie_criticality $ie($ie),)*
            )
        }

        fn message(pdu: &Pdu) -> ngap::$message<'_> {
            pdu.decode_value().unwrap()
        }

        Typed {
            decode_value: |pdu| done(pdu.decode_value::<ngap::$message<'_>>()),
            extract: |pdu| {
                let message = message(pdu);
                Box::new(move || done(read(&message)))
            },
            build: |pdu| {
                let mut values = Some(owned(read(&message(pdu)).unwrap()));
                Box::new(move || done(build(values.take().unwrap())))
            },
            rebuilt: |pdu| build(read(&message(pdu)).unwrap()).unwrap(),
        }
    }};
    (@owned $value:ident) => {
        $value
    };
    (@owned $value:ident, $borrows:lifetime) => {
        $value.into_owned()
    };
}

/// `length` octets in hexadecimal, which are not all the same.
fn octets(length: usize) -> String {
    (0..length)
        .map(|at| format!("{:02X}", (at * 7 + 3) % 251))
        .collect()
}

/// A QoS flow of a standardized 5QI.
fn qos_flow(identifier: u8, five_qi: u8) -> Value {
    json!({
        "qosFlowIdentifier": identifier,
        "qosFlowLevelQosParameters": {
            "qosCharacteristics": {"nonDynamic5QI": {"fiveQI": five_qi}},
            "allocationAndRetentionPriority": {
                "priorityLevelARP": 8,
                "pre-emptionCapability": "shall-not-trigger-pre-emption",
                "pre-emptionVulnerability": "not-pre-emptable",
            },
        },
    })
}

/// A PDU SESSION RESOURCE SETUP REQUEST TRANSFER: the bit rates of the
/// session, the tunnel of its UPF, its type and two QoS flows.
fn setup_request_transfer(session: u8) -> Value {
    json!({"protocolIEs": [
        {"PDUSessionAggregateMaximumBitRate": {
            "pDUSessionAggregateMaximumBitRateDL": 1_000_000_000,
            "pDUSessionAggregateMaximumBitRateUL": 500_000_000,
        }, "criticality": "reject"},
        {"UL-NGU-UP-TNLInformation": {"gTPTunnel": {
            "transportLayerAddress": format!("10.0.0.{session}"),
            "gTP-TEID": 0x0100_0000 + u32::from(session),
        }}, "criticality": "reject"},
        {"PDUSessionType": "ipv4", "criticality": "reject"},
        {"QosFlowSetupRequestList": [qos_flow(1, 9), qos_flow(2, 5)], "criticality": "reject"},
    ]})
}

/// The bit rates of a UE.
fn ue_bit_rates() -> Value {
    json!({
        "uEAggregateMaximumBitRateDL": 2_000_000_000,
        "uEAggregateMaximumBitRateUL": 1_000_000_000,
    })
}

/// An INITIAL CONTEXT SETUP REQUEST with two PDU sessions, the radio
/// capability of the UE and a NAS-PDU of 1,500 octets.
fn initial_context_setup_request() -> Written {
    let session = |id: u8| {
        json!({
            "pDUSessionID": id,
            "s-NSSAI": {"sST": 1, "sD": "000001"},
            "pDUSessionResourceSetupRequestTransfer": setup_request_transfer(id),
        })
    };
    let tree = json!({"message": "InitialContextSetupRequest", "ies": [
        {"AMF-UE-NGAP-ID": 1_099_511_627_775u64},
        {"RAN-UE-NGAP-ID": 4_000_000_000u32},
        {"UEAggregateMaximumBitRate": ue_bit_rates()},
        {"GUAMI": {"pLMNIdentity": "208-93", "aMFRegionID": 2, "aMFSetID": 1, "aMFPointer": 0}},
        {"PDUSessionResourceSetupListCxtReq": [session(1), session(2)]},
        {"AllowedNSSAI": [{"s-NSSAI": {"sST": 1, "sD": "000001"}}, {"s-NSSAI": {"sST": 2}}]},
        {"UESecurityCapabilities": {
            "nRencryptionAlgorithms": "E000",
            "nRintegrityProtectionAlgorithms": "E000",
            "eUTRAencryptionAlgorithms": "E000",
            "eUTRAintegrityProtectionAlgorithms": "E000",
        }},
        {"SecurityKey": octets(32)},
        {"MobilityRestrictionList": {"servingPLMN": "208-93"}},
        {"UERadioCapability": octets(400)},
        {"MaskedIMEISV": "3554900000FFFF10"},
        {"NAS-PDU": octets(1500)},
    ]});
    Written {
        name: "initial-context-setup-request",
        what: "INITIAL CONTEXT SETUP REQUEST with 2 PDU sessions and a NAS-PDU of 1,500 octets",
        tree,
        path: "/ngap/PDUSessionResourceSetupListCxtReq/1/pDUSessionResourceSetupRequestTransfer\
               /QosFlowSetupRequestList/1/qosFlowLevelQosParameters\
               /allocationAndRetentionPriority/priorityLevelARP",
        edit: (
            "/ngap/PDUSessionResourceSetupListCxtReq/1/pDUSessionResourceSetupRequestTransfer\
             /QosFlowSetupRequestList/1/qosFlowLevelQosParameters\
             /allocationAndRetentionPriority/priorityLevelARP",
            json!(3),
        ),
        typed: typed!(InitiatingMessage InitialContextSetup REJECT InitialContextSetupRequest {
            REJECT AMF_UE_NGAP_ID: AMFUENGAPID,
            REJECT RAN_UE_NGAP_ID: RANUENGAPID,
            REJECT UEAggregateMaximumBitRate: UEAggregateMaximumBitRate,
            REJECT GUAMI: GUAMI,
            REJECT PDUSessionResourceSetupListCxtReq: PDUSessionResourceSetupListCxtReq<'a>,
            REJECT AllowedNSSAI: AllowedNSSAI,
            REJECT UESecurityCapabilities: UESecurityCapabilities<'a>,
            REJECT SecurityKey: SecurityKey<'a>,
            IGNORE MobilityRestrictionList: MobilityRestrictionList<'a>,
            IGNORE UERadioCapability: UERadioCapability<'a>,
            IGNORE MaskedIMEISV: MaskedIMEISV,
            IGNORE NAS_PDU: NASPDU<'a>,
        }),
    }
}

/// A PDU SESSION RESOURCE SETUP REQUEST with eight PDU sessions, each with
/// its NAS-PDU and its transfer.
fn pdu_session_resource_setup_request() -> Written {
    let session = |id: u8| {
        json!({
            "pDUSessionID": id,
            "pDUSessionNAS-PDU": octets(96),
            "s-NSSAI": {"sST": 1, "sD": "000001"},
            "pDUSessionResourceSetupRequestTransfer": setup_request_transfer(id),
        })
    };
    let tree = json!({"message": "PDUSessionResourceSetupRequest", "ies": [
        {"AMF-UE-NGAP-ID": 1_099_511_627_775u64},
        {"RAN-UE-NGAP-ID": 4_000_000_000u32},
        {"PDUSessionResourceSetupListSUReq": (1..=8).map(session).collect::<Vec<_>>()},
        {"UEAggregateMaximumBitRate": ue_bit_rates()},
    ]});
    Written {
        name: "pdu-session-resource-setup-request",
        what: "PDU SESSION RESOURCE SETUP REQUEST with 8 PDU sessions and their transfers",
        tree,
        path: "/ngap/PDUSessionResourceSetupListSUReq/7/pDUSessionResourceSetupRequestTransfer\
               /UL-NGU-UP-TNLInformation/gTPTunnel/gTP-TEID",
        edit: (
            "/ngap/PDUSessionResourceSetupListSUReq/7/pDUSessionResourceSetupRequestTransfer\
             /UL-NGU-UP-TNLInformation/gTPTunnel/gTP-TEID",
            json!(0x0200_0008),
        ),
        typed: typed!(
            InitiatingMessage PDUSessionResourceSetup REJECT PDUSessionResourceSetupRequest {
                REJECT AMF_UE_NGAP_ID: AMFUENGAPID,
                REJECT RAN_UE_NGAP_ID: RANUENGAPID,
                REJECT PDUSessionResourceSetupListSUReq: PDUSessionResourceSetupListSUReq<'a>,
                IGNORE UEAggregateMaximumBitRate: UEAggregateMaximumBitRate,
            }
        ),
    }
}

/// How many octets the RRC container of the HANDOVER REQUIRED has, for a
/// transparent container of 4,000 octets.
const RRC_CONTAINER: usize = 3_964;

/// A HANDOVER REQUIRED between two NG-RAN nodes, with two PDU sessions and a
/// transparent container of 4,000 octets, most of which is its RRC container.
fn handover_required() -> Written {
    let cell =
        |identity: u64| json!({"nR-CGI": {"pLMNIdentity": "208-93", "nRCellIdentity": identity}});
    let container = json!({"SourceNGRANNode-ToTargetNGRANNode-TransparentContainer": {
        "rRCContainer": octets(RRC_CONTAINER),
        "pDUSessionResourceInformationList": [
            {"pDUSessionID": 1, "qosFlowInformationList": [
                {"qosFlowIdentifier": 1}, {"qosFlowIdentifier": 2},
            ]},
            {"pDUSessionID": 2, "qosFlowInformationList": [
                {"qosFlowIdentifier": 1}, {"qosFlowIdentifier": 2},
            ]},
        ],
        "targetCell-ID": cell(0x0004_0001),
        "uEHistoryInformation": [{"lastVisitedCellInformation": {"nGRANCell": {
            "globalCellID": cell(0x0003_0001),
            "cellType": {"cellSize": "medium"},
            "timeUEStayedInCell": 600,
        }}}],
    }});
    let tree = json!({"message": "HandoverRequired", "ies": [
        {"AMF-UE-NGAP-ID": 1_099_511_627_775u64},
        {"RAN-UE-NGAP-ID": 4_000_000_000u32},
        {"HandoverType": "intra5gs"},
        {"Cause": {"radioNetwork": "handover-desirable-for-radio-reason"}},
        {"TargetID": {"targetRANNodeID": {
            "globalRANNodeID": {"globalGNB-ID": {
                "pLMNIdentity": "208-93",
                "gNB-ID": {"gNB-ID": {"value": "000010", "length": 22}},
            }},
            "selectedTAI": {"pLMNIdentity": "208-93", "tAC": 1},
        }}},
        {"DirectForwardingPathAvailability": "direct-path-available"},
        {"PDUSessionResourceListHORqd": [
            {"pDUSessionID": 1, "handoverRequiredTransfer": {
                "directForwardingPathAvailability": "direct-path-available",
            }},
            {"pDUSessionID": 2, "handoverRequiredTransfer": {}},
        ]},
        {"SourceToTarget-TransparentContainer": container},
    ]});
    Written {
        name: "handover-required",
        what: "HANDOVER REQUIRED with a transparent container of 4,000 octets",
        tree,
        path: "/ngap/TargetID/targetRANNodeID/selectedTAI/tAC",
        edit: ("/ngap/TargetID/targetRANNodeID/selectedTAI/tAC", json!(2)),
        typed: typed!(InitiatingMessage HandoverPreparation REJECT HandoverRequired {
            REJECT AMF_UE_NGAP_ID: AMFUENGAPID,
            REJECT RAN_UE_NGAP_ID: RANUENGAPID,
            REJECT HandoverType: HandoverType,
            IGNORE Cause: Cause,
            REJECT TargetID: TargetID,
            IGNORE DirectForwardingPathAvailability: DirectForwardingPathAvailability,
            REJECT PDUSessionResourceListHORqd: PDUSessionResourceListHORqd<'a>,
            REJECT SourceToTarget_TransparentContainer: SourceToTargetTransparentContainer<'a>,
        }),
    }
}

/// A PAGING for sixteen tracking areas.
fn paging() -> Written {
    let area = |tac: u32| json!({"tAI": {"pLMNIdentity": "208-93", "tAC": tac}});
    let tree = json!({"message": "Paging", "ies": [
        {"UEPagingIdentity": {"fiveG-S-TMSI": {
            "aMFSetID": 1, "aMFPointer": 0, "fiveG-TMSI": 3_661_752_294u32,
        }}},
        {"PagingDRX": "v128"},
        {"TAIListForPaging": (1..=16).map(area).collect::<Vec<_>>()},
        {"PagingPriority": "priolevel1"},
    ]});
    Written {
        name: "paging",
        what: "PAGING for 16 tracking areas",
        tree,
        path: "/ngap/TAIListForPaging/15/tAI/tAC",
        edit: ("/ngap/TAIListForPaging/15/tAI/tAC", json!(17)),
        typed: typed!(InitiatingMessage Paging IGNORE Paging {
            IGNORE UEPagingIdentity: UEPagingIdentity,
            IGNORE PagingDRX: PagingDRX,
            IGNORE TAIListForPaging: TAIListForPaging,
            IGNORE PagingPriority: PagingPriority,
        }),
    }
}

/// An UPLINK NAS TRANSPORT of `count` IEs of one name: AMF-UE-NGAP-ID, the
/// smallest IE that has a name and a value.
pub(crate) fn many_ies(count: usize) -> Pdu {
    let (id, value) = (ngap::ProtocolIEID(10), vec![0x00, 0x01]);
    let ie = ngap::ProtocolIEField::new(id, ngap::Criticality::reject, value.into());
    let mut pdu = Pdu::decode(&[0x00, 0x2e, 0x40, 0x03, 0x00, 0x00, 0x00]).unwrap();
    pdu.set_protocol_ies(ngap::ProtocolIEContainer(vec![ie; count]))
        .unwrap();
    pdu
}

/// The messages of the size that a network sends.
pub(crate) fn written() -> Vec<Written> {
    vec![
        initial_context_setup_request(),
        pdu_session_resource_setup_request(),
        handover_required(),
        paging(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use codec::inspect;

    /// How many octets a string of a view is, which has them in hexadecimal.
    fn length(view: &Value, path: &str) -> usize {
        let selected = inspect::select(view, path).unwrap();
        selected[0].as_str().unwrap().len() / 2
    }

    /// How many entries a list of a view has.
    fn entries(view: &Value, path: &str) -> usize {
        inspect::select(view, path).unwrap().len()
    }

    /// The messages have the sizes that their inputs are named after.
    #[test]
    fn the_written_messages_have_the_sizes_that_they_say() {
        let view = |written: Written| Pdu::from_tree(&written.tree).unwrap().to_view().unwrap();

        let setup = view(initial_context_setup_request());
        assert_eq!(length(&setup, "/ngap/NAS-PDU/value"), 1_500);
        assert_eq!(
            entries(&setup, "/ngap/PDUSessionResourceSetupListCxtReq/value/*"),
            2
        );

        let sessions = view(pdu_session_resource_setup_request());
        assert_eq!(
            entries(&sessions, "/ngap/PDUSessionResourceSetupListSUReq/value/*"),
            8
        );

        let handover = view(handover_required());
        let container = "/ngap/SourceToTarget-TransparentContainer/value";
        assert_eq!(length(&handover, container), 4_000);

        let paging = view(paging());
        assert_eq!(entries(&paging, "/ngap/TAIListForPaging/value/*"), 16);
    }
}
