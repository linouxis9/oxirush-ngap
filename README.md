# oxirush-ngap

[![Crates.io](https://img.shields.io/crates/v/oxirush-ngap.svg)](https://crates.io/crates/oxirush-ngap)
[![Documentation](https://docs.rs/oxirush-ngap/badge.svg)](https://docs.rs/oxirush-ngap)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Decode, build and check NGAP, the protocol between a gNB and an AMF, in Rust
(3GPP TS 38.413 V19.4.0, aligned PER). The types are generated from the
specification's ASN.1, and macros build a message and read its IEs by name.

```rust
use oxirush_ngap::{build_ngap, extract_ngap_ies, ngap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Decode a UE CONTEXT RELEASE REQUEST
    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500")?;
    let pdu = NGAPPDU::decode(&bytes)?;
    assert_eq!(
        pdu.to_string(),
        "UEContextReleaseRequest (procedure 42, initiating message), AMF-UE-NGAP-ID 42, \
         RAN-UE-NGAP-ID 7",
    );

    // Read its IEs by name: `req` fails when the IE is missing, `opt` gives an Option
    extract_ngap_ies!(pdu, UEContextReleaseRequest,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
        opt cause: Cause = Cause(cause),
    );
    assert_eq!((amf_id, ran_id), (42, 7));
    assert_eq!(cause, Some(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)));

    // Build the same message with another cause, and encode it
    let pdu = build_ngap!(UEContextReleaseRequest,
        AMF_UE_NGAP_ID(42u64),
        RAN_UE_NGAP_ID(7u32),
        Cause(Cause::radioNetwork(CauseRadioNetwork::radio_connection_with_ue_lost)),
    )?;
    let octets = "002a4015000003000a0002002a005500020007000f40020540";
    assert_eq!(hex::encode(pdu.encode()?), octets);
    Ok(())
}
```

## Installation

```toml
[dependencies]
oxirush-ngap = { version = "0.8", features = ["inspect"] }
hex = "0.4" # for the examples of this page
```

| Feature | Adds | Needs |
|---------|------|-------|
| `inspect` | A PDU as a JSON tree, with paths to read and edit it | `oxirush-utils` |

The minimum supported Rust version is 1.88. The crate re-exports the
oxirush-aper that it is built on as `oxirush_ngap::aper`.

## Answer a gNB, from NG SETUP to INITIAL CONTEXT SETUP

The AMF's side of a gNB and its first UE. The gNB sets up the NG interface,
then forwards the UE's first NAS message. The AMF answers the UE in a
DOWNLINK NAS TRANSPORT, and once NAS security is on, sends the gNB the UE's
context and KgNB in an INITIAL CONTEXT SETUP REQUEST. NGAP carries each NAS
message as octets, in a NAS-PDU.

```rust
use oxirush_ngap::helpers::*;
use oxirush_ngap::{build_ngap, extract_ngap_ies, ngap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let network = plmn("208", "93")?;
    let slices = || {
        SliceSupportList(vec![SliceSupportItem { s_nssai: s_nssai(1, None), ie_extensions: None }])
    };

    // The gNB: its identity, its name, and the tracking area and slices it serves
    let gnb = GlobalRANNodeID::globalGNB_ID(global_gnb_id(network, 0x123456, 24)?);
    let plmns = BroadcastPLMNList(vec![BroadcastPLMNItem {
        plmn_identity: network,
        tai_slice_support_list: slices(),
        ie_extensions: None,
    }]);
    let area = SupportedTAItem {
        tac: TAC([0, 0, 1]),
        broadcast_plmn_list: plmns,
        ie_extensions: None,
    };
    let request = build_ngap!(NGSetupRequest,
        GlobalRANNodeID(gnb),
        RANNodeName("gnb-1"),
        SupportedTAList(SupportedTAList(vec![area])),
        DefaultPagingDRX(PagingDRX::v128),
    )?;

    // The AMF reads it, and answers with its name, its GUAMI and its slices
    let octets = request.encode()?;
    let received = NGAPPDU::decode(&octets)?;
    extract_ngap_ies!(received, NGSetupRequest,
        opt name: String = RANNodeName(name) => name.0.into_owned(),
        req areas: SupportedTAList = SupportedTAList(areas),
    );
    assert_eq!(received.to_string(), "NGSetupRequest (procedure 21, initiating message)");
    assert_eq!((name.as_deref(), areas.0.len()), (Some("gnb-1"), 1));
    let guamis = ServedGUAMIItem {
        guami: guami(network, 1, 1, 0)?,
        backup_amf_name: None,
        ie_extensions: None,
    };
    let support = PLMNSupportItem {
        plmn_identity: network,
        slice_support_list: slices(),
        ie_extensions: None,
    };
    let response = build_ngap!(NGSetupResponse,
        AMFName("amf-1"),
        ServedGUAMIList(ServedGUAMIList(vec![guamis])),
        RelativeAMFCapacity(RelativeAMFCapacity(255)),
        PLMNSupportList(PLMNSupportList(vec![support])),
    )?;
    assert_eq!(response.to_string(), "NGSetupResponse (procedure 21, successful outcome)");

    // The gNB forwards a REGISTRATION REQUEST, with the cell the UE is in
    let cell = nr_cgi(network, 0x123456, 24, 1)?;
    let location = UserLocationInformationNR {
        nr_cgi: cell,
        tai: tai(network, [0, 0, 1]),
        time_stamp: None,
        ie_extensions: None,
    };
    let registration = hex::decode("7e004179000d0102f8390000000000000010022e08a020000000000000")?;
    let initial = build_ngap!(InitialUEMessage,
        RAN_UE_NGAP_ID(7u32),
        NAS_PDU(registration),
        UserLocationInformation(UserLocationInformation::userLocationInformationNR(location)),
        RRCEstablishmentCause(RRCEstablishmentCause::mo_Signalling),
    )?;

    // The AMF reads the two IEs it needs, in place in the received octets
    let octets = initial.encode()?;
    let received = NGAPPDU::decode(&octets)?;
    extract_ngap_ies!(received, InitialUEMessage,
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
        req nas: NASPDU = NAS_PDU(nas),
    );
    assert_eq!(
        received.to_string(),
        "InitialUEMessage (procedure 15, initiating message), RAN-UE-NGAP-ID 7",
    );
    assert_eq!(nas.0.len(), 29);

    // It gives the UE an AMF UE NGAP ID, and answers with an AUTHENTICATION REQUEST
    let authentication = hex::decode(
        "7e0056000200002123553cbe9637a89d218ae64dae47bf35201055f328b43577b9b94a9ffac354dfafb3",
    )?;
    let downlink = build_ngap!(DownlinkNASTransport,
        AMF_UE_NGAP_ID(1u64),
        RAN_UE_NGAP_ID(ran_id),
        NAS_PDU(authentication),
    )?;
    assert_eq!(
        downlink.to_string(),
        "DownlinkNASTransport (procedure 4, initiating message), AMF-UE-NGAP-ID 1, \
         RAN-UE-NGAP-ID 7",
    );

    // After the security mode: the UE's GUAMI, slices, security capabilities and KgNB.
    // The capabilities are the value of the UE's NAS UE security capability IE:
    // here 5G-EA0 to 128-NEA3 and 5G-IA0 to 128-NIA3
    let mut k_gnb = [0; 32];
    let key = "c18166e13cfde1dea842c708f6e0b3372b6896664df4e865c0c78bd270852005";
    hex::decode_to_slice(key, &mut k_gnb)?;
    let setup = build_ngap!(InitialContextSetupRequest,
        AMF_UE_NGAP_ID(1u64),
        RAN_UE_NGAP_ID(ran_id),
        GUAMI(guami(network, 1, 1, 0)?),
        AllowedNSSAI(AllowedNSSAI(vec![AllowedNSSAIItem {
            s_nssai: s_nssai(1, None),
            ie_extensions: None,
        }])),
        UESecurityCapabilities(ue_security_capabilities(&[0xf0, 0xf0])),
        SecurityKey(k_gnb),
    )?;

    // The gNB checks the message against the specification, and takes the key
    let octets = setup.encode()?;
    let received = NGAPPDU::decode(&octets)?;
    let deviations = received.check()?;
    extract_ngap_ies!(received, InitialContextSetupRequest,
        req key: SecurityKey = SecurityKey(key),
    );
    assert!(deviations.is_empty());
    assert_eq!(key.0.as_octets(), k_gnb);
    Ok(())
}
```

## Decode a PDU and read its IEs

`decode` reads the envelope of a PDU: its direction, its procedure, its
criticality and the octets of its message. An IE is decoded when you read
it: an IE that does not decode fails only the read that asks for it.

```rust
use oxirush_ngap::{NgapPduKind, extract_ngap_ies, ie, ngap::*};

fn receive(octets: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    // `decode` copies nothing: the PDU borrows from `octets`
    let pdu = NGAPPDU::decode(octets)?;
    let handled = match pdu.kind() {
        NgapPduKind::UEContextReleaseRequest => {
            // Only the IEs that a field asks for are decoded
            extract_ngap_ies!(pdu, UEContextReleaseRequest,
                req amf_id: u64 = AMF_UE_NGAP_ID(id),
                req ran_id: u32 = RAN_UE_NGAP_ID(id),
            );
            // One IE, by its marker type
            let cause = pdu.ie::<ie::Cause>().transpose()?;
            format!("release of AMF UE {amf_id}, RAN UE {ran_id}: {cause:?}")
        }
        NgapPduKind::Other { direction, procedure_code } => {
            format!("procedure {procedure_code} has no {direction}")
        }
        kind => format!("{kind:?} is not handled here"),
    };
    Ok(handled)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A UE CONTEXT RELEASE REQUEST, then an NG SETUP FAILURE
    let release = receive(&hex::decode("002a4015000003000a0002002a005500020007000f40020500")?)?;
    assert_eq!(release, "release of AMF UE 42, RAN UE 7: Some(radioNetwork(user_inactivity))");
    let failure = receive(&hex::decode("40150008000001000f400188")?)?;
    assert_eq!(failure, "NGSetupFailure is not handled here");
    Ok(())
}
```

`decode_value()` gives the message as its type, `protocol_ies()` its IEs
whatever it is, and `ie::<T>()` and `ies::<T>()` the first IE of a type or
every one, on a PDU, a message or a transfer.

A field of `extract_ngap_ies!` is `req`, `opt` or `lax`, then its name, its
type and the IE it reads. What follows `=>` is what the field keeps of the
decoded value. An error goes to the enclosing function through `?`.

| Field | Is | When the IE is missing | When it does not decode |
|-------|----|------------------------|-------------------------|
| `req` | A `T` | An error of the kind `MissingIe` | An error of the kind `UndecodableIe` |
| `opt` | An `Option<T>` | `None` | The same error |
| `lax` | An `Option<T>` | `None` | `None` |

A `lax` field reads an IE the way TS 38.413 (10.3.4.2) has a receiver treat
an IE of criticality `ignore`: one that does not decode counts as absent.
When an IE is repeated, the macro and `ie` take the first, and `ies` gives
each. `message.set_ie::<ie::X>(value)` changes one IE of a message in place,
and `ie::X::field(criticality, value)` builds one.

## Build a PDU

The macros name IEs, types and fields after the ASN.1, made into Rust
identifiers:

| ASN.1 | Rust |
|---|---|
| `id-AMF-UE-NGAP-ID` (an IE) | `AMF_UE_NGAP_ID` |
| `Pre-emptionCapability` (a type) | `PreEmptionCapability` |
| `pLMNIdentity` (a field) | `plmn_identity` |

The object set of a message is the list of IEs that the ASN.1 allows in it,
each with its criticality and presence. Leave a criticality out and the macro
takes the one that the object set assigns. A name that does not exist, or a
message that the procedure does not have in that direction, does not
compile.

```rust
use oxirush_ngap::{Criticality, Message, build_ngap, ngap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The criticalities of the procedure and of the message's object set
    let pdu = build_ngap!(InitialUEMessage,
        RAN_UE_NGAP_ID(7u32),
        NAS_PDU(vec![0x7e, 0x00, 0x41]),
    )?;
    assert_eq!(pdu.criticality(), Criticality::Ignore);
    assert_eq!(hex::encode(pdu.encode()?), "000f401100000200550002000700260004037e0041");

    // The PDU of a message you already have
    let message: InitialUEMessage = pdu.decode_value()?;
    let again = NGAPPDU::of_message(&message, InitialUEMessage::CRITICALITY)?;
    assert_eq!(again, pdu);

    // An IE that the message cannot have has no criticality to take
    let error = build_ngap!(InitialUEMessage,
        TimeToWait(TimeToWait::v5s),
    ).unwrap_err();
    assert_eq!(
        error.to_string(),
        "TimeToWait is no IE of the object set of InitialUEMessage, which assigns it no \
         criticality: write its criticality",
    );
    Ok(())
}
```

## Decode transfers and handover containers

Some IEs carry another encoded value in their octets: the transfers of a PDU
session, and the container of a handover (TS 38.413, 9.3.1.20).
`oxirush_ngap::decode` reads them as the type you name, and `oxirush_ngap::encode`
writes them. The macros, `check` and `check_structure` take a transfer as
they take a message.

```rust
use oxirush_ngap::{Ie, ie, extract_ngap_ies, ngap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The transfer of a PDU SESSION RESOURCE SETUP REQUEST, as its item holds it
    let octets = hex::decode("0000010086000100")?;
    let transfer: PDUSessionResourceSetupRequestTransfer = oxirush_ngap::decode(&octets)?;
    extract_ngap_ies!(transfer, PDUSessionResourceSetupRequestTransfer,
        req kind: PDUSessionType = PDUSessionType(kind),
    );
    assert_eq!(kind, PDUSessionType::ipv4);

    // Build one, and check it against its object set
    let ipv6 = PDUSessionResourceSetupRequestTransfer {
        protocol_ies: ProtocolIEContainer(vec![
            ie::PDUSessionType::field_of::<PDUSessionResourceSetupRequestTransfer>(
                PDUSessionType::ipv6,
            )?,
        ]),
    };
    let octets = oxirush_ngap::encode(&ipv6)?;
    assert_eq!(hex::encode(&octets), "0000010086000110");
    let deviations = PDUSessionResourceSetupRequestTransfer::check(&octets)?;
    let deviations: Vec<String> = deviations.iter().map(ToString::to_string).collect();
    assert_eq!(deviations, [
        "UL-NGU-UP-TNLInformation (IE 139) is mandatory, and the message does not have it",
        "QosFlowSetupRequestList (IE 136) is mandatory, and the message does not have it",
    ]);

    // A SourceToTarget-TransparentContainer, as the type it carries
    let octets = hex::decode("000201020002f8390000012340080100")?;
    let container: SourceNGRANNodeToTargetNGRANNodeTransparentContainer =
        oxirush_ngap::decode(&octets)?;
    assert_eq!(
        format!("{:?}", container.target_cell_id),
        "nR_CGI(NRCGI { plmn_identity: PLMNIdentity(208-93), \
         nr_cell_identity: NRCellIdentity(0x1234/36), ie_extensions: None })",
    );
    assert_eq!(oxirush_ngap::encode(&container)?, octets);
    Ok(())
}
```

## Check a message, and answer a faulty one

`check()` compares a message with the object set that the ASN.1 gives it,
and lists every deviation. It refuses nothing, so a faulty message can still
be read and answered. Clause 10 of TS 38.413 says how: `cause()` gives the
protocol cause of a deviation, and `criticality_diagnostics_for_response()`
the Criticality Diagnostics of the response.

```rust
use oxirush_ngap::{Deviation, build_ngap, ngap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // An NG SETUP REQUEST with the wrong criticality, an IE it cannot have,
    // and none of its mandatory IEs but the paging DRX
    let request = build_ngap!(IGNORE NGSetupRequest,
        IGNORE DefaultPagingDRX(PagingDRX::v128),
        REJECT TimeToWait(TimeToWait::v5s),
    )?;
    let octets = request.encode()?;
    let received = NGAPPDU::decode(&octets)?;
    let deviations = received.check()?;
    let listed: Vec<String> = deviations.iter().map(ToString::to_string).collect();
    assert_eq!(listed, [
        "the criticality of the PDU is ignore, and that of its procedure is reject",
        "TimeToWait (IE 107), at position 1, is not an IE of the message",
        "GlobalRANNodeID (IE 27) is mandatory, and the message does not have it",
        "SupportedTAList (IE 102) is mandatory, and the message does not have it",
    ]);

    // The AMF rejects the procedure, and reports the IEs at fault
    let cause = deviations.iter().find_map(Deviation::cause).ok_or("no cause")?;
    let failure = build_ngap!(NGSetupFailure,
        Cause(Cause::protocol(cause)),
        CriticalityDiagnostics(received.criticality_diagnostics_for_response(&deviations)),
    )?;
    assert_eq!(failure.to_string(), "NGSetupFailure (procedure 21, unsuccessful outcome)");
    assert_eq!(
        hex::encode(failure.encode()?),
        "40150018000002000f4001620013400c190200006b00001b40006640",
    );
    Ok(())
}
```

`check()` decodes every IE. `check_structure()` looks at the container
alone: which IEs are there, in which order, how often and with which
criticality. It costs 6 to 11 times less, and a receiver that then reads the
IEs it needs pays for those alone. Neither looks inside the octets that an IE
carries. `criticality_diagnostics()` gives the diagnostics of an ERROR
INDICATION, the answer where a procedure has no failure message.

## Edit a PDU as JSON

With the `inspect` feature, a PDU is a JSON tree, to print a message or to
write one: the name of the message and its IEs by name. The [`inspect`]
functions read and edit a tree by path, under `/ngap` and the name of the IE.
A tree carries keys such as KgNB in hexadecimal: do not log it.

```rust
use oxirush_ngap::{inspect, ngap::NGAPPDU, tree};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A UE CONTEXT RELEASE REQUEST, as a tree
    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500")?;
    let mut tree = NGAPPDU::decode(&bytes)?.to_tree();
    assert_eq!(tree, tree!({"message": "UEContextReleaseRequest", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}},
    ]}));

    // Each value with the path that selects it
    let paths: Vec<String> = inspect::paths(&tree)
        .iter()
        .map(|(path, value)| format!("{path} = {value}"))
        .collect();
    assert_eq!(paths, [
        "/ngap/AMF-UE-NGAP-ID = 42",
        "/ngap/RAN-UE-NGAP-ID = 7",
        r#"/ngap/Cause/radioNetwork = "user-inactivity""#,
    ]);

    // Set a value, and pick another alternative of a CHOICE
    inspect::set(&mut tree, "/ngap/RAN-UE-NGAP-ID", tree!(9))?;
    inspect::set(&mut tree, "/ngap/Cause/nas", tree!("Normal Release"))?;
    let edited = tree!({"message": "UEContextReleaseRequest", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 9},
        {"Cause": {"nas": "normal-release"}},
    ]});
    assert_eq!(NGAPPDU::from_tree(&tree)?.to_tree(), edited);

    // A misspelt name is refused, and the tree stays as it was
    let before = tree.clone();
    let misspelt = inspect::set(&mut tree, "/ngap/Cause/radioNetwrok", tree!("unspecified"));
    let error = misspelt.unwrap_err();
    assert_eq!(
        error.to_string(),
        "\"radioNetwrok\" is not a member of Cause, which has radioNetwork, transport, nas, \
         protocol, misc, choice-Extensions, at /ngap/Cause/radioNetwrok",
    );
    assert_eq!(tree, before);
    Ok(())
}
```

The tree is the crate's own JSON, with the names of the specification, and
not the JSON Encoding Rules (ITU-T X.697).
[The `inspect` module](https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/)
has the rules of the paths and more examples: adding and removing IEs,
repeated IEs, list items, transfers, ENUMERATED names, handover containers,
and `inspect::at`, which names the type at a path and what its tree takes.

## Write a message from its tree

A tree names the message, in any case, and lists its IEs. The name gives the
direction, the procedure code and the criticalities. An IE can also be
written as its octets, whatever they are, to send a broken message on
purpose. A tree implements `Serialize` and `Deserialize`, so any format of
serde writes it and reads it back.

```rust
use oxirush_ngap::{inspect, ngap::NGAPPDU, tree};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = tree!({
        "message": "ue context release request",
        "ies": [
            {"AMF-UE-NGAP-ID": 42},
            {"RAN-UE-NGAP-ID": 7},
            {"Cause": {"radioNetwork": "user-inactivity"}},
            // An IE that the message does not have says its criticality
            {"RANNodeName": "gnb-1", "criticality": "ignore"},
            // An IE that no release has, as its octets
            {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
        ],
    });
    let pdu = NGAPPDU::from_tree(&request)?;
    assert_eq!(
        hex::encode(pdu.encode()?),
        "002a4027000005000a0002002a005500020007000f40020500005240070200676e622d31fde80003c0ffee",
    );

    // The name gave the procedure and the criticalities, which the tree writes
    // where the message does not assign them
    assert_eq!(
        pdu.to_string(),
        "UEContextReleaseRequest (procedure 42, initiating message), AMF-UE-NGAP-ID 42, \
         RAN-UE-NGAP-ID 7",
    );
    let tree = pdu.to_tree();
    let criticalities = inspect::select(&tree, "/ngap/*/criticality")?;
    assert_eq!(criticalities, [&tree!("ignore"), &tree!("reject")]);
    Ok(())
}
```

## Limits

- It is a codec: it runs no procedure, keeps no state and carries no
  message.
- `check()` does not judge conditional IEs: their conditions are prose.
- A value decoded as its type and encoded again loses what a later release
  added to it; a PDU and its tree keep those octets.
- RRC containers, and handover containers of systems other than NG-RAN and
  E-UTRAN, stay octets. E-UTRAN's are read once a program calls
  `oxirush_s1ap::inspect::register()`.
- A PDU has no size limit, and a tree takes up to 142 octets per octet:
  bound what you read from SCTP.

## Performance

| Six common PDUs | This crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | free5GC ngap 1.2.0 | pycrate 0.8.1 |
|---|---|---|---|---|---|---|
| Decoding, every IE as its type | 206 to 973 ns | 5.0–7.0× slower | 5.8–10× slower | 11–18× slower | 7.4–12× slower | 510–1,300× slower |
| Encoding | 158 to 615 ns | 12–15× slower | 14–19× slower | 11–17× slower | 6.9–8.8× slower | 520–1,400× slower |

Medians of ten rounds in turns on one core of an Intel Xeon E5-2680 v4 (a
KVM guest, Linux 7.0, Rust 1.100.0), at e09770b on 2026-11-15. A cell of
another codec says how much slower it is than this crate on the six PDUs;
the ten times of the two overlap for none.
[`benches/README.md`](benches/README.md) has each PDU, the readers and the
checks, the method and the command.

## Tests and fuzzing

```sh
cargo test --all-features
cargo run --example ngap_build_pdu
```

Each example of this page checks what it gives with `assert_eq!`, and runs
as a documentation test. The tests run 144 messages and 479 vectors that
pycrate 0.8.1 encoded from the ASN.1, and 177 PDUs that Open5GS, free5GC and
PacketRusher sent in lab runs, through the codec and the trees, and 16 trees
of each message that the tables make. No PDU of a commercial RAN or core is
among them. `fuzz/` has ten fuzz targets, and `examples/` four programs.

[CONTRIBUTING.md](CONTRIBUTING.md) has how the bindings are generated, the
vectors made and the PDUs captured, how to run the fuzz targets, and how to
contribute. [SECURITY.md](SECURITY.md) says how to report a vulnerability.

## The OxiRush family

- [oxirush-utils](https://github.com/linouxis9/oxirush-utils): the 3GPP identities, the JSON tree with its paths, and the fields and tables that the codecs are laid out on.
- [oxirush-aper](https://github.com/linouxis9/oxirush-aper): aligned PER, the runtime of the NGAP and S1AP codecs.
- [oxirush-nas](https://github.com/linouxis9/oxirush-nas): the NAS messages of 5GS and EPS.
- [oxirush-security](https://github.com/linouxis9/oxirush-security): NAS security, AKA and the derivation of keys.
- [oxirush-ngap](https://github.com/linouxis9/oxirush-ngap): NGAP, between a gNB and an AMF.
- [oxirush-s1ap](https://github.com/linouxis9/oxirush-s1ap): S1AP, between an eNB and an MME.
- [oxirush-gtp-u](https://github.com/linouxis9/oxirush-gtp-u): GTP-U tunnels, with a user-plane test peer.
- [oxirush-pfcp](https://github.com/linouxis9/oxirush-pfcp): PFCP, between the control plane and the user plane.
- [oxirush-sctp](https://github.com/linouxis9/oxirush-sctp): SCTP sockets, which carry NGAP and S1AP.

## License

Copyright 2025-2026 Valentin D'Emmanuele. Licensed under the Apache License,
Version 2.0: see [LICENSE](LICENSE).

[`inspect`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/index.html
