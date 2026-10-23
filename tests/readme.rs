// The Rust blocks of README.md, each as a function, with what the README shows under
// each: `../oxirush-aper/protocols/tests/readme.py` writes this file from the README and
// says how. Do not edit it.
#![cfg(feature = "serde")]
#![allow(unused)]

use std::cell::RefCell;

thread_local!(static PRINTED: RefCell<String> = const { RefCell::new(String::new()) });

// What a block prints is kept, to be compared with what the README shows. The line
// is written before it is kept, so that `?` in what is printed returns from the block.
macro_rules! print {
    ($($argument:tt)*) => {{
        let text = format!($($argument)*);
        PRINTED.with(|printed| printed.borrow_mut().push_str(&text))
    }};
}

macro_rules! println {
    () => {
        print!("\n")
    };
    ($($argument:tt)*) => {{
        let line = format!($($argument)*);
        print!("{line}\n")
    }};
}

/// What `block` prints.
fn printed(block: fn()) -> String {
    PRINTED.with(|printed| printed.borrow_mut().clear());
    block();
    PRINTED.with(|printed| printed.take())
}

#[test]
fn each_block_prints_what_the_readme_shows() {
    let mut differences = String::new();
    for (rank, (block, shown)) in (1..).zip(BLOCKS) {
        let printed = printed(*block);
        if printed != *shown {
            differences += &format!("the Rust block {rank} of README.md prints:\n{printed}");
            differences += &format!("and the README shows:\n{shown}\n");
        }
    }
    assert!(differences.is_empty(), "{differences}");
}

/// Each Rust block of README.md, by its rank, and what the README shows under it.
#[rustfmt::skip]
const BLOCKS: &[(fn(), &str)] = &[
    (block_1, r#"UEContextReleaseRequest
AMF=Some(42) RAN=Some(7) cause=Some(radioNetwork(user_inactivity))
"#),
    (block_2, r#"NG SETUP REQUEST from gnb-1: 1 tracking area
NG SETUP RESPONSE 2015002b000004000100070200616d662d3100600008000002f83901004000564001ff005000080002f83900000008
INITIAL UE MESSAGE of RAN UE 7: NAS-PDU 7e004179000d0102f8390000000000000010022e08a020000000000000
DOWNLINK NAS TRANSPORT 0004403e000003000a000200010055000200070026002b2a7e0056000200002123553cbe9637a89d218ae64dae47bf35201055f328b43577b9b94a9ffac354dfafb3
INITIAL CONTEXT SETUP REQUEST, 0 deviations: KgNB c18166e13cfde1dea842c708f6e0b3372b6896664df4e865c0c78bd270852005
"#),
    (block_3, r#"a release request with 3 IEs
Unsuccessful_NGSetup is not handled here
"#),
    (block_4, r#"ignore 000f401100000200550002000700260004037e0041
true
TimeToWait is no IE of the object set of InitialUEMessage, which assigns it no criticality: write its criticality
"#),
    (block_5, r#"AMF=42 RAN=7 cause=radioNetwork(user_inactivity)
"#),
    (block_6, r#"AMF=42 RAN=7
radioNetwork(user_inactivity)
"#),
    (block_7, r#"ipv4
0000010086000110
UL-NGU-UP-TNLInformation (IE 139) is mandatory, and the message does not have it
QosFlowSetupRequestList (IE 136) is mandatory, and the message does not have it
nR_CGI(NRCGI { plmn_identity: PLMNIdentity(208-93), nr_cell_identity: NRCellIdentity(0x1234/36), ie_extensions: None })
"#),
    (block_8, r#"the criticality of the PDU is reject, and that of its procedure is ignore
AMF-UE-NGAP-ID (IE 10), at position 1, comes after RAN-UE-NGAP-ID (IE 85), which the object set of the message has after it
the criticality of AMF-UE-NGAP-ID (IE 10), at position 1, is ignore, and the message assigns it reject
TimeToWait (IE 107), at position 2, is not an IE of the message
RAN-UE-NGAP-ID (IE 85), at position 3, is in the message already, at position 0
Cause (IE 15) is mandatory, and the message does not have it
"#),
    (block_9, r#"GlobalRANNodeID (IE 27) is mandatory, and the message does not have it
SupportedTAList (IE 102) is mandatory, and the message does not have it
40150015000002000f40016200134009180100001b40006640
"#),
    (block_10, r#"{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
/procedure_code = 42
/direction = "InitiatingMessage"
/criticality = "ignore"
/ngap/AMF-UE-NGAP-ID/id = 10
/ngap/AMF-UE-NGAP-ID/criticality = "reject"
/ngap/AMF-UE-NGAP-ID/value = 42
/ngap/RAN-UE-NGAP-ID/id = 85
/ngap/RAN-UE-NGAP-ID/criticality = "reject"
/ngap/RAN-UE-NGAP-ID/value = 7
/ngap/Cause/id = 15
/ngap/Cause/criticality = "ignore"
/ngap/Cause/value/radioNetwork = "user-inactivity"
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":9},{"Cause":{"nas":"normal-release"}}]}
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions; an IE itself has id, criticality, value, octets, at /ngap/Cause/radioNetwrok
"#),
    (block_11, r#"002a4020000004000a0002002a005500020007000f40020500005240070200676e622d31
"InitiatingMessage" 42 "ignore"
["reject","reject","ignore","ignore"]
"#),
    (block_12, r#"002a4010000002000a0002002afde80003c0ffee
{"id":65000,"criticality":"reject","value":"c0ffee","_decode_error":"NGAP IE 65000 is unknown","_raw_value":"c0ffee","_original_id":65000}
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":255},{"65000":{"octets":"c0ffee"},"criticality":"reject"}]}
"#),
    (block_13, r#"{"sST":1}
"#),
];

/// The Rust block 1 of README.md.
#[rustfmt::skip]
fn block_1() {
    use oxirush_ngap::{build_ngap, extract_ngap_ies, ngap::*};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Decode a UE CONTEXT RELEASE REQUEST
        let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500")?;
        let pdu = NGAP_PDU::decode(&bytes)?;
        assert_eq!((pdu.procedure_code(), pdu.is_initiating()), (42, true));
        println!("{}", pdu.procedure_name());

        // Read its IEs by name. An `opt` field is `None` when its IE is not there,
        // and what follows `=>` is what the field keeps of the decoded value
        let request: UEContextReleaseRequest = pdu.decode_value()?;
        extract_ngap_ies!(request, UEContextReleaseRequest,
            opt amf_id: u64 = AMF_UE_NGAP_ID(id),
            opt ran_id: u32 = RAN_UE_NGAP_ID(id),
            opt cause: Cause = Cause(cause) => cause,
        );
        println!("AMF={amf_id:?} RAN={ran_id:?} cause={cause:?}");

        // Build it with another cause: the direction, the procedure, the criticality
        // of the PDU and the message, then each IE with its criticality
        let rebuilt = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
            IGNORE, UEContextReleaseRequest,

            REJECT AMF_UE_NGAP_ID(42u64),
            REJECT RAN_UE_NGAP_ID(7u32),
            IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::radio_connection_with_ue_lost)),
        )?;

        // Encode it
        let encoded = hex::encode(rebuilt.encode()?);
        assert_eq!(encoded, "002a4015000003000a0002002a005500020007000f40020540");
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 2 of README.md.
#[rustfmt::skip]
fn block_2() {
    use oxirush_ngap::helpers::*;
    use oxirush_ngap::{build_ngap, extract_ngap_ies, ie, ngap::*};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let network = plmn("208", "93")?;
        let slices = || SliceSupportList(vec![SliceSupportItem::new(s_nssai(1, None), None)]);

        // The gNB: NG SETUP REQUEST, its identity, its name, and the slices of
        // the tracking area that it serves
        let gnb = GlobalRANNodeID::globalGNB_ID(global_gnb_id(network, 0x123456, 24)?);
        let plmns = BroadcastPLMNList(vec![BroadcastPLMNItem::new(network, slices(), None)]);
        let area = SupportedTAItem::new(TAC([0, 0, 1]), plmns, None);
        let request = build_ngap!(InitiatingMessage, NGSetup, NGSetupRequest,
            GlobalRANNodeID(gnb),
            RANNodeName("gnb-1"),
            SupportedTAList(SupportedTAList(vec![area])),
            DefaultPagingDRX(PagingDRX::v128),
        )?;

        // The AMF reads it, and answers with its name, its GUAMI and its slices
        let received = NGAP_PDU::decode(&request.encode()?)?;
        extract_ngap_ies!(received, NGSetupRequest,
            opt name: String = RANNodeName(name) => name.0.into_owned(),
            req areas: SupportedTAList = SupportedTAList(areas) => areas,
        );
        println!("NG SETUP REQUEST from {}: {} tracking area", name.unwrap_or_default(), areas.0.len());
        let response = build_ngap!(SuccessfulOutcome, NGSetup, NGSetupResponse,
            AMFName("amf-1"),
            ServedGUAMIList(ServedGUAMIList(vec![ServedGUAMIItem::new(guami(network, 1, 1, 0)?, None, None)])),
            RelativeAMFCapacity(RelativeAMFCapacity(255)),
            PLMNSupportList(PLMNSupportList(vec![PLMNSupportItem::new(network, slices(), None)])),
        )?;
        println!("NG SETUP RESPONSE {}", hex::encode(response.encode()?));

        // The gNB: INITIAL UE MESSAGE, with the REGISTRATION REQUEST of a UE and
        // the cell that the UE is in
        let cell = nr_cgi(network, 0x123456, 24, 1)?;
        let location = UserLocationInformationNR::new(cell, tai(network, [0, 0, 1]), None, None);
        let registration = hex::decode("7e004179000d0102f8390000000000000010022e08a020000000000000")?;
        let initial = build_ngap!(InitiatingMessage, InitialUEMessage, InitialUEMessage,
            RAN_UE_NGAP_ID(7u32),
            NAS_PDU(registration),
            UserLocationInformation(UserLocationInformation::userLocationInformationNR(location)),
            RRCEstablishmentCause(RRCEstablishmentCause::mo_Signalling),
        )?;

        // The AMF reads the IEs that it needs where they are in the octets
        let octets = initial.encode()?;
        let received = NGAP_PDU::decode_borrowed(&octets)?;
        extract_ngap_ies!(received, InitialUEMessage,
            req ran_id: u32 = RAN_UE_NGAP_ID(id),
            req nas: NASPDU = NAS_PDU(nas) => nas,
        );
        println!("INITIAL UE MESSAGE of RAN UE {ran_id}: NAS-PDU {}", hex::encode(&nas.0));

        // It answers the UE, under the AMF UE NGAP ID that it gives it: here an
        // AUTHENTICATION REQUEST
        let authentication = hex::decode(
            "7e0056000200002123553cbe9637a89d218ae64dae47bf35201055f328b43577b9b94a9ffac354dfafb3",
        )?;
        let downlink = build_ngap!(InitiatingMessage, DownlinkNASTransport, DownlinkNASTransport,
            AMF_UE_NGAP_ID(1u64),
            RAN_UE_NGAP_ID(ran_id),
            NAS_PDU(authentication),
        )?;
        println!("DOWNLINK NAS TRANSPORT {}", hex::encode(downlink.encode()?));

        // After the security mode, the context of the UE for the gNB: its GUAMI,
        // its slices, its security capabilities and KgNB
        let k_gnb = hex::decode("c18166e13cfde1dea842c708f6e0b3372b6896664df4e865c0c78bd270852005")?;
        let setup = build_ngap!(InitiatingMessage, InitialContextSetup, InitialContextSetupRequest,
            AMF_UE_NGAP_ID(1u64),
            RAN_UE_NGAP_ID(ran_id),
            GUAMI(guami(network, 1, 1, 0)?),
            AllowedNSSAI(AllowedNSSAI(vec![AllowedNSSAIItem::new(s_nssai(1, None), None)])),
            UESecurityCapabilities(ue_security_capabilities(0xe0, 0xe0)),
            SecurityKey(<[u8; 32]>::try_from(k_gnb).unwrap()),
        )?;

        // The gNB checks it against its object set, and takes the key
        let received = NGAP_PDU::decode(&setup.encode()?)?;
        let deviations = received.check()?;
        let key = received.ie::<ie::SecurityKey>().unwrap()?;
        let key = hex::encode(key.0.as_octets());
        println!("INITIAL CONTEXT SETUP REQUEST, {} deviations: KgNB {key}", deviations.len());
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 3 of README.md.
#[rustfmt::skip]
fn block_3() {
    use oxirush_ngap::{NgapPduKind, ngap::*};

    fn receive(bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
        let pdu = NGAP_PDU::decode(bytes)?;

        Ok(match pdu.kind() {
            NgapPduKind::Initiating_UEContextReleaseRequest => {
                let request: UEContextReleaseRequest = pdu.decode_value()?;
                format!("a release request with {} IEs", request.protocol_ies.0.len())
            }
            NgapPduKind::Other { direction, procedure_code } => {
                format!("procedure {procedure_code} has no {direction}")
            }
            kind => format!("{kind:?} is not handled here"),
        })
    }

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // A UE CONTEXT RELEASE REQUEST, then an NG SETUP FAILURE
        let request = hex::decode("002a4015000003000a0002002a005500020007000f40020500")?;
        println!("{}", receive(&request)?);

        let failure = hex::decode("40150008000001000f400188")?;
        println!("{}", receive(&failure)?);
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 4 of README.md.
#[rustfmt::skip]
fn block_4() {
    use oxirush_ngap::{Message, build_ngap, ngap::*};

    fn main() -> Result<(), oxirush_ngap::Error> {
        // The criticality of the procedure, and those of the object set of the message
        let pdu = build_ngap!(InitiatingMessage, InitialUEMessage, InitialUEMessage,
            RAN_UE_NGAP_ID(7u32),
            NAS_PDU(vec![0x7e, 0x00, 0x41]),
        )?;
        println!("{:?} {}", pdu.criticality(), hex::encode(pdu.encode()?));

        // The PDU of a message that is at hand
        let message: InitialUEMessage = pdu.decode_value()?;
        let again = NGAP_PDU::of_message(&message, InitialUEMessage::CRITICALITY)?;
        println!("{}", again == pdu);

        // An IE that the object set of the message does not have is assigned none
        let error = build_ngap!(InitiatingMessage, InitialUEMessage, InitialUEMessage,
            TimeToWait(TimeToWait::v5s),
        );
        println!("{}", error.unwrap_err());
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 5 of README.md.
#[rustfmt::skip]
fn block_5() {
    use oxirush_ngap::{Error, extract_ngap_ies, ngap::*};

    fn handle(request: &UEContextReleaseRequest) -> Result<(), Error> {
        extract_ngap_ies!(request, UEContextReleaseRequest,
            req amf_id: u64 = AMF_UE_NGAP_ID(id),
            req ran_id: u32 = RAN_UE_NGAP_ID(id),
            req cause: Cause = Cause(value) => value,
        );

        println!("AMF={amf_id} RAN={ran_id} cause={cause:?}");
        Ok(())
    }

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // A UE CONTEXT RELEASE REQUEST
        let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500")?;
        handle(&NGAP_PDU::decode(&bytes)?.decode_value()?)?;
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 6 of README.md.
#[rustfmt::skip]
fn block_6() {
    use oxirush_ngap::{Error, extract_ngap_ies, ie, ngap::*};

    fn main() -> Result<(), Error> {
        // A UE CONTEXT RELEASE REQUEST, read in the buffer that it was received in
        let octets = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
        let pdu = NGAP_PDU::decode_borrowed(&octets)?;

        // The IEs of its message, from the PDU: the Cause is not decoded
        extract_ngap_ies!(pdu, UEContextReleaseRequest,
            req amf_id: u64 = AMF_UE_NGAP_ID(id),
            req ran_id: u32 = RAN_UE_NGAP_ID(id),
        );
        println!("AMF={amf_id} RAN={ran_id}");

        // One IE, by its marker type
        let cause = pdu.ie::<ie::Cause>().unwrap()?;
        println!("{cause:?}");
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 7 of README.md.
#[rustfmt::skip]
fn block_7() {
    use oxirush_ngap::{build_ngap_ie, extract_ngap_ies, ngap::*};

    fn main() -> Result<(), oxirush_ngap::Error> {
        // The transfer of a PDU SESSION RESOURCE SETUP REQUEST, as its item holds it
        let octets = hex::decode("0000010086000100").unwrap();
        let transfer: PDUSessionResourceSetupRequestTransfer = decode_open_type(&octets)?;
        extract_ngap_ies!(transfer, PDUSessionResourceSetupRequestTransfer,
            req kind: PDUSessionType = PDUSessionType(kind) => kind,
        );
        println!("{kind:?}");

        // One is built as a message is, and checked against its object set
        let ipv6 = PDUSessionResourceSetupRequestTransfer::new(ProtocolIEContainer(vec![
            build_ngap_ie!(PDUSessionResourceSetupRequestTransfer,
                PDUSessionType(PDUSessionType::ipv6))?,
        ]));
        let octets = encode_open_type(&ipv6)?;
        println!("{}", hex::encode(&octets));
        for deviation in PDUSessionResourceSetupRequestTransfer::check(&octets)? {
            println!("{deviation}");
        }

        // The octets of a SourceToTarget-TransparentContainer, as the type that they carry
        let octets = hex::decode("000201020002f8390000012340080100").unwrap();
        let container: SourceNGRANNodeToTargetNGRANNodeTransparentContainer =
            decode_open_type(&octets)?;
        println!("{:?}", container.target_cell_id);
        assert_eq!(encode_open_type(&container)?.as_ref(), &octets[..]);
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 8 of README.md.
#[rustfmt::skip]
fn block_8() {
    use oxirush_ngap::{build_ngap, ngap::*};

    fn main() -> Result<(), oxirush_ngap::Error> {
        // A UE CONTEXT RELEASE REQUEST that deviates in each way, built all the same
        let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
            REJECT, UEContextReleaseRequest,
            REJECT RAN_UE_NGAP_ID(7u32),
            IGNORE AMF_UE_NGAP_ID(42u64),
            IGNORE TimeToWait(TimeToWait::v5s),
            REJECT RAN_UE_NGAP_ID(8u32),
        )?;

        // It is encoded and decoded as any other, and `check()` reports
        let received = NGAP_PDU::decode(&pdu.encode()?)?;
        for deviation in received.check()? {
            println!("{deviation}");
        }

        // The message of the first example deviates in nothing
        let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
        assert!(NGAP_PDU::decode(&bytes)?.check()?.is_empty());
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 9 of README.md.
#[rustfmt::skip]
fn block_9() {
    use oxirush_ngap::{Deviation, build_ngap, ngap::*};

    fn main() -> Result<(), oxirush_ngap::Error> {
        // An NG SETUP REQUEST with none of its mandatory IEs but the paging DRX
        let request = build_ngap!(InitiatingMessage, NGSetup, REJECT, NGSetupRequest,
            IGNORE DefaultPagingDRX(PagingDRX::v128),
        )?;
        let received = NGAP_PDU::decode(&request.encode()?)?;
        let deviations = received.check()?;

        // Clause 10.3.5: the procedure is rejected, and the missing IEs are reported
        let cause = deviations.iter().find_map(Deviation::cause).unwrap();
        let failure = build_ngap!(UnsuccessfulOutcome, NGSetup, REJECT, NGSetupFailure,
            IGNORE Cause(Cause::protocol(cause)),
            IGNORE CriticalityDiagnostics(received.criticality_diagnostics_for_response(&deviations)),
        )?;
        for deviation in &deviations {
            println!("{deviation}");
        }
        let octets = hex::encode(failure.encode()?);
        println!("{octets}");
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 10 of README.md.
#[rustfmt::skip]
fn block_10() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use oxirush_ngap::tree;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // A UE CONTEXT RELEASE REQUEST, as a tree
        let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500")?;
        let pdu = NGAP_PDU::decode(&bytes)?;
        println!("{}", pdu.to_tree());

        // Its view: each value with the path that selects it
        let mut view = pdu.to_view()?;
        for (path, value) in inspect::paths(&view) {
            println!("{path} = {value}");
        }

        // A value, and another alternative of a CHOICE in place of the one that is there
        inspect::set(&mut view, "/ngap/RAN-UE-NGAP-ID/value", tree!(9))?;
        inspect::set(&mut view, "/ngap/Cause/nas", tree!("Normal Release"))?;
        let edited = pdu.with_view(&view)?;
        println!("{}", edited.to_tree());

        // A name that the type does not have is refused, and the view is as it was
        let wrong = inspect::set(&mut view, "/ngap/Cause/radioNetwrok", tree!("unspecified"));
        println!("{}", wrong.unwrap_err());
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 11 of README.md.
#[rustfmt::skip]
fn block_11() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use oxirush_ngap::tree;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // A tree: the name of the message, whatever its case and its separators, then its IEs
        let request = tree!({
            "message": "ue context release request",
            "ies": [
                {"AMF-UE-NGAP-ID": 42},
                {"RAN-UE-NGAP-ID": 7},
                {"Cause": {"radioNetwork": "user-inactivity"}},
                // An IE that the message does not have says its criticality
                {"RANNodeName": "gnb-1", "criticality": "ignore"},
            ],
        });

        let pdu = NGAP_PDU::from_tree(&request)?;
        println!("{}", hex::encode(pdu.encode()?));

        // The name gave the direction, the procedure code and the criticalities
        let view = pdu.to_view()?;
        println!("{} {} {}", view["direction"], view["procedure_code"], view["criticality"]);
        println!("{}", tree!(inspect::select(&view, "/ngap/*/criticality")?));
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 12 of README.md.
#[rustfmt::skip]
fn block_12() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use oxirush_ngap::tree;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // `{"octets": "…"}` in place of a value is the octets of the IE, whatever they are
        let request = tree!({"message": "UEContextReleaseRequest", "ies": [
            {"AMF-UE-NGAP-ID": 42},
            {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
        ]});
        let pdu = NGAP_PDU::from_tree(&request)?;
        println!("{}", hex::encode(pdu.encode()?));

        // A view shows an IE that does not decode by its octets, and says why
        let mut view = pdu.to_view()?;
        println!("{}", view["message"]["protocolIEs"][1]);

        // In a view, the `octets` of an IE are set in place of its value
        inspect::set(&mut view, "/ngap/AMF-UE-NGAP-ID/octets", tree!("00ff"))?;
        let edited = pdu.with_view(&view)?;
        println!("{}", edited.to_tree());
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 13 of README.md.
#[rustfmt::skip]
fn block_13() {
    use oxirush_ngap::ngap::{NGAP_PDU, SNSSAI, SST};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500")?;
        let pdu = NGAP_PDU::decode(&bytes)?;
        let text = serde_json::to_string(&pdu)?;
        let read: NGAP_PDU = serde_json::from_str(&text)?;
        assert_eq!(read.encode()?, bytes);

        // A value alone has the form that a tree shows it in: an SST is a number
        let slice = SNSSAI::new(SST([1]), None, None);
        println!("{}", serde_json::to_string(&slice)?);
        Ok(())
    }

    main().unwrap();
}
