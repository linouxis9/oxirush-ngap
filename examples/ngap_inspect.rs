//! Decode an NGAP PDU, print its values with their paths, edit one IE, add two and encode
//! it again.
//!
//! ```sh
//! cargo run --example ngap_inspect --features inspect
//! ```

use oxirush_ngap::tree;
use oxirush_ngap::{Ngap, inspect, ngap::NGAPPDU};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // NG SETUP REQUEST: GlobalRANNodeID, SupportedTAList and DefaultPagingDRX.
    let wire = hex::decode(
        "00150025000003001b00080002f839000000000066000d000060b72f0002f839000002c80015400100",
    )?;
    let pdu = NGAPPDU::decode(&wire)?;
    let mut tree = pdu.to_tree();

    // The message by the name that ASN.1 gives it, then each value with the path that
    // selects it: `/ngap` stands for the IEs of the message, and an IE goes by its name.
    let set = Ngap::object_set(pdu.direction(), pdu.procedure_code());
    let name = set.expect("a message of the specification").name;
    println!("{name}");
    for (path, value) in inspect::paths(&tree) {
        println!("{path} = {value}");
    }

    // The PLMN identity is "208-93", and the TAC and the SST are numbers.
    let area = "/ngap/SupportedTAList/0";
    let tac = format!("{area}/tAC");
    let plmn = format!("{area}/broadcastPLMNList/0/pLMNIdentity");
    assert_eq!(inspect::select(&tree, &tac)?, [&tree!(0x60b72f)]);
    assert_eq!(inspect::select(&tree, &plmn)?, [&tree!("208-93")]);

    // Edit an IE through its value. The other IEs are encoded to the octets received.
    inspect::set(&mut tree, &tac, tree!(7))?;
    inspect::set(&mut tree, &plmn, tree!("001-01"))?;

    // Add an IE at the end, by its name: its value has the type of the IE, here a
    // PrintableString. The message does not assign it a criticality.
    let name = tree!({"RANNodeName": "gnb-1", "criticality": "ignore"});
    inspect::insert(&mut tree, "/ngap/-", name)?;

    // Add given octets as an IE, before the paging DRX.
    let unknown = tree!({"60000": {"octets": "C0FFEE"}, "criticality": "ignore"});
    inspect::insert(&mut tree, "/ngap/DefaultPagingDRX", unknown)?;

    // Encode the tree again.
    let edited = NGAPPDU::from_tree(&tree)?;
    println!("{}", hex::encode_upper(edited.encode()?));

    // The IE that was not edited is the octets that were received.
    let (before, after) = (pdu.protocol_ies()?, edited.protocol_ies()?);
    assert_eq!(before.0[0].value, after.0[0].value);

    Ok(())
}
