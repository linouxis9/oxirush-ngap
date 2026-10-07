//! Decode an NGAP PDU, print its values with their paths, edit one IE, add two and encode
//! it again.
//!
//! ```sh
//! cargo run --example inspect --features inspect
//! ```

use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // NG SETUP REQUEST: GlobalRANNodeID, SupportedTAList and DefaultPagingDRX.
    let wire = hex::decode(
        "00150025000003001b00080002f839000000000066000d000060b72f0002f839000002c80015400100",
    )?;
    let pdu = NGAP_PDU::decode(&wire)?;
    let mut tree = inspect::inspect_pdu(&pdu)?;
    // The message by the name that ASN.1 gives it, then each value with the path that
    // selects it: `/ngap` stands for the IEs of the message, and an IE goes by its name.
    let name = inspect::message_name(&pdu).expect("a message of TS 38.413");
    println!("{name}");
    for (path, value) in inspect::paths(&tree) {
        println!("{path} = {value}");
    }

    // The PLMN identity is "208-93", and the TAC and the SST are numbers.
    let area = "/ngap/SupportedTAList/value/0";
    let tac = inspect::select(&tree, &format!("{area}/tAC"))?;
    assert_eq!(tac, [&json!(0x60b72f)]);
    let plmn = format!("{area}/broadcastPLMNList/0/pLMNIdentity");
    assert_eq!(inspect::select(&tree, &plmn)?, [&json!("208-93")]);

    // Edit an IE through its typed value. The other IEs keep the octets received.
    inspect::set(&mut tree, &format!("{area}/tAC"), json!(7))?;
    inspect::set(&mut tree, &plmn, json!("001-01"))?;
    // Add an IE at the end: its `value` has the type of the identifier, here a
    // PrintableString.
    let name = json!({"id": "RANNodeName", "criticality": "ignore", "value": "gnb-1"});
    inspect::insert(&mut tree, "/ngap/-", name)?;
    // Add given octets as an IE, before the paging DRX: its `octets`, without `value`.
    let unknown = json!({"id": 60000, "criticality": "ignore", "octets": "C0FFEE"});
    inspect::insert(&mut tree, "/ngap/DefaultPagingDRX", unknown)?;

    let edited = inspect::encode_pdu(&tree)?;
    println!("{}", hex::encode_upper(edited.encode()?));
    // The IE that was not edited is the octets that were received.
    let tree = inspect::inspect_pdu(&edited)?;
    let node = inspect::select(&tree, "/ngap/GlobalRANNodeID/octets")?;
    assert_eq!(node, [&json!("0002F83900000000")]);
    Ok(())
}
