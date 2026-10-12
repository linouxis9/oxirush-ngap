//! Build NGAP PDUs using the `build_ngap!` and `build_ngap_ie!` ergonomic macros.
//!
//! The examples cover simple and complex context setup, UE release, handover
//! outcomes, failure handling, and standalone IE construction. The macro
//! invocations hide numeric IDs and open-type encoding; the commented
//! construction at the end shows the equivalent generated types.

use oxirush_ngap::helpers::*;
use oxirush_ngap::ngap::*;
use oxirush_ngap::{Error, build_ngap, build_ngap_ie};

fn main() -> Result<(), Error> {
    // ── 1. Simple: InitialContextSetupResponse ─────────────────────────────

    let amf_ue_id: u64 = 1;
    let ran_ue_id: u32 = 0;

    // The direction, the procedure, its criticality and the message, then
    // each IE with its criticality.
    let pdu = build_ngap!(SuccessfulOutcome, InitialContextSetup,
        REJECT, InitialContextSetupResponse,

        IGNORE AMF_UE_NGAP_ID(amf_ue_id),
        IGNORE RAN_UE_NGAP_ID(ran_ue_id),
        IGNORE PDUSessionResourceSetupListCxtRes(session_setup_response_list()),
    )?;

    println!("=== 1. InitialContextSetupResponse ===");
    print_and_encode(&pdu)?;

    // ── 2. UEContextReleaseRequest with Cause ──────────────────────────────

    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,

        REJECT AMF_UE_NGAP_ID(amf_ue_id),
        REJECT RAN_UE_NGAP_ID(ran_ue_id),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )?;

    println!("\n=== 2. UEContextReleaseRequest ===");
    print_and_encode(&pdu)?;

    // ── 3. Complex: InitialContextSetupRequest with PDU session + security ─
    //
    // NGAP carries the optional 5GS NAS PDU inside a PDU-session item. This
    // demonstrates that nested payload, fixed-size security key, GUAMI, NSSAI,
    // and UE security capabilities.

    let network = plmn("208", "93")?;
    let sessions = session_setup_request_list(vec![0x7e, 0x00, 0x42]);
    let bit_rate =
        UEAggregateMaximumBitRate::new(BitRate(1_000_000_000), BitRate(500_000_000), None);
    let allowed_slice = AllowedNSSAIItem::new(s_nssai(1, Some([0x00, 0x00, 0x01])), None);

    let pdu = build_ngap!(InitiatingMessage, InitialContextSetup,
        REJECT, InitialContextSetupRequest,

        // The UE, on each side of the interface
        REJECT AMF_UE_NGAP_ID(amf_ue_id),
        REJECT RAN_UE_NGAP_ID(ran_ue_id),

        // Its bit rate, its AMF, its sessions and its slices
        REJECT UEAggregateMaximumBitRate(bit_rate),
        REJECT GUAMI(guami(network, 1, 1, 0)?),
        REJECT PDUSessionResourceSetupListCxtReq(sessions),
        REJECT AllowedNSSAI(AllowedNSSAI(vec![allowed_slice])),

        // Its security capabilities and its key
        REJECT UESecurityCapabilities(ue_security_capabilities(0xe0, 0xe0)),
        REJECT SecurityKey([0; 32]),
    )?;

    println!("\n=== 3. InitialContextSetupRequest (complex) ===");
    print_and_encode(&pdu)?;

    // ── 4. Handover: procedure name differs from message name ──────────────
    //
    // HandoverPreparation procedure → HandoverRequired message. The target gNB
    // and PDU-session resource list are mandatory NGAP IEs.

    let target_gnb = GlobalRANNodeID::globalGNB_ID(global_gnb_id(network, 0x123456)?);
    let target_area = tai(network, [0x00, 0x00, 0x01]);
    let target = TargetID::targetRANNodeID(TargetRANNodeID::new(target_gnb, target_area, None));
    let cause = Cause::radioNetwork(CauseRadioNetwork::handover_desirable_for_radio_reason);

    let pdu = build_ngap!(InitiatingMessage, HandoverPreparation,
        REJECT, HandoverRequired,

        REJECT AMF_UE_NGAP_ID(amf_ue_id),
        REJECT RAN_UE_NGAP_ID(ran_ue_id),

        // The kind of handover, its cause and its target
        REJECT HandoverType(HandoverType::intra5gs),
        IGNORE Cause(cause),
        REJECT TargetID(target),

        // The sessions and the container that go to the target
        REJECT PDUSessionResourceListHORqd(handover_required_sessions()),
        REJECT SourceToTarget_TransparentContainer(vec![0x00, 0x01, 0x02]),
    )?;

    println!("\n=== 4. HandoverRequired (procedure=HandoverPreparation) ===");
    print_and_encode(&pdu)?;

    // ── 5. SuccessfulOutcome + UnsuccessfulOutcome ─────────────────────────

    let pdu = build_ngap!(SuccessfulOutcome, HandoverResourceAllocation,
        REJECT, HandoverRequestAcknowledge,

        IGNORE AMF_UE_NGAP_ID(amf_ue_id),
        IGNORE RAN_UE_NGAP_ID(ran_ue_id),

        IGNORE PDUSessionResourceAdmittedList(admitted_sessions()),
        REJECT TargetToSource_TransparentContainer(vec![0xAA, 0xBB]),
    )?;

    println!("\n=== 5. HandoverRequestAcknowledge (SuccessfulOutcome) ===");
    print_and_encode(&pdu)?;

    let pdu = build_ngap!(UnsuccessfulOutcome, NGSetup,
        REJECT, NGSetupFailure,

        IGNORE Cause(Cause::misc(CauseMisc::unknown_PLMN_or_SNPN)),
    )?;

    println!("\n=== 5. NGSetupFailure (UnsuccessfulOutcome) ===");
    print_and_encode(&pdu)?;

    // ── 6. build_ngap_ie! for conditional IE construction ──────────────────

    let cause_ie = build_ngap_ie!(UEContextReleaseRequest, IGNORE
        Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity))
    )?;

    println!("\n=== 6. Single IE (via build_ngap_ie!) ===");
    println!(
        "IE ID: {}, Criticality: {:?}",
        cause_ie.id.0, cause_ie.criticality
    );

    // ── Equivalent hand-written code (for comparison) ──────────────────────
    // Without macros, the same InitialContextSetupResponse (example 1) is:
    //
    //   let response = InitialContextSetupResponse::new(
    //       ProtocolIEContainer(vec![
    //           ProtocolIEField::new(
    //               ProtocolIEID(10),
    //               Criticality::ignore,
    //               encode_open_type(&AMFUENGAPID(1))?,
    //           ),
    //           ProtocolIEField::new(
    //               ProtocolIEID(85),
    //               Criticality::ignore,
    //               encode_open_type(&RANUENGAPID(0))?,
    //           ),
    //           ProtocolIEField::new(
    //               ProtocolIEID(75),
    //               Criticality::ignore,
    //               encode_open_type(&session_setup_response_list())?,
    //           ),
    //       ]),
    //   );
    //   let pdu = NGAP_PDU::successfulOutcome(SuccessfulOutcome::new(
    //       ProcedureCode(14),
    //       Criticality::reject,
    //       encode_open_type(&response)?,
    //   ));
    Ok(())
}

fn session_setup_response_list() -> PDUSessionResourceSetupListCxtRes<'static> {
    PDUSessionResourceSetupListCxtRes(vec![PDUSessionResourceSetupItemCxtRes::new(
        PDUSessionID(1),
        vec![0x01].into(),
        None,
    )])
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

fn admitted_sessions() -> PDUSessionResourceAdmittedList<'static> {
    PDUSessionResourceAdmittedList(vec![PDUSessionResourceAdmittedItem::new(
        PDUSessionID(1),
        vec![0x01].into(),
        None,
    )])
}

fn print_and_encode(pdu: &NGAP_PDU) -> Result<(), Error> {
    // Display example: "InitiatingMessage NGSetup (code=21)".
    println!("{pdu}");
    println!(
        "  procedure: {}  direction: {}  code: {}",
        pdu.procedure_name(),
        pdu.direction(),
        pdu.procedure_code()
    );

    let bytes = pdu.encode()?;
    println!("  APER ({} bytes): {}", bytes.len(), hex::encode(&bytes));
    Ok(())
}
