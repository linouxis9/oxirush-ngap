//! Build NGAP PDUs using the `build_ngap!` and `build_ngap_ie!` ergonomic macros.
//!
//! These macros eliminate the deeply nested ProtocolIEs boilerplate that APER-generated
//! types require. Compare the macro invocation with the equivalent hand-written code
//! at the bottom of this example.

use oxirush_ngap::helpers::*;
use oxirush_ngap::ngap::*;
use oxirush_ngap::{build_ngap, build_ngap_ie};

fn main() {
    // ── 1. Simple: InitialContextSetupResponse ─────────────────────────────

    let amf_ue_id: u64 = 1;
    let ran_ue_id: u32 = 0;

    let pdu = build_ngap!(SuccessfulOutcome, InitialContextSetup,
        REJECT, InitialContextSetupResponse,
        IGNORE AMF_UE_NGAP_ID(amf_ue_id),
        IGNORE RAN_UE_NGAP_ID(ran_ue_id),
    );

    println!("=== 1. InitialContextSetupResponse ===");
    print_and_encode(&pdu);

    // ── 2. UEContextReleaseRequest with Cause ──────────────────────────────

    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        REJECT, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(amf_ue_id),
        REJECT RAN_UE_NGAP_ID(ran_ue_id),
        IGNORE Cause(Cause::RadioNetwork(CauseRadioNetwork(CauseRadioNetwork::USER_INACTIVITY))),
    );

    println!("\n=== 2. UEContextReleaseRequest ===");
    print_and_encode(&pdu);

    // ── 3. Complex: InitialContextSetupRequest with security + NSSAI ───────
    //
    // Demonstrates nested struct construction, bitvec security keys, and
    // S-NSSAI lists — a real-world AMF → gNB message (TS 38.413 §8.3.1).

    let pdu = build_ngap!(InitiatingMessage, InitialContextSetup,
        REJECT, InitialContextSetupRequest,
        REJECT AMF_UE_NGAP_ID(amf_ue_id),
        REJECT RAN_UE_NGAP_ID(ran_ue_id),
        REJECT UEAggregateMaximumBitRate(UEAggregateMaximumBitRate {
            ue_aggregate_maximum_bit_rate_dl: BitRate(1_000_000_000),
            ue_aggregate_maximum_bit_rate_ul: BitRate(500_000_000),
            ie_extensions: None,
        }),
        REJECT GUAMI(guami(plmn("208", "93"), 1, 1, 0)),
        REJECT AllowedNSSAI(vec![AllowedNSSAI_Item {
            s_nssai: s_nssai(1, Some([0x00, 0x00, 0x01])),
            ie_extensions: None,
        }]),
        REJECT SecurityKey(bytes_to_bitvec(&[0u8; 32])),
        IGNORE NAS_PDU(vec![0x7e, 0x00, 0x42]),
    );

    println!("\n=== 3. InitialContextSetupRequest (complex) ===");
    print_and_encode(&pdu);

    // ── 4. Handover: procedure name ≠ message name ─────────────────────────
    //
    // HandoverPreparation procedure → HandoverRequired message.
    // Also shows SourceToTarget_TransparentContainer (underscore in IE name).

    let pdu = build_ngap!(InitiatingMessage, HandoverPreparation,
        REJECT, HandoverRequired,
        REJECT AMF_UE_NGAP_ID(amf_ue_id),
        REJECT RAN_UE_NGAP_ID(ran_ue_id),
        REJECT HandoverType(HandoverType::INTRA5GS),
        IGNORE Cause(Cause::RadioNetwork(CauseRadioNetwork(
            CauseRadioNetwork::HANDOVER_DESIRABLE_FOR_RADIO_REASON,
        ))),
        REJECT SourceToTarget_TransparentContainer(vec![0x00, 0x01, 0x02]),
    );

    println!("\n=== 4. HandoverRequired (procedure=HandoverPreparation) ===");
    print_and_encode(&pdu);

    // ── 5. SuccessfulOutcome + UnsuccessfulOutcome ─────────────────────────

    let pdu = build_ngap!(SuccessfulOutcome, HandoverResourceAllocation,
        REJECT, HandoverRequestAcknowledge,
        IGNORE AMF_UE_NGAP_ID(amf_ue_id),
        IGNORE RAN_UE_NGAP_ID(ran_ue_id),
        REJECT TargetToSource_TransparentContainer(vec![0xAA, 0xBB]),
    );

    println!("\n=== 5. HandoverRequestAcknowledge (SuccessfulOutcome) ===");
    print_and_encode(&pdu);

    let pdu = build_ngap!(UnsuccessfulOutcome, NGSetup,
        REJECT, NGSetupFailure,
        IGNORE Cause(Cause::Misc(CauseMisc(CauseMisc::UNKNOWN_PLMN_OR_SNPN))),
    );

    println!("\n=== 5. NGSetupFailure (UnsuccessfulOutcome) ===");
    print_and_encode(&pdu);

    // ── 6. build_ngap_ie! for conditional IE construction ──────────────────

    let cause_ie = build_ngap_ie!(UEContextReleaseRequest, IGNORE
        Cause(Cause::RadioNetwork(CauseRadioNetwork(CauseRadioNetwork::USER_INACTIVITY)))
    );
    println!("\n=== 6. Single IE (via build_ngap_ie!) ===");
    println!(
        "IE ID: {}, Criticality: {:?}",
        cause_ie.id.0, cause_ie.criticality.0
    );

    // ── Equivalent hand-written code (for comparison) ───────────────────
    // Without macros, the same InitialContextSetupResponse (example 1) would be:
    //
    //   NGAP_PDU::SuccessfulOutcome(SuccessfulOutcome {
    //       procedure_code: ProcedureCode(14),
    //       criticality: Criticality(Criticality::REJECT),
    //       value: SuccessfulOutcomeValue::Id_InitialContextSetup(
    //           InitialContextSetupResponse {
    //               protocol_i_es: InitialContextSetupResponseProtocolIEs(vec![
    //                   InitialContextSetupResponseProtocolIEs_Entry {
    //                       id: ProtocolIE_ID(10),
    //                       criticality: Criticality(Criticality::IGNORE),
    //                       value: InitialContextSetupResponseProtocolIEs_EntryValue
    //                           ::Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID(1)),
    //                   },
    //                   InitialContextSetupResponseProtocolIEs_Entry {
    //                       id: ProtocolIE_ID(85),
    //                       criticality: Criticality(Criticality::IGNORE),
    //                       value: InitialContextSetupResponseProtocolIEs_EntryValue
    //                           ::Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID(0)),
    //                   },
    //               ]),
    //           },
    //       ),
    //   })
}

fn print_and_encode(pdu: &NGAP_PDU) {
    // Display impl shows: "InitiatingMessage NGSetup (code=21)"
    println!("{pdu}");
    // Inspect helpers
    println!(
        "  procedure: {}  direction: {}  code: {}",
        pdu.procedure_name(),
        pdu.direction(),
        pdu.procedure_code()
    );
    // encode() method on NGAP_PDU
    let bytes = pdu.encode().expect("APER encode failed");
    println!("  APER ({} bytes): {}", bytes.len(), hex::encode(&bytes));
}
