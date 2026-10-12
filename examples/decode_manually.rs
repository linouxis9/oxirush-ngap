//! Encode and decode NGAP PDUs **without macros** using the codec and the generated types.
//!
//! This example demonstrates the complete raw-codec workflow:
//! - Constructing an `NGAP_PDU` by hand (`ProtocolIEID`, `Criticality`, the octets of
//!   an open type)
//! - Encoding and decoding with the functions of the aligned PER runtime
//! - Decoding and pattern-matching open protocol IE values
//! - Verifying encode → decode round-trip fidelity
//!
//! `oxirush_ngap::aper` is the runtime that the crate re-exports, `oxirush-aper`, so
//! the example needs no dependency on it.
//!
//! For the ergonomic `build_ngap!` / `extract_ngap_ies!` versions, see
//! `build_pdu.rs` and `extract_ies.rs`.

use oxirush_ngap::aper;
use oxirush_ngap::ngap::*;

fn main() {
    // ── 1. Build an NGSetupResponse by hand ─────────────────────────────────
    //
    // The response identifies its served GUAMIs, capacity, PLMNs, and slices.

    let plmn_identity = PLMNIdentity([0x02, 0xF8, 0x39]);

    // The GUAMI that the AMF serves.
    let guami = GUAMI::new(
        plmn_identity,
        // A bit string of 64 bits at most is a number with a length, which the type
        // of one size has.
        AMFRegionID::new(1).expect("an AMF Region ID of 8 bits"),
        AMFSetID::new(1).expect("an AMF Set ID of 10 bits"),
        AMFPointer::new(0).expect("an AMF Pointer of 6 bits"),
        None,
    );
    let served_guamis = ServedGUAMIList(vec![ServedGUAMIItem::new(guami, None, None)]);

    // The PLMN that it supports, with one slice.
    let slice = SNSSAI::new(SST([1]), Some(SD([0x00, 0x00, 0x01])), None);
    let slices = SliceSupportList(vec![SliceSupportItem::new(slice, None)]);
    let plmn_support = PLMNSupportList(vec![PLMNSupportItem::new(plmn_identity, slices, None)]);

    let amf_name = AMFName("amf-1".into());

    // The value of an IE is an open type: every entry holds its numeric ID, its
    // criticality, and the APER octets of its concrete NGAP value.
    let ies = vec![
        ProtocolIEField::new(
            ID_AMFNAME,
            Criticality::reject,
            encode_open_type(&amf_name).expect("encode AMF name"),
        ),
        ProtocolIEField::new(
            ID_SERVED_GUAMILIST,
            Criticality::reject,
            encode_open_type(&served_guamis).expect("encode served GUAMIs"),
        ),
        ProtocolIEField::new(
            ID_RELATIVE_AMFCAPACITY,
            Criticality::ignore,
            encode_open_type(&RelativeAMFCapacity(255)).expect("encode AMF capacity"),
        ),
        ProtocolIEField::new(
            ID_PLMNSUPPORT_LIST,
            Criticality::reject,
            encode_open_type(&plmn_support).expect("encode PLMN support"),
        ),
    ];
    let response = NGSetupResponse::new(ProtocolIEContainer(ies));

    // The outer PDU wraps the APER-encoded message in its direction and
    // ASN.1-derived procedure code.
    let pdu = NGAP_PDU::successfulOutcome(SuccessfulOutcome::new(
        ID_NGSETUP,
        Criticality::reject,
        encode_open_type(&response).expect("encode NG Setup Response"),
    ));

    // ── 2. Encode to APER ───────────────────────────────────────────────────
    // `pdu.encode()` is the convenience API; this deliberately uses the runtime.

    let aper_bytes = aper::encode(&pdu).expect("APER encode failed");

    println!("Encoded NGSetupResponse: {} bytes", aper_bytes.len());
    println!("APER hex: {}", hex::encode(&aper_bytes));

    // ── 3. Decode from APER ─────────────────────────────────────────────────
    // `NGAP_PDU::decode(&bytes)` is the equivalent convenience API.

    // The PDU borrows the octets of its message from `aper_bytes`: nothing is copied.
    let decoded: NGAPPDU = aper::decode(&aper_bytes).expect("APER decode failed");

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
        // `NGAP_PDU` is the PDU that owns its octets, `NGAPPDU<'static>`: a PDU that
        // borrows is matched by the name of its type.
        NGAPPDU::successfulOutcome(outcome) => {
            println!("Type:           SuccessfulOutcome");
            println!("Procedure code: {}", outcome.procedure_code.0);
            println!("Criticality:    {:?}", outcome.criticality);

            let response: NGSetupResponse =
                decode_open_type(&outcome.value).expect("decode NG Setup Response");

            for ie in &response.protocol_ies.0 {
                match ie.id {
                    ID_AMFNAME => {
                        let name: AMFName = decode_open_type(&ie.value).expect("decode AMF name");
                        println!("AMF Name:       {}", name.0);
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
                            let plmn_hex: String =
                                plmn.iter().map(|b| format!("{b:02x}")).collect();
                            println!("  PLMN: {plmn_hex}");
                        }
                    }

                    ID_PLMNSUPPORT_LIST => {
                        let list: PLMNSupportList =
                            decode_open_type(&ie.value).expect("decode PLMN support");

                        println!("PLMN Support:   {} entries", list.0.len());
                        for item in &list.0 {
                            let plmn = &item.p_lmnidentity.0;
                            let plmn_hex: String =
                                plmn.iter().map(|b| format!("{b:02x}")).collect();
                            let slices = item.slice_support_list.0.len();
                            println!("  PLMN: {plmn_hex}, slices: {slices}");
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
