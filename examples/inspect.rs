//! Decode an NGAP PDU, print its tree, edit one IE, add two and encode it again.
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
    // NG SETUP REQUEST: GlobalRANNodeID, SupportedTAList and DefaultPagingDRX.
    let wire = hex::decode(
        "00150025000003001b00080002f839000000000066000d000060b72f0002f839000002c80015400100",
    )?;
    let pdu = NGAP_PDU::decode(&wire)?;
    let mut tree = inspect::inspect_pdu(&pdu)?;
    // The PLMN identity is "208-93", and the TAC and the SST are numbers.
    println!("{}", serde_json::to_string_pretty(&tree)?);

    let ies = tree["message"]["protocolIEs"]
        .as_array_mut()
        .expect("the IEs of the message");
    // Edit an IE through its typed value. The other IEs keep the octets received.
    let areas = ies
        .iter_mut()
        .find(|ie| ie["id"] == ie_id("SupportedTAList"));
    let area = &mut areas.expect("a SupportedTAList")["value"][0];
    area["tAC"] = json!(7);
    area["broadcastPLMNList"][0]["pLMNIdentity"] = json!("001-01");
    // Add an IE: its `value` has the type of the identifier, here a PrintableString.
    ies.push(json!({
        "id": ie_id("RANNodeName"),
        "criticality": "ignore",
        "value": "gnb-1",
    }));
    // Add given octets as an IE: `_raw_value`, without `value`.
    ies.push(json!({"id": 60000, "criticality": "ignore", "_raw_value": "C0FFEE"}));

    let edited = inspect::encode_pdu(&tree)?;
    println!("{}", hex::encode_upper(edited.encode()?));
    Ok(())
}
