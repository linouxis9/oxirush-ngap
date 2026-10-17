use oxirush_ngap::aper::{Decode, Encode, decode, encode};
use oxirush_ngap::ngap::*;
use oxirush_ngap::{BitString, Bits, Error, ErrorKind, Octets, build_ngap, extract_ngap_ies};

#[test]
fn public_macros_round_trip_typed_open_message() -> Result<(), Error> {
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )?;

    let NGAP_PDU::initiatingMessage(message) = &pdu else {
        panic!("expected initiating message");
    };
    // Extension bit, octet padding, then the aligned 16-bit ProtocolIE count.
    assert_eq!(&message.value[..3], &[0x00, 0x00, 0x03]);

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
    let value = SNSSAI::new(SST([1]), Some(SD([0, 0, 1])), None);
    let wire = encode(&value).expect("encode S-NSSAI");
    let decoded: SNSSAI = decode(&wire).expect("decode S-NSSAI");
    assert_eq!(decoded, value);
}

#[test]
fn plmn_support_list_round_trips_nested_slice() {
    let slice = SNSSAI::new(SST([1]), Some(SD([0, 0, 1])), None);
    let value = PLMNSupportList(vec![PLMNSupportItem::new(
        PLMNIdentity([0x02, 0xf8, 0x39]),
        SliceSupportList(vec![SliceSupportItem::new(slice, None)]),
        None,
    )]);
    let wire = encode(&value).expect("encode PLMN support");
    let decoded: PLMNSupportList = decode(&wire).expect("decode PLMN support");
    assert_eq!(decoded, value);
}

#[test]
fn slice_support_list_round_trips() {
    let value = SliceSupportList(vec![SliceSupportItem::new(
        SNSSAI::new(SST([1]), Some(SD([0, 0, 1])), None),
        None,
    )]);
    let wire = encode(&value).expect("encode slice support");
    let decoded: SliceSupportList = decode(&wire).expect("decode slice support");
    assert_eq!(decoded, value);
}

#[test]
fn plmn_support_item_round_trips() {
    let value = PLMNSupportItem::new(
        PLMNIdentity([0x02, 0xf8, 0x39]),
        SliceSupportList(vec![SliceSupportItem::new(
            SNSSAI::new(SST([1]), Some(SD([0, 0, 1])), None),
            None,
        )]),
        None,
    );
    let wire = encode(&value).expect("encode PLMN support item");
    let decoded: PLMNSupportItem = decode(&wire).expect("decode PLMN support item");
    assert_eq!(decoded, value);
}

/// An IE name addresses that IE even when it is also the type of another IE:
/// id-OldAMF is an AMFName and id-RRC-Resume-Cause an RRCEstablishmentCause.
#[test]
fn ie_names_address_their_own_ie_before_a_same_named_type() -> Result<(), Error> {
    let name = AMFName("amf".into());
    let ie = oxirush_ngap::build_ngap_ie!(NGSetupResponse, REJECT AMFName(name.clone()))?;
    assert_eq!(ie.id, ID_AMFNAME);
    let ie = oxirush_ngap::build_ngap_ie!(DownlinkNASTransport, REJECT OldAMF(name.clone()))?;
    assert_eq!(ie.id, ID_OLD_AMF);
    let ie = oxirush_ngap::build_ngap_ie!(InitialUEMessage,
        IGNORE RRCEstablishmentCause(RRCEstablishmentCause::mo_Signalling)
    )?;
    assert_eq!(ie.id, ID_RRCESTABLISHMENT_CAUSE);

    let pdu = build_ngap!(SuccessfulOutcome, NGSetup,
        REJECT, NGSetupResponse,
        REJECT AMFName(name.clone()),
    )?;
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
/// unconstrained length for SIZE(1..65536) (X.691 (02/2021) §11.9.3.3,
/// §11.9.3.5).
#[test]
fn every_constrained_list_encodes_its_length_per_x691() -> Result<(), Error> {
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
    )?;
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
    )?;
    let wire = reset.encode().expect("encode NGReset");
    assert_eq!(
        hex::encode(&wire),
        "00140013000002000f4002000000580006400160050006"
    );
    assert_eq!(NGAP_PDU::decode(&wire).expect("decode"), reset);
    Ok(())
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
    assert_eq!(encode(&value).unwrap(), wire);
    assert_eq!(
        decode::<TargetNGRANNodeToSourceNGRANNodeFailureTransparentContainer>(&wire).unwrap(),
        value,
    );
}

/// A size constraint on a UTF8String, which is not a known-multiplier
/// character string type, is not PER-visible (X.691 (02/2021) §10.3.7): the
/// value is an unconstrained length in octets, then its UTF-8 octets (§30.6).
/// Wireshark decodes the expected octets.
#[test]
fn utf8_string_names_have_an_unconstrained_length() -> Result<(), Error> {
    let name = ExtendedAMFName::new(None, Some(AMFNameUTF8String("amf-é".into())), None);
    let pdu = build_ngap!(SuccessfulOutcome, NGSetup,
        REJECT, NGSetupResponse,
        IGNORE Extended_AMFName(name.clone()),
    )?;
    let wire = pdu.encode().expect("encode NGSetupResponse");
    assert_eq!(hex::encode(&wire), "2015000f000001011240082006616d662dc3a9");
    // The message borrows from the PDU, which is kept beside it.
    let decoded = NGAP_PDU::decode(&wire).expect("decode PDU");
    let response: NGSetupResponse = decoded.decode_value().expect("decode message");
    extract_ngap_ies!(&response, NGSetupResponse,
        req extended: ExtendedAMFName = Extended_AMFName(value) => value,
    );
    assert_eq!(extended, name);

    let node = RANNodeNameUTF8String("gNB".into());
    let wire = encode(&node).expect("encode RAN node name");
    assert_eq!(wire, [0x03, b'g', b'N', b'B']);
    assert_eq!(
        decode::<RANNodeNameUTF8String>(&wire).expect("decode RAN node name"),
        node
    );
    Ok(())
}

/// `value` is read back from the octets that it is written to. A value that borrows
/// is read with the lifetime of these octets.
macro_rules! assert_round_trip {
    ($value:expr) => {{
        fn read_as<'a, T: Decode<'a>>(_: &T, wire: &'a [u8]) -> T {
            decode(wire).expect("decode")
        }
        let value = $value;
        let wire = encode(&value).expect("encode");
        assert_eq!(read_as(&value, &wire), value);
    }};
}

/// A fixed-size BIT STRING longer than 16 bits is octet-aligned in APER
/// (X.691 (02/2021) §16.10). The expected octets are the ones Wireshark
/// decodes: padding follows the 3-bit NCC before the 256-bit NH, and the
/// 2-bit NgENB-ID choice index before the 20-bit ng-eNB ID.
#[test]
fn long_fixed_size_bit_strings_are_octet_aligned() -> Result<(), Error> {
    // The first and the last bit of the 256.
    let mut nh = [0u8; 32];
    (nh[0], nh[31]) = (0x80, 0x01);
    let context = SecurityContext::new(NextHopChainingCount(5), SecurityKey(nh.into()), None);
    let pdu = build_ngap!(SuccessfulOutcome, PathSwitchRequest,
        REJECT, PathSwitchRequestAcknowledge,
        REJECT SecurityContext(context.clone()),
    )?;
    let wire = pdu.encode().expect("encode PathSwitchRequestAcknowledge");
    assert_eq!(
        hex::encode(&wire),
        "20190028000001005d0021288000000000000000000000000000000000000000000000000000000000000001"
    );
    // The message borrows from the PDU, which is kept beside it.
    let decoded = NGAP_PDU::decode(&wire).expect("decode PDU");
    let acknowledge: PathSwitchRequestAcknowledge = decoded.decode_value().expect("decode message");
    extract_ngap_ies!(&acknowledge, PathSwitchRequestAcknowledge,
        req security_context: SecurityContext = SecurityContext(value) => value,
    );
    assert_eq!(security_context, context);

    // One bit in three, from the first.
    let bits = |length: usize| {
        let number = (0..length).fold(0, |number, bit| number << 1 | u64::from(bit % 3 == 0));
        Bits::new(number, length).unwrap()
    };
    let node = GlobalRANNodeID::globalNgENB_ID(GlobalNgENBID::new(
        oxirush_ngap::helpers::plmn("208", "93")?,
        NgENBID::macroNgENB_ID(bits(20)),
        None,
    ));
    let pdu = build_ngap!(InitiatingMessage, NGSetup,
        REJECT, NGSetupRequest,
        REJECT GlobalRANNodeID(node.clone()),
    )?;
    let wire = pdu.encode().expect("encode NGSetupRequest");
    assert_eq!(hex::encode(&wire), "0015000f000001001b00084002f83900924920");
    // The message borrows from the PDU, which is kept beside it.
    let decoded = NGAP_PDU::decode(&wire).expect("decode PDU");
    let request: NGSetupRequest = decoded.decode_value().expect("decode message");
    extract_ngap_ies!(&request, NGSetupRequest,
        req global_ran_node_id: GlobalRANNodeID = GlobalRANNodeID(value) => value,
    );
    assert_eq!(global_ran_node_id, node);

    // The other such types, each after an unaligned field.
    for id in [
        NgENBID::shortMacroNgENB_ID(bits(18)),
        NgENBID::longMacroNgENB_ID(bits(21)),
    ] {
        assert_round_trip!(id);
    }
    for id in [
        ENBID::macroENB_ID(bits(20)),
        ENBID::homeENB_ID(bits(28)),
        ENBID::short_macroENB_ID(bits(18)),
        ENBID::long_macroENB_ID(bits(21)),
    ] {
        assert_round_trip!(id);
    }
    // SIZE(32, ...): an extension bit, then the root size as above. A later version
    // may have another size, so the value is a bit string of any length.
    let long = |length: usize| BitString::from(bits(length));
    for id in [TNGFID::tNGF_ID(long(32)), TNGFID::tNGF_ID(long(40))] {
        assert_round_trip!(id);
    }
    for id in [TWIFID::tWIF_ID(long(32)), TWIFID::tWIF_ID(long(40))] {
        assert_round_trip!(id);
    }
    // The first bit alone.
    let first = |length: usize| Bits::new(1 << (length - 1), length).unwrap();
    assert_round_trip!(RIMInformation::new(
        GNBSetID(first(22)),
        RIMInformationRIMRSDetection::rs_detected,
        None,
    ));
    assert_round_trip!(CellCAGList(vec![CAGID(first(32))]));
    assert_round_trip!(SNPNMobilityInformation::new(NID(first(44)), None));
    Ok(())
}

/// The IEs whose ASN.1 type is a plain OCTET STRING have IE names too:
/// id-NGAP-Message (42) carries the rerouted InitialUEMessage, which
/// Wireshark decodes from the expected octets.
#[test]
fn octet_string_ies_have_ie_names() -> Result<(), Error> {
    let initial = build_ngap!(InitiatingMessage, InitialUEMessage,
        IGNORE, InitialUEMessage,
        REJECT RAN_UE_NGAP_ID(7u32),
        REJECT NAS_PDU(vec![0x7e, 0x00, 0x41]),
    )?
    .encode()
    .expect("encode InitialUEMessage");
    let reroute = build_ngap!(InitiatingMessage, RerouteNASRequest,
        REJECT, RerouteNASRequest,
        REJECT RAN_UE_NGAP_ID(7u32),
        REJECT NGAP_Message(initial.clone()),
    )?;
    let wire = reroute.encode().expect("encode RerouteNASRequest");
    assert_eq!(
        hex::encode(&wire),
        "00240023000002005500020007002a001615000f401100000200550002000700260004037e0041"
    );
    // The message borrows from the PDU, which is kept beside it.
    let decoded = NGAP_PDU::decode(&wire).expect("decode PDU");
    let request: RerouteNASRequest = decoded.decode_value().expect("decode message");
    assert_eq!(request.protocol_ies.0[1].id, ID_NGAP_MESSAGE);
    extract_ngap_ies!(&request, RerouteNASRequest,
        req message: Vec<u8> = NGAP_Message(value) => value.to_vec(),
    );
    assert_eq!(message, initial);
    Ok(())
}

/// An extracted field and its binding may share a name.
#[test]
fn extraction_binding_may_share_the_field_name() -> Result<(), Error> {
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(42u64),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )?;
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

/// A message with the identifier that the functions below require, or without it.
fn release_request(id: Option<u64>) -> UEContextReleaseRequest<'static> {
    let ie =
        |id: u64| oxirush_ngap::build_ngap_ie!(UEContextReleaseRequest, REJECT AMF_UE_NGAP_ID(id));
    let ies = id.map(|id| ie(id).expect("an identifier"));
    UEContextReleaseRequest::new(ProtocolIEContainer(ies.into_iter().collect()))
}

/// A `req` field that is not there is the error of the enclosing function: the
/// macro converts its error as `?` converts one, here into an error type of the
/// caller that has a `From` of the error of the crate.
#[test]
fn a_missing_required_ie_is_an_error_of_the_callers_own_type() {
    #[derive(Debug, PartialEq)]
    enum Refused {
        Codec(ErrorKind),
    }
    impl From<Error> for Refused {
        fn from(error: Error) -> Self {
            Self::Codec(error.kind())
        }
    }
    fn id(request: &UEContextReleaseRequest) -> Result<u64, Refused> {
        extract_ngap_ies!(request, UEContextReleaseRequest,
            req amf_id: u64 = AMF_UE_NGAP_ID(id),
        );
        Ok(amf_id)
    }
    let missing = Refused::Codec(ErrorKind::MissingIe);
    assert_eq!(id(&release_request(None)), Err(missing));
    assert_eq!(id(&release_request(Some(42))), Ok(42));
}

/// The same in a function whose error is a `Box<dyn Error>`, which holds the error
/// of the crate.
#[test]
fn a_missing_required_ie_is_a_boxed_error() {
    fn id(request: &UEContextReleaseRequest) -> Result<u64, Box<dyn std::error::Error>> {
        extract_ngap_ies!(request, UEContextReleaseRequest,
            req amf_id: u64 = AMF_UE_NGAP_ID(id),
        );
        Ok(amf_id)
    }
    let error = id(&release_request(None)).unwrap_err();
    let missing = error.downcast_ref::<Error>().expect("the error");
    assert_eq!(missing.kind(), ErrorKind::MissingIe);
    assert_eq!(
        error.to_string(),
        "this UEContextReleaseRequest has no AMF-UE-NGAP-ID, which the field `amf_id` requires"
    );
    assert_eq!(id(&release_request(Some(42))).unwrap(), 42);
}

/// And in a function whose error is the error of the crate itself, which needs no
/// conversion.
#[test]
fn a_missing_required_ie_is_an_error_of_the_crate() {
    fn id(request: &UEContextReleaseRequest) -> Result<u64, Error> {
        extract_ngap_ies!(request, UEContextReleaseRequest,
            req amf_id: u64 = AMF_UE_NGAP_ID(id),
        );
        Ok(amf_id)
    }
    let missing = id(&release_request(None)).unwrap_err();
    assert_eq!(missing.kind(), ErrorKind::MissingIe);
    assert_eq!(id(&release_request(Some(42))), Ok(42));
}

/// An open type holds a complete encoding, in which an empty encoding is one
/// zero octet (X.691 (02/2021) §11.1.4, §11.2.1). No NGAP IE value is
/// empty, but the helper takes any type, such as NULL.
#[test]
fn an_empty_open_type_value_is_one_zero_octet() {
    let value = encode_open_type(&()).expect("encode NULL");
    assert_eq!(value, [0]);
    decode_open_type::<()>(&value).expect("decode NULL");
}

/// A number of an extensible constraint whose root is several ranges, as
/// `ExpectedActivityPeriod ::= INTEGER (1..30|40|50|60|80|100|120|150|180|181, ...)`:
/// 96 is in the range of the root, 1 to 181, and is none of its values. X.691
/// (02/2021) 13.1 has the extension bit set for a value that "is not within the range
/// of the extension root", so 96 is written in the eight bits of the root, as 181 is.
/// pycrate 0.8.1 writes it after the extension bit, as it writes 182, and each of the
/// two reads what the other writes.
#[test]
fn a_number_between_the_ranges_of_an_extensible_root_is_written_in_the_root() {
    let period = |value: u8| -> ExpectedUEActivityBehaviour {
        let period = ExpectedActivityPeriod(u64::from(value));
        ExpectedUEActivityBehaviour::new(Some(period), None, None, None)
    };
    let written = |value| hex::encode(encode(&period(value)).unwrap());
    let read = |octets: &str| {
        let octets = hex::decode(octets).unwrap();
        decode::<ExpectedUEActivityBehaviour>(&octets).unwrap()
    };

    // The extension bit of the SEQUENCE, a bit for each of its four members, the
    // extension bit of the number, then the number less one in eight bits.
    assert_eq!(written(30), "4074");
    assert_eq!(written(181), "42d0");
    assert_eq!(written(96), "417c");
    assert_eq!(read("417c"), period(96));
    // After the extension bit: a length, and the number.
    assert_eq!(written(182), "440200b6");
    assert_eq!(read("440200b6"), period(182));
    // What pycrate writes for 96.
    assert_eq!(read("440160"), period(96));
}

/// The octets of a string of a size that a vector is made for, as
/// `generate.py` of oxirush-aper fills it.
fn pattern(octets: usize) -> String {
    (0..octets).map(|at| format!("{:02x}", at % 251)).collect()
}

/// `octets`, which pycrate wrote, are a value of `T` that is written as the same octets,
/// and are none with an octet less.
fn as_pycrate_writes_it<T: Decode<'static> + Encode>(octets: &str) -> T {
    // The value borrows from the octets, which a test keeps to its end.
    let octets: &'static [u8] = hex::decode(octets).unwrap().leak();
    let value = decode::<T>(octets).expect("decode");
    assert_eq!(hex::encode(encode(&value).unwrap()), hex::encode(octets));
    assert!(decode::<T>(&octets[..octets.len() - 1]).is_err());
    value
}

/// A string whose size has a range of 256 or more is after a length of two octets,
/// which starts an octet (X.691 (02/2021) §11.5.7.3, §16.11, §17.8): the four members
/// and alternatives of the bindings that are such a string, as pycrate 0.8.1 writes a
/// value of each.
#[test]
fn a_string_with_a_length_of_two_octets_is_written_as_pycrate_writes_it() {
    // receiveStatusOfUL-PDCP-SDUs BIT STRING (SIZE(1..2048)), of 300 bits and of 2048:
    // the two counts of the PDCP SN, the length less one, then the bits.
    let bits = "a5".repeat(37);
    let status: DRBStatusUL12 = as_pycrate_writes_it(&format!("4000000000012b{bits}b0"));
    assert_eq!(status.receive_status_of_ul_pdcp_sdus.unwrap().len(), 300);
    let longest = format!("400000000007ff{}", "a5".repeat(256));
    let status: DRBStatusUL12 = as_pycrate_writes_it(&longest);
    assert_eq!(status.receive_status_of_ul_pdcp_sdus.unwrap().len(), 2048);

    // singleDevice and groupDevices BIT STRING (SIZE(1..1024, ...)): 300 bits and 1024
    // in the root, and 1025 after the extension bit, with an unconstrained length.
    type Devices = AIoTDeviceIdentificationRequested<'static>;
    let Devices::singleDevice(device) = as_pycrate_writes_it(&format!("00012b{bits}b0")) else {
        panic!("a single device");
    };
    assert_eq!(device.len(), 300);
    let Devices::groupDevices(group) = as_pycrate_writes_it(&format!("4003ff{}", "a5".repeat(128)))
    else {
        panic!("a group of devices");
    };
    assert_eq!(group.len(), 1024);
    let Devices::groupDevices(group) =
        as_pycrate_writes_it(&format!("608401{}80", "a5".repeat(128)))
    else {
        panic!("a group of devices");
    };
    assert_eq!(group.len(), 1025);

    // containerForAppLayerMeasConfig OCTET STRING (SIZE(1..8000)), of 300 octets.
    let config = format!("1000adcd4894fa3b000000012b{}", pattern(300));
    let config: UEAppLayerMeasConfigInfo = as_pycrate_writes_it(&config);
    assert_eq!(
        config.container_for_app_layer_meas_config.unwrap().len(),
        300
    );
}

/// The extension container of a value owns its octets, which a peer seldom
/// sends: a TAI that has one has no lifetime, outlives the octets that it is
/// decoded from, and keeps the container.
#[test]
fn a_value_keeps_its_extension_container_after_its_octets() {
    let sent = TAI::new(
        PLMNIdentity([0x02, 0xf8, 0x39]),
        TAC([0x00, 0x00, 0x01]),
        Some(ProtocolExtensionContainer(vec![
            ProtocolExtensionField::new(
                ProtocolExtensionID(1),
                Criticality::ignore,
                Octets::from(vec![0x00]),
            ),
        ])),
    );
    let octets = encode(&sent).unwrap();
    let kept: TAI = decode(&octets).unwrap();
    drop(octets);
    let extensions = kept.ie_extensions.as_ref().unwrap();
    assert!(!extensions.0[0].extension_value.is_borrowed());
    assert_eq!(kept, sent);
}

/// A SEQUENCE is a struct that a literal builds outside the crate, with the
/// members that `new` takes in their order.
#[test]
fn a_sequence_is_built_by_a_literal() {
    let (plmn_identity, tac) = (PLMNIdentity([0x02, 0xf8, 0x39]), TAC([0x00, 0x00, 0x01]));
    let literal = TAI {
        plmn_identity,
        tac,
        ie_extensions: None,
    };
    assert_eq!(literal, TAI::new(plmn_identity, tac, None));
}
