//! Demonstrate `extract_ngap_ies!` across representative NGAP procedures.
//!
//! The three sections decode complete NGAP PDUs and extract required/optional
//! IEs, custom expressions, nested handover choices, and a 5GS NAS PDU carried
//! inside the Initial Context Setup PDU-session list.

use oxirush_ngap::helpers::*;
use oxirush_ngap::ngap::*;
use oxirush_ngap::{Error, build_ngap, extract_ngap_ies};

// ── 1. Simple extraction: required IDs + optional cause ────────────────────

fn handle_release_request(pdu: &NGAP_PDU) -> Result<Vec<String>, Error> {
    // The message of the PDU, as its type: another message is an error.
    let msg: UEContextReleaseRequest = pdu.decode_value()?;

    // A `req` field whose IE is absent is an error of the kind `MissingIe`.
    // An `opt` field is an Option<T>, `None` when its IE is absent.
    // An IE that is there and does not decode is an error for both.
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

fn handle_handover_required(pdu: &NGAP_PDU) -> Result<Vec<String>, Error> {
    let msg: HandoverRequired = pdu.decode_value()?;

    // Demonstrates concrete ENUMERATED values, a nested TargetID CHOICE,
    // transparent-container lengths, and custom extraction expressions.
    extract_ngap_ies!(&msg, HandoverRequired,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        req ran_id: u32 = RAN_UE_NGAP_ID(id),

        opt ho_type: HandoverType = HandoverType(value) => value,
        opt cause_str: String = Cause(cause) => format!("{cause:?}"),

        opt target: String = TargetID(target_value) => match target_value {
            TargetID::targetRANNodeID(value) => match value.global_ran_node_id {
                GlobalRANNodeID::globalGNB_ID(gnb) => match plmn_from(&gnb.plmn_identity) {
                    Some((mcc, mnc)) => format!("target gNB in PLMN {mcc}-{mnc}"),
                    None => "target gNB without a PLMN identity".to_string(),
                },
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

fn handle_initial_context_setup(pdu: &NGAP_PDU) -> Result<Vec<String>, Error> {
    let msg: InitialContextSetupRequest = pdu.decode_value()?;

    extract_ngap_ies!(&msg, InitialContextSetupRequest,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        req ran_id: u32 = RAN_UE_NGAP_ID(id),

        opt sessions: PDUSessionResourceSetupListCxtReq =
            PDUSessionResourceSetupListCxtReq(list) => list,

        opt ambr_dl: String = UEAggregateMaximumBitRate(ambr) =>
            ambr.ue_aggregate_maximum_bit_rate_dl.0.to_string(),
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

fn main() -> Result<(), Error> {
    // ── Build test PDUs ─────────────────────────────────────────────────────

    // 1. UEContextReleaseRequest
    let release_pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,

        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )?;

    // 2. HandoverRequired
    let cause = Cause::radioNetwork(CauseRadioNetwork::handover_desirable_for_radio_reason);
    let handover_pdu = build_ngap!(InitiatingMessage, HandoverPreparation,
        REJECT, HandoverRequired,

        REJECT AMF_UE_NGAP_ID(100u64),
        REJECT RAN_UE_NGAP_ID(50u32),

        REJECT HandoverType(HandoverType::intra5gs),
        IGNORE Cause(cause),
        REJECT TargetID(target_gnb()?),

        REJECT PDUSessionResourceListHORqd(handover_required_sessions()),
        REJECT SourceToTarget_TransparentContainer(vec![0xDE, 0xAD, 0xBE, 0xEF]),
    )?;

    // 3. InitialContextSetupRequest. The NAS PDU is nested in a PDU-session
    // item, as specified by TS 38.413.
    let network = plmn("208", "93")?;
    let bit_rate =
        UEAggregateMaximumBitRate::new(BitRate(1_000_000_000), BitRate(500_000_000), None);
    let sessions = session_setup_request_list(vec![0x7e, 0x00, 0x42, 0x01]);
    let allowed_slice = AllowedNSSAIItem::new(s_nssai(1, Some([0x00, 0x00, 0x01])), None);
    let ics_pdu = build_ngap!(InitiatingMessage, InitialContextSetup,
        REJECT, InitialContextSetupRequest,

        REJECT AMF_UE_NGAP_ID(200u64),
        REJECT RAN_UE_NGAP_ID(10u32),

        REJECT UEAggregateMaximumBitRate(bit_rate),
        REJECT GUAMI(guami(network, 1, 1, 0)?),
        REJECT PDUSessionResourceSetupListCxtReq(sessions),
        REJECT AllowedNSSAI(AllowedNSSAI(vec![allowed_slice])),

        REJECT UESecurityCapabilities(ue_security_capabilities(0xe0, 0xe0)),
        REJECT SecurityKey([0; 32]),
    )?;

    // Encode → decode round-trip, then extract.
    type Handler = fn(&NGAP_PDU) -> Result<Vec<String>, Error>;
    let cases: [(&str, &NGAP_PDU, Handler); 3] = [
        (
            "UEContextReleaseRequest",
            &release_pdu,
            handle_release_request,
        ),
        ("HandoverRequired", &handover_pdu, handle_handover_required),
        (
            "InitialContextSetupRequest",
            &ics_pdu,
            handle_initial_context_setup,
        ),
    ];

    for (name, pdu, handler) in cases {
        let bytes = pdu.encode()?;
        let decoded = NGAP_PDU::decode(&bytes)?;

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

    // The `ngap_decode_manually` example reads the IEs of a message by hand, which is
    // what `extract_ngap_ies!` does for each field.
    Ok(())
}

fn target_gnb() -> Result<TargetID, Error> {
    let network = plmn("208", "93")?;
    let gnb = GlobalRANNodeID::globalGNB_ID(global_gnb_id(network, 0x123456, 24)?);
    let area = tai(network, [0x00, 0x00, 0x01]);

    Ok(TargetID::targetRANNodeID(TargetRANNodeID::new(
        gnb, area, None,
    )))
}

fn session_setup_request_list(nas: Vec<u8>) -> PDUSessionResourceSetupListCxtReq<'static> {
    PDUSessionResourceSetupListCxtReq(vec![PDUSessionResourceSetupItemCxtReq::new(
        PDUSessionID(1),
        Some(NASPDU::from(nas)),
        s_nssai(1, Some([0x00, 0x00, 0x01])),
        vec![0x01].into(),
        None,
    )])
}

fn handover_required_sessions() -> PDUSessionResourceListHORqd<'static> {
    PDUSessionResourceListHORqd(vec![PDUSessionResourceItemHORqd::new(
        PDUSessionID(1),
        vec![0x01].into(),
        None,
    )])
}

fn nested_nas_len(list: &PDUSessionResourceSetupListCxtReq) -> Option<usize> {
    list.0
        .iter()
        .find_map(|item| item.nas_pdu.as_ref().map(|nas| nas.0.len()))
}
