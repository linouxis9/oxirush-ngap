# oxirush-ngap

[![Crates.io](https://img.shields.io/crates/v/oxirush-ngap.svg)](https://crates.io/crates/oxirush-ngap)
[![Documentation](https://docs.rs/oxirush-ngap/badge.svg)](https://docs.rs/oxirush-ngap)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

NGAP, the protocol between a gNB and an AMF (3GPP TS 38.413), in Rust: types
generated from the specification's ASN.1, and macros that build a message
and read its IEs by name.

```rust
use oxirush_ngap::{build_ngap, extract_ngap_ies, ngap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Decode a UE CONTEXT RELEASE REQUEST
    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500")?;
    let pdu = NGAPPDU::decode(&bytes)?;
    println!("{pdu}");

    // Read its IEs by name: `req` fails when the IE is missing, `opt` gives an Option
    extract_ngap_ies!(pdu, UEContextReleaseRequest,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
        opt cause: Cause = Cause(cause),
    );
    println!("AMF UE {amf_id}, RAN UE {ran_id}, cause {cause:?}");

    // Build the same message with another cause, and encode it
    let pdu = build_ngap!(UEContextReleaseRequest,
        AMF_UE_NGAP_ID(42u64),
        RAN_UE_NGAP_ID(7u32),
        Cause(Cause::radioNetwork(CauseRadioNetwork::radio_connection_with_ue_lost)),
    )?;
    println!("{}", hex::encode(pdu.encode()?));
    Ok(())
}
```

```text
UEContextReleaseRequest (procedure 42, initiating message), AMF-UE-NGAP-ID 42, RAN-UE-NGAP-ID 7
AMF UE 42, RAN UE 7, cause Some(radioNetwork(user_inactivity))
002a4015000003000a0002002a005500020007000f40020540
```

Each example is followed by its output.

## Installation

```toml
[dependencies]
oxirush-ngap = "0.5"
# With the JSON forms:
# oxirush-ngap = { version = "0.5", features = ["inspect"] }
# The examples of this page also use:
# hex = "0.4"
```

| Feature | Adds |
|---------|------|
| `inspect` | A PDU as JSON: its tree, and paths to read and edit it |
| `serde` | `Serialize` and `Deserialize` for every type, through its tree. Turns on `inspect` |

The minimum supported Rust version is 1.88. The crate depends on one exact
version of oxirush-aper, whose types (`Octets`, `Bits`, `Error`) the
bindings use. It re-exports it as `oxirush_ngap::aper`.

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
    let area = SupportedTAItem { tac: TAC([0, 0, 1]), broadcast_plmn_list: plmns, ie_extensions: None };
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
    println!("{received}: {}, {} tracking area", name.unwrap_or_default(), areas.0.len());
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
    println!("{response}");

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
    println!("{received}: a NAS-PDU of {} octets", nas.0.len());

    // It gives the UE an AMF UE NGAP ID, and answers with an AUTHENTICATION REQUEST
    let authentication = hex::decode(
        "7e0056000200002123553cbe9637a89d218ae64dae47bf35201055f328b43577b9b94a9ffac354dfafb3",
    )?;
    let downlink = build_ngap!(DownlinkNASTransport,
        AMF_UE_NGAP_ID(1u64),
        RAN_UE_NGAP_ID(ran_id),
        NAS_PDU(authentication),
    )?;
    println!("{downlink}");

    // After the security mode: the UE's GUAMI, slices, security capabilities and KgNB
    let mut k_gnb = [0; 32];
    hex::decode_to_slice("c18166e13cfde1dea842c708f6e0b3372b6896664df4e865c0c78bd270852005", &mut k_gnb)?;
    let setup = build_ngap!(InitialContextSetupRequest,
        AMF_UE_NGAP_ID(1u64),
        RAN_UE_NGAP_ID(ran_id),
        GUAMI(guami(network, 1, 1, 0)?),
        AllowedNSSAI(AllowedNSSAI(vec![AllowedNSSAIItem {
            s_nssai: s_nssai(1, None),
            ie_extensions: None,
        }])),
        UESecurityCapabilities(ue_security_capabilities(0xe0, 0xe0)),
        SecurityKey(k_gnb),
    )?;

    // The gNB checks the message against the specification, and takes the key
    let octets = setup.encode()?;
    let received = NGAPPDU::decode(&octets)?;
    let deviations = received.check()?;
    extract_ngap_ies!(received, InitialContextSetupRequest,
        req key: SecurityKey = SecurityKey(key),
    );
    println!("{received}: {} deviations, KgNB as sent: {}", deviations.len(), key.0.as_octets() == k_gnb);
    Ok(())
}
```

```text
NGSetupRequest (procedure 21, initiating message): gnb-1, 1 tracking area
NGSetupResponse (procedure 21, successful outcome)
InitialUEMessage (procedure 15, initiating message), RAN-UE-NGAP-ID 7: a NAS-PDU of 29 octets
DownlinkNASTransport (procedure 4, initiating message), AMF-UE-NGAP-ID 1, RAN-UE-NGAP-ID 7
InitialContextSetupRequest (procedure 14, initiating message), AMF-UE-NGAP-ID 1, RAN-UE-NGAP-ID 7: 0 deviations, KgNB as sent: true
```

## Decode a PDU and read its IEs

`decode` reads the envelope of a PDU: its direction, its procedure, its
criticality and the octets of its message. An IE is decoded when you read
it: an IE that does not decode fails only the read that asks for it.

```rust
use oxirush_ngap::{NgapPduKind, extract_ngap_ies, ie, ngap::*};

fn receive(octets: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // `decode` copies nothing: the PDU borrows from `octets`
    let pdu = NGAPPDU::decode(octets)?;
    match pdu.kind() {
        NgapPduKind::UEContextReleaseRequest => {
            // Only the IEs that a field asks for are decoded
            extract_ngap_ies!(pdu, UEContextReleaseRequest,
                req amf_id: u64 = AMF_UE_NGAP_ID(id),
                req ran_id: u32 = RAN_UE_NGAP_ID(id),
            );
            // One IE, by its marker type
            let cause = pdu.ie::<ie::Cause>().transpose()?;
            println!("release of AMF UE {amf_id}, RAN UE {ran_id}: {cause:?}");
        }
        NgapPduKind::Other { direction, procedure_code } => {
            println!("procedure {procedure_code} has no {direction}");
        }
        kind => println!("{kind:?} is not handled here"),
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A UE CONTEXT RELEASE REQUEST, then an NG SETUP FAILURE
    receive(&hex::decode("002a4015000003000a0002002a005500020007000f40020500")?)?;
    receive(&hex::decode("40150008000001000f400188")?)?;
    Ok(())
}
```

```text
release of AMF UE 42, RAN UE 7: Some(radioNetwork(user_inactivity))
NGSetupFailure is not handled here
```

| Method | Does |
|--------|------|
| `decode` | Decode one PDU, which borrows from the octets and copies nothing |
| `encode` | Encode it |
| `procedure_code`, `direction`, `kind` | `42`, `Direction::Initiating`, and an `NgapPduKind` (`Other` for a procedure that the specification lacks in that direction) |
| `decode_value`, `message` | The message as its type |
| `protocol_ies`, `set_protocol_ies` | The IEs of the message, whatever it is |
| `ie::<T>`, `ies::<T>` | The first IE of a type, or every one in order. A message and a transfer have them too |

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

| ASN.1 | Rust |
|---|---|
| `id-AMF-UE-NGAP-ID` (an IE) | `AMF_UE_NGAP_ID` |
| `Pre-emptionCapability` (a type) | `PreEmptionCapability` |
| `pLMNIdentity` (a field) | `plmn_identity` |

Leave a criticality out and the macro takes the one that TS 38.413 assigns.
A name that does not exist, or a message that the procedure does not have in
that direction, does not compile.

```rust
use oxirush_ngap::{Message, build_ngap, ngap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The criticalities of the procedure and of the message's object set
    let pdu = build_ngap!(InitialUEMessage,
        RAN_UE_NGAP_ID(7u32),
        NAS_PDU(vec![0x7e, 0x00, 0x41]),
    )?;
    println!("{} {}", pdu.criticality(), hex::encode(pdu.encode()?));

    // The PDU of a message you already have
    let message: InitialUEMessage = pdu.decode_value()?;
    let again = NGAPPDU::of_message(&message, InitialUEMessage::CRITICALITY)?;
    println!("{}", again == pdu);

    // An IE that the message cannot have has no criticality to take
    let error = build_ngap!(InitialUEMessage,
        TimeToWait(TimeToWait::v5s),
    );
    if let Err(error) = error {
        println!("{error}");
    }
    Ok(())
}
```

```text
ignore 000f401100000200550002000700260004037e0041
true
TimeToWait is no IE of the object set of InitialUEMessage, which assigns it no criticality: write its criticality
```

## Transfers and handover containers

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
    println!("{kind:?}");

    // Build one, and check it against its object set
    let ipv6 = PDUSessionResourceSetupRequestTransfer {
        protocol_ies: ProtocolIEContainer(vec![
            ie::PDUSessionType::field_of::<PDUSessionResourceSetupRequestTransfer>(PDUSessionType::ipv6)?,
        ]),
    };
    let octets = oxirush_ngap::encode(&ipv6)?;
    println!("{}", hex::encode(&octets));
    for deviation in PDUSessionResourceSetupRequestTransfer::check(&octets)? {
        println!("{deviation}");
    }

    // A SourceToTarget-TransparentContainer, as the type it carries
    let octets = hex::decode("000201020002f8390000012340080100")?;
    let container: SourceNGRANNodeToTargetNGRANNodeTransparentContainer =
        oxirush_ngap::decode(&octets)?;
    println!("{:?}", container.target_cell_id);
    assert_eq!(oxirush_ngap::encode(&container)?, octets);
    Ok(())
}
```

```text
ipv4
0000010086000110
UL-NGU-UP-TNLInformation (IE 139) is mandatory, and the message does not have it
QosFlowSetupRequestList (IE 136) is mandatory, and the message does not have it
nR_CGI(NRCGI { plmn_identity: PLMNIdentity(208-93), nr_cell_identity: NRCellIdentity(0x1234/36), ie_extensions: None })
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
    for deviation in &deviations {
        println!("{deviation}");
    }

    // The AMF rejects the procedure, and reports the IEs at fault
    let cause = deviations.iter().find_map(Deviation::cause).ok_or("no cause")?;
    let failure = build_ngap!(NGSetupFailure,
        Cause(Cause::protocol(cause)),
        CriticalityDiagnostics(received.criticality_diagnostics_for_response(&deviations)),
    )?;
    println!("{failure}: {}", hex::encode(failure.encode()?));
    Ok(())
}
```

```text
the criticality of the PDU is ignore, and that of its procedure is reject
TimeToWait (IE 107), at position 1, is not an IE of the message
GlobalRANNodeID (IE 27) is mandatory, and the message does not have it
SupportedTAList (IE 102) is mandatory, and the message does not have it
NGSetupFailure (procedure 21, unsuccessful outcome): 40150018000002000f4001620013400c190200006b00001b40006640
```

`check()` decodes every IE. `check_structure()` looks at the container
alone: which IEs are there, in which order, how often and with which
criticality. It costs 6 to 11 times less, and a receiver that then reads the
IEs it needs pays for those alone. Neither looks inside the octets that an IE
carries. `criticality_diagnostics()` gives the diagnostics of an ERROR
INDICATION, the answer where a procedure has no failure message.

## A PDU as JSON

With the `inspect` feature, a PDU is a JSON tree, to print a message or to
write one: the name of the message and its IEs by name. The [`inspect`]
functions read and edit a tree by path, under `/ngap` and the name of the IE.
A tree, and serde's form, carry keys such as KgNB in hexadecimal: do not
log them.

```rust
use oxirush_ngap::{inspect, ngap::NGAPPDU, tree};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A UE CONTEXT RELEASE REQUEST, as a tree
    let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500")?;
    let mut tree = NGAPPDU::decode(&bytes)?.to_tree();
    println!("{tree}");

    // Each value with the path that selects it
    for (path, value) in inspect::paths(&tree) {
        println!("{path} = {value}");
    }

    // Set a value, and pick another alternative of a CHOICE
    inspect::set(&mut tree, "/ngap/RAN-UE-NGAP-ID", tree!(9))?;
    inspect::set(&mut tree, "/ngap/Cause/nas", tree!("Normal Release"))?;
    println!("{}", NGAPPDU::from_tree(&tree)?.to_tree());

    // A misspelt name is refused, and the tree stays as it was
    if let Err(error) = inspect::set(&mut tree, "/ngap/Cause/radioNetwrok", tree!("unspecified")) {
        println!("{error}");
    }
    Ok(())
}
```

```text
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
/ngap/AMF-UE-NGAP-ID = 42
/ngap/RAN-UE-NGAP-ID = 7
/ngap/Cause/radioNetwork = "user-inactivity"
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":9},{"Cause":{"nas":"normal-release"}}]}
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions, at /ngap/Cause/radioNetwrok
```

The [`inspect`] module has the rules of the paths and more examples: adding
and removing IEs, repeated IEs, list items, transfers, ENUMERATED names and
handover containers.

## Write a message from its tree

A tree names the message, in any case, and lists its IEs. The name gives the
direction, the procedure code and the criticalities. An IE can also be
written as its octets, whatever they are, to send a broken message on
purpose. With the `serde` feature, `Serialize` and `Deserialize` go through
the tree, in any format that serde has.

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
    println!("{}", hex::encode(pdu.encode()?));

    // The name gave the procedure and the criticalities, which the tree writes
    // where the message does not assign them
    println!("{pdu}");
    println!("{}", tree!(inspect::select(&pdu.to_tree(), "/ngap/*/criticality")?));
    Ok(())
}
```

```text
002a4027000005000a0002002a005500020007000f40020500005240070200676e622d31fde80003c0ffee
UEContextReleaseRequest (procedure 42, initiating message), AMF-UE-NGAP-ID 42, RAN-UE-NGAP-ID 7
["ignore","reject"]
```

## Limits

- It is a codec: it runs no procedure, keeps no state and carries no
  message.
- `check()` does not judge conditional IEs: their conditions are prose.
- A value that you decode as its type and encode again loses what a later
  release added to it. A PDU keeps the octets of its IEs until you decode
  one, and a tree keeps those of an IE whose value has no place for them.
- The tree is the crate's own JSON, with the names of the specification. It
  is not the JSON Encoding Rules (ITU-T X.697).
- RRC containers, and handover containers of other systems than NG-RAN and
  E-UTRAN, stay octets. Those of E-UTRAN are read by the tables of
  oxirush-s1ap, which a program registers with
  `oxirush_s1ap::inspect::register()`.
- A PDU has no size limit, and time and memory grow with its size: a tree
  takes up to 142 octets per octet ([`to_tree`]). Bound what you read from
  SCTP.
- The bindings are those of TS 38.413 V19.4.0 (Release 19), in aligned PER
  (ITU-T X.691 (02/2021)).
- No PDU of a commercial RAN or core is among the test fixtures.

## Performance

On one core of an AMD Ryzen 7 5700G (Linux 7.0, Rust 1.98.1), at aad0ccf on
2026-10-18, the macro reads the IEs that its fields ask for in 24 to 52 ns
without allocating, and `check_structure()` takes 64 to 198 ns.
[`benches/`](benches/README.md) has the figures by message.

### Against other implementations

Six common PDUs, decoded and encoded by this crate and five other NGAP
codecs, at the same commit and on the same machine. Decoding is the PDU and
each of its IEs as its type, as the others do; `NGAPPDU::decode` alone
takes 8 to 10 ns. A ratio is the time of the other implementation over this
crate's:

| | Decoding | Encoding |
|---|---|---|
| this crate | 88 to 525 ns | 63 to 277 ns |
| rasn 0.29 | 6.0x to 9.2x | 10x to 14x |
| hampi 0.7.2 | 6.7x to 11x | 21x to 27x |
| asn1c (Open5GS 2.8.0) | 11x to 19x | 12x to 17x |
| free5GC ngap 1.2.0 | 7.5x to 13x | 7.5x to 9.5x |
| pycrate 0.8.1 | 550x to 1,500x | 710x to 1,600x |

[`benches/`](benches/README.md#the-six-pdus) has the figures of each PDU and
the method. From the root of the repository, with oxirush-aper beside it:

```sh
python3 ../oxirush-aper/protocols/benches/compare/compare.py --core 3 --perf "sudo -n perf"
```

## Tests and fuzzing

The tests run 144 messages and 479 vectors that pycrate 0.8.1 encoded from
the ASN.1, and 177 PDUs that Open5GS, free5GC and PacketRusher sent in lab
runs, through the codec and the trees. `oracle/` compares the
crate with the codec of rasn 0.29 on each of them, also cut short,
lengthened and with bits flipped. `fuzz/` has six
[cargo-fuzz](https://crates.io/crates/cargo-fuzz) targets: `decode`, `json`,
`paths` and `exact` read any octets, and `edit` and `tree` edit the PDUs of
the tests and write new ones. CI runs each for a minute, and the Fuzz
workflow of oxirush-aper an hour each night. `examples/` has four programs.

```sh
cargo test --all-features
cd oracle && cargo test     # against the codec of rasn
../oxirush-aper/protocols/fuzz/seed.sh && RUSTFLAGS="--cfg oxirush_ngap_hooks" cargo +nightly fuzz run json -- -max_total_time=60
cargo run --example ngap_build_pdu
```

Each Rust block of this page is checked with what it prints, by
`tests/readme.rs`, which `protocols/tests/readme.py` of oxirush-aper writes
from this page. [CONTRIBUTING.md](CONTRIBUTING.md) says how the bindings are
generated, how the vectors were made and the PDUs captured, and what a change
to them or to the pinned revisions takes.

A pull request comes with a test for what it changes. Sign off your commits
(`git commit -s`): by contributing you agree to the [Developer Certificate
of Origin](https://developercertificate.org/). [SECURITY.md](SECURITY.md)
says how to report a vulnerability.

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
[`to_tree`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/ngap/ngap_pdu_descriptions/enum.NGAPPDU.html#method.to_tree
