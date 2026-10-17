//! What `check()` reports of a PDU against the object set of its message, and that it
//! refuses nothing: a message that deviates is built by the macros, written from a
//! tree, encoded and decoded as any other.
//!
//! TS 38.413 clause 9.4.1 has what a sender constructs: the IEs in the order of the
//! object set, a mandatory IE once, and an optional or a conditional IE at most once.
//! Clause 10.3 has what a receiver does with a message that is not so: with an IE that
//! it does not comprehend (10.3.4), with a missing one (10.3.5), and with IEs in a
//! wrong order or with too many occurrences (10.3.6). These tests are for a tool that
//! sends such a message to see what its peer does.

use oxirush_ngap::Octets;
use oxirush_ngap::ngap::*;
use oxirush_ngap::{Deviation, Error, ErrorKind, build_ngap};

/// What `check()` reports of `pdu`, which is the same for the PDU that its octets
/// decode to. With the `inspect` feature, `tree` is the same message written as a
/// tree: it has the same octets.
fn checked(pdu: &NGAP_PDU, tree: &str) -> Vec<Deviation> {
    let deviations = pdu.check().unwrap();
    let octets = pdu.encode().unwrap();
    let received = NGAP_PDU::decode(&octets).unwrap();
    assert_eq!(received, *pdu);
    assert_eq!(received.check().unwrap(), deviations);
    #[cfg(feature = "inspect")]
    {
        let tree = serde_json::from_str(tree).unwrap();
        let written = NGAP_PDU::from_tree(&tree).unwrap();
        assert_eq!(written.encode().unwrap(), octets, "{tree}");
        assert_eq!(written.check().unwrap(), deviations);
    }
    let _ = tree;
    deviations
}

fn cause() -> Cause {
    Cause::radioNetwork(CauseRadioNetwork::user_inactivity)
}

/// The messages of the fixtures: the name of each, and the octets of its PDU.
fn fixtures() -> impl Iterator<Item = (&'static str, Vec<u8>)> {
    vectors(include_str!("fixtures/messages.tsv"))
}

/// The vectors of a file of the fixtures: the name of each, and the octets of its PDU.
fn vectors(file: &'static str) -> impl Iterator<Item = (&'static str, Vec<u8>)> {
    let lines = file.lines();
    let lines = lines.filter(|line| !line.starts_with('#') && !line.is_empty());
    lines.map(|line| {
        let mut fields = line.split('\t');
        let name = fields.next().unwrap();
        (name, hex::decode(fields.next().unwrap()).unwrap())
    })
}

#[test]
fn a_message_as_its_object_set_has_it_deviates_in_nothing() -> Result<(), Error> {
    // UE CONTEXT RELEASE REQUEST: its three mandatory IEs, in the order of the set.
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(cause()),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}}
    ]}"#;
    assert_eq!(checked(&pdu, tree), []);
    Ok(())
}

#[test]
fn a_mandatory_ie_that_is_missing_is_reported() -> Result<(), Error> {
    // Without its Cause, whose presence is mandatory.
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7}
    ]}"#;
    let missing = Deviation::MissingIe {
        id: 15,
        name: "Cause",
        criticality: Criticality::ignore,
    };
    assert_eq!(checked(&pdu, tree), [missing]);
    Ok(())
}

#[test]
fn an_ie_that_is_there_twice_is_reported() -> Result<(), Error> {
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(cause()),
        REJECT AMF_UE_NGAP_ID(43u64),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}},
        {"AMF-UE-NGAP-ID": 43}
    ]}"#;
    let repeated = Deviation::RepeatedIe {
        id: 10,
        name: "AMF-UE-NGAP-ID",
        position: 3,
        first: 0,
    };
    assert_eq!(checked(&pdu, tree), [repeated]);
    Ok(())
}

#[test]
fn an_ie_of_another_message_is_reported() -> Result<(), Error> {
    // TimeToWait is an IE of the failures of the setup procedures.
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE TimeToWait(TimeToWait::v5s),
        IGNORE Cause(cause()),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"TimeToWait": "v5s", "criticality": "ignore"},
        {"Cause": {"radioNetwork": "user-inactivity"}}
    ]}"#;
    let unexpected = Deviation::UnexpectedIe {
        id: 107,
        name: Some("TimeToWait"),
        position: 2,
        criticality: Criticality::ignore,
    };
    assert_eq!(checked(&pdu, tree), [unexpected]);
    Ok(())
}

#[test]
fn ies_that_are_out_of_the_order_of_the_object_set_are_reported() -> Result<(), Error> {
    // The two identifiers in the other order.
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT RAN_UE_NGAP_ID(7u32),
        REJECT AMF_UE_NGAP_ID(42u64),
        IGNORE Cause(cause()),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "ies": [
        {"RAN-UE-NGAP-ID": 7},
        {"AMF-UE-NGAP-ID": 42},
        {"Cause": {"radioNetwork": "user-inactivity"}}
    ]}"#;
    let out_of_order = Deviation::OutOfOrder {
        id: 10,
        name: "AMF-UE-NGAP-ID",
        position: 1,
        after: 85,
        after_name: "RAN-UE-NGAP-ID",
    };
    assert_eq!(checked(&pdu, tree), [out_of_order]);
    Ok(())
}

#[test]
fn a_criticality_that_is_not_the_assigned_one_is_reported() -> Result<(), Error> {
    // The Cause with `reject`, where the object set of the message has `ignore`.
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        REJECT Cause(cause()),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}, "criticality": "reject"}
    ]}"#;
    let criticality = Deviation::IeCriticality {
        id: 15,
        name: "Cause",
        position: 2,
        criticality: Criticality::reject,
        assigned: Criticality::ignore,
    };
    assert_eq!(checked(&pdu, tree), [criticality]);

    // The PDU with `reject`, where the procedure has `ignore`.
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        REJECT, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(cause()),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "criticality": "reject", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}}
    ]}"#;
    let of_the_pdu = Deviation::PduCriticality {
        criticality: Criticality::reject,
        assigned: Criticality::ignore,
    };
    assert_eq!(checked(&pdu, tree), [of_the_pdu]);
    Ok(())
}

#[test]
fn every_deviation_of_a_message_is_reported_in_the_order_of_the_message() -> Result<(), Error> {
    // The message of the README: each sentence is what `Display` says.
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        REJECT, UEContextReleaseRequest,
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE AMF_UE_NGAP_ID(42u64),
        IGNORE TimeToWait(TimeToWait::v5s),
        REJECT RAN_UE_NGAP_ID(8u32),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "criticality": "reject", "ies": [
        {"RAN-UE-NGAP-ID": 7},
        {"AMF-UE-NGAP-ID": 42, "criticality": "ignore"},
        {"TimeToWait": "v5s", "criticality": "ignore"},
        {"RAN-UE-NGAP-ID": 8}
    ]}"#;
    let said: Vec<_> = checked(&pdu, tree)
        .iter()
        .map(|deviation| deviation.to_string())
        .collect();
    assert_eq!(
        said,
        [
            "the criticality of the PDU is reject, and that of its procedure is ignore",
            "AMF-UE-NGAP-ID (IE 10), at position 1, comes after RAN-UE-NGAP-ID (IE 85), which \
             the object set of the message has after it",
            "the criticality of AMF-UE-NGAP-ID (IE 10), at position 1, is ignore, and the \
             message assigns it reject",
            "TimeToWait (IE 107), at position 2, is not an IE of the message",
            "RAN-UE-NGAP-ID (IE 85), at position 3, is in the message already, at position 0",
            "Cause (IE 15) is mandatory, and the message does not have it",
        ]
    );

    // An IE that the protocol does not have is told by its identifier.
    let mut unknown = pdu.clone();
    let mut ies = unknown.protocol_ies()?.into_owned();
    ies.0.truncate(1);
    let octets = Octets::from(vec![0xc0, 0xff]);
    ies.0
        .push(ProtocolIEField::new(60000u16, Criticality::ignore, octets));
    unknown.set_protocol_ies(ies)?;
    let said = unknown.check()?[1].to_string();
    assert_eq!(
        said,
        "IE 60000, at position 1, is not an IE of the message, nor of the protocol"
    );
    Ok(())
}

#[test]
fn an_encoding_that_is_not_the_canonical_one_is_read_and_reported() -> Result<(), Error> {
    use oxirush_ngap::aper::{decode_strict, decode_with_faults};
    use oxirush_ngap::{Fault, Faults};
    // UE CONTEXT RELEASE REQUEST, as its sender is to write it.
    let canonical = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(cause()),
    )?;
    assert_eq!(canonical.check()?, []);
    let octets = canonical.encode()?;
    let whole = decode_with_faults::<NGAPPDU>(&octets)?;
    assert_eq!(whole, (canonical.clone(), Faults::default()));

    // The same message from an encoder that is not canonical. The value of its first
    // IE, a number of 0..1099511627775, has the number of its octets in three bits,
    // padding, then its octets: here two octets of which the first is zero (X.691
    // §11.3.6), after padding of which a bit is one (§11.1.4).
    let mut ies = canonical.protocol_ies()?.into_owned();
    ies.0[0].value = Octets::owned(vec![0x21, 0x00, 0x2a]);
    let mut other = canonical.clone();
    other.set_protocol_ies(ies)?;
    // And the length of that value in two octets (§11.9.3.6): the message starts after
    // the four octets of the PDU, and the length is its seventh octet.
    let mut octets = other.encode()?;
    octets.insert(10, 0x80);
    octets[3] += 1;
    let other = NGAP_PDU::decode(&octets)?;

    // It is read as the same message.
    let ies = other.protocol_ies()?;
    let read: AMFUENGAPID = decode_open_type(&ies.0[0].value)?;
    assert_eq!(read.0, 42);
    assert_eq!(ies.0.len(), 3);
    // And each of the three is reported, with its clause and the bit at which it is:
    // that of the message first, then those of the value.
    let ie = Some((10, "AMF-UE-NGAP-ID", 0));
    let deviations = other.check()?;
    let not_canonical = |ie, fault, bit| Deviation::NotCanonical { ie, fault, bit };
    assert_eq!(
        deviations,
        [
            not_canonical(None, Fault::LengthInTwoOctets, 48),
            not_canonical(ie, Fault::Padding, 3),
            not_canonical(ie, Fault::LongNumber, 8),
        ]
    );
    assert_eq!(
        deviations[0].to_string(),
        "the message is not written as its sender is to write it: at bit 48, a length of 127 \
         or less is in two octets (X.691 §11.9.3.6)"
    );
    assert_eq!(
        deviations[2].to_string(),
        "the value of AMF-UE-NGAP-ID (IE 10), at position 0, is not written as its sender is to \
         write it: at bit 8, a whole number is in more octets than it needs (X.691 §11.3.6, \
         11.4.6)"
    );
    // A receiver answers nothing for them.
    for deviation in &deviations {
        assert_eq!((deviation.ie(), deviation.cause()), (None, None));
        assert!(deviation.diagnostics_item().is_none() && deviation.type_of_error().is_none());
    }

    // The octets before the message are the PDU, which was read when it was decoded: a
    // decoding of the PDU has their faults, and a strict one refuses them. Here the
    // length of the message is in two octets.
    let mut octets = canonical.encode()?;
    octets.insert(3, 0x80);
    assert_eq!(NGAP_PDU::decode(&octets)?, canonical);
    let (pdu, faults) = decode_with_faults::<NGAPPDU>(&octets)?;
    assert_eq!(pdu, canonical);
    assert_eq!(
        (faults.len(), faults[0].fault, faults[0].bit),
        (1, Fault::LengthInTwoOctets, 24)
    );
    let error = decode_strict::<NGAPPDU>(&octets).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Decode);
    assert!(
        error
            .to_string()
            .contains("at bit 24, a length of 127 or less is in two octets")
    );
    assert_eq!(decode_strict::<NGAPPDU>(&canonical.encode()?)?, canonical);
    Ok(())
}

#[test]
fn a_conditional_ie_is_not_reported_missing() -> Result<(), Error> {
    // HANDOVER COMMAND, with its four mandatory IEs. Its NASSecurityParametersFromNGRAN
    // is conditional: TS 38.413 clause 9.2.3.2 has it present when the Handover Type is
    // 5GStoEPS or 5GtoUTRA, which is prose that the object set does not have.
    let pdu = build_ngap!(SuccessfulOutcome, HandoverPreparation,
        REJECT, HandoverCommand,
        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        REJECT HandoverType(HandoverType::intra5gs),
        REJECT TargetToSource_TransparentContainer(vec![0x00]),
    )?;
    assert_eq!(pdu.check()?, []);
    Ok(())
}

#[test]
fn what_has_no_object_set_is_not_checked_against_one() -> Result<(), Error> {
    // The private message has private IEs: the criticality of its PDU is all that the
    // specification assigns it.
    let (_, wire) = fixtures()
        .find(|(name, _)| *name == "PrivateMessage")
        .expect("the private message");
    let private = NGAP_PDU::decode(&wire)?;
    assert_eq!(private.check()?, []);
    let NGAP_PDU::initiatingMessage(mut held) = private else {
        panic!("an initiating message");
    };
    held.criticality = Criticality::reject;
    let of_the_pdu = Deviation::PduCriticality {
        criticality: Criticality::reject,
        assigned: Criticality::ignore,
    };
    let private = NGAP_PDU::initiatingMessage(held.clone());
    assert_eq!(private.check()?, [of_the_pdu]);

    // The eighth finding of the review of the move of the paths to oxirush-utils: a
    // procedure that the specification does not have in that direction has no object set,
    // and is the one deviation, which a receiver answers by the criticality of the PDU (TS
    // 38.413 clause 10.3.4.1): it rejects the procedure, or ignores it and notifies the
    // sender, with an Error Indication that has the procedure code, the triggering message
    // and the procedure criticality, or ignores it.
    held.procedure_code = ProcedureCode(200);
    for (criticality, cause) in [
        (
            Criticality::reject,
            Some(CauseProtocol::abstract_syntax_error_reject),
        ),
        (
            Criticality::notify,
            Some(CauseProtocol::abstract_syntax_error_ignore_and_notify),
        ),
        (Criticality::ignore, None),
    ] {
        held.criticality = criticality;
        let pdu = NGAP_PDU::initiatingMessage(held.clone());
        let deviations = pdu.check()?;
        let unknown = Deviation::UnknownProcedure {
            procedure_code: 200,
            direction: "InitiatingMessage",
            criticality,
        };
        assert_eq!(deviations, [unknown]);
        assert_eq!(deviations[0].cause(), cause);
        assert_eq!(deviations[0].ie(), None);
        assert_eq!(deviations[0].diagnostics_item(), None);
        assert_eq!(
            pdu.criticality_diagnostics(&deviations),
            CriticalityDiagnostics::new(
                Some(ProcedureCode(200)),
                Some(TriggeringMessage::initiating_message),
                Some(criticality),
                None,
                None,
            )
        );
    }
    assert_eq!(
        NGAP_PDU::initiatingMessage(held).check()?[0].to_string(),
        "the InitiatingMessage of procedure 200 is no message of the specification, and \
         the criticality of the PDU is ignore"
    );

    // A message that does not decode is an error, and no deviation.
    let request = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
    )?;
    let NGAP_PDU::initiatingMessage(mut held) = request else {
        panic!("an initiating message");
    };
    held.value = Octets::from(vec![0x00, 0x00, 0x03, 0x00]);
    let error = NGAP_PDU::initiatingMessage(held).check().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Decode);
    Ok(())
}

#[test]
fn the_messages_of_the_fixtures_are_checked_against_their_object_sets() {
    let mut deviating = Vec::new();
    let mut checked = 0;
    for (name, wire) in fixtures() {
        let pdu = NGAP_PDU::decode(&wire).unwrap();
        let found = pdu
            .check()
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        checked += 1;
        for deviation in found {
            deviating.push(format!("{name}: {deviation}"));
        }
    }
    assert_eq!(checked, 144);
    assert_eq!(deviating, Vec::<String>::new(), "{deviating:#?}");
}

/// The vectors with content have the IEs of the object sets as pycrate has them from
/// the ASN.1: each IE that a set has, in the order of the set and with its criticality.
/// The vectors that cover the extension of a constraint hold a value outside its root,
/// which is their one deviation.
#[test]
fn the_vectors_with_content_are_checked_against_their_object_sets() {
    let (mut checked, mut outside) = (0, 0);
    for (name, wire) in vectors(include_str!("fixtures/vectors.tsv")) {
        let pdu = NGAP_PDU::decode(&wire).unwrap();
        let found = pdu
            .check()
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        for deviation in &found {
            assert!(
                matches!(deviation, Deviation::OutsideRoot { .. }),
                "{name}: {deviation}"
            );
            // An encoding that is strict refuses what the vector holds.
            assert!(
                deviation
                    .to_string()
                    .contains("outside the root of its constraint")
            );
        }
        outside += found.len();
        checked += 1;
    }
    assert!(checked > 144, "{checked} vectors");
    assert_eq!(
        outside, 22,
        "the IEs of the vectors that are outside a root"
    );
}
