//! The keys of NGAP that `Debug` prints by their length alone: K_gNB and the next hop
//! NH, which are a `SecurityKey`, as the generator is told. TOY, the protocol of the
//! tests of oxirush-aper, has the tests of how a key and what holds one print.

use oxirush_ngap::ngap::*;
use oxirush_ngap::{Error, build_ngap};

/// A key of which every octet is `5a`, and one of which every octet is `a5`.
const KGNB: [u8; 32] = [0x5a; 32];
const NH: [u8; 32] = [0xa5; 32];

/// What `Debug` printed has no four octets of either key in hexadecimal, in either
/// case, nor as numbers.
fn has_no_key(printed: &str) -> bool {
    let lower = printed.to_ascii_lowercase();
    !["5a5a5a5a", "a5a5a5a5", "90, 90, 90", "165, 165, 165"]
        .iter()
        .any(|key| lower.contains(key))
}

#[test]
fn debug_of_a_message_with_a_key_shows_the_length_of_its_octets() -> Result<(), Error> {
    // An INITIAL CONTEXT SETUP REQUEST with K_gNB
    let pdu = build_ngap!(REJECT InitialContextSetupRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        REJECT SecurityKey(SecurityKey::from(KGNB)),
    )?;
    let octets = pdu.encode()?;
    assert!(hex::encode(&octets).contains(&"5a".repeat(32)));
    let printed = format!("{:?} {:?}", NGAPPDU::decode(&octets)?, pdu.protocol_ies()?);
    assert!(has_no_key(&printed), "{printed}");
    assert!(printed.contains("value: 32 octets"), "{printed}");
    Ok(())
}

#[test]
fn debug_of_a_message_with_a_security_context_shows_no_next_hop() -> Result<(), Error> {
    // A PATH SWITCH REQUEST ACKNOWLEDGE with the next hop NH
    let context = SecurityContext {
        next_hop_chaining_count: NextHopChainingCount(1),
        next_hop_nh: SecurityKey::from(NH),
        ie_extensions: None,
    };
    let pdu = build_ngap!(REJECT PathSwitchRequestAcknowledge,
        IGNORE AMF_UE_NGAP_ID(1u64),
        IGNORE RAN_UE_NGAP_ID(7u32),
        REJECT SecurityContext(context),
    )?;
    assert!(hex::encode(pdu.encode()?).contains(&"a5".repeat(32)));
    let printed = format!("{pdu:?} {:?}", pdu.protocol_ies()?);
    assert!(has_no_key(&printed), "{printed}");
    assert!(
        printed.contains("id: ProtocolIEID(93), criticality: Reject, value: 33 octets"),
        "{printed}"
    );
    Ok(())
}
