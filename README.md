# oxirush-ngap

[![Crates.io](https://img.shields.io/crates/v/oxirush-ngap.svg)](https://crates.io/crates/oxirush-ngap)
[![Documentation](https://docs.rs/oxirush-ngap/badge.svg)](https://docs.rs/oxirush-ngap)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

A Rust codec for NGAP, the NG Application Protocol between an NG-RAN node and
an AMF (3GPP TS 38.413): types generated from the ASN.1 of the specification,
aligned PER by [`oxirush-aper`](https://github.com/linouxis9/oxirush-aper), and
macros that build a message and read its IEs by name.

```rust
use oxirush_ngap::{build_ngap, extract_ngap_ies, ngap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Decode a UE CONTEXT RELEASE REQUEST
    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500")?;
    let pdu = NGAP_PDU::decode(&bytes)?;
    assert_eq!((pdu.procedure_code(), pdu.is_initiating()), (42, true));
    println!("{}", pdu.procedure_name());

    // Read its IEs by name. An `opt` field is `None` when its IE is not there,
    // and what follows `=>` is what the field keeps of the decoded value
    let request: UEContextReleaseRequest = pdu.decode_value()?;
    extract_ngap_ies!(request, UEContextReleaseRequest,
        opt amf_id: u64 = AMF_UE_NGAP_ID(id),
        opt ran_id: u32 = RAN_UE_NGAP_ID(id),
        opt cause: Cause = Cause(cause) => cause,
    );
    println!("AMF={amf_id:?} RAN={ran_id:?} cause={cause:?}");

    // Build it with another cause: the direction, the procedure, the criticality
    // of the PDU and the message, then each IE with its criticality
    let rebuilt = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,

        REJECT AMF_UE_NGAP_ID(42u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::radio_connection_with_ue_lost)),
    )?;

    // Encode it
    let encoded = hex::encode(rebuilt.encode()?);
    assert_eq!(encoded, "002a4015000003000a0002002a005500020007000f40020540");
    Ok(())
}
```

```text
UEContextReleaseRequest
AMF=Some(42) RAN=Some(7) cause=Some(radioNetwork(user_inactivity))
```

The text under a block is what the block prints.

This page describes 0.5.0, which is not on crates.io yet.

## Installation

```toml
[dependencies]
oxirush-ngap = "0.5"
# With the JSON forms:
# oxirush-ngap = { version = "0.5", features = ["inspect"] }
# What the examples of this page write octets in hexadecimal and JSON as text with:
# hex = "0.4"
# serde_json = "1"
```

| Feature | Adds |
|---------|------|
| `inspect` | A PDU as JSON, a `oxirush_ngap::tree::Value`: its tree, its view and the paths of its values, with the members in the order of the specification |
| `serde` | `Serialize` and `Deserialize` for every type of the bindings, through its tree. It turns on `inspect` |

Until 0.5.0 is on crates.io, with `oxirush-aper` 0.1.0 and `oxirush-utils`
0.1.0 that it takes, a program takes the crate from its repository and the two
others at the revisions that it is tested with:

```toml
[dependencies]
oxirush-ngap = { git = "https://github.com/linouxis9/oxirush-ngap", features = ["inspect"] }

[patch.crates-io]
oxirush-aper = { git = "https://github.com/linouxis9/oxirush-aper", rev = "6252c750dde9ba669bcc3501dbf08ffec0638758" }
oxirush-utils = { git = "https://github.com/linouxis9/oxirush-utils", rev = "2f99edcc40baa5cf7397f94ecfc131dc1143bac5" }
```

In a clone of the repository, cargo takes them with `--config`, as CI does:

```bash
cargo test --all-features \
  --config 'patch.crates-io.oxirush-aper.git="https://github.com/linouxis9/oxirush-aper"' \
  --config 'patch.crates-io.oxirush-aper.rev="6252c750dde9ba669bcc3501dbf08ffec0638758"' \
  --config 'patch.crates-io.oxirush-utils.git="https://github.com/linouxis9/oxirush-utils"' \
  --config 'patch.crates-io.oxirush-utils.rev="2f99edcc40baa5cf7397f94ecfc131dc1143bac5"'
```

The minimum supported Rust version is 1.88. The crate depends on one version of
`oxirush-aper` exactly, which is released with it, and the values of the
bindings are its types, as `Octets`, `Bits` and `Error`: a program names them
through `oxirush_ngap::aper`.

## Answer a gNB, from NG SETUP to the INITIAL CONTEXT SETUP

The AMF's side of a gNB and of the first UE that it serves: the gNB sets up
the NG interface, then sends the first NAS message of the UE; the AMF answers
the UE in a DOWNLINK NAS TRANSPORT, and gives the gNB the UE's security
context, KgNB, in an INITIAL CONTEXT SETUP REQUEST. The NAS-PDUs are octets
of [oxirush-nas](https://github.com/linouxis9/oxirush-nas), and KgNB is the
one that [oxirush-security](https://github.com/linouxis9/oxirush-security)
derives from the KAMF of its 5G AKA example.

```rust
use oxirush_ngap::helpers::*;
use oxirush_ngap::{build_ngap, extract_ngap_ies, ie, ngap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let network = plmn("208", "93")?;
    let slices = || SliceSupportList(vec![SliceSupportItem::new(s_nssai(1, None), None)]);

    // The gNB: NG SETUP REQUEST, its identity, its name, and the slices of
    // the tracking area that it serves
    let gnb = GlobalRANNodeID::globalGNB_ID(global_gnb_id(network, 0x123456, 24)?);
    let plmns = BroadcastPLMNList(vec![BroadcastPLMNItem::new(network, slices(), None)]);
    let area = SupportedTAItem::new(TAC([0, 0, 1]), plmns, None);
    let request = build_ngap!(InitiatingMessage, NGSetup, NGSetupRequest,
        GlobalRANNodeID(gnb),
        RANNodeName("gnb-1"),
        SupportedTAList(SupportedTAList(vec![area])),
        DefaultPagingDRX(PagingDRX::v128),
    )?;

    // The AMF reads it, and answers with its name, its GUAMI and its slices
    let received = NGAP_PDU::decode(&request.encode()?)?;
    extract_ngap_ies!(received, NGSetupRequest,
        opt name: String = RANNodeName(name) => name.0.into_owned(),
        req areas: SupportedTAList = SupportedTAList(areas) => areas,
    );
    println!("NG SETUP REQUEST from {}: {} tracking area", name.unwrap_or_default(), areas.0.len());
    let response = build_ngap!(SuccessfulOutcome, NGSetup, NGSetupResponse,
        AMFName("amf-1"),
        ServedGUAMIList(ServedGUAMIList(vec![ServedGUAMIItem::new(guami(network, 1, 1, 0)?, None, None)])),
        RelativeAMFCapacity(RelativeAMFCapacity(255)),
        PLMNSupportList(PLMNSupportList(vec![PLMNSupportItem::new(network, slices(), None)])),
    )?;
    println!("NG SETUP RESPONSE {}", hex::encode(response.encode()?));

    // The gNB: INITIAL UE MESSAGE, with the REGISTRATION REQUEST of a UE and
    // the cell that the UE is in
    let cell = nr_cgi(network, 0x123456, 24, 1)?;
    let location = UserLocationInformationNR::new(cell, tai(network, [0, 0, 1]), None, None);
    let registration = hex::decode("7e004179000d0102f8390000000000000010022e08a020000000000000")?;
    let initial = build_ngap!(InitiatingMessage, InitialUEMessage, InitialUEMessage,
        RAN_UE_NGAP_ID(7u32),
        NAS_PDU(registration),
        UserLocationInformation(UserLocationInformation::userLocationInformationNR(location)),
        RRCEstablishmentCause(RRCEstablishmentCause::mo_Signalling),
    )?;

    // The AMF reads the IEs that it needs where they are in the octets
    let octets = initial.encode()?;
    let received = NGAP_PDU::decode_borrowed(&octets)?;
    extract_ngap_ies!(received, InitialUEMessage,
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
        req nas: NASPDU = NAS_PDU(nas) => nas,
    );
    println!("INITIAL UE MESSAGE of RAN UE {ran_id}: NAS-PDU {}", hex::encode(&nas.0));

    // It answers the UE, under the AMF UE NGAP ID that it gives it: here an
    // AUTHENTICATION REQUEST
    let authentication = hex::decode(
        "7e0056000200002123553cbe9637a89d218ae64dae47bf35201055f328b43577b9b94a9ffac354dfafb3",
    )?;
    let downlink = build_ngap!(InitiatingMessage, DownlinkNASTransport, DownlinkNASTransport,
        AMF_UE_NGAP_ID(1u64),
        RAN_UE_NGAP_ID(ran_id),
        NAS_PDU(authentication),
    )?;
    println!("DOWNLINK NAS TRANSPORT {}", hex::encode(downlink.encode()?));

    // After the security mode, the context of the UE for the gNB: its GUAMI,
    // its slices, its security capabilities and KgNB
    let k_gnb = hex::decode("c18166e13cfde1dea842c708f6e0b3372b6896664df4e865c0c78bd270852005")?;
    let setup = build_ngap!(InitiatingMessage, InitialContextSetup, InitialContextSetupRequest,
        AMF_UE_NGAP_ID(1u64),
        RAN_UE_NGAP_ID(ran_id),
        GUAMI(guami(network, 1, 1, 0)?),
        AllowedNSSAI(AllowedNSSAI(vec![AllowedNSSAIItem::new(s_nssai(1, None), None)])),
        UESecurityCapabilities(ue_security_capabilities(0xe0, 0xe0)),
        SecurityKey(<[u8; 32]>::try_from(k_gnb).unwrap()),
    )?;

    // The gNB checks it against its object set, and takes the key
    let received = NGAP_PDU::decode(&setup.encode()?)?;
    let deviations = received.check()?;
    let key = received.ie::<ie::SecurityKey>().unwrap()?;
    let key = hex::encode(key.0.as_octets());
    println!("INITIAL CONTEXT SETUP REQUEST, {} deviations: KgNB {key}", deviations.len());
    Ok(())
}
```

```text
NG SETUP REQUEST from gnb-1: 1 tracking area
NG SETUP RESPONSE 2015002b000004000100070200616d662d3100600008000002f83901004000564001ff005000080002f83900000008
INITIAL UE MESSAGE of RAN UE 7: NAS-PDU 7e004179000d0102f8390000000000000010022e08a020000000000000
DOWNLINK NAS TRANSPORT 0004403e000003000a000200010055000200070026002b2a7e0056000200002123553cbe9637a89d218ae64dae47bf35201055f328b43577b9b94a9ffac354dfafb3
INITIAL CONTEXT SETUP REQUEST, 0 deviations: KgNB c18166e13cfde1dea842c708f6e0b3372b6896664df4e865c0c78bd270852005
```

## With oxirush-nas and oxirush-security

The same exchange, with the NAS-PDUs built and read by
[oxirush-nas](https://github.com/linouxis9/oxirush-nas) and the keys of
[oxirush-security](https://github.com/linouxis9/oxirush-security): the
REGISTRATION REQUEST of the INITIAL UE MESSAGE, the SECURITY MODE COMMAND
that the AMF protects with the new NAS security context in a DOWNLINK NAS
TRANSPORT, and KgNB from KAMF and the uplink NAS COUNT.

```rust,ignore
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
```

```text
RAN UE 7: 5GMM RegistrationRequest (type=InitialRegistration, FOR=0, ngKSI=no key, identity=SUCI (PLMN=208-93, routing-indicator=0, scheme=0, key=0, MSIN=0000000001), UE-SecCap=EA0 EA1 EA2 / IA0 IA1 IA2)
00044023000003000a00020001005500020007002600100f7e03a46a0b81007e005d220002e0e0
KgNB c18166e13cfde1dea842c708f6e0b3372b6896664df4e865c0c78bd270852005, 0 deviations
```

## Decode a PDU

```rust
use oxirush_ngap::{NgapPduKind, ngap::*};

fn receive(bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let pdu = NGAP_PDU::decode(bytes)?;

    Ok(match pdu.kind() {
        NgapPduKind::Initiating_UEContextReleaseRequest => {
            let request: UEContextReleaseRequest = pdu.decode_value()?;
            format!("a release request with {} IEs", request.protocol_ies.0.len())
        }
        NgapPduKind::Other { direction, procedure_code } => {
            format!("procedure {procedure_code} has no {direction}")
        }
        kind => format!("{kind:?} is not handled here"),
    })
}

// A UE CONTEXT RELEASE REQUEST, then an NG SETUP FAILURE
let request = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
println!("{}", receive(&request).unwrap());

let failure = hex::decode("40150008000001000f400188").unwrap();
println!("{}", receive(&failure).unwrap());
```

```text
a release request with 3 IEs
Unsuccessful_NGSetup is not handled here
```

`decode` reads the envelope of a PDU alone: its direction, its procedure code,
its criticality and the octets of its message. `decode_value`, `protocol_ies`
and `check` read the message, and what does not decode there is their error.

| Of an `NGAP_PDU` | Gives |
|------------------|-------|
| `decode(&bytes)`, `decode_borrowed(&bytes)` | The PDU of the octets of one message, whole. `decode_borrowed` copies nothing: the PDU borrows from `bytes` |
| `encode()`, `encode_into(&mut octets)` | Its octets |
| `procedure_code()`, `procedure_name()`, `direction()`, `kind()` | `42`, `"UEContextReleaseRequest"`, `"InitiatingMessage"`, and an `NgapPduKind`, which is `Other` for a procedure that the specification does not have in that direction |
| `decode_value()`, `message()` | The message as its type, which borrows from the PDU, or from what the PDU borrows |
| `protocol_ies()`, `set_protocol_ies(ies)` | The IEs of its message, whatever the message is |
| `check()` | How it deviates from the object set of its message |

## Build a PDU

An IE is named by its ASN.1 identifier without `id-` and with `_` for `-`
(`AMF_UE_NGAP_ID`), a type by the name that ASN.1 gives it without its hyphens
(the IE `FiveG_S_TMSI` has the type `FiveGSTMSI`), and a field by the words of
its member in lower case (`pLMNIdentity` is `plmn_identity`). A criticality
that a call leaves out is the one that TS 38.413 assigns, and a name that the
crate does not have, or a message that the procedure does not have in that
direction, does not compile.

```rust
use oxirush_ngap::{Message, build_ngap, ngap::*};

fn main() -> Result<(), oxirush_ngap::Error> {
    // The criticality of the procedure, and those of the object set of the message
    let pdu = build_ngap!(InitiatingMessage, InitialUEMessage, InitialUEMessage,
        RAN_UE_NGAP_ID(7u32),
        NAS_PDU(vec![0x7e, 0x00, 0x41]),
    )?;
    println!("{:?} {}", pdu.criticality(), hex::encode(pdu.encode()?));

    // The PDU of a message that is at hand
    let message: InitialUEMessage = pdu.decode_value()?;
    let again = NGAP_PDU::of_message(&message, InitialUEMessage::CRITICALITY)?;
    println!("{}", again == pdu);

    // An IE that the object set of the message does not have is assigned none
    let error = build_ngap!(InitiatingMessage, InitialUEMessage, InitialUEMessage,
        TimeToWait(TimeToWait::v5s),
    );
    println!("{}", error.unwrap_err());
    Ok(())
}
```

```text
ignore 000f401100000200550002000700260004037e0041
true
TimeToWait is no IE of the object set of InitialUEMessage, which assigns it no criticality: write its criticality
```

## Read the IEs of a message

```rust
use oxirush_ngap::{Error, extract_ngap_ies, ngap::*};

fn handle(request: &UEContextReleaseRequest) -> Result<(), Error> {
    extract_ngap_ies!(request, UEContextReleaseRequest,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
        req cause: Cause = Cause(value) => value,
    );

    println!("AMF={amf_id} RAN={ran_id} cause={cause:?}");
    Ok(())
}
```

A field is `req`, `opt` or `lax`, its name and its type, then the IE with a
name for its decoded value; after `=>` comes what the field keeps of it.

The macro takes the PDU too, as the message that it holds, without
`decode_value()`: it walks the IEs of the message once, where they are, and
decodes those that a field asks for. `ie::<T>()` and `ies::<T>()` of a PDU, of
a message and of a transfer read one IE, by its marker type in
`oxirush_ngap::ie`: the first occurrence, and every occurrence in order.

```rust
use oxirush_ngap::{Error, extract_ngap_ies, ie, ngap::*};

fn main() -> Result<(), Error> {
    // A UE CONTEXT RELEASE REQUEST, read in the buffer that it was received in
    let octets = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    let pdu = NGAP_PDU::decode_borrowed(&octets)?;

    // The IEs of its message, from the PDU: the Cause is not decoded
    extract_ngap_ies!(pdu, UEContextReleaseRequest,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
    );
    println!("AMF={amf_id} RAN={ran_id}");

    // One IE, by its marker type
    let cause = pdu.ie::<ie::Cause>().unwrap()?;
    println!("{cause:?}");
    Ok(())
}
```

```text
AMF=42 RAN=7
radioNetwork(user_inactivity)
```

When a message has an IE twice, the macro, `ie` and `with_ngap_ie_mut!` take
the first one and decode no other; `ies` gives each, and `check()` reports the
second as `Deviation::RepeatedIe`.

| Field | Is | When the IE is not there | When it does not decode |
|-------|----|--------------------------|-------------------------|
| `req` | A `T` | An error of the kind `MissingIe` | An error of the kind `UndecodableIe` |
| `opt` | An `Option<T>` | `None` | The same error |
| `lax` | An `Option<T>` | `None` | `None` |

The error is that of the enclosing function, converted by `?`. A `lax` field
reads an IE as TS 38.413 clause 10.3.4.2 has a receiver read one of the
criticality `ignore`: as if it had not been received when it does not decode.

`with_ngap_ie_mut!` changes one IE of a message in place, and `build_ngap_ie!`
builds one: their documentation has an example of each.

## A transfer and a handover container, typed

A transfer is an `OCTET STRING` that contains a value of its own, and the
container of a handover one that holds the type which TS 38.413 clause
9.3.1.20 names. `decode_open_type` reads such octets as the type that the
caller names, and `encode_open_type` writes them. The macros take a transfer
of protocol IEs as they take a message.

```rust
use oxirush_ngap::{build_ngap_ie, extract_ngap_ies, ngap::*};

fn main() -> Result<(), oxirush_ngap::Error> {
    // The transfer of a PDU SESSION RESOURCE SETUP REQUEST, as its item holds it
    let octets = hex::decode("0000010086000100").unwrap();
    let transfer: PDUSessionResourceSetupRequestTransfer = decode_open_type(&octets)?;
    extract_ngap_ies!(transfer, PDUSessionResourceSetupRequestTransfer,
        req kind: PDUSessionType = PDUSessionType(kind) => kind,
    );
    println!("{kind:?}");

    // One is built as a message is
    let ipv6 = PDUSessionResourceSetupRequestTransfer::new(ProtocolIEContainer(vec![
        build_ngap_ie!(PDUSessionResourceSetupRequestTransfer,
            REJECT PDUSessionType(PDUSessionType::ipv6))?,
    ]));
    let octets = hex::encode(encode_open_type(&ipv6)?);
    println!("{octets}");

    // The octets of a SourceToTarget-TransparentContainer, as the type that they carry
    let octets = hex::decode("000201020002f8390000012340080100").unwrap();
    let container: SourceNGRANNodeToTargetNGRANNodeTransparentContainer =
        decode_open_type(&octets)?;
    println!("{:?}", container.target_cell_id);
    assert_eq!(encode_open_type(&container)?.as_ref(), &octets[..]);
    Ok(())
}
```

```text
ipv4
0000010086000110
nR_CGI(NRCGI { plmn_identity: PLMNIdentity(208-93), nr_cell_identity: NRCellIdentity(0x1234/36), ie_extensions: None })
```

## Check a message, and answer one that is faulty

`check()` reports how a message deviates from the object set that the ASN.1
gives it, and refuses nothing.

```rust
use oxirush_ngap::{build_ngap, ngap::*};

fn main() -> Result<(), oxirush_ngap::Error> {
    // A UE CONTEXT RELEASE REQUEST that deviates in each way, built all the same
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        REJECT, UEContextReleaseRequest,
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE AMF_UE_NGAP_ID(42u64),
        IGNORE TimeToWait(TimeToWait::v5s),
        REJECT RAN_UE_NGAP_ID(8u32),
    )?;

    // It is encoded and decoded as any other, and `check()` reports
    let received = NGAP_PDU::decode(&pdu.encode()?)?;
    for deviation in received.check()? {
        println!("{deviation}");
    }

    // The message of the first example deviates in nothing
    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    assert!(NGAP_PDU::decode(&bytes)?.check()?.is_empty());
    Ok(())
}
```

```text
the criticality of the PDU is reject, and that of its procedure is ignore
AMF-UE-NGAP-ID (IE 10), at position 1, comes after RAN-UE-NGAP-ID (IE 85), which the object set of the message has after it
the criticality of AMF-UE-NGAP-ID (IE 10), at position 1, is ignore, and the message assigns it reject
TimeToWait (IE 107), at position 2, is not an IE of the message
RAN-UE-NGAP-ID (IE 85), at position 3, is in the message already, at position 0
Cause (IE 15) is mandatory, and the message does not have it
```

| `Deviation` | Is |
|-------------|----|
| `PduCriticality`, `IeCriticality` | A criticality of the PDU or of an IE that is not the assigned one |
| `UnexpectedIe`, `RepeatedIe`, `OutOfOrder`, `MissingIe` | An IE that the message cannot have, has twice, has in a wrong order, or lacks |
| `UndecodableIe` | An IE whose value does not decode as its type |
| `OutsideRoot` | A value of a later version of its type, outside the root of an extensible constraint |
| `NotCanonical` | A value that is whole in another form than the one that ITU-T X.691 gives it: a `Fault`, with its clause and its bit |
| `UnknownProcedure` | A procedure that the specification does not have in the direction of the PDU, as one that a later release adds |

`check()` decodes the value of each IE, not the IEs that a value holds as
octets, and does not judge a conditional IE: its condition is prose.

`check_structure()` compares the container alone with the object set: the
presence, the order, the repetition and the criticality of the IEs, which is
what TS 38.413 clause 10.3 has a receiver answer, and decodes no value. A
receiver that checks the structure of a message and reads the IEs that it
needs, with the macro or `ie`, pays for those IEs alone: decoding an IE checks
its value, and one that does not decode is an error of the kind
`UndecodableIe` there, which `Deviation::of` makes a deviation of. `check()`
is for a tool that looks at every value: it costs 6 to 11 times as much
([`benches/`](benches/README.md)).


Clause 10 of TS 38.413 has what the receiver of a message that deviates
answers: `cause()` of a deviation is the protocol cause, and
`criticality_diagnostics_for_response()` of the PDU the Criticality Diagnostics
IE of a response, with an item for each IE of the criticality `reject` or
`notify` (`criticality_diagnostics()` is that of an ERROR INDICATION).

```rust
use oxirush_ngap::{Deviation, build_ngap, ngap::*};

fn main() -> Result<(), oxirush_ngap::Error> {
    // An NG SETUP REQUEST with none of its mandatory IEs but the paging DRX
    let request = build_ngap!(InitiatingMessage, NGSetup, REJECT, NGSetupRequest,
        IGNORE DefaultPagingDRX(PagingDRX::v128),
    )?;
    let received = NGAP_PDU::decode(&request.encode()?)?;
    let deviations = received.check()?;

    // Clause 10.3.5: the procedure is rejected, and the missing IEs are reported
    let cause = deviations.iter().find_map(Deviation::cause).unwrap();
    let failure = build_ngap!(UnsuccessfulOutcome, NGSetup, REJECT, NGSetupFailure,
        IGNORE Cause(Cause::protocol(cause)),
        IGNORE CriticalityDiagnostics(received.criticality_diagnostics_for_response(&deviations)),
    )?;
    for deviation in &deviations {
        println!("{deviation}");
    }
    let octets = hex::encode(failure.encode()?);
    println!("{octets}");
    Ok(())
}
```

```text
GlobalRANNodeID (IE 27) is mandatory, and the message does not have it
SupportedTAList (IE 102) is mandatory, and the message does not have it
40150015000002000f40016200134009180100001b40006640
```

## A PDU as JSON

With the `inspect` feature a PDU is JSON in two forms: a tree, in which a
message is shown and written, and a view, which keeps what was received and
is read and edited by paths with the functions of the [`inspect`] module.
`/ngap` is the IEs of the message, and an IE goes by its name.

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use oxirush_ngap::tree;

// A UE CONTEXT RELEASE REQUEST, as a tree
let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
println!("{}", pdu.to_tree());

// Its view: each value with the path that selects it
let mut view = pdu.to_view().unwrap();
for (path, value) in inspect::paths(&view) {
    println!("{path} = {value}");
}

// A value, and another alternative of a CHOICE in place of the one that is there
inspect::set(&mut view, "/ngap/RAN-UE-NGAP-ID/value", tree!(9)).unwrap();
inspect::set(&mut view, "/ngap/Cause/nas", tree!("Normal Release")).unwrap();
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree());

// A name that the type does not have is refused, and the view is as it was
let wrong = inspect::set(&mut view, "/ngap/Cause/radioNetwrok", tree!("unspecified"));
println!("{}", wrong.unwrap_err());
```

```text
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
/procedure_code = 42
/direction = "InitiatingMessage"
/criticality = "ignore"
/ngap/AMF-UE-NGAP-ID/id = 10
/ngap/AMF-UE-NGAP-ID/criticality = "reject"
/ngap/AMF-UE-NGAP-ID/value = 42
/ngap/RAN-UE-NGAP-ID/id = 85
/ngap/RAN-UE-NGAP-ID/criticality = "reject"
/ngap/RAN-UE-NGAP-ID/value = 7
/ngap/Cause/id = 15
/ngap/Cause/criticality = "ignore"
/ngap/Cause/value/radioNetwork = "user-inactivity"
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":9},{"Cause":{"nas":"normal-release"}}]}
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions; an IE itself has id, criticality, value, octets, at /ngap/Cause/radioNetwrok
```

The [`inspect`] module has examples of the other tasks: add or remove an IE,
a repeated IE, an item of a list, a transfer, the names of an ENUMERATED, a
path checked before use, the container of a handover; and the rules of the
paths, of the edits and of the values.

## Write a message from its tree

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use oxirush_ngap::tree;

// A tree: the name of the message, whatever its case and its separators, then its IEs
let request = tree!({
    "message": "ue context release request",
    "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}},
        // An IE that the message does not have says its criticality
        {"RANNodeName": "gnb-1", "criticality": "ignore"},
    ],
});

let pdu = NGAP_PDU::from_tree(&request).unwrap();
println!("{}", hex::encode(pdu.encode().unwrap()));

// The name gave the direction, the procedure code and the criticalities
let view = pdu.to_view().unwrap();
println!("{} {} {}", view["direction"], view["procedure_code"], view["criticality"]);
println!("{}", tree!(inspect::select(&view, "/ngap/*/criticality").unwrap()));
```

```text
002a4020000004000a0002002a005500020007000f40020500005240070200676e622d31
"InitiatingMessage" 42 "ignore"
["reject","reject","ignore","ignore"]
```

An IE may be its octets, whatever they are:

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use oxirush_ngap::tree;

// `{"octets": "…"}` in place of a value is the octets of the IE, whatever they are
let request = tree!({"message": "UEContextReleaseRequest", "ies": [
    {"AMF-UE-NGAP-ID": 42},
    {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
]});
let pdu = NGAP_PDU::from_tree(&request).unwrap();
println!("{}", hex::encode(pdu.encode().unwrap()));

// A view shows an IE that does not decode by its octets, and says why
let mut view = pdu.to_view().unwrap();
println!("{}", view["message"]["protocolIEs"][1]);

// In a view, the `octets` of an IE are set in place of its value
inspect::set(&mut view, "/ngap/AMF-UE-NGAP-ID/octets", tree!("00ff")).unwrap();
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree());
```

```text
002a4010000002000a0002002afde80003c0ffee
{"id":65000,"criticality":"reject","value":"c0ffee","_decode_error":"NGAP IE 65000 is unknown","_raw_value":"c0ffee","_original_id":65000}
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":255},{"65000":{"octets":"c0ffee"},"criticality":"reject"}]}
```

## Any format of serde

With the `serde` feature, a PDU is serialized as its tree and deserialized
from one, in any format that serde has, and each other type of the bindings
as the tree of a PDU shows it.

```rust
use oxirush_ngap::ngap::{NGAP_PDU, SNSSAI, SST};

let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
let text = serde_json::to_string(&pdu).unwrap();
let read: NGAP_PDU = serde_json::from_str(&text).unwrap();
assert_eq!(read.encode().unwrap(), bytes);

// A value alone has the form that a tree shows it in: an SST is a number
let slice = SNSSAI::new(SST([1]), None, None);
println!("{}", serde_json::to_string(&slice).unwrap());
```

```text
{"sST":1}
```

## Limits

- Procedures, state and transport: it is a codec.
- Judge a conditional IE: `check()` reports what the object sets say, and a
  condition is prose.
- Keep what a later release added to a value that is decoded as its type and
  encoded again: a PDU keeps the octets of its IEs until one is decoded, and
  a tree and a view keep them.
- Write or read the JSON Encoding Rules (ITU-T X.697): a tree is the JSON of
  the crate, by the names of the specification. The tests compare it with the
  JER of rasn, and read the values of pycrate in those rules, through
  functions that the documentation hides.
- Look into an RRC container, or into the container of a handover for
  another system than NG-RAN: they stay octets.
- The crate has no limit on the octets of a PDU, and each call takes time and
  memory in proportion to them, a view up to 143 octets for an octet
  ([`to_view`]): a receiver bounds what a peer makes it spend by what it reads
  from SCTP.
- The bindings are those of TS 38.413 V19.4.0 (Release 19), from the archive
  `38413-j40.zip`, in aligned PER (ITU-T X.691 (02/2021)).
- No PDU of a commercial RAN or core is among the fixtures: what such equipment
  sends that open-source nodes did not is untested.

## Performance

On one core of an AMD Ryzen 7 5700G (16 threads, Linux 7.0, Rust 1.98.1),
measured on a quiet machine on 2026-10-18, the median of three runs: the
macro on a PDU reads the IEs that its fields ask for in 24 to 52 ns, and
`ie::<T>()` one IE in 27 to 64 ns, without an allocation; `check_structure()`
takes 64 to 198 ns, and `check()` 6 to 11 times as much.
[`benches/`](benches/README.md) has these figures by message, every figure
of the comparison for HandoverRequest, the clean builds and a message of
65,535 IEs.

### Against other implementations

The crate against five other implementations of NGAP, on the six PDUs of
`benches/compare/pdus.tsv`, which each decodes and encodes. The envelope alone
is `NGAP_PDU::decode`, which leaves the value of each IE in its octets; "this
crate" decodes the PDU and then every IE of its message as its type, which is
what the others do; and encoding writes the PDU from the typed values of its
IEs, as `build_ngap!` does.

Decoding, ns per call, and the ratio to this crate:

| PDU (octets) | this crate, the envelope alone | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | free5GC ngap 1.2.0 | pycrate 0.8.1 |
|---|---|---|---|---|---|---|---|
| UEContextReleaseRequest (25) | 8 | 88 | 636 (7.2x) | 594 (6.7x) | 1,670 (18.9x) | 1,150 (13.0x) | 68,916 (778.7x) |
| InitialUEMessage (70) | 8 | 151 | 1,389 (9.2x) | 1,706 (11.3x) | 2,797 (18.5x) | 1,878 (12.4x) | 127,498 (842.1x) |
| InitialContextSetupRequest (212) | 10 | 495 | 3,628 (7.3x) | 4,364 (8.8x) | 6,486 (13.1x) | 4,484 (9.1x) | 431,384 (871.0x) |
| NGSetupResponse (58) | 8 | 316 | 1,881 (6.0x) | 2,324 (7.4x) | 3,532 (11.2x) | 2,354 (7.5x) | 172,873 (547.9x) |
| PDUSessionResourceSetupRequest (192) | 10 | 180 | 1,230 (6.8x) | 1,805 (10.0x) | 3,177 (17.6x) | 2,099 (11.6x) | 267,008 (1481.7x) |
| HandoverRequest (238) | 8 | 525 | 3,948 (7.5x) | 5,017 (9.5x) | 7,324 (13.9x) | 4,993 (9.5x) | 465,894 (886.7x) |

Encoding, ns per call, and the ratio to this crate:

| PDU (octets) | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | free5GC ngap 1.2.0 | pycrate 0.8.1 |
|---|---|---|---|---|---|---|
| UEContextReleaseRequest (25) | 63 | 889 (14.0x) | 1,314 (20.8x) | 898 (14.2x) | 578 (9.1x) | 45,156 (713.4x) |
| InitialUEMessage (70) | 99 | 1,265 (12.8x) | 2,640 (26.6x) | 1,474 (14.9x) | 888 (9.0x) | 77,259 (779.6x) |
| InitialContextSetupRequest (212) | 251 | 2,638 (10.5x) | 6,159 (24.5x) | 3,352 (13.4x) | 2,021 (8.0x) | 288,997 (1150.9x) |
| NGSetupResponse (58) | 128 | 1,453 (11.4x) | 3,048 (23.8x) | 1,492 (11.7x) | 964 (7.5x) | 106,966 (835.7x) |
| PDUSessionResourceSetupRequest (192) | 119 | 1,347 (11.3x) | 3,054 (25.6x) | 1,987 (16.7x) | 1,128 (9.5x) | 192,710 (1618.1x) |
| HandoverRequest (238) | 277 | 3,053 (11.0x) | 6,941 (25.0x) | 3,819 (13.8x) | 2,295 (8.3x) | 309,128 (1114.8x) |

The time is the fastest of three runs of a call repeated for about 0.2 s, and
the cycles and the instructions those that `perf stat` counts for the calls
alone (the difference between a run of twice the calls and one of the calls).
The allocations and the octets that they ask for are counted by an allocator
that counts in Rust, by `malloc`, `calloc` and `realloc` for asn1c, by
`runtime.MemStats` for Go; pycrate's are not counted. The most memory held is
the high-water mark of the resident memory of the program (`VmHWM`), its runtime
included. The figures were taken on an AMD Ryzen 7 5700G (16 threads, Linux
7.0, Rust 1.98.1, Go 1.27, Python 3.14), each program on one core, measured on
a quiet machine on 2026-10-18: each is the median of three runs of the script,
which fetches each implementation at a pinned version: hampi 0.7.2 and rasn
0.29.0 from crates.io, Open5GS v2.8.0 from GitHub, the ngap package v1.2.0 of
free5GC through its go.mod, pycrate 0.8.1 from PyPI, and the ASN.1 from the
archive of 3GPP. From the root of the repository, with oxirush-aper beside
it, whose `protocols/benches/compare/` has the script, and with what the
script says it needs:

```sh
python3 ../oxirush-aper/protocols/benches/compare/compare.py --core 3 --perf "sudo -n perf"
```

## Tests and fuzzing

The tests read the 144 messages and the 479 vectors that pycrate 0.8.1
encoded from the ASN.1, and 177 PDUs that the AMFs of Open5GS 2.8.0 and
free5GC 4.2.3 and PacketRusher as gNB sent in lab runs, through the codec, the
views and the trees; `oracle/` compares each, cut, lengthened and with changed
bits, with the codec of rasn 0.29, and `protocols/tests/package.sh` of
oxirush-aper runs the tests from the package alone. `fuzz/` has three [cargo-fuzz](https://crates.io/crates/cargo-fuzz)
targets, `decode`, `view` and `paths`, which CI runs for a minute each from the
PDUs of the tests and from each of them under a procedure code that the
specification does not have. `examples/` has four programs, which build,
read, decode by hand and inspect messages.

```bash
cargo test --all-features   # with the --config lines of Installation
cd oracle && cargo test     # against the codec of rasn
../oxirush-aper/protocols/fuzz/seed.sh && RUSTFLAGS="--cfg oxirush_ngap_hooks" cargo +nightly fuzz run view -- -max_total_time=60
cargo run --example ngap_build_pdu
```

The bindings are generated: [CONTRIBUTING.md](CONTRIBUTING.md) says how, how
the vectors were made and the PDUs captured, and what a change to them or to
the pinned revisions takes. A pull request comes with a test for what it
changes; the Rust blocks of this page are checked with what they print
(`tests/readme.rs`, which `protocols/tests/readme.py` of oxirush-aper writes
from this page). The blocks that use other crates of the family run in the
package of [`family/`](family/), which takes those crates at the revisions
that its manifest names. Sign off your commits (`git commit -s`): by
contributing you agree to the [Developer Certificate of
Origin](https://developercertificate.org/).

## The OxiRush family

- [oxirush-utils](https://github.com/linouxis9/oxirush-utils): the 3GPP identities, and the JSON tree with its paths that the others share.
- [oxirush-aper](https://github.com/linouxis9/oxirush-aper): aligned PER, the runtime of the NGAP and S1AP codecs.
- [oxirush-nas](https://github.com/linouxis9/oxirush-nas): the NAS messages of 5GS and EPS.
- [oxirush-security](https://github.com/linouxis9/oxirush-security): NAS security, AKA and the derivation of keys.
- [oxirush-ngap](https://github.com/linouxis9/oxirush-ngap): NGAP, between a gNB and an AMF.
- [oxirush-s1ap](https://github.com/linouxis9/oxirush-s1ap): S1AP, between an eNB and an MME.
- [oxirush-gtp-u](https://github.com/linouxis9/oxirush-gtp-u): GTP-U tunnels, with a test UPF.
- [oxirush-sctp](https://github.com/linouxis9/oxirush-sctp): SCTP sockets, which carry NGAP and S1AP.

## License

Copyright 2025-2026 Valentin D'Emmanuele. Licensed under the Apache License,
Version 2.0: see [LICENSE](LICENSE).

[`inspect`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/index.html
[`to_view`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/ngap/ngap_pdu_descriptions/enum.NGAPPDU.html#method.to_view
