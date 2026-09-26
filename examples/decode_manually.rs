//! Encode and decode NGAP PDUs **without macros** using rasn and the generated types.
//!
//! This example demonstrates the complete raw-codec workflow:
//! - Constructing an `NGAP_PDU` by hand (`ProtocolIEID`, `Criticality`, `Any`)
//! - Encoding and decoding with the raw rasn APER functions
//! - Decoding and pattern-matching open protocol IE values
//! - Verifying encode → decode round-trip fidelity
//!
//! For the ergonomic `build_ngap!` / `extract_ngap_ies!` versions, see
//! `build_pdu.rs` and `extract_ies.rs`.

use rasn::types::{FixedBitString, FixedOctetString, PrintableString};

use oxirush_ngap::ngap::*;

fn main() {
    // ── 1. Build an NGSetupResponse by hand ─────────────────────────────────
    //
    // The response identifies its served GUAMIs, capacity, PLMNs, and slices.

    let plmn_identity = PLMNIdentity(FixedOctetString::new([0x02, 0xF8, 0x39]));
    let guami = GUAMI::new(
        plmn_identity.clone(),
        AMFRegionID(fixed_bits::<8>(1)),
        AMFSetID(fixed_bits::<10>(1)),
        AMFPointer(fixed_bits::<6>(0)),
        None,
    );
    let served_guamis = ServedGUAMIList(vec![ServedGUAMIItem::new(guami, None, None)]);
    let plmn_support = PLMNSupportList(vec![PLMNSupportItem::new(
        plmn_identity,
        SliceSupportList(vec![SliceSupportItem::new(
            SNSSAI::new(
                SST(FixedOctetString::new([1])),
                Some(SD(FixedOctetString::new([0x00, 0x00, 0x01]))),
                None,
            ),
            None,
        )]),
        None,
    )]);
    let amf_name = AMFName(PrintableString::try_from("OxiRushAMF").expect("valid AMF name"));

    // In rasn's stable opaque-open-type representation, every entry holds its
    // numeric ID, criticality, and the APER bytes of its concrete NGAP value.
    let ies = vec![
        AnonymousNGSetupResponseProtocolIEs::new(
            ID_AMFNAME,
            Criticality::reject,
            encode_open_type(&amf_name).expect("encode AMF name"),
        ),
        AnonymousNGSetupResponseProtocolIEs::new(
            ID_SERVED_GUAMILIST,
            Criticality::reject,
            encode_open_type(&served_guamis).expect("encode served GUAMIs"),
        ),
        AnonymousNGSetupResponseProtocolIEs::new(
            ID_RELATIVE_AMFCAPACITY,
            Criticality::ignore,
            encode_open_type(&RelativeAMFCapacity(255)).expect("encode AMF capacity"),
        ),
        AnonymousNGSetupResponseProtocolIEs::new(
            ID_PLMNSUPPORT_LIST,
            Criticality::reject,
            encode_open_type(&plmn_support).expect("encode PLMN support"),
        ),
    ];
    let response = NGSetupResponse::new(NGSetupResponseProtocolIEs(ies));

    // The outer PDU wraps the APER-encoded message in its direction and
    // ASN.1-derived procedure code.
    let pdu = NGAP_PDU::successfulOutcome(SuccessfulOutcome::new(
        ID_NGSETUP,
        Criticality::reject,
        encode_open_type(&response).expect("encode NG Setup Response"),
    ));

    // ── 2. Encode to APER ───────────────────────────────────────────────────
    // `pdu.encode()` is the convenience API; this deliberately uses raw rasn.

    let aper_bytes = rasn::aper::encode(&pdu).expect("APER encode failed");
    println!("Encoded NGSetupResponse: {} bytes", aper_bytes.len());
    println!("APER hex: {}", hex::encode(&aper_bytes));

    // ── 3. Decode from APER ─────────────────────────────────────────────────
    // `NGAP_PDU::decode(&bytes)` is the equivalent convenience API.

    let decoded: NGAP_PDU = rasn::aper::decode(&aper_bytes).expect("APER decode failed");

    println!("\n=== Decoded: {decoded} ===");
    println!(
        "Procedure: {}, Direction: {}\n",
        decoded.procedure_name(),
        decoded.direction()
    );

    // ── 4. Inspect the decoded PDU and its typed open values ────────────────
    //
    // Without extraction macros, match the direction, decode the message open
    // type, iterate `protocol_ies`, then decode each recognized IE open type.

    match &decoded {
        NGAP_PDU::successfulOutcome(outcome) => {
            println!("Type:           SuccessfulOutcome");
            println!("Procedure code: {}", outcome.procedure_code.0);
            println!("Criticality:    {:?}", outcome.criticality);

            let response: NGSetupResponse =
                decode_open_type(&outcome.value).expect("decode NG Setup Response");
            for ie in &response.protocol_ies.0 {
                match ie.id {
                    ID_AMFNAME => {
                        let name: AMFName = decode_open_type(&ie.value).expect("decode AMF name");
                        println!(
                            "AMF Name:       {}",
                            String::from_utf8_lossy(name.0.as_bytes())
                        );
                    }
                    ID_RELATIVE_AMFCAPACITY => {
                        let capacity: RelativeAMFCapacity =
                            decode_open_type(&ie.value).expect("decode AMF capacity");
                        println!("AMF Capacity:   {}", capacity.0);
                    }
                    ID_SERVED_GUAMILIST => {
                        let list: ServedGUAMIList =
                            decode_open_type(&ie.value).expect("decode served GUAMIs");
                        println!("Served GUAMIs:  {} entries", list.0.len());
                        for item in &list.0 {
                            let plmn = &item.g_uami.p_lmnidentity.0;
                            println!(
                                "  PLMN: {}",
                                plmn.iter().map(|b| format!("{b:02x}")).collect::<String>()
                            );
                        }
                    }
                    ID_PLMNSUPPORT_LIST => {
                        let list: PLMNSupportList =
                            decode_open_type(&ie.value).expect("decode PLMN support");
                        println!("PLMN Support:   {} entries", list.0.len());
                        for item in &list.0 {
                            println!(
                                "  PLMN: {}, slices: {}",
                                item.p_lmnidentity
                                    .0
                                    .iter()
                                    .map(|b| format!("{b:02x}"))
                                    .collect::<String>(),
                                item.slice_support_list.0.len()
                            );
                        }
                    }
                    _ => println!("  (other IE: id={})", ie.id.0),
                }
            }
        }
        _ => println!("Unexpected PDU type"),
    }

    // ── 5. Verify round-trip ────────────────────────────────────────────────

    let re_encoded = decoded.encode().expect("re-encode failed");
    assert_eq!(aper_bytes, re_encoded, "Round-trip mismatch!");
    println!("\nRound-trip OK ({} bytes)", re_encoded.len());
}

fn fixed_bits<const N: usize>(value: u64) -> FixedBitString<N> {
    let mut bits = FixedBitString::<N>::ZERO;
    for index in 0..N {
        bits.set(index, (value >> (N - index - 1)) & 1 == 1);
    }
    bits
}
