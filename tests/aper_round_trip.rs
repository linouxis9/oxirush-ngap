use oxirush_ngap::ngap::*;
use oxirush_ngap::{build_ngap, extract_ngap_ies, macros::MissingIeError};

#[test]
fn public_macros_round_trip_typed_open_message() -> Result<(), MissingIeError> {
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    );

    let NGAP_PDU::initiatingMessage(message) = &pdu else {
        panic!("expected initiating message");
    };
    // Extension bit, octet padding, then the aligned 16-bit ProtocolIE count.
    assert_eq!(&message.value.as_bytes()[..3], &[0x00, 0x00, 0x03]);

    let wire = pdu.encode().expect("encode PDU");
    let decoded = NGAP_PDU::decode(&wire).expect("decode PDU");
    assert_eq!(decoded, pdu);

    let request: UEContextReleaseRequest = decoded.decode_value().expect("decode message");
    extract_ngap_ies!(&request, UEContextReleaseRequest,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
        req cause: Cause = Cause(value) => value,
    );
    assert_eq!(amf_id, 42);
    assert_eq!(ran_id, 7);
    assert_eq!(
        cause,
        Cause::radioNetwork(CauseRadioNetwork::user_inactivity)
    );
    Ok(())
}

#[test]
fn snssai_round_trips_with_optional_sd() {
    use rasn::types::FixedOctetString;

    let value = SNSSAI::new(
        SST(FixedOctetString::new([1])),
        Some(SD(FixedOctetString::new([0, 0, 1]))),
        None,
    );
    let wire = rasn::aper::encode(&value).expect("encode S-NSSAI");
    let decoded: SNSSAI = rasn::aper::decode(&wire).expect("decode S-NSSAI");
    assert_eq!(decoded, value);
}

#[test]
fn plmn_support_list_round_trips_nested_slice() {
    use rasn::types::FixedOctetString;

    let slice = SNSSAI::new(
        SST(FixedOctetString::new([1])),
        Some(SD(FixedOctetString::new([0, 0, 1]))),
        None,
    );
    let value = PLMNSupportList(vec![PLMNSupportItem::new(
        PLMNIdentity(FixedOctetString::new([0x02, 0xf8, 0x39])),
        SliceSupportList(vec![SliceSupportItem::new(slice, None)]),
        None,
    )]);
    let wire = rasn::aper::encode(&value).expect("encode PLMN support");
    let decoded: PLMNSupportList = rasn::aper::decode(&wire).expect("decode PLMN support");
    assert_eq!(decoded, value);
}

#[test]
fn slice_support_list_round_trips() {
    use rasn::types::FixedOctetString;

    let value = SliceSupportList(vec![SliceSupportItem::new(
        SNSSAI::new(
            SST(FixedOctetString::new([1])),
            Some(SD(FixedOctetString::new([0, 0, 1]))),
            None,
        ),
        None,
    )]);
    let wire = rasn::aper::encode(&value).expect("encode slice support");
    let decoded: SliceSupportList = rasn::aper::decode(&wire).expect("decode slice support");
    assert_eq!(decoded, value);
}

#[test]
fn plmn_support_item_round_trips() {
    use rasn::types::FixedOctetString;

    let value = PLMNSupportItem::new(
        PLMNIdentity(FixedOctetString::new([0x02, 0xf8, 0x39])),
        SliceSupportList(vec![SliceSupportItem::new(
            SNSSAI::new(
                SST(FixedOctetString::new([1])),
                Some(SD(FixedOctetString::new([0, 0, 1]))),
                None,
            ),
            None,
        )]),
        None,
    );
    let wire = rasn::aper::encode(&value).expect("encode PLMN support item");
    let decoded: PLMNSupportItem = rasn::aper::decode(&wire).expect("decode PLMN support item");
    assert_eq!(decoded, value);
}

/// An IE name addresses that IE even when it is also the type of another IE:
/// id-OldAMF is an AMFName and id-RRC-Resume-Cause an RRCEstablishmentCause.
#[test]
fn ie_names_address_their_own_ie_before_a_same_named_type() -> Result<(), MissingIeError> {
    let name = AMFName(rasn::types::PrintableString::try_from("amf").expect("printable"));
    let ie = oxirush_ngap::build_ngap_ie!(NGSetupResponse, REJECT AMFName(name.clone()));
    assert_eq!(ie.id, ID_AMFNAME);
    let ie = oxirush_ngap::build_ngap_ie!(DownlinkNASTransport, REJECT OldAMF(name.clone()));
    assert_eq!(ie.id, ID_OLD_AMF);
    let ie = oxirush_ngap::build_ngap_ie!(InitialUEMessage,
        IGNORE RRCEstablishmentCause(RRCEstablishmentCause::mo_Signalling)
    );
    assert_eq!(ie.id, ID_RRCESTABLISHMENT_CAUSE);

    let pdu = build_ngap!(SuccessfulOutcome, NGSetup,
        REJECT, NGSetupResponse,
        REJECT AMFName(name.clone()),
    );
    let response: NGSetupResponse = pdu.decode_value().expect("decode message");
    extract_ngap_ies!(&response, NGSetupResponse,
        req amf_name: AMFName = AMFName(value) => value,
    );
    assert_eq!(amf_name, name);
    Ok(())
}

/// Constrained lists whose `rasn` attribute rustfmt splits over several
/// lines use the corrected APER codec too. The expected octets are the ones
/// Wireshark decodes: an octet-aligned length for SIZE(1..256), and an
/// unconstrained length for SIZE(1..65536) (X.691 §11.9.3.3, §11.9.3.5).
#[test]
fn every_constrained_list_encodes_its_length_per_x691() {
    let diagnostics = CriticalityDiagnostics::new(
        Some(ProcedureCode(14)),
        Some(TriggeringMessage::initiating_message),
        Some(Criticality::reject),
        Some(CriticalityDiagnosticsIEList(vec![
            CriticalityDiagnosticsIEItem::new(
                Criticality::reject,
                ID_GUAMI,
                TypeOfError::missing,
                None,
            ),
        ])),
        None,
    );
    let error = build_ngap!(InitiatingMessage, ErrorIndication,
        IGNORE, ErrorIndication,
        IGNORE Cause(Cause::protocol(CauseProtocol::abstract_syntax_error_reject)),
        IGNORE CriticalityDiagnostics(diagnostics),
    );
    let wire = error.encode().expect("encode ErrorIndication");
    assert_eq!(
        hex::encode(&wire),
        "00094014000002000f40016200134008780e000000001c40"
    );
    assert_eq!(NGAP_PDU::decode(&wire).expect("decode"), error);

    let reset = build_ngap!(InitiatingMessage, NGReset,
        REJECT, NGReset,
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::unspecified)),
        REJECT ResetType(ResetType::partOfNG_Interface(UEAssociatedLogicalNGConnectionList(vec![
            UEAssociatedLogicalNGConnectionItem::new(Some(AMFUENGAPID(5)), Some(RANUENGAPID(6)), None),
        ]))),
    );
    let wire = reset.encode().expect("encode NGReset");
    assert_eq!(
        hex::encode(&wire),
        "00140013000002000f4002000000580006400160050006"
    );
    assert_eq!(NGAP_PDU::decode(&wire).expect("decode"), reset);
}

/// A long element name makes rustfmt split both the SEQUENCE OF attribute
/// and declaration. The count still uses aligned PER's constrained integer
/// encoding (X.691 (02/2021) §20.6, §11.9.3.3, §11.5.7.3).
#[test]
fn multiline_extension_list_has_an_aligned_count() {
    let list =
        NGAPIESupportInformationResponseList(vec![NGAPIESupportInformationResponseItem::new(
            ProtocolIEID(10),
            NGAPIESupportInformationResponseItemNgapProtocolIESupportInfo::supported,
            NGAPIESupportInformationResponseItemNgapProtocolIEPresenceInfo::present,
            None,
        )]);
    let extension = ProtocolExtensionField::new(
        ProtocolExtensionID(356),
        Criticality::ignore,
        encode_open_type(&list).unwrap(),
    );
    let value = TargetNGRANNodeToSourceNGRANNodeFailureTransparentContainer::new(
        None,
        Some(ProtocolExtensionContainer(vec![
            extension.clone(),
            extension,
        ])),
    );
    let wire = hex::decode("2000010164400400000a000164400400000a00").unwrap();
    assert_eq!(rasn::aper::encode(&value).unwrap(), wire);
    assert_eq!(
        rasn::aper::decode::<TargetNGRANNodeToSourceNGRANNodeFailureTransparentContainer>(&wire)
            .unwrap(),
        value,
    );
}

/// A size constraint on a UTF8String, which is not a known-multiplier
/// character string type, is not PER-visible (X.691 §9.3.6): the value is an
/// unconstrained length in octets, then its UTF-8 octets (§27.6, 07/2002
/// numbering). Wireshark decodes the expected octets.
#[test]
fn utf8_string_names_have_an_unconstrained_length() -> Result<(), MissingIeError> {
    let name = ExtendedAMFName::new(None, Some(AMFNameUTF8String("amf-é".into())), None);
    let pdu = build_ngap!(SuccessfulOutcome, NGSetup,
        REJECT, NGSetupResponse,
        IGNORE Extended_AMFName(name.clone()),
    );
    let wire = pdu.encode().expect("encode NGSetupResponse");
    assert_eq!(hex::encode(&wire), "2015000f000001011240082006616d662dc3a9");
    let response: NGSetupResponse = NGAP_PDU::decode(&wire)
        .expect("decode PDU")
        .decode_value()
        .expect("decode message");
    extract_ngap_ies!(&response, NGSetupResponse,
        req extended: ExtendedAMFName = Extended_AMFName(value) => value,
    );
    assert_eq!(extended, name);

    let node = RANNodeNameUTF8String("gNB".into());
    let wire = rasn::aper::encode(&node).expect("encode RAN node name");
    assert_eq!(wire, [0x03, b'g', b'N', b'B']);
    assert_eq!(
        rasn::aper::decode::<RANNodeNameUTF8String>(&wire).expect("decode RAN node name"),
        node
    );
    Ok(())
}

fn assert_round_trip<T>(value: &T)
where
    T: rasn::Decode + rasn::Encode + PartialEq + std::fmt::Debug,
{
    let wire = rasn::aper::encode(value).expect("encode");
    assert_eq!(&rasn::aper::decode::<T>(&wire).expect("decode"), value);
}

/// A fixed-size BIT STRING longer than 16 bits is octet-aligned in APER
/// (X.691 (07/2002) §15.10). The expected octets are the ones Wireshark
/// decodes: padding follows the 3-bit NCC before the 256-bit NH, and the
/// 2-bit NgENB-ID choice index before the 20-bit ng-eNB ID.
#[test]
fn long_fixed_size_bit_strings_are_octet_aligned() -> Result<(), MissingIeError> {
    use rasn::types::{BitString, FixedBitString};

    let mut nh = FixedBitString::<256>::ZERO;
    nh.set(0, true);
    nh.set(255, true);
    let context = SecurityContext::new(NextHopChainingCount(5), SecurityKey(nh), None);
    let pdu = build_ngap!(SuccessfulOutcome, PathSwitchRequest,
        REJECT, PathSwitchRequestAcknowledge,
        REJECT SecurityContext(context.clone()),
    );
    let wire = pdu.encode().expect("encode PathSwitchRequestAcknowledge");
    assert_eq!(
        hex::encode(&wire),
        "20190028000001005d0021288000000000000000000000000000000000000000000000000000000000000001"
    );
    let acknowledge: PathSwitchRequestAcknowledge = NGAP_PDU::decode(&wire)
        .expect("decode PDU")
        .decode_value()
        .expect("decode message");
    extract_ngap_ies!(&acknowledge, PathSwitchRequestAcknowledge,
        req security_context: SecurityContext = SecurityContext(value) => value,
    );
    assert_eq!(security_context, context);

    let bits = |length: usize| (0..length).map(|bit| bit % 3 == 0).collect::<BitString>();
    let node = GlobalRANNodeID::globalNgENB_ID(GlobalNgENBID::new(
        oxirush_ngap::helpers::plmn("208", "93"),
        NgENBID::macroNgENB_ID(bits(20)),
        None,
    ));
    let pdu = build_ngap!(InitiatingMessage, NGSetup,
        REJECT, NGSetupRequest,
        REJECT GlobalRANNodeID(node.clone()),
    );
    let wire = pdu.encode().expect("encode NGSetupRequest");
    assert_eq!(hex::encode(&wire), "0015000f000001001b00084002f83900924920");
    let request: NGSetupRequest = NGAP_PDU::decode(&wire)
        .expect("decode PDU")
        .decode_value()
        .expect("decode message");
    extract_ngap_ies!(&request, NGSetupRequest,
        req global_ran_node_id: GlobalRANNodeID = GlobalRANNodeID(value) => value,
    );
    assert_eq!(global_ran_node_id, node);

    // The other such types, each after an unaligned field.
    for id in [
        NgENBID::shortMacroNgENB_ID(bits(18)),
        NgENBID::longMacroNgENB_ID(bits(21)),
    ] {
        assert_round_trip(&id);
    }
    for id in [
        ENBID::macroENB_ID(bits(20)),
        ENBID::homeENB_ID(bits(28)),
        ENBID::short_macroENB_ID(bits(18)),
        ENBID::long_macroENB_ID(bits(21)),
    ] {
        assert_round_trip(&id);
    }
    // SIZE(32, ...): an extension bit, then the root size as above.
    for id in [TNGFID::tNGF_ID(bits(32)), TNGFID::tNGF_ID(bits(40))] {
        assert_round_trip(&id);
    }
    for id in [TWIFID::tWIF_ID(bits(32)), TWIFID::tWIF_ID(bits(40))] {
        assert_round_trip(&id);
    }
    let mut set_id = FixedBitString::<22>::ZERO;
    set_id.set(0, true);
    assert_round_trip(&RIMInformation::new(
        GNBSetID(set_id),
        RIMInformationRIMRSDetection::rs_detected,
        None,
    ));
    let mut cag_id = FixedBitString::<32>::ZERO;
    cag_id.set(0, true);
    assert_round_trip(&CellCAGList(vec![CAGID(cag_id)]));
    let mut nid = FixedBitString::<44>::ZERO;
    nid.set(0, true);
    assert_round_trip(&SNPNMobilityInformation::new(NID(nid), None));
    Ok(())
}

/// The IEs whose ASN.1 type is a plain OCTET STRING have IE names too:
/// id-NGAP-Message (42) carries the rerouted InitialUEMessage, which
/// Wireshark decodes from the expected octets.
#[test]
fn octet_string_ies_have_ie_names() -> Result<(), MissingIeError> {
    let initial = build_ngap!(InitiatingMessage, InitialUEMessage,
        IGNORE, InitialUEMessage,
        REJECT RAN_UE_NGAP_ID(7u32),
        REJECT NAS_PDU(vec![0x7e, 0x00, 0x41]),
    )
    .encode()
    .expect("encode InitialUEMessage");
    let reroute = build_ngap!(InitiatingMessage, RerouteNASRequest,
        REJECT, RerouteNASRequest,
        REJECT RAN_UE_NGAP_ID(7u32),
        REJECT NGAP_Message(initial.clone()),
    );
    let wire = reroute.encode().expect("encode RerouteNASRequest");
    assert_eq!(
        hex::encode(&wire),
        "00240023000002005500020007002a001615000f401100000200550002000700260004037e0041"
    );
    let request: RerouteNASRequest = NGAP_PDU::decode(&wire)
        .expect("decode PDU")
        .decode_value()
        .expect("decode message");
    assert_eq!(request.protocol_ies.0[1].id, ID_NGAP_MESSAGE);
    extract_ngap_ies!(&request, RerouteNASRequest,
        req message: Vec<u8> = NGAP_Message(value) => value.to_vec(),
    );
    assert_eq!(message, initial);
    Ok(())
}

/// An extracted field and its binding may share a name.
#[test]
fn extraction_binding_may_share_the_field_name() -> Result<(), MissingIeError> {
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    );
    let request: UEContextReleaseRequest = pdu.decode_value().expect("decode message");
    extract_ngap_ies!(&request, UEContextReleaseRequest,
        req amf_ue_ngap_id: u64 = AMF_UE_NGAP_ID(amf_ue_ngap_id),
        opt cause: Cause = Cause(cause) => cause,
    );
    assert_eq!(amf_ue_ngap_id, 42);
    assert_eq!(
        cause,
        Some(Cause::radioNetwork(CauseRadioNetwork::user_inactivity))
    );
    Ok(())
}

/// An open type holds a complete encoding, in which an empty encoding is one
/// zero octet (X.691 (07/2002) §10.1.4, §10.2.1). No NGAP IE value is
/// empty, but the helper takes any type, such as NULL.
#[test]
fn an_empty_open_type_value_is_one_zero_octet() {
    let value = encode_open_type(&()).expect("encode NULL");
    assert_eq!(value.as_bytes(), [0]);
    decode_open_type::<()>(&value).expect("decode NULL");
}
