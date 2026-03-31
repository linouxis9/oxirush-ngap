//! Build NGAP PDUs using the `build_ngap!` and `build_ngap_ie!` ergonomic macros.
//!
//! These macros eliminate the deeply nested ProtocolIEs boilerplate that APER-generated
//! types require. Compare the macro invocation with the equivalent hand-written code
//! at the bottom of this example.

use asn1_codecs::aper::AperCodec;
use asn1_codecs::PerCodecData;
use oxirush_ngap::macros::*;
use oxirush_ngap::ngap::*;
use oxirush_ngap::{build_ngap, build_ngap_ie};

fn main() {
    // ── Build an InitialContextSetupResponse with build_ngap! ───────────

    let amf_ue_id: u64 = 1;
    let ran_ue_id: u32 = 0;

    let pdu = build_ngap!(SuccessfulOutcome, Id_InitialContextSetup,
        PROC_INITIAL_CONTEXT_SETUP, REJECT, InitialContextSetupResponse,
        IGNORE IE_AMF_UE_NGAP_ID => Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID(amf_ue_id)),
        IGNORE IE_RAN_UE_NGAP_ID => Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID(ran_ue_id)),
    );

    println!("=== InitialContextSetupResponse (via build_ngap!) ===");
    print_and_encode(&pdu);

    // ── Build a UEContextReleaseRequest ─────────────────────────────────

    let pdu = build_ngap!(InitiatingMessage, Id_UEContextReleaseRequest,
        PROC_UE_CONTEXT_RELEASE_REQUEST, REJECT, UEContextReleaseRequest,
        REJECT IE_AMF_UE_NGAP_ID => Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID(amf_ue_id)),
        REJECT IE_RAN_UE_NGAP_ID => Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID(ran_ue_id)),
        IGNORE IE_CAUSE => Id_Cause(
            Cause::RadioNetwork(CauseRadioNetwork(CauseRadioNetwork::USER_INACTIVITY))
        ),
    );

    println!("\n=== UEContextReleaseRequest (via build_ngap!) ===");
    print_and_encode(&pdu);

    // ── Build a single IE with build_ngap_ie! ───────────────────────────

    let cause_ie = build_ngap_ie!(UEContextReleaseRequest, IGNORE IE_CAUSE =>
        Id_Cause(Cause::RadioNetwork(CauseRadioNetwork(CauseRadioNetwork::USER_INACTIVITY)))
    );
    println!("\n=== Single IE (via build_ngap_ie!) ===");
    println!("IE ID: {}, Criticality: {:?}", cause_ie.id.0, cause_ie.criticality.0);

    // ── Equivalent hand-written code (for comparison) ───────────────────
    // Without macros, the same InitialContextSetupResponse would be:
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
    let mut output = PerCodecData::new_aper();
    pdu.aper_encode(&mut output).expect("APER encode failed");
    let bytes = output.into_bytes();
    println!("APER ({} bytes): {}", bytes.len(), hex::encode(&bytes));
}
