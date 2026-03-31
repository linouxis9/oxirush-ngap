//! Demonstrate the `extract_ngap_ies!` macro for extracting NGAP protocol IEs.
//!
//! Shows how to decode an NGAP PDU and extract specific IEs with required/optional
//! semantics and custom expressions, eliminating manual iteration and matching.

use asn1_codecs::aper::AperCodec;
use asn1_codecs::PerCodecData;
use oxirush_ngap::macros::*;
use oxirush_ngap::ngap::*;
use oxirush_ngap::{build_ngap, extract_ngap_ies};

/// Simulates a handler that extracts IEs from a UEContextReleaseRequest.
///
/// - `amf_id` and `ran_id` are required — returns `Err(MissingIeError)` if absent
/// - `cause` is optional, with a custom expression
fn handle_release_request(pdu: &NGAP_PDU) -> Result<Vec<String>, MissingIeError> {
    let msg = match pdu {
        NGAP_PDU::InitiatingMessage(msg) => match &msg.value {
            InitiatingMessageValue::Id_UEContextReleaseRequest(req) => req,
            _ => return Ok(vec![]),
        },
        _ => return Ok(vec![]),
    };

    // Extract IEs using the `extract_ngap_ies!` macro
    // The `extract_ngap_ies!` macro pulls typed fields from a decoded message.
    // - `req` required fields that are missing cause the enclosing function (e.g. `handle_release_request` to return `Err(MissingIeError)`
    // - `opt` optional fields are to be checked as `Option<T>`
    extract_ngap_ies!(&msg.protocol_i_es.0, UEContextReleaseRequestProtocolIEs_EntryValue,
        req amf_id: u64     = Id_AMF_UE_NGAP_ID(id),
        req ran_id: u32     = Id_RAN_UE_NGAP_ID(id),
        opt cause:  String  = Id_Cause(c) => format!("{c:?}"),
    );

    // After the macro:
    // - amf_id: u64        (unwrapped, guaranteed present)
    // - ran_id: u32        (unwrapped, guaranteed present)
    // - cause:  Option<String>
    let mut result = vec![
        format!("AMF-UE-NGAP-ID: {amf_id}"),
        format!("RAN-UE-NGAP-ID: {ran_id}"),
    ];
    if let Some(c) = cause {
        result.push(format!("Cause: {c}"));
    }
    Ok(result)
}

fn main() {
    // Build a UEContextReleaseRequest with the build_ngap! macro
    let pdu = build_ngap!(InitiatingMessage, Id_UEContextReleaseRequest,
        PROC_UE_CONTEXT_RELEASE_REQUEST, REJECT, UEContextReleaseRequest,
        REJECT IE_AMF_UE_NGAP_ID => Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID(42)),
        REJECT IE_RAN_UE_NGAP_ID => Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID(7)),
        IGNORE IE_CAUSE => Id_Cause(
            Cause::RadioNetwork(CauseRadioNetwork(CauseRadioNetwork::USER_INACTIVITY))
        ),
    );

    // Encode to APER, then decode back (simulating network round-trip)
    let mut output = PerCodecData::new_aper();
    pdu.aper_encode(&mut output).expect("encode failed");
    let bytes = output.into_bytes();

    let mut codec = PerCodecData::from_slice_aper(&bytes);
    let decoded = NGAP_PDU::aper_decode(&mut codec).expect("decode failed");

    // Extract IEs from the decoded PDU
    println!("=== extract_ngap_ies! extraction ===\n");
    match handle_release_request(&decoded) {
        Ok(extracted) => {
            for line in &extracted {
                println!("  {line}");
            }
        }
        Err(e) => {
            eprintln!("  Error: {e}");
        }
    }

    // Without the macro, the equivalent code would be:
    //
    //   let mut amf_id: Option<u64> = None;
    //   let mut ran_id: Option<u32> = None;
    //   let mut cause: Option<String> = None;
    //   for ie in &msg.protocol_i_es.0 {
    //       match &ie.value {
    //           ...::Id_AMF_UE_NGAP_ID(id) => { amf_id = Some(id.0); }
    //           ...::Id_RAN_UE_NGAP_ID(id) => { ran_id = Some(id.0); }
    //           ...::Id_Cause(c) => { cause = Some(format!("{c:?}")); }
    //           _ => {}
    //       }
    //   }
    //   let amf_id = amf_id.ok_or(MissingIeError { ie_name: "amf_id" })?;
    //   let ran_id = ran_id.ok_or(MissingIeError { ie_name: "ran_id" })?;
    //   // cause stays as Option<String>
}
