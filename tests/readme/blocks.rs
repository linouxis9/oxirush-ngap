// The Rust blocks of README.md, each as a function. `tests/readme.rs` writes
// this file and says how: do not edit it.
#![allow(unused)]

use std::cell::RefCell;

thread_local!(static PRINTED: RefCell<String> = const { RefCell::new(String::new()) });

// What a block prints is kept, to be compared with what the README shows.
macro_rules! println {
    () => {
        PRINTED.with(|printed| printed.borrow_mut().push('\n'))
    };
    ($($argument:tt)*) => {
        PRINTED.with(|printed| {
            let line = format!($($argument)*);
            let mut printed = printed.borrow_mut();
            printed.push_str(&line);
            printed.push('\n');
        })
    };
}

/// What `block` prints.
pub fn printed(block: fn()) -> String {
    PRINTED.with(|printed| printed.borrow_mut().clear());
    block();
    PRINTED.with(|printed| printed.take())
}

pub const BLOCKS: &[fn()] = &[
    block_1,
    block_2,
    block_3,
    block_4,
    block_5,
    block_6,
    block_7,
    block_8,
    block_9,
    block_10,
    block_11,
    block_12,
    block_13,
    block_14,
    block_15,
    block_16,
    block_17,
    block_18,
    block_19,
    block_20,
    block_21,
    block_22,
    block_23,
    block_24,
];

/// The Rust block 1 of README.md.
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
fn block_2() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use serde_json::json;

    // The same message as a tree: its name and its IEs by name
    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    let pdu = NGAP_PDU::decode(&bytes).unwrap();
    println!("{}", pdu.to_tree().unwrap());

    // Its view is edited at a path, and is a PDU again
    let mut view = pdu.to_view().unwrap();
    let lost = json!("radio-connection-with-ue-lost");
    inspect::set(&mut view, "/ngap/Cause/radioNetwork", lost).unwrap();
    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree().unwrap());

    // Its octets are those that the typed example built
    let octets = hex::encode(edited.encode().unwrap());
    assert_eq!(octets, "002a4015000003000a0002002a005500020007000f40020540");
}

/// The Rust block 3 of README.md.
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
fn block_4() {
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

/// The Rust block 5 of README.md.
fn block_5() {
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

/// The Rust block 6 of README.md.
fn block_6() {
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

/// The Rust block 7 of README.md.
fn block_7() {
    use oxirush_ngap::{extract_ngap_ies, ngap::*};

    // The cause of a request, when it has one that is read
    fn cause<'a>(request: &'a UEContextReleaseRequest<'a>) -> Option<Cause<'a>> {
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

/// The Rust block 8 of README.md.
fn block_8() {
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

/// The Rust block 9 of README.md.
fn block_9() {
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

/// The Rust block 10 of README.md.
fn block_10() {
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

/// The Rust block 11 of README.md.
fn block_11() {
    use oxirush_ngap::helpers::*;

    // A PLMN from its MCC and its MNC, and what is identified within it
    let network = plmn("208", "93").unwrap();
    let amf = guami(network, 1, 1, 0).unwrap(); // region, set, pointer
    let tracking_area = tai(network, [0x00, 0x00, 0x01]); // TAC
    let cell = nr_cgi(network, 0x123456, 1).unwrap(); // gNB, cell
    let gnb = global_gnb_id(network, 0x123456).unwrap();

    // A bit string from octets, and the NR algorithms of a UE: the first
    // octet of its encryption bitmap, then of its integrity bitmap
    let algorithm_mask = oxirush_ngap::BitString::from([0xe0]);
    let capabilities = ue_security_capabilities(0xe0, 0xe0);

    // What is no MCC, and an AMF Set ID that does not fit in its 10 bits
    println!("{}", plmn("20", "93").unwrap_err());
    println!("{}", guami(network, 1, 1024, 0).unwrap_err());
}

/// The Rust block 12 of README.md.
fn block_12() {
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

/// The Rust block 13 of README.md.
fn block_13() {
    use oxirush_ngap::ngap::NGAP_PDU;

    // UE CONTEXT RELEASE REQUEST
    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    let pdu = NGAP_PDU::decode(&bytes).unwrap();

    // Its tree, then its third IE as the view has it
    println!("{}", pdu.to_tree().unwrap());
    let view = pdu.to_view().unwrap();
    println!("{}", view["message"]["protocolIEs"][2]);
}

/// The Rust block 14 of README.md.
fn block_14() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use serde_json::json;

    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    let pdu = NGAP_PDU::decode(&bytes).unwrap();
    let view = pdu.to_view().unwrap();

    // `/ngap` is the IEs of the message, and an IE goes by its name
    let select = |path| inspect::select(&view, path).unwrap();
    assert_eq!(select("/ngap/RAN-UE-NGAP-ID/value"), [&json!(7)]);
    assert_eq!(select("/ngap/Cause/radioNetwork"), [&json!("user-inactivity")]);
    assert_eq!(select("/ngap/Cause/octets"), [&json!("0500")]);

    // By its position, by its name whatever its case and its separators, by its
    // identifier, and each IE with `*`
    assert_eq!(select("/ngap/1/value"), select("/NGAP/ran_ue_ngap_id/value"));
    assert_eq!(select("/ngap/@id=85/value"), [&json!(7)]);
    assert_eq!(select("/ngap/*/criticality").len(), 3);

    // Nothing where the message could have a value and has none
    assert!(select("/ngap/Cause/nas").is_empty());
    assert!(select("/ngap/GUAMI/pLMNIdentity").is_empty());

    // Each value with the path that selects it
    for (path, value) in inspect::paths(&view) {
        println!("{path} = {value}");
    }
}

/// The Rust block 15 of README.md.
fn block_15() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use serde_json::json;

    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    let pdu = NGAP_PDU::decode(&bytes).unwrap();
    let mut view = pdu.to_view().unwrap();

    // A value, and another alternative of a CHOICE in place of the one that is there
    inspect::set(&mut view, "/ngap/RAN-UE-NGAP-ID/value", json!(9)).unwrap();
    inspect::set(&mut view, "/ngap/Cause/nas", json!("Normal Release")).unwrap();
    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree().unwrap());

    // What was not edited is the octets that were received
    let after = edited.to_view().unwrap();
    let octets = inspect::select(&after, "/ngap/AMF-UE-NGAP-ID/octets").unwrap();
    assert_eq!(octets, [&json!("002A")]);

    // A name that the type does not have is refused, and the view is as it was
    let wrong = inspect::set(&mut view, "/ngap/Cause/radioNetwrok", json!("unspecified"));
    println!("{}", wrong.unwrap_err());
}

/// The Rust block 16 of README.md.
fn block_16() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use serde_json::json;

    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    let pdu = NGAP_PDU::decode(&bytes).unwrap();
    let mut view = pdu.to_view().unwrap();

    // An IE is added at the end, by its name and its typed value
    let name = json!({"id": "RANNodeName", "criticality": "ignore", "value": "gnb-1"});
    inspect::insert(&mut view, "/ngap/-", name).unwrap();

    // Before another IE, by its number and its octets
    let unknown = json!({"id": 60000, "criticality": "ignore", "octets": "c0ffee"});
    inspect::insert(&mut view, "/ngap/Cause", unknown).unwrap();

    // An IE is taken out by its name
    inspect::remove(&mut view, "/ngap/AMF-UE-NGAP-ID").unwrap();

    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree().unwrap()["ies"]);
}

/// The Rust block 17 of README.md.
fn block_17() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use serde_json::json;

    // A tree: the name of the message, whatever its case and its separators, then its IEs
    let request = json!({
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
    println!("{}", json!(inspect::select(&view, "/ngap/*/criticality").unwrap()));
}

/// The Rust block 18 of README.md.
fn block_18() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use serde_json::json;

    // `{"octets": "…"}` in place of a value is the octets of the IE, whatever they are
    let request = json!({"message": "UEContextReleaseRequest", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
    ]});
    let pdu = NGAP_PDU::from_tree(&request).unwrap();
    println!("{}", hex::encode(pdu.encode().unwrap()));

    // A view shows an IE that does not decode by its octets, and says why
    let mut view = pdu.to_view().unwrap();
    println!("{}", view["message"]["protocolIEs"][1]);

    // In a view, the `octets` of an IE are set in place of its value
    inspect::set(&mut view, "/ngap/AMF-UE-NGAP-ID/octets", json!("00ff")).unwrap();
    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree().unwrap());
}

/// The Rust block 19 of README.md.
fn block_19() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use serde_json::json;

    // A message that has one IE twice
    let twice = json!({"message": "UEContextReleaseRequest", "ies": [
        {"AMF-UE-NGAP-ID": 1},
        {"RAN-UE-NGAP-ID": 7},
        {"AMF-UE-NGAP-ID": 2},
    ]});
    let pdu = NGAP_PDU::from_tree(&twice).unwrap();
    let mut view = pdu.to_view().unwrap();

    // The name selects both, in the order of the message
    let both = inspect::select(&view, "/ngap/AMF-UE-NGAP-ID/value").unwrap();
    assert_eq!(both, [&json!(1), &json!(2)]);

    // Each has its position, which is how `paths` names it, and its name may follow
    inspect::set(&mut view, "/ngap/2/AMF-UE-NGAP-ID/value", json!(3)).unwrap();

    // After a position, the name of another IE is an error that says which IE is there
    println!("{}", inspect::select(&view, "/ngap/2/RAN-UE-NGAP-ID/value").unwrap_err());

    // The values, each with its path
    for (path, value) in inspect::paths(&view) {
        if path.ends_with("/value") {
            println!("{path} = {value}");
        }
    }
}

/// The Rust block 20 of README.md.
fn block_20() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use serde_json::{Value, json};

    // A message whose one IE is a list of two items
    let request = json!({"message": "InitialContextSetupRequest", "ies": [
        {"AllowedNSSAI": [{"s-NSSAI": {"sST": 1}}, {"s-NSSAI": {"sST": 2, "sD": "000001"}}]},
    ]});
    let pdu = NGAP_PDU::from_tree(&request).unwrap();
    let mut view = pdu.to_view().unwrap();

    // An item by its position, and each of them after `value`
    let select = |view, path| -> Vec<Value> {
        let selected = inspect::select(view, path).unwrap();
        selected.into_iter().cloned().collect()
    };
    assert_eq!(select(&view, "/ngap/AllowedNSSAI/1/s-NSSAI/sD"), [json!("000001")]);
    assert_eq!(select(&view, "/ngap/AllowedNSSAI/value/*/s-NSSAI/sST"), [json!(1), json!(2)]);

    // An optional member that an item does not have selects nothing
    assert!(select(&view, "/ngap/AllowedNSSAI/0/s-NSSAI/sD").is_empty());

    // An item is added at the end, and one is taken out
    inspect::insert(&mut view, "/ngap/AllowedNSSAI/-", json!({"s-NSSAI": {"sST": 3}})).unwrap();
    inspect::remove(&mut view, "/ngap/AllowedNSSAI/0").unwrap();

    // `null` takes an optional member out
    inspect::set(&mut view, "/ngap/AllowedNSSAI/0/s-NSSAI/sD", Value::Null).unwrap();

    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree().unwrap());
}

/// The Rust block 21 of README.md.
fn block_21() {
    use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    use serde_json::json;

    // A message with a transfer, written as the message that it contains
    let request = json!({"message": "PDUSessionResourceSetupRequest", "ies": [
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
    assert_eq!(inspect::select(&view, &kind).unwrap(), [&json!("ipv4")]);

    // The same value by the path that says `value` and `decoded`
    let long = "/ngap/PDUSessionResourceSetupListSUReq/value/0\
        /pDUSessionResourceSetupRequestTransfer/decoded/PDUSessionType/value";
    assert_eq!(inspect::select(&view, long), inspect::select(&view, &kind));

    // And by the names that the tree shows, joined: the IE by its position, then its name
    let joined = format!("{transfer}/protocolIEs/0/PDUSessionType/value");
    assert_eq!(inspect::select(&view, &joined), inspect::select(&view, &kind));

    // An edit of the message that it contains is encoded into the transfer
    inspect::set(&mut view, &kind, json!("ipv6")).unwrap();
    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree().unwrap()["ies"][2]);
}

/// The Rust block 22 of README.md.
fn block_22() {
    use oxirush_ngap::inspect;

    // The names that the value at a path can have, as the specification spells them
    let names = inspect::enumerated_at("UEContextReleaseRequest", "/ngap/Cause/nas").unwrap();
    println!("{:?}", names.unwrap());

    // A value of another type has none
    let choice = inspect::enumerated_at("UEContextReleaseRequest", "/ngap/Cause/value");
    assert_eq!(choice, Ok(None));
}

/// The Rust block 23 of README.md.
fn block_23() {
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

/// The Rust block 24 of README.md.
fn block_24() {
    use oxirush_ngap::{ErrorKind, inspect, ngap::NGAP_PDU};
    use serde_json::json;

    // Written: the name of the type with its value, in place of the octets
    let name = "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer";
    let required = json!({"message": "HandoverRequired", "ies": [
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
    println!("{}", pdu.to_tree().unwrap()["ies"][2]);

    // Read: a path goes through the name of the type
    let mut view = pdu.to_view().unwrap();
    let container = format!("/ngap/SourceToTarget-TransparentContainer/{name}");
    let cell = format!("{container}/targetCell-ID/nR-CGI/nRCellIdentity");
    assert_eq!(inspect::check_path("HandoverRequired", &cell), Ok(()));

    // The path is an error until `open` has decoded the octets, and selects the value after that
    assert_eq!(inspect::select(&view, &cell).unwrap_err().kind(), ErrorKind::NotOpened);
    inspect::open(&mut view, &cell).unwrap();
    assert_eq!(inspect::select(&view, &cell).unwrap(), [&json!(4660)]);

    // What the octets carry, decoded
    let decoded = format!("{container}/decoded");
    println!("{}", inspect::select(&view, &decoded).unwrap()[0]);

    // An edit goes into it, and what is not edited is sent as it was received
    inspect::set(&mut view, &cell, json!(7)).unwrap();
    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree().unwrap()["ies"][2]);
}
