//! The kind of a PDU and the initiating message that it answers: every outcome of the
//! registry answers the initiating message of its procedure, and an initiating message
//! answers none. TOY, the protocol of the tests of oxirush-aper, has the tests of the
//! macro that writes it.

use oxirush_ngap::ngap::*;
use oxirush_ngap::{Criticality, Direction, Ngap, NgapPduKind as Kind, Octets};

/// The kind of a PDU of a direction and a procedure code, which are all that `kind()`
/// reads of it.
fn kind(direction: Direction, code: u8) -> Kind {
    let (procedure_code, criticality) = (ProcedureCode(code), Criticality::Ignore);
    let value = Octets::from(&[]);
    let pdu = match direction {
        Direction::Initiating => NGAPPDU::initiatingMessage(InitiatingMessage {
            procedure_code,
            criticality,
            value,
        }),
        Direction::Successful => NGAPPDU::successfulOutcome(SuccessfulOutcome {
            procedure_code,
            criticality,
            value,
        }),
        Direction::Unsuccessful => NGAPPDU::unsuccessfulOutcome(UnsuccessfulOutcome {
            procedure_code,
            criticality,
            value,
        }),
    };
    pdu.kind()
}

#[test]
fn every_outcome_answers_the_initiating_message_of_its_procedure() {
    let mut outcomes = 0;
    for set in Ngap::object_sets() {
        let message = kind(set.direction, set.procedure_code);
        assert!(!matches!(message, Kind::Other { .. }), "{}", set.name);
        if set.direction == Direction::Initiating {
            assert_eq!(message.initiating(), None, "{}", set.name);
            continue;
        }
        let initiating = kind(Direction::Initiating, set.procedure_code);
        assert!(!matches!(initiating, Kind::Other { .. }), "{}", set.name);
        assert_eq!(message.initiating(), Some(initiating), "{}", set.name);
        outcomes += 1;
    }
    // TS 38.413: 36 successful and 21 unsuccessful outcomes
    assert_eq!(outcomes, 57);
}

#[test]
fn the_outcomes_of_the_procedures_of_a_handover_and_of_a_ue_context() {
    for (outcome, initiating) in [
        (Kind::HandoverCommand, Kind::HandoverRequired),
        (Kind::HandoverPreparationFailure, Kind::HandoverRequired),
        (Kind::HandoverRequestAcknowledge, Kind::HandoverRequest),
        (Kind::HandoverFailure, Kind::HandoverRequest),
        (Kind::PathSwitchRequestAcknowledge, Kind::PathSwitchRequest),
        (Kind::PathSwitchRequestFailure, Kind::PathSwitchRequest),
        (
            Kind::InitialContextSetupResponse,
            Kind::InitialContextSetupRequest,
        ),
        (
            Kind::InitialContextSetupFailure,
            Kind::InitialContextSetupRequest,
        ),
        (
            Kind::UEContextReleaseComplete,
            Kind::UEContextReleaseCommand,
        ),
        (Kind::NGSetupResponse, Kind::NGSetupRequest),
        (Kind::NGSetupFailure, Kind::NGSetupRequest),
    ] {
        assert_eq!(outcome.initiating(), Some(initiating), "{outcome:?}");
    }
    // A class 2 procedure has no outcome: these answer none, nor does the outcome of a
    // procedure that the specification does not have.
    for message in [
        Kind::UEContextReleaseRequest,
        Kind::HandoverNotify,
        Kind::UplinkNASTransport,
        Kind::LocationReport,
        kind(Direction::Successful, 255),
    ] {
        assert_eq!(message.initiating(), None, "{message:?}");
    }
}
