/*
   OxiRush
   Copyright 2025-2026 Valentin D'Emmanuele

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! The macros that build and change a PDU of NGAP, and read and change its IEs.

use oxirush_ngap::ngap::*;
use oxirush_ngap::{Error, ErrorKind, Ie, ie};
use oxirush_ngap::{build_ngap, extract_ngap_ies};

#[test]
fn mutating_macro_updates_newtype_payload() -> Result<(), Error> {
    let mut pdu = build_ngap!(IGNORE UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )?;

    let NGAPPDU::initiatingMessage(message) = &mut pdu else {
        panic!("expected initiating message");
    };
    let request: UEContextReleaseRequest =
        oxirush_aper::decode(&message.value).expect("decode request");
    let mut request = request;

    let changed = request.set_ie::<ie::AMF_UE_NGAP_ID>(42u64);
    assert_eq!(changed, Ok(true));

    let ie = request
        .protocol_ies
        .0
        .iter()
        .find(|ie| ie.id.0 == 10)
        .expect("AMF UE ID");
    let id: AMFUENGAPID = oxirush_aper::decode(&ie.value).expect("decode ID");
    assert_eq!(id.0, 42);
    Ok(())
}

/// The identifier of a request, as the macro reads it.
fn id(request: &UEContextReleaseRequest) -> Result<u64, Error> {
    extract_ngap_ies!(request, UEContextReleaseRequest,
        req id: u64 = AMF_UE_NGAP_ID(id),
    );
    Ok(id)
}

#[test]
fn an_ie_that_is_there_twice_is_read_and_changed_at_the_same_place() -> Result<(), Error> {
    let ids = |request: &UEContextReleaseRequest| -> Vec<Vec<u8>> {
        let ies = request.protocol_ies.0.iter();
        ies.map(|ie| ie.value.to_vec()).collect()
    };
    let entry = |id: u64| ie::AMF_UE_NGAP_ID::field(Criticality::Reject, id);
    let mut request = UEContextReleaseRequest {
        protocol_ies: ProtocolIEContainer(vec![entry(1)?, entry(2)?]),
    };
    // The first one is read, and is the one that is changed.
    assert_eq!(id(&request)?, 1);
    let before = ids(&request);
    let changed = request.set_ie::<ie::AMF_UE_NGAP_ID>(42u64);
    assert_eq!(changed, Ok(true));
    assert_eq!(id(&request)?, 42);
    let after = ids(&request);
    assert_ne!(after[0], before[0]);
    assert_eq!(after[1], before[1]);
    Ok(())
}

#[test]
fn the_first_of_two_is_read_and_a_second_that_does_not_decode_is_not() -> Result<(), Error> {
    // An entry with the identifier of this IE whose octets are not its value,
    // after one that decodes and before it.
    let undecodable = || {
        let octets = oxirush_ngap::Octets::from(vec![0xff]);
        ProtocolIEField {
            id: ProtocolIEID(10),
            criticality: Criticality::Reject,
            value: octets,
        }
    };
    let entry = ie::AMF_UE_NGAP_ID::field(Criticality::Reject, 1u64)?;
    // The first one decodes: the macro and `set_ie` take it, and decode no other.
    let mut request = UEContextReleaseRequest {
        protocol_ies: ProtocolIEContainer(vec![entry.clone(), undecodable()]),
    };
    assert_eq!(id(&request)?, 1);
    let changed = request.set_ie::<ie::AMF_UE_NGAP_ID>(42u64);
    assert_eq!(changed, Ok(true));
    assert_eq!(id(&request)?, 42);
    // The first one does not decode: it is the error of the macro, and `set_ie`
    // replaces it, without reading it.
    let mut request = UEContextReleaseRequest {
        protocol_ies: ProtocolIEContainer(vec![undecodable(), entry.clone()]),
    };
    assert_eq!(id(&request).unwrap_err().kind(), ErrorKind::UndecodableIe);
    assert_eq!(request.set_ie::<ie::AMF_UE_NGAP_ID>(42u64), Ok(true));
    assert_eq!(id(&request)?, 42);
    Ok(())
}

#[test]
fn builder_macros_cover_ie_and_outcome_forms() -> Result<(), Error> {
    let ie = ie::AMF_UE_NGAP_ID::field(Criticality::Reject, 1u64)?;
    assert_eq!(ie.id.0, 10);

    // The direction and the procedure code of a PDU are those of its message.
    let initiating = build_ngap!(REJECT NGSetupRequest,)?;
    assert_eq!(initiating.direction(), oxirush_ngap::Direction::Initiating);
    assert_eq!(initiating.procedure_code(), 21);

    let successful = build_ngap!(REJECT NGSetupResponse,)?;
    assert_eq!(successful.direction(), oxirush_ngap::Direction::Successful);
    assert_eq!(successful.procedure_code(), 21);

    let unsuccessful = build_ngap!(REJECT NGSetupFailure,)?;
    assert_eq!(
        unsuccessful.direction(),
        oxirush_ngap::Direction::Unsuccessful
    );
    assert_eq!(unsuccessful.procedure_code(), 21);
    Ok(())
}

/// The IEs whose type is an OCTET STRING (CONTAINING ...) carry the
/// encoded transfer as octets.
#[test]
fn octet_string_containing_ies_have_macros() -> Result<(), Error> {
    let transfer = MBSSessionSetupOrModRequestTransfer {
        protocol_ies: ProtocolIEContainer(vec![]),
    };
    let octets = oxirush_aper::encode(&transfer).expect("encode transfer");
    let ie = ie::MBSSessionSetupRequestTransfer::field(Criticality::Reject, octets.clone())?;
    assert_eq!(ie.id.0, 315);

    let request = BroadcastSessionSetupRequest {
        protocol_ies: ProtocolIEContainer(vec![ie]),
    };
    extract_ngap_ies!(request, BroadcastSessionSetupRequest,
        req extracted: Vec<u8> = MBSSessionSetupRequestTransfer(value) => value.to_vec(),
    );
    assert_eq!(extracted, octets);
    let decoded: MBSSessionSetupOrModRequestTransfer =
        oxirush_aper::decode(&extracted).expect("decode transfer");
    assert_eq!(decoded, transfer);
    Ok(())
}
