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
    (block_1, r#"UEContextReleaseRequest (procedure 42, initiating message), AMF-UE-NGAP-ID 42, RAN-UE-NGAP-ID 7
AMF UE 42, RAN UE 7, cause Some(radioNetwork(user_inactivity))
002a4015000003000a0002002a005500020007000f40020540
"#),
    (block_2, r#"NGSetupRequest (procedure 21, initiating message): gnb-1, 1 tracking area
NGSetupResponse (procedure 21, successful outcome)
InitialUEMessage (procedure 15, initiating message), RAN-UE-NGAP-ID 7: a NAS-PDU of 29 octets
DownlinkNASTransport (procedure 4, initiating message), AMF-UE-NGAP-ID 1, RAN-UE-NGAP-ID 7
InitialContextSetupRequest (procedure 14, initiating message), AMF-UE-NGAP-ID 1, RAN-UE-NGAP-ID 7: 0 deviations, KgNB as sent: true
"#),
    (block_3, r#"release of AMF UE 42, RAN UE 7: Some(radioNetwork(user_inactivity))
Unsuccessful_NGSetup is not handled here
"#),
    (block_4, r#"ignore 000f401100000200550002000700260004037e0041
true
TimeToWait is no IE of the object set of InitialUEMessage, which assigns it no criticality: write its criticality
"#),
    (block_5, r#"ipv4
0000010086000110
UL-NGU-UP-TNLInformation (IE 139) is mandatory, and the message does not have it
QosFlowSetupRequestList (IE 136) is mandatory, and the message does not have it
nR_CGI(NRCGI { plmn_identity: PLMNIdentity(208-93), nr_cell_identity: NRCellIdentity(0x1234/36), ie_extensions: None })
"#),
    (block_6, r#"the criticality of the PDU is ignore, and that of its procedure is reject
TimeToWait (IE 107), at position 1, is not an IE of the message
GlobalRANNodeID (IE 27) is mandatory, and the message does not have it
SupportedTAList (IE 102) is mandatory, and the message does not have it
NGSetupFailure (procedure 21, unsuccessful outcome): 40150018000002000f4001620013400c190200006b00001b40006640
"#),
    (block_7, r#"{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
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
    (block_8, r#"002a4027000005000a0002002a005500020007000f40020500005240070200676e622d31fde80003c0ffee
"InitiatingMessage" 42 "ignore"
["reject","reject","ignore","ignore","reject"]
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
        println!("{pdu}");

        // Read its IEs by name: `req` fails when the IE is missing, `opt` gives an Option
        extract_ngap_ies!(pdu, UEContextReleaseRequest,
            req amf_id: u64 = AMF_UE_NGAP_ID(id),
            req ran_id: u32 = RAN_UE_NGAP_ID(id),
            opt cause: Cause = Cause(cause) => cause,
        );
        println!("AMF UE {amf_id}, RAN UE {ran_id}, cause {cause:?}");

        // Build the same message with another cause, and encode it
        let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest, UEContextReleaseRequest,
            AMF_UE_NGAP_ID(42u64),
            RAN_UE_NGAP_ID(7u32),
            Cause(Cause::radioNetwork(CauseRadioNetwork::radio_connection_with_ue_lost)),
        )?;
        println!("{}", hex::encode(pdu.encode()?));
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 2 of README.md.
#[rustfmt::skip]
fn block_2() {
    use oxirush_ngap::helpers::*;
    use oxirush_ngap::{build_ngap, extract_ngap_ies, ngap::*};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let network = plmn("208", "93")?;
        let slices = || SliceSupportList(vec![SliceSupportItem::new(s_nssai(1, None), None)]);

        // The gNB: its identity, its name, and the tracking area and slices it serves
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
        println!("{received}: {}, {} tracking area", name.unwrap_or_default(), areas.0.len());
        let response = build_ngap!(SuccessfulOutcome, NGSetup, NGSetupResponse,
            AMFName("amf-1"),
            ServedGUAMIList(ServedGUAMIList(vec![ServedGUAMIItem::new(guami(network, 1, 1, 0)?, None, None)])),
            RelativeAMFCapacity(RelativeAMFCapacity(255)),
            PLMNSupportList(PLMNSupportList(vec![PLMNSupportItem::new(network, slices(), None)])),
        )?;
        println!("{response}");

        // The gNB forwards a REGISTRATION REQUEST, with the cell the UE is in
        let cell = nr_cgi(network, 0x123456, 24, 1)?;
        let location = UserLocationInformationNR::new(cell, tai(network, [0, 0, 1]), None, None);
        let registration = hex::decode("7e004179000d0102f8390000000000000010022e08a020000000000000")?;
        let initial = build_ngap!(InitiatingMessage, InitialUEMessage, InitialUEMessage,
            RAN_UE_NGAP_ID(7u32),
            NAS_PDU(registration),
            UserLocationInformation(UserLocationInformation::userLocationInformationNR(location)),
            RRCEstablishmentCause(RRCEstablishmentCause::mo_Signalling),
        )?;

        // The AMF reads the two IEs it needs, in place in the received octets
        let octets = initial.encode()?;
        let received = NGAP_PDU::decode_borrowed(&octets)?;
        extract_ngap_ies!(received, InitialUEMessage,
            req ran_id: u32 = RAN_UE_NGAP_ID(id),
            req nas: NASPDU = NAS_PDU(nas) => nas,
        );
        println!("{received}: a NAS-PDU of {} octets", nas.0.len());

        // It gives the UE an AMF UE NGAP ID, and answers with an AUTHENTICATION REQUEST
        let authentication = hex::decode(
            "7e0056000200002123553cbe9637a89d218ae64dae47bf35201055f328b43577b9b94a9ffac354dfafb3",
        )?;
        let downlink = build_ngap!(InitiatingMessage, DownlinkNASTransport, DownlinkNASTransport,
            AMF_UE_NGAP_ID(1u64),
            RAN_UE_NGAP_ID(ran_id),
            NAS_PDU(authentication),
        )?;
        println!("{downlink}");

        // After the security mode: the UE's GUAMI, slices, security capabilities and KgNB
        let mut k_gnb = [0; 32];
        hex::decode_to_slice("c18166e13cfde1dea842c708f6e0b3372b6896664df4e865c0c78bd270852005", &mut k_gnb)?;
        let setup = build_ngap!(InitiatingMessage, InitialContextSetup, InitialContextSetupRequest,
            AMF_UE_NGAP_ID(1u64),
            RAN_UE_NGAP_ID(ran_id),
            GUAMI(guami(network, 1, 1, 0)?),
            AllowedNSSAI(AllowedNSSAI(vec![AllowedNSSAIItem::new(s_nssai(1, None), None)])),
            UESecurityCapabilities(ue_security_capabilities(0xe0, 0xe0)),
            SecurityKey(k_gnb),
        )?;

        // The gNB checks the message against the specification, and takes the key
        let received = NGAP_PDU::decode(&setup.encode()?)?;
        let deviations = received.check()?;
        extract_ngap_ies!(received, InitialContextSetupRequest,
            req key: SecurityKey = SecurityKey(key) => key,
        );
        println!("{received}: {} deviations, KgNB as sent: {}", deviations.len(), key.0.as_octets() == k_gnb);
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 3 of README.md.
#[rustfmt::skip]
fn block_3() {
    use oxirush_ngap::{NgapPduKind, extract_ngap_ies, ie, ngap::*};

    fn receive(octets: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // `decode_borrowed` copies nothing: the PDU borrows from `octets`
        let pdu = NGAP_PDU::decode_borrowed(octets)?;
        match pdu.kind() {
            NgapPduKind::Initiating_UEContextReleaseRequest => {
                // Only the IEs that a field asks for are decoded
                extract_ngap_ies!(pdu, UEContextReleaseRequest,
                    req amf_id: u64 = AMF_UE_NGAP_ID(id),
                    req ran_id: u32 = RAN_UE_NGAP_ID(id),
                );
                // One IE, by its marker type
                let cause = pdu.ie::<ie::Cause>().transpose()?;
                println!("release of AMF UE {amf_id}, RAN UE {ran_id}: {cause:?}");
            }
            NgapPduKind::Other { direction, procedure_code } => {
                println!("procedure {procedure_code} has no {direction}");
            }
            kind => println!("{kind:?} is not handled here"),
        }
        Ok(())
    }

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // A UE CONTEXT RELEASE REQUEST, then an NG SETUP FAILURE
        receive(&hex::decode("002a4015000003000a0002002a005500020007000f40020500")?)?;
        receive(&hex::decode("40150008000001000f400188")?)?;
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 4 of README.md.
#[rustfmt::skip]
fn block_4() {
    use oxirush_ngap::{Message, build_ngap, ngap::*};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // The criticalities of the procedure and of the message's object set
        let pdu = build_ngap!(InitiatingMessage, InitialUEMessage, InitialUEMessage,
            RAN_UE_NGAP_ID(7u32),
            NAS_PDU(vec![0x7e, 0x00, 0x41]),
        )?;
        println!("{:?} {}", pdu.criticality(), hex::encode(pdu.encode()?));

        // The PDU of a message you already have
        let message: InitialUEMessage = pdu.decode_value()?;
        let again = NGAP_PDU::of_message(&message, InitialUEMessage::CRITICALITY)?;
        println!("{}", again == pdu);

        // An IE that the message cannot have has no criticality to take
        let error = build_ngap!(InitiatingMessage, InitialUEMessage, InitialUEMessage,
            TimeToWait(TimeToWait::v5s),
        );
        if let Err(error) = error {
            println!("{error}");
        }
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 5 of README.md.
#[rustfmt::skip]
fn block_5() {
    use oxirush_ngap::{build_ngap_ie, extract_ngap_ies, ngap::*};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // The transfer of a PDU SESSION RESOURCE SETUP REQUEST, as its item holds it
        let octets = hex::decode("0000010086000100")?;
        let transfer: PDUSessionResourceSetupRequestTransfer = decode_open_type(&octets)?;
        extract_ngap_ies!(transfer, PDUSessionResourceSetupRequestTransfer,
            req kind: PDUSessionType = PDUSessionType(kind) => kind,
        );
        println!("{kind:?}");

        // Build one, and check it against its object set
        let ipv6 = PDUSessionResourceSetupRequestTransfer::new(ProtocolIEContainer(vec![
            build_ngap_ie!(PDUSessionResourceSetupRequestTransfer,
                PDUSessionType(PDUSessionType::ipv6))?,
        ]));
        let octets = encode_open_type(&ipv6)?;
        println!("{}", hex::encode(&octets));
        for deviation in PDUSessionResourceSetupRequestTransfer::check(&octets)? {
            println!("{deviation}");
        }

        // A SourceToTarget-TransparentContainer, as the type it carries
        let octets = hex::decode("000201020002f8390000012340080100")?;
        let container: SourceNGRANNodeToTargetNGRANNodeTransparentContainer =
            decode_open_type(&octets)?;
        println!("{:?}", container.target_cell_id);
        assert_eq!(encode_open_type(&container)?.as_ref(), &octets[..]);
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 6 of README.md.
#[rustfmt::skip]
fn block_6() {
    use oxirush_ngap::{Deviation, build_ngap, ngap::*};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // An NG SETUP REQUEST with the wrong criticality, an IE it cannot have,
        // and none of its mandatory IEs but the paging DRX
        let request = build_ngap!(InitiatingMessage, NGSetup, IGNORE, NGSetupRequest,
            IGNORE DefaultPagingDRX(PagingDRX::v128),
            REJECT TimeToWait(TimeToWait::v5s),
        )?;
        let received = NGAP_PDU::decode(&request.encode()?)?;
        let deviations = received.check()?;
        for deviation in &deviations {
            println!("{deviation}");
        }

        // The AMF rejects the procedure, and reports the IEs at fault
        let cause = deviations.iter().find_map(Deviation::cause).ok_or("no cause")?;
        let failure = build_ngap!(UnsuccessfulOutcome, NGSetup, NGSetupFailure,
            Cause(Cause::protocol(cause)),
            CriticalityDiagnostics(received.criticality_diagnostics_for_response(&deviations)),
        )?;
        println!("{failure}: {}", hex::encode(failure.encode()?));
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 7 of README.md.
#[rustfmt::skip]
fn block_7() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU, tree};

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

        // Set a value, and pick another alternative of a CHOICE
        inspect::set(&mut view, "/ngap/RAN-UE-NGAP-ID/value", tree!(9))?;
        inspect::set(&mut view, "/ngap/Cause/nas", tree!("Normal Release"))?;
        println!("{}", pdu.with_view(&view)?.to_tree());

        // A misspelt name is refused, and the view stays as it was
        if let Err(error) = inspect::set(&mut view, "/ngap/Cause/radioNetwrok", tree!("unspecified")) {
            println!("{error}");
        }
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 8 of README.md.
#[rustfmt::skip]
fn block_8() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU, tree};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let request = tree!({
            "message": "ue context release request",
            "ies": [
                {"AMF-UE-NGAP-ID": 42},
                {"RAN-UE-NGAP-ID": 7},
                {"Cause": {"radioNetwork": "user-inactivity"}},
                // An IE that the message does not have says its criticality
                {"RANNodeName": "gnb-1", "criticality": "ignore"},
                // An IE that no release has, as its octets
                {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
            ],
        });
        let pdu = NGAP_PDU::from_tree(&request)?;
        println!("{}", hex::encode(pdu.encode()?));

        let view = pdu.to_view()?;
        println!("{} {} {}", view["direction"], view["procedure_code"], view["criticality"]);
        println!("{}", tree!(inspect::select(&view, "/ngap/*/criticality")?));
        Ok(())
    }

    main().unwrap();
}
