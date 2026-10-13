//! The protocol IEs of a PDU, whatever its message: read, and put back.

use oxirush_ngap::Octets;
use oxirush_ngap::ngap::*;
use oxirush_ngap::{Error, ErrorKind, build_ngap, build_ngap_ie};

/// The messages of the fixtures, then the vectors with content: the name of each, and
/// the octets of its PDU.
fn fixtures() -> impl Iterator<Item = (&'static str, Vec<u8>)> {
    let messages = include_str!("fixtures/messages.tsv").lines();
    let lines = messages.chain(include_str!("fixtures/vectors.tsv").lines());
    let lines = lines.filter(|line| !line.starts_with('#') && !line.is_empty());
    lines.map(|line| {
        let mut fields = line.split('\t');
        let name = fields.next().unwrap();
        (name, hex::decode(fields.next().unwrap()).unwrap())
    })
}

/// A UE CONTEXT RELEASE REQUEST with its three IEs, the message of the README.
const RELEASE_REQUEST: &str = "002a4015000003000a0002002a005500020007000f40020500";

#[test]
fn the_ies_of_every_message_are_read_and_put_back_as_the_same_octets() {
    let mut read = 0;
    for (name, wire) in fixtures() {
        let mut pdu = NGAP_PDU::decode(&wire).unwrap();
        // The private message has private IEs: see the test of the errors.
        if name.split('/').next() == Some("PrivateMessage") {
            continue;
        }
        // The IEs borrow from the PDU: they are owned before the PDU is changed.
        let ies = pdu.protocol_ies();
        let ies = ies.unwrap_or_else(|error| panic!("{name}: {error}"));
        let ies = ies.into_owned();
        pdu.set_protocol_ies(ies).unwrap();
        assert_eq!(pdu.encode().unwrap(), wire, "{name}");
        read += 1;
    }
    // Each message of the fixtures but the private one, and each vector.
    assert!(read > 143, "{read} messages");
}

#[test]
fn a_message_that_the_macros_built_has_the_ies_that_it_was_built_with() -> Result<(), Error> {
    let cause = || Cause::radioNetwork(CauseRadioNetwork::user_inactivity);
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(cause()),
    )?;
    let built = vec![
        build_ngap_ie!(UEContextReleaseRequest, REJECT AMF_UE_NGAP_ID(42u64))?,
        build_ngap_ie!(UEContextReleaseRequest, REJECT RAN_UE_NGAP_ID(7u32))?,
        build_ngap_ie!(UEContextReleaseRequest, IGNORE Cause(cause()))?,
    ];
    assert_eq!(pdu.protocol_ies()?.0, built);

    // They are those of the typed message, and of the octets of the README.
    let request: UEContextReleaseRequest = pdu.decode_value()?;
    assert_eq!(pdu.protocol_ies()?, request.protocol_ies);
    let received = NGAP_PDU::decode(&hex::decode(RELEASE_REQUEST).unwrap())?;
    assert_eq!(received.protocol_ies()?.0, built);
    Ok(())
}

#[test]
fn an_unknown_ie_and_a_repeated_one_are_kept_with_their_criticality() -> Result<(), Error> {
    // An IE of the message, an IE that the protocol does not have, the first again with
    // another value and another criticality, and an IE of another message.
    let unknown = ProtocolIEField::new(
        60000u16,
        Criticality::notify,
        Octets::from(vec![0xc0, 0xff]),
    );
    let mut again = build_ngap_ie!(UEContextReleaseRequest, REJECT AMF_UE_NGAP_ID(2u64))?;
    again.criticality = Criticality::ignore;
    let ies = vec![
        build_ngap_ie!(UEContextReleaseRequest, REJECT AMF_UE_NGAP_ID(1u64))?,
        unknown,
        again,
        build_ngap_ie!(NGSetupFailure, NOTIFY TimeToWait(TimeToWait::v5s))?,
    ];

    // Put into a message that had three other IEs: the PDU is the same but for them.
    let mut pdu = NGAP_PDU::decode(&hex::decode(RELEASE_REQUEST).unwrap())?;
    pdu.set_protocol_ies(ProtocolIEContainer(ies.clone()))?;
    assert_eq!(pdu.kind(), NgapPduKind::Initiating_UEContextReleaseRequest);
    let NGAP_PDU::initiatingMessage(held) = &pdu else {
        panic!("{pdu}");
    };
    assert_eq!(held.criticality, Criticality::ignore);

    // They are read back as they were written, from the PDU and from its octets.
    assert_eq!(pdu.protocol_ies()?.0, ies);
    let received = NGAP_PDU::decode(&pdu.encode()?)?;
    assert_eq!(received, pdu);
    let read = received.protocol_ies()?.0;
    assert_eq!(read, ies);
    let told: Vec<_> = read.iter().map(|ie| (ie.id.0, ie.criticality)).collect();
    assert_eq!(
        told,
        [
            (10, Criticality::reject),
            (60000, Criticality::notify),
            (10, Criticality::ignore),
            (107, Criticality::notify),
        ]
    );
    Ok(())
}

#[test]
fn the_message_of_an_unknown_procedure_is_read_as_a_container_of_protocol_ies() -> Result<(), Error>
{
    // The IEs of the release request, under a procedure code that TS 38.413 does not have.
    let known = NGAP_PDU::decode(&hex::decode(RELEASE_REQUEST).unwrap())?;
    let NGAP_PDU::initiatingMessage(mut held) = known.clone() else {
        panic!("{known}");
    };
    held.procedure_code = ProcedureCode(200);
    let mut pdu = NGAP_PDU::initiatingMessage(held);
    assert!(matches!(pdu.kind(), NgapPduKind::Other { .. }));
    let mut ies = pdu.protocol_ies()?.into_owned();
    assert_eq!(ies, known.protocol_ies()?);
    ies.0.pop();
    pdu.set_protocol_ies(ies)?;
    assert_eq!(pdu.protocol_ies()?.0.len(), 2);
    assert_eq!(pdu.procedure_code(), 200);
    Ok(())
}

#[test]
fn a_message_without_protocol_ies_and_octets_that_are_none_are_errors() -> Result<(), Error> {
    // The private message has private IEs, and no container of protocol IEs.
    let (_, wire) = fixtures()
        .find(|(name, _)| *name == "PrivateMessage")
        .expect("the private message");
    let mut private = NGAP_PDU::decode(&wire)?;
    let before = private.clone();
    let read = private.protocol_ies().unwrap_err();
    assert_eq!(read.kind(), ErrorKind::WrongMessage);
    assert_eq!(
        read.to_string(),
        "the message of this PDU is PrivateMessage, which has no container of protocol IEs"
    );
    let written = private.set_protocol_ies(ProtocolIEContainer(vec![]));
    assert_eq!(written, Err(read));
    assert_eq!(private, before);

    // Octets that are no message: the message is named.
    let request = NGAP_PDU::decode(&hex::decode(RELEASE_REQUEST).unwrap())?;
    let NGAP_PDU::initiatingMessage(mut held) = request.clone() else {
        panic!("{request}");
    };
    held.value = Octets::from(vec![0x00, 0x00, 0x03, 0x00]);
    let short = NGAP_PDU::initiatingMessage(held.clone());
    let error = short.protocol_ies().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Decode);
    assert_eq!(
        error.to_string(),
        "the UEContextReleaseRequest of this PDU does not decode: the octets end before the \
         value does, in its field `protocol_ies.id`"
    );
    // The same octets under a procedure that the specification does not have.
    held.procedure_code = ProcedureCode(200);
    let error = NGAP_PDU::initiatingMessage(held)
        .protocol_ies()
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Decode);
    assert_eq!(
        error.to_string(),
        "the message of this PDU does not decode as a container of protocol IEs: the octets \
         end before the value does, in its field `protocol_ies.id`"
    );

    // More IEs than a container has: the PDU is as it was.
    let ie = build_ngap_ie!(UEContextReleaseRequest, REJECT RAN_UE_NGAP_ID(7u32))?;
    let mut pdu = request.clone();
    let error = pdu
        .set_protocol_ies(ProtocolIEContainer(vec![ie; 65536]))
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Encode);
    assert!(
        error
            .to_string()
            .starts_with("these 65536 protocol IEs do not encode as a message: "),
        "{error}"
    );
    assert_eq!(pdu, request);
    Ok(())
}

/// A field of the macro that extracts is the value of the IE when it has the type of
/// the IE, and the one value that the type is a name for when it has the type of that
/// value: an AMF-UE-NGAP-ID is a name for a number, and a Cause is a name for nothing.
#[test]
fn a_field_takes_the_value_of_an_ie_as_its_type_or_as_the_value_that_it_names() -> Result<(), Error>
{
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )?;
    let request: UEContextReleaseRequest = pdu.decode_value()?;
    oxirush_ngap::extract_ngap_ies!(request, UEContextReleaseRequest,
        req cause: Cause = Cause(cause),
        req id: AMFUENGAPID = AMF_UE_NGAP_ID(id),
        req number: u64 = AMF_UE_NGAP_ID(id),
        opt ran: RANUENGAPID = RAN_UE_NGAP_ID(id),
        lax ran_number: u32 = RAN_UE_NGAP_ID(id),
        opt twice: u64 = AMF_UE_NGAP_ID(id) => id.0 * 2,
    );
    assert_eq!(
        cause,
        Cause::radioNetwork(CauseRadioNetwork::user_inactivity)
    );
    assert_eq!((id, number), (AMFUENGAPID(42), 42));
    assert_eq!((ran, ran_number), (Some(RANUENGAPID(7)), Some(7)));
    assert_eq!(twice, Some(84));
    Ok(())
}

/// An IE that is in no message is built without the name of one: the name of a message
/// before an IE says where the IE goes, and gives the same IE.
#[test]
fn an_ie_is_built_with_the_name_of_its_message_or_without_one() -> Result<(), Error> {
    let named = build_ngap_ie!(UEContextReleaseRequest, REJECT AMF_UE_NGAP_ID(42u64))?;
    let alone = build_ngap_ie!(REJECT AMF_UE_NGAP_ID(42u64))?;
    assert_eq!(named, alone);
    assert_eq!((alone.id.0, alone.criticality), (10, Criticality::reject));
    assert_eq!(alone.value, [0x00, 0x2a]);
    Ok(())
}
