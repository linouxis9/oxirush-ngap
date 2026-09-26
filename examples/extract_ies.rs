//! Demonstrate `extract_ngap_ies!` across representative NGAP procedures.
//!
//! The three sections decode complete NGAP PDUs and extract required/optional
//! IEs, custom expressions, nested handover choices, and a 5GS NAS PDU carried
//! inside the Initial Context Setup PDU-session list.

use rasn::types::FixedBitString;

use oxirush_ngap::helpers::*;
use oxirush_ngap::macros::MissingIeError;
use oxirush_ngap::ngap::*;
use oxirush_ngap::{build_ngap, extract_ngap_ies};

// ── 1. Simple extraction: required IDs + optional cause ────────────────────

fn handle_release_request(pdu: &NGAP_PDU) -> Result<Vec<String>, MissingIeError> {
    let Some(msg): Option<UEContextReleaseRequest> =
        decode_initiating(pdu, ID_UECONTEXT_RELEASE_REQUEST.0)
    else {
        return Ok(vec![]);
    };

    // `req` fields return Err(MissingIeError) if absent or invalid.
    // `opt` fields become Option<T>.
    // Without `=> expr`, extraction defaults to `.0` (newtype unwrap).
    extract_ngap_ies!(&msg, UEContextReleaseRequest,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
        opt cause: String = Cause(c) => format!("{c:?}"),
    );

    let mut result = vec![
        format!("AMF-UE-NGAP-ID: {amf_id}"),
        format!("RAN-UE-NGAP-ID: {ran_id}"),
    ];
    if let Some(cause) = cause {
        result.push(format!("Cause: {cause}"));
    }
    Ok(result)
}

// ── 2. Complex extraction: handover with pattern matching ──────────────────

fn handle_handover_required(pdu: &NGAP_PDU) -> Result<Vec<String>, MissingIeError> {
    let Some(msg): Option<HandoverRequired> = decode_initiating(pdu, ID_HANDOVER_PREPARATION.0)
    else {
        return Ok(vec![]);
    };

    // Demonstrates concrete ENUMERATED values, a nested TargetID CHOICE,
    // transparent-container lengths, and custom extraction expressions.
    extract_ngap_ies!(&msg, HandoverRequired,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
        opt ho_type: HandoverType = HandoverType(value) => value,
        opt cause_str: String = Cause(cause) => format!("{cause:?}"),
        opt target: String = TargetID(target_value) => match target_value {
            TargetID::targetRANNodeID(value) => match value.global_rannode_id {
                GlobalRANNodeID::globalGNB_ID(gnb) => {
                    let (mcc, mnc) = plmn_from(&gnb.p_lmnidentity);
                    format!("target gNB in PLMN {mcc}-{mnc}")
                }
                _ => "non-gNB target".to_string(),
            },
            _ => "non-NG-RAN target".to_string(),
        },
        opt container_len: usize =
            SourceToTarget_TransparentContainer(container) => container.0.len(),
    );

    Ok(vec![
        format!("AMF-UE-NGAP-ID: {amf_id}"),
        format!("RAN-UE-NGAP-ID: {ran_id}"),
        format!("HandoverType: {ho_type:?}"),
        format!("Cause: {cause_str:?}"),
        format!("Target: {target:?}"),
        format!("S2T container bytes: {container_len:?}"),
    ])
}

// ── 3. InitialContextSetupRequest: many IEs + nested PDU session ───────────

fn handle_initial_context_setup(pdu: &NGAP_PDU) -> Result<Vec<String>, MissingIeError> {
    let Some(msg): Option<InitialContextSetupRequest> =
        decode_initiating(pdu, ID_INITIAL_CONTEXT_SETUP.0)
    else {
        return Ok(vec![]);
    };

    extract_ngap_ies!(&msg, InitialContextSetupRequest,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
        opt sessions: PDUSessionResourceSetupListCxtReq =
            PDUSessionResourceSetupListCxtReq(list) => list,
        opt ambr_dl: String = UEAggregateMaximumBitRate(ambr) =>
            ambr.u_eaggregate_maximum_bit_rate_dl.0.to_string(),
    );

    let session_count = sessions.as_ref().map_or(0, |list| list.0.len());
    let nas_len = sessions.as_ref().and_then(nested_nas_len).unwrap_or(0);
    Ok(vec![
        format!("AMF-UE-NGAP-ID: {amf_id}"),
        format!("RAN-UE-NGAP-ID: {ran_id}"),
        format!("PDU sessions: {session_count}"),
        format!("5GS NAS PDU: {nas_len} bytes"),
        format!("DL AMBR: {ambr_dl:?} bps"),
    ])
}

fn main() {
    // ── Build test PDUs ─────────────────────────────────────────────────────

    // 1. UEContextReleaseRequest
    let release_pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    );

    // 2. HandoverRequired
    let handover_pdu = build_ngap!(InitiatingMessage, HandoverPreparation,
        REJECT, HandoverRequired,
        REJECT AMF_UE_NGAP_ID(100u64),
        REJECT RAN_UE_NGAP_ID(50u32),
        REJECT HandoverType(HandoverType::intra5gs),
        IGNORE Cause(Cause::radioNetwork(
            CauseRadioNetwork::handover_desirable_for_radio_reason,
        )),
        REJECT TargetID(target_gnb()),
        REJECT PDUSessionResourceListHORqd(handover_required_sessions()),
        REJECT SourceToTarget_TransparentContainer(vec![0xDE, 0xAD, 0xBE, 0xEF]),
    );

    // 3. InitialContextSetupRequest. The NAS PDU is nested in a PDU-session
    // item, as specified by TS 38.413.
    let network = plmn("208", "93");
    let ics_pdu = build_ngap!(InitiatingMessage, InitialContextSetup,
        REJECT, InitialContextSetupRequest,
        REJECT AMF_UE_NGAP_ID(200u64),
        REJECT RAN_UE_NGAP_ID(10u32),
        REJECT UEAggregateMaximumBitRate(UEAggregateMaximumBitRate::new(
            BitRate(1_000_000_000u64.into()),
            BitRate(500_000_000u64.into()),
            None,
        )),
        REJECT GUAMI(guami(network, 1, 1, 0)),
        REJECT PDUSessionResourceSetupListCxtReq(session_setup_request_list(vec![
            0x7e, 0x00, 0x42, 0x01,
        ])),
        REJECT AllowedNSSAI(AllowedNSSAI(vec![AllowedNSSAIItem::new(
            s_nssai(1, Some([0x00, 0x00, 0x01])),
            None,
        )])),
        REJECT UESecurityCapabilities(ue_security_capabilities(&[0xe0, 0xe0])),
        REJECT SecurityKey(SecurityKey(FixedBitString::<256>::ZERO)),
    );

    // Encode → decode round-trip, then extract.
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
        let bytes = pdu.encode().expect("encode failed");
        let decoded = NGAP_PDU::decode(&bytes).expect("decode failed");

        println!("=== {name} ({decoded}) ===");
        match handler(&decoded) {
            Ok(lines) => {
                for line in &lines {
                    println!("  {line}");
                }
            }
            Err(error) => eprintln!("  Error: {error}"),
        }
        println!();
    }

    // ── Equivalent hand-written extraction (for comparison) ────────────────
    // Without `extract_ngap_ies!`, release-request extraction would manually:
    //
    //   let mut amf_id = None;
    //   for ie in &msg.protocol_ies.0 {
    //       if ie.id == ID_AMF_UE_NGAP_ID {
    //           if let Ok(value) = decode_open_type::<AMFUENGAPID>(&ie.value) {
    //               amf_id = Some(value.0);
    //           }
    //       }
    //   }
    //   let amf_id = amf_id.ok_or(MissingIeError { ie_name: "amf_id" })?;
}

fn decode_initiating<T: rasn::Decode>(pdu: &NGAP_PDU, procedure_code: u8) -> Option<T> {
    match pdu {
        NGAP_PDU::initiatingMessage(message) if message.procedure_code.0 == procedure_code => {
            decode_open_type(&message.value).ok()
        }
        _ => None,
    }
}

fn target_gnb() -> TargetID {
    let network = plmn("208", "93");
    TargetID::targetRANNodeID(TargetRANNodeID::new(
        GlobalRANNodeID::globalGNB_ID(global_gnb_id(network.clone(), 0x123456)),
        tai(network, &[0x00, 0x00, 0x01]),
        None,
    ))
}

fn session_setup_request_list(nas: Vec<u8>) -> PDUSessionResourceSetupListCxtReq {
    PDUSessionResourceSetupListCxtReq(vec![PDUSessionResourceSetupItemCxtReq::new(
        PDUSessionID(1),
        Some(NASPDU::from(nas)),
        s_nssai(1, Some([0x00, 0x00, 0x01])),
        vec![0x01].into(),
        None,
    )])
}

fn handover_required_sessions() -> PDUSessionResourceListHORqd {
    PDUSessionResourceListHORqd(vec![PDUSessionResourceItemHORqd::new(
        PDUSessionID(1),
        vec![0x01].into(),
        None,
    )])
}

fn nested_nas_len(list: &PDUSessionResourceSetupListCxtReq) -> Option<usize> {
    list.0
        .iter()
        .find_map(|item| item.n_as_pdu.as_ref().map(|nas| nas.0.len()))
}
