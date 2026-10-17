//! The readers of the IEs of a PDU and of a message: `ies::<T>()` and `ie::<T>()`, the
//! macro that extracts IEs from a PDU without `decode_value`, and `check_structure()`. On
//! the PDUs of the fixtures, they read what the codec of the message reads.

use oxirush_ngap::ngap::*;
use oxirush_ngap::{Deviation, Error, ErrorKind, Octets, build_ngap, extract_ngap_ies, ie};

/// The PDUs of the fixtures and of the captures: the name of each, and its octets.
fn fixtures() -> impl Iterator<Item = (&'static str, Vec<u8>)> {
    let files = [
        include_str!("fixtures/messages.tsv"),
        include_str!("fixtures/captured.tsv"),
    ];
    let lines = files.into_iter().flat_map(str::lines);
    let lines = lines.filter(|line| !line.starts_with('#') && !line.is_empty());
    lines.map(|line| {
        let mut fields = line.split('\t');
        let name = fields.next().unwrap();
        (name, hex::decode(fields.next().unwrap()).unwrap())
    })
}

/// A UE CONTEXT RELEASE REQUEST of these IEs.
fn release_request() -> NGAP_PDU {
    build_ngap!(
        InitiatingMessage,
        UEContextReleaseRequest,
        UEContextReleaseRequest,
        AMF_UE_NGAP_ID(42u64),
        RAN_UE_NGAP_ID(7u32),
        Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )
    .unwrap()
}

#[test]
fn an_ie_is_read_from_the_pdu_as_from_its_message() -> Result<(), Error> {
    let octets = release_request().encode()?;
    let pdu = NGAP_PDU::decode_borrowed(&octets)?;
    assert_eq!(pdu.ie::<ie::RAN_UE_NGAP_ID>().unwrap()?, RANUENGAPID(7));
    assert_eq!(pdu.ie::<ie::AMF_UE_NGAP_ID>().unwrap()?, AMFUENGAPID(42));
    assert!(pdu.ie::<ie::NAS_PDU>().is_none());
    let message: UEContextReleaseRequest = pdu.decode_value()?;
    assert_eq!(message.ie::<ie::RAN_UE_NGAP_ID>().unwrap()?, RANUENGAPID(7));
    // The name of the type is a name of the IE of that type.
    assert_eq!(message.ie::<ie::RANUENGAPID>().unwrap()?, RANUENGAPID(7));
    Ok(())
}

#[test]
fn the_macro_reads_a_pdu_without_decode_value() -> Result<(), Error> {
    fn read(pdu: &NGAP_PDU) -> Result<(u64, u32), Error> {
        extract_ngap_ies!(pdu, UEContextReleaseRequest,
            req amf: u64 = AMF_UE_NGAP_ID(id),
            req ran: u32 = RAN_UE_NGAP_ID(id),
        );
        Ok((amf, ran))
    }
    assert_eq!(read(&release_request())?, (42, 7));
    // A PDU of another message is an error of the kind `WrongMessage`.
    let setup = build_ngap!(
        InitiatingMessage,
        NGSetup,
        NGSetupRequest,
        DefaultPagingDRX(PagingDRX::v128),
    )?;
    assert_eq!(read(&setup).unwrap_err().kind(), ErrorKind::WrongMessage);
    Ok(())
}

#[test]
fn a_nas_pdu_in_fragments_is_read_and_kept() -> Result<(), Error> {
    // 20,000 octets come in two fragments in the open type of the IE, and in two more in
    // the OCTET STRING of the NAS-PDU.
    let nas: Vec<u8> = (0..20_000).map(|at| (at % 253) as u8).collect();
    let pdu = build_ngap!(
        InitiatingMessage,
        DownlinkNASTransport,
        DownlinkNASTransport,
        AMF_UE_NGAP_ID(42u64),
        RAN_UE_NGAP_ID(7u32),
        NAS_PDU(nas.clone()),
    )?;
    let octets = pdu.encode()?;
    let pdu = NGAP_PDU::decode_borrowed(&octets)?;
    assert_eq!(pdu.ie::<ie::NAS_PDU>().unwrap()?.0[..], nas[..]);
    fn read(pdu: &NGAPPDU<'_>) -> Result<Vec<u8>, Error> {
        extract_ngap_ies!(pdu, DownlinkNASTransport,
            req nas: Vec<u8> = NAS_PDU(nas) => nas.0.to_vec(),
        );
        Ok(nas)
    }
    assert_eq!(read(&pdu)?, nas);
    assert!(pdu.check()?.is_empty());
    Ok(())
}

#[test]
fn every_ie_of_the_fixtures_is_read_as_the_codec_of_its_message_reads_it() -> Result<(), Error> {
    let mut read = 0;
    for (name, octets) in fixtures() {
        let pdu = NGAP_PDU::decode(&octets)?;
        let Ok(ies) = pdu.protocol_ies() else {
            continue;
        };
        // The NAS-PDUs and the causes, as each reader gives them: a value that does not
        // decode is an error of the kind `UndecodableIe` of the reader.
        let of = |id: u16| ies.0.iter().filter(move |ie| ie.id.0 == id);
        let nas: Vec<Result<NASPDU, ErrorKind>> = of(38)
            .map(|ie| decode_open_type(&ie.value).map_err(|_| ErrorKind::UndecodableIe))
            .collect();
        let read_nas = pdu
            .ies::<ie::NAS_PDU>()
            .map(|nas| nas.map_err(|error| error.kind()));
        assert_eq!(read_nas.collect::<Vec<_>>(), nas, "{name}");
        let causes: Vec<Result<Cause, ErrorKind>> = of(15)
            .map(|ie| decode_open_type(&ie.value).map_err(|_| ErrorKind::UndecodableIe))
            .collect();
        let read_causes = pdu
            .ies::<ie::Cause>()
            .map(|cause| cause.map_err(|error| error.kind()));
        assert_eq!(read_causes.collect::<Vec<_>>(), causes, "{name}");
        read += 1;
    }
    assert!(read > 200, "{read}");
    Ok(())
}

#[test]
fn check_structure_reports_what_check_reports_of_the_container() -> Result<(), Error> {
    let of_values = |deviation: &Deviation| {
        matches!(
            deviation,
            Deviation::UndecodableIe { .. }
                | Deviation::OutsideRoot { .. }
                | Deviation::NotCanonical { .. }
        )
    };
    let mut compared = 0;
    for (name, octets) in fixtures() {
        let pdu = NGAP_PDU::decode(&octets)?;
        let (Ok(all), Ok(structure)) = (pdu.check(), pdu.check_structure()) else {
            assert_eq!(
                pdu.check().is_err(),
                pdu.check_structure().is_err(),
                "{name}"
            );
            continue;
        };
        let of_container: Vec<_> = all
            .into_iter()
            .filter(|deviation| !of_values(deviation))
            .collect();
        assert_eq!(structure, of_container, "{name}");
        compared += 1;
    }
    assert!(compared > 200, "{compared}");
    Ok(())
}

#[test]
fn a_repeated_ie_is_there_twice_for_ies_and_once_for_the_macro() -> Result<(), Error> {
    let pdu = build_ngap!(
        InitiatingMessage,
        UEContextReleaseRequest,
        UEContextReleaseRequest,
        AMF_UE_NGAP_ID(42u64),
        RAN_UE_NGAP_ID(7u32),
        RAN_UE_NGAP_ID(8u32),
        Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )?;
    let every: Vec<_> = pdu.ies::<ie::RAN_UE_NGAP_ID>().collect::<Result<_, _>>()?;
    assert_eq!(every, [RANUENGAPID(7), RANUENGAPID(8)]);
    extract_ngap_ies!(pdu, UEContextReleaseRequest, lax ran: u32 = RAN_UE_NGAP_ID(id));
    assert_eq!(ran, Some(7));
    assert!(matches!(
        pdu.check_structure()?[..],
        [Deviation::RepeatedIe {
            id: 85,
            position: 2,
            first: 1,
            ..
        }]
    ));
    Ok(())
}

#[test]
fn octets_of_the_message_that_end_early_are_an_error_of_every_reader() -> Result<(), Error> {
    let pdu = release_request();
    let NGAPPDU::initiatingMessage(mut message) = pdu else {
        unreachable!("a request")
    };
    // The message without its last octet, which its Cause takes.
    let mut octets = message.value.to_vec();
    octets.pop();
    message.value = Octets::from(octets);
    let cut = NGAPPDU::initiatingMessage(message);
    assert_eq!(cut.check_structure().unwrap_err().kind(), ErrorKind::Decode);
    assert_eq!(cut.check().unwrap_err().kind(), ErrorKind::Decode);
    let ids: Vec<_> = cut.ies::<ie::RAN_UE_NGAP_ID>().collect();
    assert_eq!(ids[0], Ok(RANUENGAPID(7)));
    assert_eq!(ids[1].as_ref().unwrap_err().kind(), ErrorKind::Decode);
    fn read(pdu: &NGAP_PDU) -> Result<u32, Error> {
        extract_ngap_ies!(pdu, UEContextReleaseRequest, req ran: u32 = RAN_UE_NGAP_ID(id));
        Ok(ran)
    }
    assert_eq!(read(&cut).unwrap_err().kind(), ErrorKind::Decode);
    Ok(())
}
