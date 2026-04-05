//! Demonstrate the `extract_ngap_ies!` macro for extracting NGAP protocol IEs.
//!
//! Shows how to decode an NGAP PDU and extract specific IEs with required/optional
//! semantics and custom expressions, eliminating manual iteration and matching.

use oxirush_ngap::macros::MissingIeError;
use oxirush_ngap::ngap::*;
use oxirush_ngap::{build_ngap, extract_ngap_ies};

// ── 1. Simple extraction: required IDs + optional cause ────────────────────

fn handle_release_request(pdu: &NGAP_PDU) -> Result<Vec<String>, MissingIeError> {
    let msg = match pdu {
        NGAP_PDU::InitiatingMessage(msg) => match &msg.value {
            InitiatingMessageValue::Id_UEContextReleaseRequest(req) => req,
            _ => return Ok(vec![]),
        },
        _ => return Ok(vec![]),
    };

    // `req` fields return Err(MissingIeError) if absent.
    // `opt` fields become Option<T>.
    // Without `=> expr`, defaults to `.0` (newtype unwrap).
    extract_ngap_ies!(msg, UEContextReleaseRequest,
        req amf_id: u64     = AMF_UE_NGAP_ID(id),
        req ran_id: u32     = RAN_UE_NGAP_ID(id),
        opt cause:  String  = Cause(c) => format!("{c:?}"),
    );

    let mut result = vec![
        format!("AMF-UE-NGAP-ID: {amf_id}"),
        format!("RAN-UE-NGAP-ID: {ran_id}"),
    ];
    if let Some(c) = cause {
        result.push(format!("Cause: {c}"));
    }
    Ok(result)
}

// ── 2. Complex extraction: handover with pattern matching ──────────────────

fn handle_handover_required(pdu: &NGAP_PDU) -> Result<Vec<String>, MissingIeError> {
    let msg = match pdu {
        NGAP_PDU::InitiatingMessage(msg) => match &msg.value {
            InitiatingMessageValue::Id_HandoverPreparation(req) => req,
            _ => return Ok(vec![]),
        },
        _ => return Ok(vec![]),
    };

    // Demonstrates:
    // - Extracting nested enum variants with `if let` pattern matching
    // - Cloning opaque containers
    // - Default newtype unwrap (HandoverType → .0)
    extract_ngap_ies!(msg, HandoverRequired,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
        opt ho_type: u8 = HandoverType(ht),
        opt cause_str: String = Cause(c) => format!("{c:?}"),
        opt container_len: usize =
            SourceToTarget_TransparentContainer(c) => c.0.len(),
    );

    let mut result = vec![
        format!("AMF-UE-NGAP-ID: {amf_id}"),
        format!("RAN-UE-NGAP-ID: {ran_id}"),
        format!("HandoverType: {:?}", ho_type),
        format!("Cause: {:?}", cause_str),
        format!("S2T container bytes: {:?}", container_len),
    ];
    Ok(result)
}

// ── 3. InitialContextSetupRequest: many IEs, nested structs ────────────────

fn handle_initial_context_setup(pdu: &NGAP_PDU) -> Result<Vec<String>, MissingIeError> {
    let msg = match pdu {
        NGAP_PDU::InitiatingMessage(msg) => match &msg.value {
            InitiatingMessageValue::Id_InitialContextSetup(req) => req,
            _ => return Ok(vec![]),
        },
        _ => return Ok(vec![]),
    };

    extract_ngap_ies!(msg, InitialContextSetupRequest,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
        opt nas_pdu: Vec<u8> = NAS_PDU(pdu) => pdu.0.clone(),
        opt ambr_dl: u64 = UEAggregateMaximumBitRate(ambr) =>
            ambr.ue_aggregate_maximum_bit_rate_dl.0,
    );

    Ok(vec![
        format!("AMF-UE-NGAP-ID: {amf_id}"),
        format!("RAN-UE-NGAP-ID: {ran_id}"),
        format!("NAS PDU: {} bytes", nas_pdu.map(|p| p.len()).unwrap_or(0)),
        format!("DL AMBR: {:?} bps", ambr_dl),
    ])
}

fn main() {
    // ── Build test PDUs ─────────────────────────────────────────────────────

    // 1. UEContextReleaseRequest
    let release_pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        REJECT, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(Cause::RadioNetwork(CauseRadioNetwork(CauseRadioNetwork::USER_INACTIVITY))),
    );

    // 2. HandoverRequired
    let handover_pdu = build_ngap!(InitiatingMessage, HandoverPreparation,
        REJECT, HandoverRequired,
        REJECT AMF_UE_NGAP_ID(100u64),
        REJECT RAN_UE_NGAP_ID(50u32),
        REJECT HandoverType(HandoverType::INTRA5GS),
        IGNORE Cause(Cause::RadioNetwork(CauseRadioNetwork(
            CauseRadioNetwork::HANDOVER_DESIRABLE_FOR_RADIO_REASON,
        ))),
        REJECT SourceToTarget_TransparentContainer(vec![0xDE, 0xAD, 0xBE, 0xEF]),
    );

    // 3. InitialContextSetupRequest
    let ics_pdu = build_ngap!(InitiatingMessage, InitialContextSetup,
        REJECT, InitialContextSetupRequest,
        REJECT AMF_UE_NGAP_ID(200u64),
        REJECT RAN_UE_NGAP_ID(10u32),
        REJECT UEAggregateMaximumBitRate(UEAggregateMaximumBitRate {
            ue_aggregate_maximum_bit_rate_dl: BitRate(1_000_000_000),
            ue_aggregate_maximum_bit_rate_ul: BitRate(500_000_000),
            ie_extensions: None,
        }),
        IGNORE NAS_PDU(vec![0x7e, 0x00, 0x42, 0x01]),
    );

    // Encode → decode round-trip, then extract
    for (name, pdu, handler) in [
        (
            "UEContextReleaseRequest",
            &release_pdu,
            handle_release_request as fn(&NGAP_PDU) -> Result<Vec<String>, MissingIeError>,
        ),
        ("HandoverRequired", &handover_pdu, handle_handover_required),
        (
            "InitialContextSetupRequest",
            &ics_pdu,
            handle_initial_context_setup,
        ),
    ] {
        // encode() / decode() convenience methods
        let bytes = pdu.encode().expect("encode failed");
        let decoded = NGAP_PDU::decode(&bytes).expect("decode failed");

        println!("=== {name} ({decoded}) ===");
        match handler(&decoded) {
            Ok(lines) => {
                for line in &lines {
                    println!("  {line}");
                }
            }
            Err(e) => eprintln!("  Error: {e}"),
        }
        println!();
    }

    // ── Equivalent hand-written code (for comparison) ───────────────────
    // Without the macro, extract_ngap_ies!(msg, UEContextReleaseRequest, ...)
    // would be:
    //
    //   let mut amf_id: Option<u64> = None;
    //   let mut ran_id: Option<u32> = None;
    //   let mut cause: Option<String> = None;
    //   for ie in &msg.protocol_i_es.0 {
    //       match &ie.value {
    //           UEContextReleaseRequestProtocolIEs_EntryValue::Id_AMF_UE_NGAP_ID(id) => {
    //               amf_id = Some(id.0);
    //           }
    //           UEContextReleaseRequestProtocolIEs_EntryValue::Id_RAN_UE_NGAP_ID(id) => {
    //               ran_id = Some(id.0);
    //           }
    //           UEContextReleaseRequestProtocolIEs_EntryValue::Id_Cause(c) => {
    //               cause = Some(format!("{c:?}"));
    //           }
    //           _ => {}
    //       }
    //   }
    //   let amf_id = amf_id.ok_or(MissingIeError { ie_name: "amf_id" })?;
    //   let ran_id = ran_id.ok_or(MissingIeError { ie_name: "ran_id" })?;
    //   // cause stays as Option<String>
}
