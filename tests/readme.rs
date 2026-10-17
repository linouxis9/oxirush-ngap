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
    (block_2, r#"{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"radio-connection-with-ue-lost"}}]}
"#),
    (block_3, r#"a release request with 3 IEs
Unsuccessful_NGSetup is not handled here
"#),
    (block_4, r#""#),
    (block_5, r#"UE 7: a NAS-PDU of 3 octets
"#),
    (block_6, r#"a NAS-PDU of 3 octets
"#),
    (block_7, r#"ignore 000f401100000200550002000700260004037e0041
true
TimeToWait is no IE of the object set of InitialUEMessage, which assigns it no criticality: write its criticality
"#),
    (block_8, r#""#),
    (block_9, r#"AMF=42 RAN=7
radioNetwork(user_inactivity)
"#),
    (block_10, r#"Some(radioNetwork(user_inactivity))
None
"#),
    (block_11, r#""#),
    (block_12, r#"15 ignore 88
NGSetup 4015000d000002000f400188006b400120
"#),
    (block_13, r#"ipv4
0000010086000110
nR_CGI(NRCGI { plmn_identity: PLMNIdentity(208-93), nr_cell_identity: NRCellIdentity(0x1234/36), ie_extensions: None })
"#),
    (block_14, r#"the criticality of the PDU is reject, and that of its procedure is ignore
AMF-UE-NGAP-ID (IE 10), at position 1, comes after RAN-UE-NGAP-ID (IE 85), which the object set of the message has after it
the criticality of AMF-UE-NGAP-ID (IE 10), at position 1, is ignore, and the message assigns it reject
TimeToWait (IE 107), at position 2, is not an IE of the message
RAN-UE-NGAP-ID (IE 85), at position 3, is in the message already, at position 0
Cause (IE 15) is mandatory, and the message does not have it
"#),
    (block_15, r#"GlobalRANNodeID (IE 27) is mandatory, and the message does not have it
SupportedTAList (IE 102) is mandatory, and the message does not have it
40150015000002000f40016200134009180100001b40006640
"#),
    (block_16, r#""20" is not an MCC: it has three digits, 0 to 9
the AMF Set ID 1024 does not fit in 10 bits
"#),
    (block_17, r#"the message of this PDU is UEContextReleaseRequest, not NGSetupRequest: kind() says which message a PDU holds
these octets do not decode as NGAP-PDU: the octets end before the value does, in its field `value`
"#),
    (block_18, r#"{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
{"id":15,"criticality":"ignore","value":{"radioNetwork":"user-inactivity"},"_raw_value":"0500","_original_id":15,"_ie_name":"Cause"}
"#),
    (block_19, r#"/procedure_code = 42
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
"#),
    (block_20, r#"{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":9},{"Cause":{"nas":"normal-release"}}]}
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions; an IE itself has id, criticality, value, octets, at /ngap/Cause/radioNetwrok
"#),
    (block_21, r#"[{"RAN-UE-NGAP-ID":7},{"60000":{"octets":"c0ffee"},"criticality":"ignore"},{"Cause":{"radioNetwork":"user-inactivity"}},{"RANNodeName":"gnb-1","criticality":"ignore"}]
"#),
    (block_22, r#"002a4020000004000a0002002a005500020007000f40020500005240070200676e622d31
"InitiatingMessage" 42 "ignore"
["reject","reject","ignore","ignore"]
"#),
    (block_23, r#"002a4010000002000a0002002afde80003c0ffee
{"id":65000,"criticality":"reject","value":"c0ffee","_decode_error":"NGAP IE 65000 is unknown","_raw_value":"c0ffee","_original_id":65000}
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":255},{"65000":{"octets":"c0ffee"},"criticality":"reject"}]}
"#),
    (block_24, r#"this IE is AMF-UE-NGAP-ID, not RAN-UE-NGAP-ID: "RAN-UE-NGAP-ID" is not a member of this value, which has none, at /ngap/2/RAN-UE-NGAP-ID/value
/ngap/0/value = 1
/ngap/RAN-UE-NGAP-ID/value = 7
/ngap/2/value = 3
"#),
    (block_25, r#"{"message":"InitialContextSetupRequest","ies":[{"AllowedNSSAI":[{"s-NSSAI":{"sST":2}},{"s-NSSAI":{"sST":3}}]}]}
"#),
    (block_26, r#"{"_raw_transfer":"0000010086000100","decoded":{"protocolIEs":[{"id":134,"criticality":"reject","value":"ipv4","_raw_value":"00","_original_id":134,"_ie_name":"PDUSessionType"}]}}
{"PDUSessionResourceSetupListSUReq":[{"pDUSessionID":5,"s-NSSAI":{"sST":1},"pDUSessionResourceSetupRequestTransfer":{"protocolIEs":[{"PDUSessionType":"ipv6","criticality":"reject"}]}}]}
"#),
    (block_27, r#"["normal-release", "authentication-failure", "deregister", "unspecified", "uE-not-in-PLMN-serving-area", "mobile-IAB-not-authorized", "iAB-not-authorized", "no-aiot-NAS-response"]
"#),
    (block_28, r#""radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions; an IE itself has id, criticality, value, octets, at /ngap/Cause/radioNetwrok
"valeu" is not a member of this value, which has none; an IE itself has id, criticality, value, octets, at /ngap/RAN-UE-NGAP-ID/valeu
"GUAMI" is not an IE of UEContextReleaseRequest, which has AMF-UE-NGAP-ID, RAN-UE-NGAP-ID, PDUSessionResourceListCxtRelReq, Cause, GW-Context-Release-Indication, at /ngap/GUAMI
"#),
    (block_29, r#"{"SourceToTarget-TransparentContainer":"000201020002f8390000012340080100"}
{"rRCContainer":"0102","targetCell-ID":{"nR-CGI":{"pLMNIdentity":"208-93","nRCellIdentity":4660}},"uEHistoryInformation":[{"lastVisitedCellInformation":{"eUTRANCell":"00"}}]}
{"SourceToTarget-TransparentContainer":"000201020002f8390000000070080100"}
"#),
    (block_30, r#"{"sST":1}
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
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use oxirush_ngap::tree;

    // The same message as a tree: its name and its IEs by name
    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    let pdu = NGAP_PDU::decode(&bytes).unwrap();
    println!("{}", pdu.to_tree());

    // Its view is edited at a path, and is a PDU again
    let mut view = pdu.to_view().unwrap();
    let lost = tree!("radio-connection-with-ue-lost");
    inspect::set(&mut view, "/ngap/Cause/radioNetwork", lost).unwrap();
    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree());

    // Its octets are those that the typed example built
    let octets = hex::encode(edited.encode().unwrap());
    assert_eq!(octets, "002a4015000003000a0002002a005500020007000f40020540");
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

    // A UE CONTEXT RELEASE REQUEST, then an NG SETUP FAILURE
    let request = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    println!("{}", receive(&request).unwrap());

    let failure = hex::decode("40150008000001000f400188").unwrap();
    println!("{}", receive(&failure).unwrap());
}

/// The Rust block 4 of README.md.
#[rustfmt::skip]
fn block_4() {
    use oxirush_ngap::ngap::{Cause, GUAMI, NRCGI, TAI};

    // What holds no octets of its own is kept as it is decoded
    struct Cell {
        tai: TAI,
        cgi: NRCGI,
        guami: GUAMI,
        last_cause: Option<Cause>,
    }
}

/// The Rust block 5 of README.md.
#[rustfmt::skip]
fn block_5() {
    use oxirush_ngap::{Error, build_ngap, extract_ngap_ies, ngap::*};

    // What a node keeps of a UE after its first message
    struct Ue {
        ran_id: u32,
        nas: NASPDU<'static>,
    }

    fn admit(pdu: &NGAP_PDU) -> Result<Ue, Error> {
        // `message` borrows from `pdu`, and `nas` from `message`
        let message: InitialUEMessage = pdu.decode_value()?;
        extract_ngap_ies!(message, InitialUEMessage,
            req ran_id: u32 = RAN_UE_NGAP_ID(id),
            req nas: NASPDU = NAS_PDU(nas) => nas,
        );
        // Kept beyond the PDU: the octets are copied here, once
        Ok(Ue { ran_id, nas: nas.into_owned() })
    }

    fn main() -> Result<(), Error> {
        let pdu = build_ngap!(InitiatingMessage, InitialUEMessage, IGNORE, InitialUEMessage,
            REJECT RAN_UE_NGAP_ID(7u32),
            REJECT NAS_PDU(vec![0x7e, 0x00, 0x41]),
        )?;
        let ue = admit(&pdu)?;
        drop(pdu);
        println!("UE {}: a NAS-PDU of {} octets", ue.ran_id, ue.nas.0.len());
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 6 of README.md.
#[rustfmt::skip]
fn block_6() {
    use oxirush_ngap::{Error, build_ngap, extract_ngap_ies, ngap::*};

    async fn deliver(nas: &[u8]) {
        println!("a NAS-PDU of {} octets", nas.len());
    }

    async fn handle(received: Vec<u8>) -> Result<(), Error> {
        // Nothing of `received` is copied
        let pdu = NGAP_PDU::decode_borrowed(&received)?;
        let message: UplinkNASTransport = pdu.decode_value()?;
        extract_ngap_ies!(message, UplinkNASTransport,
            req nas: NASPDU = NAS_PDU(nas) => nas,
        );
        deliver(&nas.0).await;
        Ok(())
    }

    fn main() -> Result<(), Error> {
        let transport = build_ngap!(InitiatingMessage, UplinkNASTransport, IGNORE, UplinkNASTransport,
            REJECT AMF_UE_NGAP_ID(1u64),
            REJECT RAN_UE_NGAP_ID(7u32),
            REJECT NAS_PDU(vec![0x7e, 0x00, 0x5d]),
        )?;
        let handler = handle(transport.encode()?);

        // A runtime may move the handler to another thread
        fn is_send(_: &impl Send) {}
        is_send(&handler);

        // Polled here without a runtime: this handler never waits
        let mut handler = std::pin::pin!(handler);
        let mut context = std::task::Context::from_waker(std::task::Waker::noop());
        assert!(handler.as_mut().poll(&mut context).is_ready());
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 7 of README.md.
#[rustfmt::skip]
fn block_7() {
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

/// The Rust block 8 of README.md.
#[rustfmt::skip]
fn block_8() {
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
}

/// The Rust block 9 of README.md.
#[rustfmt::skip]
fn block_9() {
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

/// The Rust block 10 of README.md.
#[rustfmt::skip]
fn block_10() {
    use oxirush_ngap::{extract_ngap_ies, ngap::*};

    // The cause of a request, when it has one that is read
    fn cause(request: &UEContextReleaseRequest<'_>) -> Option<Cause> {
        extract_ngap_ies!(request, UEContextReleaseRequest,
            lax cause: Cause = Cause(value) => value,
        );
        cause
    }

    // A UE CONTEXT RELEASE REQUEST, then the same with an octet of its Cause spoiled
    for octets in [
        "002a4015000003000a0002002a005500020007000f40020500",
        "002a4015000003000a0002002a005500020007000f4002e500",
    ] {
        let pdu = NGAP_PDU::decode(&hex::decode(octets).unwrap()).unwrap();
        println!("{:?}", cause(&pdu.decode_value().unwrap()));
    }
}

/// The Rust block 11 of README.md.
#[rustfmt::skip]
fn block_11() {
    use oxirush_ngap::{build_ngap_ie, ngap::*, with_ngap_ie_mut};

    // A message with one IE, which is built alone
    let mut message = UplinkNASTransport::new(ProtocolIEContainer(vec![
        build_ngap_ie!(UplinkNASTransport, REJECT AMF_UE_NGAP_ID(1u64)).unwrap(),
    ]));

    // The IE is decoded, changed and encoded again in its place
    let updated = with_ngap_ie_mut!(message, UplinkNASTransport,
        AMF_UE_NGAP_ID(id) = 42u64
    );
    assert_eq!(updated, Ok(true));
}

/// The Rust block 12 of README.md.
#[rustfmt::skip]
fn block_12() {
    use oxirush_ngap::{build_ngap_ie, ngap::*};

    fn main() -> Result<(), oxirush_ngap::Error> {
        // An NG SETUP FAILURE, as any PDU that was received
        let bytes = hex::decode("40150008000001000f400188").unwrap();
        let mut pdu = NGAP_PDU::decode(&bytes)?;

        // Its IEs as they were received: identifier, criticality and octets. They
        // borrow from the PDU, and are owned here, as the PDU is changed below
        let mut ies = pdu.protocol_ies()?.into_owned();
        for ie in &ies.0 {
            let octets = hex::encode(&ie.value);
            println!("{} {:?} {octets}", ie.id.0, ie.criticality);
        }

        // One more IE, and the container is the message of the same PDU again
        let wait = build_ngap_ie!(NGSetupFailure, IGNORE TimeToWait(TimeToWait::v5s))?;
        ies.0.push(wait);
        pdu.set_protocol_ies(ies)?;
        let octets = hex::encode(pdu.encode()?);
        println!("{} {octets}", pdu.procedure_name());
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 13 of README.md.
#[rustfmt::skip]
fn block_13() {
    use oxirush_ngap::{build_ngap_ie, extract_ngap_ies, ngap::*};

    fn main() -> Result<(), oxirush_ngap::Error> {
        // The transfer of a PDU SESSION RESOURCE SETUP REQUEST, as its item holds it
        let octets = hex::decode("0000010086000100").unwrap();
        let transfer: PDUSessionResourceSetupRequestTransfer = decode_open_type(&octets)?;
        extract_ngap_ies!(transfer, PDUSessionResourceSetupRequestTransfer,
            req kind: PDUSessionType = PDUSessionType(kind) => kind,
        );
        println!("{kind:?}");

        // One is built as a message is
        let ipv6 = PDUSessionResourceSetupRequestTransfer::new(ProtocolIEContainer(vec![
            build_ngap_ie!(PDUSessionResourceSetupRequestTransfer,
                REJECT PDUSessionType(PDUSessionType::ipv6))?,
        ]));
        let octets = hex::encode(encode_open_type(&ipv6)?);
        println!("{octets}");

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

/// The Rust block 14 of README.md.
#[rustfmt::skip]
fn block_14() {
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

/// The Rust block 15 of README.md.
#[rustfmt::skip]
fn block_15() {
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

/// The Rust block 16 of README.md.
#[rustfmt::skip]
fn block_16() {
    use oxirush_ngap::helpers::*;

    // A PLMN from its MCC and its MNC, and what is identified within it
    let network = plmn("208", "93").unwrap();
    let amf = guami(network, 1, 1, 0).unwrap(); // region, set, pointer
    let tracking_area = tai(network, [0x00, 0x00, 0x01]); // TAC
    let cell = nr_cgi(network, 0x123456, 24, 1).unwrap(); // gNB of 24 bits, cell
    let gnb = global_gnb_id(network, 0x123456, 24).unwrap();
    let ng_enb = global_ng_enb_id(network, NgEnbKind::Macro, 0x12345).unwrap();
    let lte_cell = eutra_cgi(network, NgEnbKind::Macro, 0x12345, 7).unwrap(); // ng-eNB, cell

    // A bit string from octets, and the NR algorithms of a UE: the first
    // octet of its encryption bitmap, then of its integrity bitmap
    let algorithm_mask = oxirush_ngap::BitString::from([0xe0]);
    let capabilities = ue_security_capabilities(0xe0, 0xe0);

    // What is no MCC, and an AMF Set ID that does not fit in its 10 bits
    println!("{}", plmn("20", "93").unwrap_err());
    println!("{}", guami(network, 1, 1024, 0).unwrap_err());
}

/// The Rust block 17 of README.md.
#[rustfmt::skip]
fn block_17() {
    use oxirush_ngap::{ErrorKind, ngap::*};

    // A UE CONTEXT RELEASE REQUEST is not decoded as an NG SETUP REQUEST
    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    let pdu = NGAP_PDU::decode(&bytes).unwrap();
    let wrong = pdu.decode_value::<NGSetupRequest>().unwrap_err();
    assert_eq!(wrong.kind(), ErrorKind::WrongMessage);
    println!("{wrong}");

    // Octets that end early
    let short = NGAP_PDU::decode(&bytes[..10]).unwrap_err();
    assert_eq!(short.kind(), ErrorKind::Decode);
    println!("{short}");
}

/// The Rust block 18 of README.md.
#[rustfmt::skip]
fn block_18() {
    use oxirush_ngap::ngap::NGAP_PDU;

    // UE CONTEXT RELEASE REQUEST
    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    let pdu = NGAP_PDU::decode(&bytes).unwrap();

    // Its tree, then its third IE as the view has it
    println!("{}", pdu.to_tree());
    let view = pdu.to_view().unwrap();
    println!("{}", view["message"]["protocolIEs"][2]);
}

/// The Rust block 19 of README.md.
#[rustfmt::skip]
fn block_19() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use oxirush_ngap::tree;

    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    let pdu = NGAP_PDU::decode(&bytes).unwrap();
    let view = pdu.to_view().unwrap();

    // `/ngap` is the IEs of the message, and an IE goes by its name
    let select = |path| inspect::select(&view, path).unwrap();
    assert_eq!(select("/ngap/RAN-UE-NGAP-ID/value"), [&tree!(7)]);
    assert_eq!(select("/ngap/Cause/radioNetwork"), [&tree!("user-inactivity")]);
    assert_eq!(select("/ngap/Cause/octets"), [&tree!("0500")]);

    // By its position, by its name whatever its case and its separators, by its
    // identifier, and each IE with `*`
    assert_eq!(select("/ngap/1/value"), select("/NGAP/ran_ue_ngap_id/value"));
    assert_eq!(select("/ngap/@id=85/value"), [&tree!(7)]);
    assert_eq!(select("/ngap/*/criticality").len(), 3);

    // Nothing where the message could have a value and has none
    assert!(select("/ngap/Cause/nas").is_empty());
    assert!(select("/ngap/GUAMI/pLMNIdentity").is_empty());

    // Each value with the path that selects it
    for (path, value) in inspect::paths(&view) {
        println!("{path} = {value}");
    }
}

/// The Rust block 20 of README.md.
#[rustfmt::skip]
fn block_20() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use oxirush_ngap::tree;

    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    let pdu = NGAP_PDU::decode(&bytes).unwrap();
    let mut view = pdu.to_view().unwrap();

    // A value, and another alternative of a CHOICE in place of the one that is there
    inspect::set(&mut view, "/ngap/RAN-UE-NGAP-ID/value", tree!(9)).unwrap();
    inspect::set(&mut view, "/ngap/Cause/nas", tree!("Normal Release")).unwrap();
    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree());

    // What was not edited is the octets that were received
    let after = edited.to_view().unwrap();
    let octets = inspect::select(&after, "/ngap/AMF-UE-NGAP-ID/octets").unwrap();
    assert_eq!(octets, [&tree!("002a")]);

    // A name that the type does not have is refused, and the view is as it was
    let wrong = inspect::set(&mut view, "/ngap/Cause/radioNetwrok", tree!("unspecified"));
    println!("{}", wrong.unwrap_err());
}

/// The Rust block 21 of README.md.
#[rustfmt::skip]
fn block_21() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use oxirush_ngap::tree;

    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    let pdu = NGAP_PDU::decode(&bytes).unwrap();
    let mut view = pdu.to_view().unwrap();

    // An IE is added at the end, by its name and its typed value
    let name = tree!({"id": "RANNodeName", "criticality": "ignore", "value": "gnb-1"});
    inspect::insert(&mut view, "/ngap/-", name).unwrap();

    // Before another IE, by its number and its octets
    let unknown = tree!({"id": 60000, "criticality": "ignore", "octets": "c0ffee"});
    inspect::insert(&mut view, "/ngap/Cause", unknown).unwrap();

    // An IE is taken out by its name
    inspect::remove(&mut view, "/ngap/AMF-UE-NGAP-ID").unwrap();

    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree()["ies"]);
}

/// The Rust block 22 of README.md.
#[rustfmt::skip]
fn block_22() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use oxirush_ngap::tree;

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

    let pdu = NGAP_PDU::from_tree(&request).unwrap();
    println!("{}", hex::encode(pdu.encode().unwrap()));

    // The name gave the direction, the procedure code and the criticalities
    let view = pdu.to_view().unwrap();
    println!("{} {} {}", view["direction"], view["procedure_code"], view["criticality"]);
    println!("{}", tree!(inspect::select(&view, "/ngap/*/criticality").unwrap()));
}

/// The Rust block 23 of README.md.
#[rustfmt::skip]
fn block_23() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use oxirush_ngap::tree;

    // `{"octets": "…"}` in place of a value is the octets of the IE, whatever they are
    let request = tree!({"message": "UEContextReleaseRequest", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
    ]});
    let pdu = NGAP_PDU::from_tree(&request).unwrap();
    println!("{}", hex::encode(pdu.encode().unwrap()));

    // A view shows an IE that does not decode by its octets, and says why
    let mut view = pdu.to_view().unwrap();
    println!("{}", view["message"]["protocolIEs"][1]);

    // In a view, the `octets` of an IE are set in place of its value
    inspect::set(&mut view, "/ngap/AMF-UE-NGAP-ID/octets", tree!("00ff")).unwrap();
    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree());
}

/// The Rust block 24 of README.md.
#[rustfmt::skip]
fn block_24() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use oxirush_ngap::tree;

    // A message that has one IE twice
    let twice = tree!({"message": "UEContextReleaseRequest", "ies": [
        {"AMF-UE-NGAP-ID": 1},
        {"RAN-UE-NGAP-ID": 7},
        {"AMF-UE-NGAP-ID": 2},
    ]});
    let pdu = NGAP_PDU::from_tree(&twice).unwrap();
    let mut view = pdu.to_view().unwrap();

    // The name selects both, in the order of the message
    let both = inspect::select(&view, "/ngap/AMF-UE-NGAP-ID/value").unwrap();
    assert_eq!(both, [&tree!(1), &tree!(2)]);

    // Each has its position, which is how `paths` names it, and its name may follow
    inspect::set(&mut view, "/ngap/2/AMF-UE-NGAP-ID/value", tree!(3)).unwrap();

    // After a position, the name of another IE is an error that says which IE is there
    println!("{}", inspect::select(&view, "/ngap/2/RAN-UE-NGAP-ID/value").unwrap_err());

    // The values, each with its path
    for (path, value) in inspect::paths(&view) {
        if path.ends_with("/value") {
            println!("{path} = {value}");
        }
    }
}

/// The Rust block 25 of README.md.
#[rustfmt::skip]
fn block_25() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use oxirush_ngap::{tree, tree::Value};

    // A message whose one IE is a list of two items
    let request = tree!({"message": "InitialContextSetupRequest", "ies": [
        {"AllowedNSSAI": [{"s-NSSAI": {"sST": 1}}, {"s-NSSAI": {"sST": 2, "sD": "000001"}}]},
    ]});
    let pdu = NGAP_PDU::from_tree(&request).unwrap();
    let mut view = pdu.to_view().unwrap();

    // An item by its position, and each of them after `value`
    let select = |view, path| -> Vec<Value> {
        let selected = inspect::select(view, path).unwrap();
        selected.into_iter().cloned().collect()
    };
    assert_eq!(select(&view, "/ngap/AllowedNSSAI/1/s-NSSAI/sD"), [tree!("000001")]);
    assert_eq!(select(&view, "/ngap/AllowedNSSAI/value/*/s-NSSAI/sST"), [tree!(1), tree!(2)]);

    // An optional member that an item does not have selects nothing
    assert!(select(&view, "/ngap/AllowedNSSAI/0/s-NSSAI/sD").is_empty());

    // An item is added at the end, and one is taken out
    inspect::insert(&mut view, "/ngap/AllowedNSSAI/-", tree!({"s-NSSAI": {"sST": 3}})).unwrap();
    inspect::remove(&mut view, "/ngap/AllowedNSSAI/0").unwrap();

    // `null` takes an optional member out
    inspect::set(&mut view, "/ngap/AllowedNSSAI/0/s-NSSAI/sD", Value::Null).unwrap();

    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree());
}

/// The Rust block 26 of README.md.
#[rustfmt::skip]
fn block_26() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use oxirush_ngap::tree;

    // A message with a transfer, written as the message that it contains
    let request = tree!({"message": "PDUSessionResourceSetupRequest", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"PDUSessionResourceSetupListSUReq": [{
            "pDUSessionID": 5,
            "s-NSSAI": {"sST": 1},
            "pDUSessionResourceSetupRequestTransfer": {"protocolIEs": [
                {"PDUSessionType": "ipv4", "criticality": "reject"},
            ]},
        }]},
    ]});
    let pdu = NGAP_PDU::from_tree(&request).unwrap();
    let mut view = pdu.to_view().unwrap();

    // In a view, the transfer is its octets beside its decoded value
    let transfer = "/ngap/PDUSessionResourceSetupListSUReq/0/pDUSessionResourceSetupRequestTransfer";
    println!("{}", inspect::select(&view, transfer).unwrap()[0]);

    // The IEs of the message that it contains go by name in its place
    let kind = format!("{transfer}/PDUSessionType/value");
    assert_eq!(inspect::select(&view, &kind).unwrap(), [&tree!("ipv4")]);

    // The same value by the path that says `value` and `decoded`
    let long = "/ngap/PDUSessionResourceSetupListSUReq/value/0\
        /pDUSessionResourceSetupRequestTransfer/decoded/PDUSessionType/value";
    assert_eq!(inspect::select(&view, long), inspect::select(&view, &kind));

    // And by the names that the tree shows, joined: the IE by its position, then its name
    let joined = format!("{transfer}/protocolIEs/0/PDUSessionType/value");
    assert_eq!(inspect::select(&view, &joined), inspect::select(&view, &kind));

    // An edit of the message that it contains is encoded into the transfer
    inspect::set(&mut view, &kind, tree!("ipv6")).unwrap();
    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree()["ies"][2]);
}

/// The Rust block 27 of README.md.
#[rustfmt::skip]
fn block_27() {
    use oxirush_ngap::inspect;

    // The names that the value at a path can have, as the specification spells them
    let names = inspect::enumerated_at("UEContextReleaseRequest", "/ngap/Cause/nas").unwrap();
    println!("{:?}", names.unwrap());

    // A value of another type has none
    let choice = inspect::enumerated_at("UEContextReleaseRequest", "/ngap/Cause/value");
    assert_eq!(choice, Ok(None));
}

/// The Rust block 28 of README.md.
#[rustfmt::skip]
fn block_28() {
    use oxirush_ngap::inspect;

    // Whether a path can select anything in a message of a name, without a view
    let check = |path| inspect::check_path("UEContextReleaseRequest", path);
    assert_eq!(check("/ngap/Cause/nas"), Ok(()));
    assert_eq!(check("/ngap/*/radioNetwork"), Ok(()));

    // A name that is written wrong, and an IE that the message does not have
    for wrong in ["/ngap/Cause/radioNetwrok", "/ngap/RAN-UE-NGAP-ID/valeu", "/ngap/GUAMI"] {
        println!("{}", check(wrong).unwrap_err());
    }
}

/// The Rust block 29 of README.md.
#[rustfmt::skip]
fn block_29() {
    use oxirush_ngap::{ErrorKind, inspect, ngap::NGAP_PDU};
    use oxirush_ngap::tree;

    // Written: the name of the type with its value, in place of the octets
    let name = "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer";
    let required = tree!({"message": "HandoverRequired", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"SourceToTarget-TransparentContainer": {name: {
            "rRCContainer": "0102",
            "targetCell-ID": {"nR-CGI": {"pLMNIdentity": "208-93", "nRCellIdentity": 4660}},
            "uEHistoryInformation": [{"lastVisitedCellInformation": {"eUTRANCell": "00"}}],
        }}},
    ]});

    // A tree shows the octets that the container was encoded as
    let pdu = NGAP_PDU::from_tree(&required).unwrap();
    println!("{}", pdu.to_tree()["ies"][2]);

    // Read: a path goes through the name of the type
    let mut view = pdu.to_view().unwrap();
    let container = format!("/ngap/SourceToTarget-TransparentContainer/{name}");
    let cell = format!("{container}/targetCell-ID/nR-CGI/nRCellIdentity");
    assert_eq!(inspect::check_path("HandoverRequired", &cell), Ok(()));

    // The path is an error until `open` has decoded the octets, and selects the value after that
    assert_eq!(inspect::select(&view, &cell).unwrap_err().kind(), ErrorKind::NotOpened);
    inspect::open(&mut view, &cell).unwrap();
    assert_eq!(inspect::select(&view, &cell).unwrap(), [&tree!(4660)]);

    // What the octets carry, decoded
    let decoded = format!("{container}/decoded");
    println!("{}", inspect::select(&view, &decoded).unwrap()[0]);

    // An edit goes into it, and what is not edited is sent as it was received
    inspect::set(&mut view, &cell, tree!(7)).unwrap();
    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree()["ies"][2]);
}

/// The Rust block 30 of README.md.
#[rustfmt::skip]
fn block_30() {
    use oxirush_ngap::ngap::{NGAP_PDU, SNSSAI, SST};

    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    let pdu = NGAP_PDU::decode(&bytes).unwrap();
    let text = serde_json::to_string(&pdu).unwrap();
    let read: NGAP_PDU = serde_json::from_str(&text).unwrap();
    assert_eq!(read.encode().unwrap(), bytes);

    // A value alone has the form that a tree shows it in: an SST is a number
    let slice = SNSSAI::new(SST([1]), None, None);
    println!("{}", serde_json::to_string(&slice).unwrap());
}
