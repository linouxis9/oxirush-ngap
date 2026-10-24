//! `Display` of a PDU: the name of its message, its procedure code and its direction,
//! then the identifiers of its UE, whether the message has them as two IEs or as one,
//! UE-NGAP-IDs, which holds the pair or the AMF UE NGAP ID alone.

use oxirush_ngap::ngap::*;
use oxirush_ngap::{Error, build_ngap};

/// The PDU that the octets of `pdu` decode to: what a peer receives.
fn received(pdu: &NGAP_PDU) -> Result<NGAP_PDU, Error> {
    NGAP_PDU::decode(&pdu.encode()?)
}

#[test]
fn a_pdu_shows_the_identifiers_of_its_ue_however_its_message_carries_them() -> Result<(), Error> {
    let cause = || Cause::nas(CauseNas::normal_release);

    // UE CONTEXT RELEASE COMMAND, with the pair
    let pair = UENGAPIDPair::new(AMFUENGAPID(42), RANUENGAPID(7), None);
    let command = build_ngap!(
        InitiatingMessage,
        UEContextRelease,
        UEContextReleaseCommand,
        UE_NGAP_IDs(UENGAPIDs::uE_NGAP_ID_pair(pair)),
        Cause(cause()),
    )?;
    assert_eq!(
        received(&command)?.to_string(),
        "UEContextReleaseCommand (procedure 41, initiating message), AMF-UE-NGAP-ID 42, RAN-UE-NGAP-ID 7"
    );

    // and with the AMF UE NGAP ID alone
    let command = build_ngap!(
        InitiatingMessage,
        UEContextRelease,
        UEContextReleaseCommand,
        UE_NGAP_IDs(UENGAPIDs::aMF_UE_NGAP_ID(AMFUENGAPID(42))),
        Cause(cause()),
    )?;
    assert_eq!(
        received(&command)?.to_string(),
        "UEContextReleaseCommand (procedure 41, initiating message), AMF-UE-NGAP-ID 42"
    );

    // UE CONTEXT RELEASE COMPLETE has them as two IEs, and NG SETUP REQUEST none
    let complete = build_ngap!(
        SuccessfulOutcome,
        UEContextRelease,
        UEContextReleaseComplete,
        AMF_UE_NGAP_ID(42u64),
        RAN_UE_NGAP_ID(7u32),
    )?;
    assert_eq!(
        received(&complete)?.to_string(),
        "UEContextReleaseComplete (procedure 41, successful outcome), AMF-UE-NGAP-ID 42, RAN-UE-NGAP-ID 7"
    );
    let setup = build_ngap!(
        InitiatingMessage,
        NGSetup,
        NGSetupRequest,
        DefaultPagingDRX(PagingDRX::v128),
    )?;
    assert_eq!(
        received(&setup)?.to_string(),
        "NGSetupRequest (procedure 21, initiating message)"
    );
    Ok(())
}
