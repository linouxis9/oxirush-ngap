// The blocks of the README that use other crates of the family, each as a
// function. `tests/readme.rs` writes this file and says how: do not edit it.
#![allow(unused)]

use std::cell::RefCell;

thread_local!(static PRINTED: RefCell<String> = const { RefCell::new(String::new()) });

// What a block prints is kept, to be compared with what the README shows.
macro_rules! println {
    () => {
        PRINTED.with(|printed| printed.borrow_mut().push('\n'))
    };
    // The line is made first, where the block is: its arguments may have a
    // `?` or an `await`.
    ($($argument:tt)*) => {{
        let line = format!($($argument)*);
        PRINTED.with(|printed| {
            let mut printed = printed.borrow_mut();
            printed.push_str(&line);
            printed.push('\n');
        })
    }};
}

/// What `block` prints.
pub fn printed(block: fn()) -> String {
    PRINTED.with(|printed| printed.borrow_mut().clear());
    block();
    PRINTED.with(|printed| printed.take())
}

pub const BLOCKS: &[fn()] = &[
    block_1,
];

/// The block 1 of the README that uses other crates.
fn block_1() {
    use oxirush_nas::nas_5gs::*;
    use oxirush_ngap::{build_ngap, extract_ngap_ies, helpers, ngap::*};
    use oxirush_security::nas_5gs::derive_kgnb;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // The UE: an initial registration, with a SUCI of the null scheme and
        // its security capability
        let suci = ImsiSuci::null(Plmn::new("208", "93")?, RoutingIndicator::default(), "0000000001")?;
        let kind = NasFGsRegistrationType::from_registration_type(RegistrationType::InitialRegistration);
        let no_key = NasKeySetIdentifier::from_key_set_identifier(KeySetIdentifier::NoKey)?;
        let identity = NasFGsMobileIdentity::from_suci(&Suci::Imsi(suci))?;
        let capability = NasUeSecurityCapability::from_capabilities(0xe0, 0xe0);
        let request = NasRegistrationRequest::new(kind, no_key, identity)
            .with_ue_security_capability(capability.clone());

        // The gNB: the INITIAL UE MESSAGE carries its octets
        let network = helpers::plmn("208", "93")?;
        let cell = helpers::nr_cgi(network, 0x123456, 24, 1)?;
        let location = UserLocationInformationNR::new(cell, helpers::tai(network, [0, 0, 1]), None, None);
        let initial = build_ngap!(InitiatingMessage, InitialUEMessage, InitialUEMessage,
            RAN_UE_NGAP_ID(7u32),
            NAS_PDU(Nas5gsMessage::new_5gmm(request).encode()?),
            UserLocationInformation(UserLocationInformation::userLocationInformationNR(location)),
            RRCEstablishmentCause(RRCEstablishmentCause::mo_Signalling),
        )?;

        // The AMF reads the NAS-PDU as a NAS message
        extract_ngap_ies!(initial, InitialUEMessage,
            req ran_id: u32 = RAN_UE_NGAP_ID(id),
            req nas: NASPDU = NAS_PDU(nas) => nas,
        );
        println!("RAN UE {ran_id}: {}", Nas5gsMessage::decode(&nas.0)?);

        // After 5G AKA it has KAMF. It protects the SECURITY MODE COMMAND with
        // the new context, and sends it in DOWNLINK NAS TRANSPORT
        let k_amf = "9d63b519775a92ca861ca6a50d848fa8ebf160ea7b73735a85b33737e73c55b4";
        let k_amf = KAmf::from_slice(&hex::decode(k_amf)?)?;
        let (nia, nea) = (IntegrityAlgorithm::NIA2, CipheringAlgorithm::NEA2);
        let mut context = NasSecurityContext::from_fresh_kamf(&k_amf, nia, nea)?;
        let ngksi = NasKeySetIdentifier::from_key_set_identifier(KeySetIdentifier::Native(0))?;
        let algorithms = NasSecurityAlgorithms::from_algorithms(nea, nia);
        let command = Nas5gsMessage::new_5gmm(NasSecurityModeCommand::new(algorithms, ngksi, capability));
        let header = Nas5gsSecurityHeaderType::IntegrityProtectedWithNewFiveGNasSecurityContext;
        let downlink = build_ngap!(InitiatingMessage, DownlinkNASTransport, DownlinkNASTransport,
            AMF_UE_NGAP_ID(1u64),
            RAN_UE_NGAP_ID(ran_id),
            NAS_PDU(context.protect(&command, header, Direction::Downlink)?),
        )?;
        println!("{}", hex::encode(downlink.encode()?));

        // The SECURITY MODE COMPLETE came with NAS COUNT 0: KgNB is derived from
        // it, for the INITIAL CONTEXT SETUP REQUEST
        let k_gnb = derive_kgnb(&k_amf, NasCount::ZERO);
        let setup = build_ngap!(InitiatingMessage, InitialContextSetup, InitialContextSetupRequest,
            AMF_UE_NGAP_ID(1u64),
            RAN_UE_NGAP_ID(ran_id),
            GUAMI(helpers::guami(network, 1, 1, 0)?),
            AllowedNSSAI(AllowedNSSAI(vec![AllowedNSSAIItem::new(helpers::s_nssai(1, None), None)])),
            UESecurityCapabilities(helpers::ue_security_capabilities(0xe0, 0xe0)),
            SecurityKey(*k_gnb.as_octets()),
        )?;
        println!("KgNB {}, {} deviations", hex::encode(k_gnb.as_octets()), setup.check()?.len());
        Ok(())
    }
    main().unwrap();
}
