#![cfg(feature = "serde")]
//! `Serialize` and `Deserialize`, with the feature `serde`: a PDU as its tree, and each
//! other type of the bindings as its value in the form of its ASN.1 type.

use oxirush_ngap::ngap::*;
use serde_json::json;

const RELEASE: &str = "002a4015000003000a0002002a005500020007000f40020500";

#[test]
fn a_pdu_is_serialized_as_its_tree_and_deserialized_from_it() {
    let tree = json!({"message": "UEContextReleaseRequest", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}},
    ]});
    let pdu: NGAP_PDU = serde_json::from_value(tree.clone()).unwrap();
    assert_eq!(hex::encode(pdu.encode().unwrap()), RELEASE);
    assert_eq!(serde_json::to_value(&pdu).unwrap(), tree);
    assert_eq!(
        serde_json::to_string(&pdu).unwrap(),
        pdu.to_tree().unwrap().to_string()
    );
    // A PDU that borrows its octets is the same tree.
    let octets = hex::decode(RELEASE).unwrap();
    let borrowed = NGAP_PDU::decode_borrowed(&octets).unwrap();
    assert_eq!(serde_json::to_value(&borrowed).unwrap(), tree);

    // What the tree refuses, the format refuses with the same words.
    let wrong = json!({"message": "UEContextReleaseRequest", "ies": [{"AMF-UE-NGAP-ID": -1}]});
    let error = serde_json::from_value::<NGAP_PDU>(wrong.clone()).unwrap_err();
    let refused = NGAP_PDU::from_tree(&wrong).unwrap_err();
    assert_eq!(error.to_string(), refused.to_string());

    // A PDU without a tree, of a procedure that the protocol does not have in its
    // direction, is not serialized.
    let unknown = hex::decode("00ff0003000000").unwrap();
    let unknown = NGAP_PDU::decode(&unknown).unwrap();
    let error = serde_json::to_value(&unknown).unwrap_err();
    assert_eq!(
        error.to_string(),
        unknown.to_tree().unwrap_err().to_string()
    );
}

#[test]
fn a_type_of_the_bindings_is_serialized_as_the_form_of_its_asn1_type() {
    let cause = Cause::radioNetwork(CauseRadioNetwork::user_inactivity);
    let written = json!({"radioNetwork": "user-inactivity"});
    assert_eq!(serde_json::to_value(&cause).unwrap(), written);
    assert_eq!(serde_json::from_value::<Cause>(written).unwrap(), cause);

    // The octets of a string, in hexadecimal: a PLMN identity, an SST and an SD are
    // octets here, where the tree of a PDU shows them as they are usually written.
    let slice = SNSSAI::new(SST([1]), Some(SD([0, 0, 0xab])), None);
    let written = json!({"sST": "01", "sD": "0000ab"});
    assert_eq!(serde_json::to_value(&slice).unwrap(), written);
    assert_eq!(serde_json::from_value::<SNSSAI>(written).unwrap(), slice);
    let network = PLMNIdentity([0x02, 0xf8, 0x39]);
    assert_eq!(serde_json::to_value(network).unwrap(), json!("02f839"));
    let upper: PLMNIdentity = serde_json::from_value(json!("02F839")).unwrap();
    assert_eq!(upper, network);

    // A number outside the constraint of its type is refused, as the tree refuses it.
    let error = serde_json::from_value::<AMF_UE_NGAP_ID>(json!(1u64 << 40)).unwrap_err();
    assert_eq!(
        error.to_string(),
        "this is not a value of AMF-UE-NGAP-ID: a number from 0 to 1099511627775 is \
         expected, and 1099511627776 is written"
    );
}
