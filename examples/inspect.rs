//! Decode an NGAP PDU, print its tree, edit one IE, add another and encode it again.
//!
//! ```sh
//! cargo run --example inspect --features inspect
//! ```

use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use serde_json::json;

/// The number of the IE that ASN.1 names `id-<name>`.
fn ie_id(name: &str) -> u16 {
    let known = inspect::ie_names().iter().find(|(_, known)| *known == name);
    known.expect("an IE of TS 38.413").0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // UE CONTEXT RELEASE REQUEST: AMF-UE-NGAP-ID, RAN-UE-NGAP-ID and Cause.
    let wire = hex::decode("002a4015000003000a00020000005500020000000f40020000")?;
    let pdu = NGAP_PDU::decode(&wire)?;
    let mut tree = inspect::inspect_pdu(&pdu)?;
    println!("{}", serde_json::to_string_pretty(&tree)?);

    let ies = tree["message"]["protocolIEs"]
        .as_array_mut()
        .expect("the IEs of the message");
    // Edit an IE through its typed value. The other IEs keep the octets received.
    let cause = ies.iter_mut().find(|ie| ie["id"] == ie_id("Cause"));
    cause.expect("a Cause")["value"] = json!({"radioNetwork": "user-inactivity"});
    // Add an IE: its `value` has the type of the identifier. Given octets would be sent
    // as `_raw_value`, without `value`.
    ies.push(json!({
        "id": ie_id("PDUSessionResourceListCxtRelReq"),
        "criticality": "reject",
        "value": [{"pDUSessionID": 5}],
    }));

    let edited = inspect::encode_pdu(&tree)?;
    println!("{}", hex::encode_upper(edited.encode()?));
    Ok(())
}
