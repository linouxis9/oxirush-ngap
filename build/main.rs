//! The generator of the bindings of the crate: the generator of oxirush-aper, which
//! oxirush-ngap and oxirush-s1ap share, with the names of NGAP. It runs rasn-compiler on
//! the ASN.1 of the specification, and writes the bindings, the registry, the inspection
//! and the oracle of the tests.

use generator::protocol::{Protocol, generate};

/// The IEs whose octets ASN.1 leaves open, each with the type of this module that TS
/// 38.413 gives them for a handover between NG-RAN nodes: clauses 9.3.1.20, 9.3.1.21
/// and 9.3.1.186. For another target system the octets are those of its own
/// specification.
const CARRIED: &[(&str, &str)] = &[
    (
        "SourceToTarget-TransparentContainer",
        "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer",
    ),
    (
        "TargetToSource-TransparentContainer",
        "TargetNGRANNode-ToSourceNGRANNode-TransparentContainer",
    ),
    (
        "TargettoSource-Failure-TransparentContainer",
        "TargetNGRANNode-ToSourceNGRANNode-FailureTransparentContainer",
    ),
];

/// The examples of the types that a program meets first, which the documentation of each
/// has after what the type is in ASN.1.
const EXAMPLES: &[(&str, &str)] = &[
    (
        "PDUSessionResourceSetupRequestTransfer",
        r#"A transfer of protocol IEs: the macros build and read its IEs as those of a message,
each with the criticality that its object set assigns, and `check` compares its octets
with that set.

```
use oxirush_ngap::aper::BitString;
use oxirush_ngap::{build_ngap_ie, ngap::*};

// The UPF's end of the tunnel, the type of the session, and its QoS flow
let tunnel = GTPTunnel::new(TransportLayerAddress(BitString::from([10, 0, 0, 1])), GTPTEID([0, 0, 0, 1]), None);
let flow = QosFlowSetupRequestItem::new(
    QosFlowIdentifier(1),
    QosFlowLevelQosParameters::new(
        QosCharacteristics::nonDynamic5QI(NonDynamic5QIDescriptor::new(FiveQI(9), None, None, None, None)),
        AllocationAndRetentionPriority::new(PriorityLevelARP(8),
            PreEmptionCapability::shall_not_trigger_pre_emption,
            PreEmptionVulnerability::not_pre_emptable, None),
        None, None, None, None,
    ),
    None,
    None,
);
let transfer = PDUSessionResourceSetupRequestTransfer::new(ProtocolIEContainer(vec![
    build_ngap_ie!(PDUSessionResourceSetupRequestTransfer,
        UL_NGU_UP_TNLInformation(UPTransportLayerInformation::gTPTunnel(tunnel)))?,
    build_ngap_ie!(PDUSessionResourceSetupRequestTransfer, PDUSessionType(PDUSessionType::ipv4))?,
    build_ngap_ie!(PDUSessionResourceSetupRequestTransfer,
        QosFlowSetupRequestList(QosFlowSetupRequestList(vec![flow])))?,
]));

// The octets that an item of a PDU SESSION RESOURCE SETUP REQUEST holds
let octets = encode_open_type(&transfer)?;
assert!(PDUSessionResourceSetupRequestTransfer::check(&octets)?.is_empty());

// Without its QoS flows, a mandatory IE
let mut partial = transfer.clone();
partial.protocol_ies.0.pop();
let found = PDUSessionResourceSetupRequestTransfer::check(&encode_open_type(&partial)?)?;
assert_eq!(found[0].to_string(), "QosFlowSetupRequestList (IE 136) is mandatory, and the message does not have it");
# Ok::<(), oxirush_ngap::Error>(())
```"#,
    ),
    (
        "PDUSessionResourceSetupItemSUReq",
        r#"```
use oxirush_ngap::helpers::s_nssai;
use oxirush_ngap::{ie, ngap::*};

// PDU session 5 of the slice of SST 1, with the octets of its transfer: here one of
// the PDU session type alone
let transfer = hex::decode("0000010086000100").unwrap();
let item = PDUSessionResourceSetupItemSUReq::new(PDUSessionID(5), None, s_nssai(1, None), transfer.into(), None);

// The transfer, read from the item
let read: PDUSessionResourceSetupRequestTransfer =
    decode_open_type(&item.pdu_session_resource_setup_request_transfer)?;
assert_eq!(read.ie::<ie::PDUSessionType>().unwrap()?, PDUSessionType::ipv4);
# Ok::<(), oxirush_ngap::Error>(())
```"#,
    ),
    (
        "GTPTunnel",
        r#"```
use oxirush_ngap::aper::BitString;
use oxirush_ngap::ngap::*;
use std::net::Ipv4Addr;

// The end of a tunnel: an IPv4 address in 32 bits, and its TEID
let address = Ipv4Addr::new(10, 0, 0, 1);
let tunnel = GTPTunnel::new(
    TransportLayerAddress(BitString::from(address.octets())),
    GTPTEID(0x2001u32.to_be_bytes()),
    None,
);
assert_eq!(tunnel.transport_layer_address.0.len(), 32);
assert_eq!(tunnel.transport_layer_address.0.as_octets(), address.octets());
assert_eq!(u32::from_be_bytes(tunnel.gtp_teid.0), 0x2001);
```"#,
    ),
    (
        "UserLocationInformationNR",
        r#"```
use oxirush_ngap::helpers::{nr_cgi, plmn, tai};
use oxirush_ngap::ngap::*;

// The cell and the tracking area of a UE, as an INITIAL UE MESSAGE has them
let network = plmn("208", "93")?;
let location = UserLocationInformationNR::new(
    nr_cgi(network, 0x123456, 24, 1)?,
    tai(network, [0x00, 0x00, 0x01]),
    None,
    None,
);
let ie = UserLocationInformation::userLocationInformationNR(location);
assert_eq!(hex::encode(encode_open_type(&ie)?), "4002f839123456001002f839000001");
# Ok::<(), oxirush_ngap::Error>(())
```"#,
    ),
];

fn main() -> anyhow::Result<()> {
    generate(&Protocol {
        name: "NGAP",
        modules: "ngap",
        carried: CARRIED,
        keys: &["SecurityKey"],
        plmn: "PLMNIdentity",
        aliases: &[
            ("AMF_UE_NGAP_ID", "AMFUENGAPID"),
            ("RAN_UE_NGAP_ID", "RANUENGAPID"),
        ],
        release_request: "002a4015000003000a0002002a005500020007000f40020500",
        release_display: "UEContextReleaseRequest (procedure 42, initiating message), AMF-UE-NGAP-ID 42, RAN-UE-NGAP-ID 7",
        other_message: "NGSetupRequest",
        no_message: "PDUSessionResourceSetupRequestTransfer",
        examples: EXAMPLES,
        // The bindings that rasn-compiler resolves need nothing else.
        correct: Ok,
    })
}
