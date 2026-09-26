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
