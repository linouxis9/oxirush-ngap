#![cfg(feature = "serde")]
//! `Serialize` and `Deserialize`, with the feature `serde`: a PDU as its tree, and each
//! other type of the bindings as its tree too, which is the form that the tree of a PDU
//! shows it in.

use oxirush_ngap::build_ngap;
use oxirush_ngap::ngap::*;
use oxirush_ngap::{tree, tree::Value};

const RELEASE: &str = "002a4015000003000a0002002a005500020007000f40020500";
const LATER: &str = include_str!("fixtures/later.tsv");

/// What serde_json writes of `value`, as a tree.
fn written<T: serde_core::Serialize + ?Sized>(value: &T) -> Value {
    serde_json::to_string(value).unwrap().parse().unwrap()
}

/// What serde_json reads of the text of `tree`.
fn read<T: serde_core::de::DeserializeOwned>(tree: &Value) -> Result<T, serde_json::Error> {
    serde_json::from_str(&tree.to_string())
}

#[test]
fn a_pdu_is_serialized_as_its_tree_and_deserialized_from_it() {
    let tree = tree!({"message": "UEContextReleaseRequest", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}},
    ]});
    let pdu: NGAP_PDU = read(&tree).unwrap();
    assert_eq!(hex::encode(pdu.encode().unwrap()), RELEASE);
    assert_eq!(written(&pdu), tree);
    assert_eq!(
        serde_json::to_string(&pdu).unwrap(),
        pdu.to_tree().to_string()
    );
    // A PDU that borrows its octets is the same tree.
    let octets = hex::decode(RELEASE).unwrap();
    let borrowed = NGAP_PDU::decode_borrowed(&octets).unwrap();
    assert_eq!(written(&borrowed), tree);

    // What the tree refuses, the format refuses with the same words.
    let wrong = tree!({"message": "UEContextReleaseRequest", "ies": [{"AMF-UE-NGAP-ID": -1}]});
    let error = read::<NGAP_PDU>(&wrong).unwrap_err();
    let refused = NGAP_PDU::from_tree(&wrong).unwrap_err();
    assert!(
        error.to_string().starts_with(&refused.to_string()),
        "{error}"
    );
}

/// The fifth finding of the review of the move of the paths to oxirush-utils: a PDU of a
/// procedure that the protocol does not have, and a private message, decoded and could not
/// be serialized.
#[test]
fn every_pdu_that_decodes_is_serialized_and_read_back() {
    for (octets, tree) in [
        // A procedure that the protocol does not have: its code and its direction.
        (
            "00ff0003000000",
            tree!({"procedure_code": 255, "direction": "InitiatingMessage", "ies": [],
                   "criticality": "reject"}),
        ),
        // A private message, which has no list of IEs: its octets.
        (
            "001f400a000000000000000211e8",
            tree!({"message": "PrivateMessage", "octets": "000000000000000211e8"}),
        ),
        // A message that does not decode as its type: its octets.
        (
            "002a400300ffff",
            tree!({"message": "UEContextReleaseRequest", "octets": "00ffff"}),
        ),
    ] {
        let pdu = NGAP_PDU::decode(&hex::decode(octets).unwrap()).unwrap();
        assert_eq!(written(&pdu), tree, "{octets}");
        let back: NGAP_PDU = read(&tree).unwrap();
        assert_eq!(hex::encode(back.encode().unwrap()), octets);
    }
}

/// The second finding of the review of the move of the paths to oxirush-utils: a round trip
/// through the tree or serde gave the octets without what a later release added. An IE
/// whose typed value has no place for it is its octets in the tree, and every vector of
/// `later.tsv` comes back as it was received.
#[test]
fn a_round_trip_gives_back_what_a_later_release_added() {
    let mut count = 0;
    for line in LATER.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<_> = line.split('\t').collect();
        let (name, later) = (fields[0], fields[1]);
        let pdu = NGAP_PDU::decode(&hex::decode(later).unwrap()).unwrap();
        let tree = pdu.to_tree();
        let again = NGAP_PDU::from_tree(&tree).unwrap();
        assert_eq!(hex::encode(again.encode().unwrap()), later, "{name}");
        let back: NGAP_PDU = read(&written(&pdu)).unwrap();
        assert_eq!(hex::encode(back.encode().unwrap()), later, "{name}");
        // What is kept as octets is an IE of the message, or what one holds.
        assert!(tree.to_string().contains("\"octets\""), "{name}: {tree}");
        count += 1;
    }
    assert_eq!(count, 9);
}

/// The sixth finding of the review of the move of the paths to oxirush-utils: a value
/// serialized alone had the form of its ASN.1 type, and the tree of a PDU the readable one.
/// It has the readable one alone too, and is read in either.
#[test]
fn a_type_of_the_bindings_is_serialized_as_the_tree_of_a_pdu_shows_it() {
    let cause = Cause::radioNetwork(CauseRadioNetwork::user_inactivity);
    let shown = tree!({"radioNetwork": "user-inactivity"});
    assert_eq!(written(&cause), shown);
    assert_eq!(read::<Cause>(&shown).unwrap(), cause);

    // An SST is a number, an SD its octets, a PLMN identity its MCC and its MNC, and the
    // parts of a GUAMI numbers.
    let slice = SNSSAI::new(SST([1]), Some(SD([0, 0, 0xab])), None);
    let shown = tree!({"sST": 1, "sD": "0000ab"});
    assert_eq!(written(&slice), shown);
    assert_eq!(read::<SNSSAI>(&shown).unwrap(), slice);
    let network = PLMNIdentity([0x02, 0xf8, 0x39]);
    assert_eq!(written(&network), tree!("208-93"));
    let guami = oxirush_ngap::helpers::guami(network, 1, 1, 0).unwrap();
    assert_eq!(
        written(&guami),
        tree!({"pLMNIdentity": "208-93", "aMFRegionID": 1, "aMFSetID": 1, "aMFPointer": 0})
    );
    // The form of the ASN.1 type is read too, in either case, after 0x where its
    // hexadecimal is digits alone, which would be read as a number too.
    assert_eq!(
        read::<SNSSAI>(&tree!({"sST": "0x01", "sD": "0000AB"})).unwrap(),
        slice
    );
    assert!(read::<SNSSAI>(&tree!({"sST": "01"})).is_err());
    assert_eq!(read::<PLMNIdentity>(&tree!("02F839")).unwrap(), network);
    // Octets that are no PLMN identity stay octets.
    let other = PLMNIdentity([0xff, 0xff, 0xff]);
    assert_eq!(written(&other), tree!("ffffff"));
    assert_eq!(read::<PLMNIdentity>(&tree!("ffffff")).unwrap(), other);
    // The same value in a tree of a PDU.
    let pdu = build_ngap!(SuccessfulOutcome, NGSetup, REJECT, NGSetupResponse,
        REJECT AMFName(AMFName("amf".into())),
        REJECT ServedGUAMIList(ServedGUAMIList(vec![ServedGUAMIItem::new(guami.clone(), None, None)])),
        IGNORE RelativeAMFCapacity(255u8),
        REJECT PLMNSupportList(PLMNSupportList(vec![PLMNSupportItem::new(
            network,
            SliceSupportList(vec![SliceSupportItem::new(slice.clone(), None)]),
            None,
        )])),
    )
    .unwrap();
    let tree = pdu.to_tree();
    assert_eq!(
        tree["ies"][1]["ServedGUAMIList"][0]["gUAMI"],
        written(&guami)
    );
    assert_eq!(
        tree["ies"][3]["PLMNSupportList"][0]["sliceSupportList"][0]["s-NSSAI"],
        written(&slice)
    );

    // A number outside the constraint of its type is refused, as the tree refuses it.
    let error = read::<AMF_UE_NGAP_ID>(&tree!(1u64 << 40)).unwrap_err();
    assert_eq!(
        error.to_string(),
        "this is not a value of AMF-UE-NGAP-ID: a number from 0 to 1099511627775 is \
         expected, and 1099511627776 is written"
    );
}

/// The seventh finding of the review of the move of the paths to oxirush-utils: a name
/// written twice in an object of JSON was read as its last value.
#[test]
fn a_name_written_twice_is_refused() {
    let twice = r#"{"message":"NGSetupRequest","message":"UEContextReleaseRequest","ies":[]}"#;
    let error = serde_json::from_str::<NGAP_PDU>(twice).unwrap_err();
    assert!(
        error
            .to_string()
            .starts_with("the member \"message\" is written twice in an object"),
        "{error}"
    );
    let twice = r#"{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42,"AMF-UE-NGAP-ID":43}]}"#;
    assert!(serde_json::from_str::<NGAP_PDU>(twice).is_err());
    assert!(twice.parse::<Value>().is_err());
    let twice = r#"{"sST":1,"sST":2}"#;
    assert!(serde_json::from_str::<SNSSAI>(twice).is_err());
}

/// The review of the release candidate found that YAML reads an unquoted `true` as a
/// boolean, which was refused for an ENUMERATED that has a value named `true`, as 43
/// types of NGAP have: it is that name, and is written back quoted.
#[test]
fn an_unquoted_true_of_yaml_is_the_name_of_an_enumerated() {
    let yaml = "message: UEContextReleaseRequest\nies:\n- AMF-UE-NGAP-ID: 1\n- RAN-UE-NGAP-ID: 2\n\
                - Cause: {nas: normal-release}\n- GW-Context-Release-Indication: true\n";
    let pdu: NGAP_PDU = serde_norway::from_str(yaml).unwrap();
    assert_eq!(
        pdu.to_tree()["ies"][3],
        tree!({"GW-Context-Release-Indication": "true"})
    );
    let quoted = serde_norway::from_str::<NGAP_PDU>(&yaml.replace(": true", ": 'true'")).unwrap();
    assert_eq!(quoted, pdu);
    assert!(
        serde_norway::to_string(&pdu)
            .unwrap()
            .contains("GW-Context-Release-Indication: 'true'")
    );
    // A type without such a name takes no boolean.
    let other = yaml.replace("{nas: normal-release}", "{nas: true}");
    let error = serde_norway::from_str::<NGAP_PDU>(&other)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("a name is expected, and true is written"),
        "{error}"
    );
}
