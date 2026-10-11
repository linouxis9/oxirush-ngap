# oxirush-ngap

[![Crates.io](https://img.shields.io/crates/v/oxirush-ngap.svg)](https://crates.io/crates/oxirush-ngap)
[![Documentation](https://docs.rs/oxirush-ngap/badge.svg)](https://docs.rs/oxirush-ngap)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

A Rust codec for NGAP, the NG Application Protocol between an NG-RAN node and
an AMF, per 3GPP TS 38.413. Its types are generated from the ASN.1 of the
specification and encoded in aligned PER with
[`rasn`](https://crates.io/crates/rasn), and macros build a message and read
its IEs by their names.

```rust
use oxirush_ngap::{build_ngap, extract_ngap_ies, ngap::*};

// Decode a UE CONTEXT RELEASE REQUEST
let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
assert_eq!((pdu.procedure_code(), pdu.is_initiating()), (42, true));
println!("{}", pdu.procedure_name());

// Read its IEs by name. An `opt` field is `None` when its IE is not there,
// and what follows `=>` is what the field keeps of the decoded value
let request: UEContextReleaseRequest = pdu.decode_value().unwrap();
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
);

// Encode it
let encoded = hex::encode(rebuilt.encode().unwrap());
assert_eq!(encoded, "002a4015000003000a0002002a005500020007000f40020540");
```

```text
UEContextReleaseRequest
AMF=Some(42) RAN=Some(7) cause=Some(radioNetwork(user_inactivity))
```

That needs no feature, only the `hex` crate for the octets of this page. With
the optional `inspect` feature, and `serde_json` beside it, a PDU is also
JSON, which is read, edited and written by the names of the specification:

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use serde_json::json;

// The same message as a tree: its name and its IEs by name
let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
println!("{}", inspect::message_tree(&pdu).unwrap());

// Its view is edited at a path, and is a PDU again
let mut view = inspect::message_view(&pdu).unwrap();
let lost = json!("radio-connection-with-ue-lost");
inspect::set(&mut view, "/ngap/Cause/radioNetwork", lost).unwrap();
let edited = inspect::message_from_view(&view).unwrap();
println!("{}", inspect::message_tree(&edited).unwrap());

// Its octets are those that the typed example built
let octets = hex::encode(edited.encode().unwrap());
assert_eq!(octets, "002a4015000003000a0002002a005500020007000f40020540");
```

```json
{"ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}],"message":"UEContextReleaseRequest"}
{"ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"radio-connection-with-ue-lost"}}],"message":"UEContextReleaseRequest"}
```

[`oxirush-s1ap`](https://crates.io/crates/oxirush-s1ap) is the same crate for S1AP, TS 36.413,
and its README has the sections of this one.

[Installation](#installation) ·
[Covered](#what-the-crate-covers) ·
[Not covered](#what-the-crate-does-not-cover) ·
[Typed API](#typed-api) ·
[JSON cookbook](#json-trees-views-and-paths) ·
[Rules](#rules) ·
[Examples](#examples) ·
[Specifications](#specifications) ·
[Tests](#tests) ·
[Generated code](#generated-code) ·
[API reference](https://docs.rs/oxirush-ngap)

## Installation

```toml
[dependencies]
oxirush-ngap = "0.5"
# With the optional feature:
# oxirush-ngap = { version = "0.5", features = ["inspect"] }
```

| Feature | Adds | Through |
|---------|------|---------|
| `inspect` | A PDU as a `serde_json::Value`: its tree, in which a message is shown and written as its name and its IEs, its view and the paths of its values | `serde_json` |

No feature is on by default. The minimum supported Rust version is 1.88. The
examples of this page also use the `hex` crate, and `serde_json` for JSON.
The values of the bindings are those of `rasn`, such as `Integer`,
`FixedBitString`, `OctetString` and `Any`, and so are the errors of the
codec. The crate re-exports it as `oxirush_ngap::rasn`, which is where the
examples take these names from: a crate that names them needs no dependency
on `rasn` of its own.

The bindings are checked in and large, and with `inspect` the compiler also
builds JER and APER code for every type of the protocol. A clean
`cargo build -j 8` of the crate and its dependencies, in the `dev` profile
where the row does not say `--release`:

| Build | Time | Peak memory |
|-------|------|-------------|
| Rust 1.88, no feature | 23 s | 1.2 GiB |
| Rust 1.88, `inspect` | 60 s | 6.1 GiB |
| Rust 1.88, `inspect`, `CARGO_PROFILE_DEV_DEBUG=0` | 46 s | 4.3 GiB |
| Rust 1.88, `inspect`, `--release` | 99 s | 4.1 GiB |
| Rust 1.98, `inspect` | 60 s | 5.1 GiB |

Each row is two clean builds of an exported copy of the crate, with a target
directory of its own, on a machine that does nothing else: the mean of their
times, which are within two seconds of each other, and the higher of their
peaks. A peak is the summed resident memory of cargo and its compilers,
sampled four times a second, on an eight-core x86-64 Linux machine. A build of
the tests with every feature and `-j 4` peaked at 5.8 GiB, and at 3.8 GiB
without debug information.

## What the crate covers

| Area | Contents |
|------|----------|
| Bindings | Rust types for the six ASN.1 modules of TS 38.413 V19.4.0, generated with [`rasn-compiler`](https://crates.io/crates/rasn-compiler) and checked in |
| Codec | Aligned PER encoding and decoding through `rasn`, with the corrections of [rasn integration](#rasn-integration) |
| Names | Flat re-exports of the generated types, and the aliases `NGAP_PDU`, `AMF_UE_NGAP_ID` and `RAN_UE_NGAP_ID`. IE identifiers, procedure codes, procedure names, directions and the kinds of a PDU (`NgapPduKind`) come from the ASN.1 |
| Macros | `build_ngap!`, `build_ngap_ie!`, `extract_ngap_ies!` and `with_ngap_ie_mut!`: typed values in and out of the open types |
| Helpers | PLMN, core network, tracking area, cell, radio node and slice identities, strings of bits and UE security capabilities |
| `inspect` | A decoded PDU as a JSON tree, in which a message is shown and written as its name and its IEs, and as a view that is read and edited by paths and encoded again |

## What the crate does not cover

| Subject | Limit |
|---------|-------|
| Procedures | It is a codec: no procedure, no state and no transport. It does not check that the mandatory and conditional IEs of a message are there, nor that a criticality is the one that the specification assigns: the caller does. |
| Unknown additions | A PDU keeps the octets of its open types. A typed value that is decoded and encoded again loses the additions of a later release. |
| RRC, other systems | An RRC container stays octets. The container of a handover is read as the NG-RAN type of TS 38.413 alone ([below](#the-container-of-a-handover)). |
| `inspect`: two values | An IE with an integer of 2^63 or more stays octets beside a `_decode_error`, and so does one with a `BIT STRING` of another size than the one that its extensible type gives, as an algorithm mask of 24 bits: rasn's JER does not represent them. |
| `inspect`: criticalities | The tables have those of the object set of each message of a procedure, and of each single container. Anywhere else an IE says its own, as the [rules](#rules) list. |
| `inspect`: `check_path` | What the types do not say is not refused: the value of an IE selected by `@id=N` whose identifier is unknown or has no type where it is, the entries of a container of a value when they are selected by position, and a segment under such an IE. |

## Typed API

### Decode a PDU

| Of an `NGAP_PDU` | Gives |
|------------------|-------|
| `NGAP_PDU::decode(&bytes)` | The PDU of the octets of one message. Octets that are incomplete, and octets after the message, are an error |
| `encode()` | Its octets, a `Vec<u8>` |
| `procedure_code()`, `procedure_name()` | The code and the name of its procedure: `42` and `"UEContextReleaseRequest"` |
| `direction()` | `"InitiatingMessage"`, `"SuccessfulOutcome"` or `"UnsuccessfulOutcome"`; `is_initiating()`, `is_successful()` and `is_unsuccessful()` say which |
| `kind()` | An `NgapPduKind`: one variant for each message, and `Other` for a procedure that the specification does not have in that direction |
| `decode_value()` | The message, as the type that the caller names |

The kind of a PDU says which message to decode:

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

### Build a PDU

`build_ngap!` takes, in this order:

| Argument | Is |
|----------|----|
| The direction | `InitiatingMessage`, `SuccessfulOutcome` or `UnsuccessfulOutcome` |
| The procedure | Its name, which gives the procedure code |
| The criticality of the PDU | `REJECT`, `IGNORE` or `NOTIFY` |
| The message | The name of its type, such as `UEContextReleaseRequest`: the one that the procedure has in that direction, or the call does not compile |
| Each IE | `CRITICALITY IeName(value)`: the name gives the identifier, and the value is converted with `.into()` |

The name of an IE is its ASN.1 identifier without `id-` and with `_` for `-`:
`id-AMF-UE-NGAP-ID` is `AMF_UE_NGAP_ID`. The name of a type stands for the IE
of that type where one IE alone has it, as `RANUENGAPID` for
`RAN_UE_NGAP_ID`. A name that the crate does not have does not compile, and a
value that does not encode, such as an integer outside its constraint, is a
panic.

A criticality says what a receiver does with what it does not comprehend
(TS 38.413 clause 10.3.2). The specification assigns one to each procedure
and to each IE of a message, in the tables of clause 9.2. The macros write
the one that they are given and check none; with the `inspect` feature,
[`message_from_tree`] writes a message with those that are assigned.

### Read the IEs of a message

```rust
use oxirush_ngap::{extract_ngap_ies, macros::MissingIeError, ngap::*};

fn handle(request: &UEContextReleaseRequest) -> Result<(), MissingIeError> {
    extract_ngap_ies!(request, UEContextReleaseRequest,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
        req cause: Cause = Cause(value) => value,
    );

    println!("AMF={amf_id} RAN={ran_id} cause={cause:?}");
    Ok(())
}
```

A field is `req` or `opt`, its name and its type, then the IE with a name for
its decoded value. After `=>` comes what the field keeps of that value;
without it, the field is the `.0` of the generated newtype.

| Field | Is | When the IE is absent or does not decode |
|-------|----|------------------------------------------|
| `req` | A `T` | The enclosing function returns an `Err` that names the field: a `MissingIeError`, converted as `?` converts one, so the error type of the function is `MissingIeError`, `Box<dyn std::error::Error>` or any type with a `From<MissingIeError>` |
| `opt` | An `Option<T>` | The field is `None` |

### Change or build one IE

```rust
use oxirush_ngap::{build_ngap_ie, ngap::*, with_ngap_ie_mut};

// A message with one IE, which is built alone
let mut message = UplinkNASTransport::new(ProtocolIEContainer(vec![
    build_ngap_ie!(UplinkNASTransport, REJECT AMF_UE_NGAP_ID(1u64)),
]));

// The IE is decoded, changed and encoded again in its place
let updated = with_ngap_ie_mut!(message, UplinkNASTransport,
    AMF_UE_NGAP_ID(id) = 42u64
);
assert!(updated);
```

`build_ngap_ie!` builds one entry of the IEs of a message, a
`ProtocolIEField`. `with_ngap_ie_mut!` decodes one IE, changes it and encodes
it again, and says whether it did: `= value` sets the `.0` of a newtype, and
`=> expression` changes the decoded value and gives the `bool` that the macro
returns. Of an IE that a message has twice, it and `extract_ngap_ies!` take
the same one: the last that decodes.

### Identities and capabilities

```rust
use oxirush_ngap::helpers::*;

// A PLMN from its MCC and its MNC, and what is identified within it
let network = plmn("208", "93");
let amf = guami(network.clone(), 1, 1, 0); // region, set, pointer
let tracking_area = tai(network.clone(), &[0x00, 0x00, 0x01]); // TAC
let cell = nr_cgi(network.clone(), 0x123456, 1); // gNB, cell
let gnb = global_gnb_id(network, 0x123456);

// A string of bits from octets, and the NR algorithms of a UE: the first
// octet of its encryption bitmap, then of its integrity bitmap
let algorithm_mask = bytes_to_bitvec(&[0xe0]);
let capabilities = ue_security_capabilities(&[0xe0, 0xe0]);
```

### Key types

| Type | Description |
|------|-------------|
| `NGAP_PDU` / `NGAPPDU` | The PDU: an initiating message, a successful or an unsuccessful outcome |
| `InitiatingMessage` | Procedure code, criticality, and the message as an open type |
| `ProtocolIEContainer` / `ProtocolIEField` | The IEs of a message: identifier, criticality, and value as an open type |
| `AMFUENGAPID` | 40-bit AMF UE NGAP identifier, a `u64` |
| `RANUENGAPID` | RAN UE NGAP identifier |
| `Cause` | Radio network, transport, NAS, protocol or miscellaneous cause |
| `PLMNIdentity` | Three octets of TBCD |
| `TAI` | PLMN and a TAC of three octets |
| `NRCGI` | PLMN and a 36-bit NR cell identity |
| `SNSSAI` | Slice/service type and optional slice differentiator |
| `NASPDU` | A 5GS NAS message, as octets |

## JSON: trees, views and paths

With the `inspect` feature a PDU is a `serde_json::Value` in two forms. A
tree is the short form, in which a message is shown and written: its name
and its IEs by name. A view keeps what was received, and is read and edited
by paths. The errors of these functions are `String`s.

| | Tree | View |
|---|------|------|
| From a PDU | [`message_tree`] | [`message_view`] |
| An IE is | its name with its value | an entry: its `id`, its `criticality`, its typed `value`, and what was received |
| It has | the name of the message and its IEs, without what the specification assigns | the procedure code, the direction, the criticality and the message, in the ASN.1 JSON encoding |
| Read and edited | as JSON | by paths: [`select`], [`paths`], [`set`], [`insert`], [`remove`], [`open`] |
| Back to a PDU | [`message_from_tree`]: a message written from the tree alone | [`message_from_view`]: what was not edited keeps the octets received |

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};

// UE CONTEXT RELEASE REQUEST
let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();

// Its tree, then its third IE as the view has it
println!("{}", inspect::message_tree(&pdu).unwrap());
let view = inspect::message_view(&pdu).unwrap();
println!("{}", view["message"]["protocolIEs"][2]);
```

```json
{"ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}],"message":"UEContextReleaseRequest"}
{"_ie_name":"Cause","_original_id":15,"_raw_value":"0500","criticality":"ignore","id":15,"value":{"radioNetwork":"user-inactivity"}}
```

The JSON of this page is what the Rust above it prints, and the recipes
start from that message.

### Read an IE

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use serde_json::json;

let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
let view = inspect::message_view(&pdu).unwrap();

// `/ngap` is the IEs of the message, and an IE goes by its name
let select = |path| inspect::select(&view, path).unwrap();
assert_eq!(select("/ngap/RAN-UE-NGAP-ID/value"), [&json!(7)]);
assert_eq!(select("/ngap/Cause/radioNetwork"), [&json!("user-inactivity")]);
assert_eq!(select("/ngap/Cause/octets"), [&json!("0500")]);

// By its position, by its name whatever its case and its separators, by its
// identifier, and each IE with `*`
assert_eq!(select("/ngap/1/value"), select("/NGAP/ran_ue_ngap_id/value"));
assert_eq!(select("/ngap/@id=85/value"), [&json!(7)]);
assert_eq!(select("/ngap/*/criticality").len(), 3);

// Nothing where the message could have a value and has none
assert!(select("/ngap/Cause/nas").is_empty());
assert!(select("/ngap/GUAMI/pLMNIdentity").is_empty());

// Each value with the path that selects it
for (path, value) in inspect::paths(&view) {
    println!("{path} = {value}");
}
```

```text
/criticality = "ignore"
/direction = "InitiatingMessage"
/ngap/AMF-UE-NGAP-ID/criticality = "reject"
/ngap/AMF-UE-NGAP-ID/id = 10
/ngap/AMF-UE-NGAP-ID/value = 42
/ngap/RAN-UE-NGAP-ID/criticality = "reject"
/ngap/RAN-UE-NGAP-ID/id = 85
/ngap/RAN-UE-NGAP-ID/value = 7
/ngap/Cause/criticality = "ignore"
/ngap/Cause/id = 15
/ngap/Cause/value/radioNetwork = "user-inactivity"
/procedure_code = 42
```

### Change a value

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use serde_json::json;

let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
let mut view = inspect::message_view(&pdu).unwrap();

// A value, and another alternative of a CHOICE in place of the one that is there
inspect::set(&mut view, "/ngap/RAN-UE-NGAP-ID/value", json!(9)).unwrap();
inspect::set(&mut view, "/ngap/Cause/nas", json!("Normal Release")).unwrap();
let edited = inspect::message_from_view(&view).unwrap();
println!("{}", inspect::message_tree(&edited).unwrap());

// What was not edited is the octets that were received
let after = inspect::message_view(&edited).unwrap();
let octets = inspect::select(&after, "/ngap/AMF-UE-NGAP-ID/octets").unwrap();
assert_eq!(octets, [&json!("002A")]);

// A name that the type does not have is refused, and the view is as it was
let wrong = inspect::set(&mut view, "/ngap/Cause/radioNetwrok", json!("unspecified"));
println!("{}", wrong.unwrap_err());
```

```text
{"ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":9},{"Cause":{"nas":"normal-release"}}],"message":"UEContextReleaseRequest"}
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions, at /ngap/Cause/radioNetwrok; an IE itself has id, criticality, value, octets
```

### Add or remove an IE

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use serde_json::json;

let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
let mut view = inspect::message_view(&pdu).unwrap();

// An IE is added at the end, by its name and its typed value
let name = json!({"id": "RANNodeName", "criticality": "ignore", "value": "gnb-1"});
inspect::insert(&mut view, "/ngap/-", name).unwrap();

// Before another IE, by its number and its octets
let unknown = json!({"id": 60000, "criticality": "ignore", "octets": "c0ffee"});
inspect::insert(&mut view, "/ngap/Cause", unknown).unwrap();

// An IE is taken out by its name
inspect::remove(&mut view, "/ngap/AMF-UE-NGAP-ID").unwrap();

let edited = inspect::message_from_view(&view).unwrap();
println!("{}", inspect::message_tree(&edited).unwrap()["ies"]);
```

```json
[{"RAN-UE-NGAP-ID":7},{"60000":{"octets":"C0FFEE"},"criticality":"ignore"},{"Cause":{"radioNetwork":"user-inactivity"}},{"RANNodeName":"gnb-1","criticality":"ignore"}]
```

### Write a message from its name and its IEs

```rust
use oxirush_ngap::inspect;
use serde_json::json;

// A tree: the name of the message, whatever its case and its separators, then its IEs
let request = json!({
    "message": "ue context release request",
    "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}},
        // An IE that the message does not have says its criticality
        {"RANNodeName": "gnb-1", "criticality": "ignore"},
    ],
});

let pdu = inspect::message_from_tree(&request).unwrap();
println!("{}", hex::encode(pdu.encode().unwrap()));

// The name gave the direction, the procedure code and the criticalities
let view = inspect::message_view(&pdu).unwrap();
println!("{} {} {}", view["direction"], view["procedure_code"], view["criticality"]);
println!("{}", json!(inspect::select(&view, "/ngap/*/criticality").unwrap()));
```

```text
002a4020000004000a0002002a005500020007000f40020500005240070200676e622d31
"InitiatingMessage" 42 "ignore"
["reject","reject","ignore","ignore"]
```

### Send octets as they are

```rust
use oxirush_ngap::inspect;
use serde_json::json;

// `{"octets": "…"}` in place of a value is the octets of the IE, whatever they are
let request = json!({"message": "UEContextReleaseRequest", "ies": [
    {"AMF-UE-NGAP-ID": 42},
    {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
]});
let pdu = inspect::message_from_tree(&request).unwrap();
println!("{}", hex::encode(pdu.encode().unwrap()));

// A view shows an IE that does not decode by its octets, and says why
let mut view = inspect::message_view(&pdu).unwrap();
println!("{}", view["message"]["protocolIEs"][1]);

// In a view, the `octets` of an IE are set in place of its value
inspect::set(&mut view, "/ngap/AMF-UE-NGAP-ID/octets", json!("00ff")).unwrap();
let edited = inspect::message_from_view(&view).unwrap();
println!("{}", inspect::message_tree(&edited).unwrap());
```

```text
002a4010000002000a0002002afde80003c0ffee
{"_decode_error":"NGAP IE 65000 is unknown","_original_id":65000,"_raw_value":"C0FFEE","criticality":"reject","id":65000,"value":"C0FFEE"}
{"ies":[{"AMF-UE-NGAP-ID":255},{"65000":{"octets":"C0FFEE"},"criticality":"reject"}],"message":"UEContextReleaseRequest"}
```

### A repeated IE

```rust
use oxirush_ngap::inspect;
use serde_json::json;

// A message that has one IE twice
let twice = json!({"message": "UEContextReleaseRequest", "ies": [
    {"AMF-UE-NGAP-ID": 1},
    {"RAN-UE-NGAP-ID": 7},
    {"AMF-UE-NGAP-ID": 2},
]});
let pdu = inspect::message_from_tree(&twice).unwrap();
let mut view = inspect::message_view(&pdu).unwrap();

// The name selects both, in the order of the message
let both = inspect::select(&view, "/ngap/AMF-UE-NGAP-ID/value").unwrap();
assert_eq!(both, [&json!(1), &json!(2)]);

// Each has its position, which is how `paths` names it, and its name may follow
inspect::set(&mut view, "/ngap/2/AMF-UE-NGAP-ID/value", json!(3)).unwrap();

// After a position, the name of another IE is an error that says which IE is there
println!("{}", inspect::select(&view, "/ngap/2/RAN-UE-NGAP-ID/value").unwrap_err());

// The values, each with its path
for (path, value) in inspect::paths(&view) {
    if path.ends_with("/value") {
        println!("{path} = {value}");
    }
}
```

```text
this IE is AMF-UE-NGAP-ID, not RAN-UE-NGAP-ID: "RAN-UE-NGAP-ID" is not a member of this value, which has none, at /ngap/2/RAN-UE-NGAP-ID/value
/ngap/0/value = 1
/ngap/RAN-UE-NGAP-ID/value = 7
/ngap/2/value = 3
```

### An item of a list

```rust
use oxirush_ngap::inspect;
use serde_json::{Value, json};

// A message whose one IE is a list of two items
let request = json!({"message": "InitialContextSetupRequest", "ies": [
    {"AllowedNSSAI": [{"s-NSSAI": {"sST": 1}}, {"s-NSSAI": {"sST": 2, "sD": "000001"}}]},
]});
let pdu = inspect::message_from_tree(&request).unwrap();
let mut view = inspect::message_view(&pdu).unwrap();

// An item by its position, and each of them after `value`
let select = |view, path| -> Vec<Value> {
    let selected = inspect::select(view, path).unwrap();
    selected.into_iter().cloned().collect()
};
assert_eq!(select(&view, "/ngap/AllowedNSSAI/1/s-NSSAI/sD"), [json!("000001")]);
assert_eq!(select(&view, "/ngap/AllowedNSSAI/value/*/s-NSSAI/sST"), [json!(1), json!(2)]);

// An optional member that an item does not have selects nothing
assert!(select(&view, "/ngap/AllowedNSSAI/0/s-NSSAI/sD").is_empty());

// An item is added at the end, and one is taken out
inspect::insert(&mut view, "/ngap/AllowedNSSAI/-", json!({"s-NSSAI": {"sST": 3}})).unwrap();
inspect::remove(&mut view, "/ngap/AllowedNSSAI/0").unwrap();

// `null` takes an optional member out
inspect::set(&mut view, "/ngap/AllowedNSSAI/0/s-NSSAI/sD", Value::Null).unwrap();

let edited = inspect::message_from_view(&view).unwrap();
println!("{}", inspect::message_tree(&edited).unwrap());
```

```json
{"ies":[{"AllowedNSSAI":[{"s-NSSAI":{"sST":2}},{"s-NSSAI":{"sST":3}}]}],"message":"InitialContextSetupRequest"}
```

### A transfer and its members

A transfer is an `OCTET STRING` that contains a type. A view shows its decoded
value beside its octets, and a tree its decoded value alone.

```rust
use oxirush_ngap::inspect;
use serde_json::json;

// A message with a transfer, written as the message that it contains
let request = json!({"message": "PDUSessionResourceSetupRequest", "ies": [
    {"AMF-UE-NGAP-ID": 42},
    {"RAN-UE-NGAP-ID": 7},
    {"PDUSessionResourceSetupListSUReq": [{
        "pDUSessionID": 5,
        "s-NSSAI": {"sST": 1},
        "pDUSessionResourceSetupRequestTransfer": {"protocolIEs": [
            {"PDUSessionType": "ipv4", "criticality": "reject"},
        ]},
    }]},
]});
let pdu = inspect::message_from_tree(&request).unwrap();
let mut view = inspect::message_view(&pdu).unwrap();

// In a view, the transfer is its octets beside its decoded value
let transfer = "/ngap/PDUSessionResourceSetupListSUReq/0/pDUSessionResourceSetupRequestTransfer";
println!("{}", inspect::select(&view, transfer).unwrap()[0]);

// The IEs of the message that it contains go by name in its place
let kind = format!("{transfer}/PDUSessionType/value");
assert_eq!(inspect::select(&view, &kind).unwrap(), [&json!("ipv4")]);

// The same value by the path that says `value` and `decoded`
let long = "/ngap/PDUSessionResourceSetupListSUReq/value/0\
    /pDUSessionResourceSetupRequestTransfer/decoded/PDUSessionType/value";
assert_eq!(inspect::select(&view, long), inspect::select(&view, &kind));

// And by the names that the tree shows, joined: the IE by its position, then its name
let joined = format!("{transfer}/protocolIEs/0/PDUSessionType/value");
assert_eq!(inspect::select(&view, &joined), inspect::select(&view, &kind));

// An edit of the message that it contains is encoded into the transfer
inspect::set(&mut view, &kind, json!("ipv6")).unwrap();
let edited = inspect::message_from_view(&view).unwrap();
println!("{}", inspect::message_tree(&edited).unwrap()["ies"][2]);
```

```json
{"_raw_transfer":"0000010086000100","decoded":{"protocolIEs":[{"_ie_name":"PDUSessionType","_original_id":134,"_raw_value":"00","criticality":"reject","id":134,"value":"ipv4"}]}}
{"PDUSessionResourceSetupListSUReq":[{"pDUSessionID":5,"pDUSessionResourceSetupRequestTransfer":{"protocolIEs":[{"PDUSessionType":"ipv6","criticality":"reject"}]},"s-NSSAI":{"sST":1}}]}
```

### The names of an ENUMERATED

```rust
use oxirush_ngap::inspect;

// The names that the value at a path can have, as the specification spells them
let names = inspect::enumerated_at("UEContextReleaseRequest", "/ngap/Cause/nas").unwrap();
println!("{:?}", names.unwrap());

// A value of another type has none
let choice = inspect::enumerated_at("UEContextReleaseRequest", "/ngap/Cause/value");
assert_eq!(choice, Ok(None));
```

```text
["normal-release", "authentication-failure", "deregister", "unspecified", "uE-not-in-PLMN-serving-area", "mobile-IAB-not-authorized", "iAB-not-authorized", "no-aiot-NAS-response"]
```

A name is written in any case and with any separators, as `"Normal Release"`
above; a tree and a view show it as the specification spells it.

### Check a path before use

```rust
use oxirush_ngap::inspect;

// Whether a path can select anything in a message of a name, without a view
let check = |path| inspect::check_path("UEContextReleaseRequest", path);
assert_eq!(check("/ngap/Cause/nas"), Ok(()));
assert_eq!(check("/ngap/*/radioNetwork"), Ok(()));

// A name that is written wrong, and an IE that the message does not have
for wrong in ["/ngap/Cause/radioNetwrok", "/ngap/RAN-UE-NGAP-ID/valeu", "/ngap/GUAMI"] {
    println!("{}", check(wrong).unwrap_err());
}
```

```text
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions, at /ngap/Cause/radioNetwrok; an IE itself has id, criticality, value, octets
"valeu" is not a member of this value, which has none, at /ngap/RAN-UE-NGAP-ID/valeu; an IE itself has id, criticality, value, octets
"GUAMI" is not an IE of UEContextReleaseRequest, which has AMF-UE-NGAP-ID, RAN-UE-NGAP-ID, PDUSessionResourceListCxtRelReq, Cause, GW-Context-Release-Indication, at /ngap/GUAMI
```

### The messages and their IEs

What ASN.1 says of each message, which `check_path` reads, is also given as
it is, without a view:

| Function | Gives |
|----------|-------|
| [`message_name`], [`message_named`], [`message_names`] | The name that ASN.1 gives the message of a PDU, such as `InitialContextSetupResponse`; the direction and the procedure code of a name; all of them |
| [`message_ies`] | The IEs of the object set of a message: the identifier of each and whether it is mandatory, in the order of the set |
| [`message_ie_criticalities`], [`message_criticality`] | The criticality that the set assigns to each, and that of the procedure |
| [`item_criticality`] | The criticality of an IE that a value holds alone, in a `ProtocolIE-SingleContainer`, when its object sets give it one |
| [`ie_names`] | The identifier and the name of each IE of an object set |

### The container of a handover

TS 38.413 gives the content of three IEs by reference, where the ASN.1 has an
`OCTET STRING`: between NG-RAN nodes they carry a type of this specification,
and for another target system octets of its own. Nothing in a message says
which, so a tree and a view show the octets, and the one that writes or reads
them names the type.

| The octets of | carry a | Clause |
|---------------|---------|--------|
| `SourceToTarget-TransparentContainer` | `SourceNGRANNode-ToTargetNGRANNode-TransparentContainer` | 9.3.1.20 |
| `TargetToSource-TransparentContainer` | `TargetNGRANNode-ToSourceNGRANNode-TransparentContainer` | 9.3.1.21 |
| `TargettoSource-Failure-TransparentContainer` | `TargetNGRANNode-ToSourceNGRANNode-FailureTransparentContainer` | 9.3.1.186 |

```rust
use oxirush_ngap::inspect;
use serde_json::json;

// Written: the name of the type with its value, in place of the octets
let name = "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer";
let required = json!({"message": "HandoverRequired", "ies": [
    {"AMF-UE-NGAP-ID": 42},
    {"RAN-UE-NGAP-ID": 7},
    {"SourceToTarget-TransparentContainer": {name: {
        "rRCContainer": "0102",
        "targetCell-ID": {"nR-CGI": {"pLMNIdentity": "208-93", "nRCellIdentity": 4660}},
        "uEHistoryInformation": [{"lastVisitedCellInformation": {"eUTRANCell": "00"}}],
    }}},
]});

// A tree shows the octets that the container was encoded as
let pdu = inspect::message_from_tree(&required).unwrap();
println!("{}", inspect::message_tree(&pdu).unwrap()["ies"][2]);

// Read: a path goes through the name of the type
let mut view = inspect::message_view(&pdu).unwrap();
let container = format!("/ngap/SourceToTarget-TransparentContainer/{name}");
let cell = format!("{container}/targetCell-ID/nR-CGI/nRCellIdentity");
assert_eq!(inspect::check_path("HandoverRequired", &cell), Ok(()));

// It is an error until `open` decoded the octets, and selects after
assert!(inspect::select(&view, &cell).unwrap_err().contains("open decodes them"));
inspect::open(&mut view, &cell).unwrap();
assert_eq!(inspect::select(&view, &cell).unwrap(), [&json!(4660)]);

// What the octets carry, decoded
let decoded = format!("{container}/decoded");
println!("{}", inspect::select(&view, &decoded).unwrap()[0]);

// An edit goes into it, and what is not edited is sent as it was received
inspect::set(&mut view, &cell, json!(7)).unwrap();
let edited = inspect::message_from_view(&view).unwrap();
println!("{}", inspect::message_tree(&edited).unwrap()["ies"][2]);
```

```text
{"SourceToTarget-TransparentContainer":"000201020002F8390000012340080100"}
{"rRCContainer":"0102","targetCell-ID":{"nR-CGI":{"nRCellIdentity":4660,"pLMNIdentity":"208-93"}},"uEHistoryInformation":[{"lastVisitedCellInformation":{"eUTRANCell":"00"}}]}
{"SourceToTarget-TransparentContainer":"000201020002F8390000000070080100"}
```

The `rRCContainer` stays octets: it is a message of RRC.

### Rules

The documentation of the [`inspect`] module has the full wording, under the
headings that the rows link to.

| Subject | Rule |
|---------|------|
| [Paths] | A JSON pointer into the view, where `/ngap` stands for the IEs of the message and `*` for each entry of a list or each member of a value. |
| An IE | The name that ASN.1 gives it after `id-`, its position (`/ngap/0`) or its identifier (`/ngap/@id=85`); `-` is the end of the list. A name selects each IE of that name. After the position of an IE, its name may follow, so the names and the positions that a tree shows, joined, are a path; that of another IE is an error that says which IE is there. |
| Under an IE | `id`, `criticality`, the typed `value`, and `octets`: what it was received as, in hexadecimal. The members that start with `_` keep what was received. |
| An IE of an extension container | Its typed value is its `extensionValue`. Its type is that of its identifier, or the one that the type which the container extends gives it: `id-CurrentQoSParaSetIndex` is an `AlternativeQoSParaSetIndex` in a QoS flow that is associated with a tunnel and an `AlternativeQoSParaSetNotifyIndex` in one that a notification is about. |
| `value`, `decoded` | May be left out of a path. A segment is first a member of the IE or of the transfer itself: `/ngap/Cause/*` is the members of the IE. A member of a value that has the name of one of them is reached after `value`, and only a string of bits whose size varies has one, its `value`: `value/value`. |
| [Names] | Letters and digits, whatever their case and whatever is between them, for the root, the IEs, the members of a value, the messages and the values of an `ENUMERATED`: `/NGAP/ran_ue_ngap_id` is `/ngap/RAN-UE-NGAP-ID`. |
| [What is not there] | An IE that the message does not have, an `OPTIONAL` member that is absent and another alternative of a `CHOICE` select nothing. A name that the type cannot have is an error, which lists the members of the value, then those of its IE or transfer. |
| [Edits] | `set` writes a member by its ASN.1 name, or an alternative of a `CHOICE` in place of the one that is there. `null` or `remove` takes an optional member out, and `insert` adds before the entry that the path selects, or at `-`. An edit that selects nothing is an error, and the view is as it was. |
| Octets | What is not edited keeps the octets received. An edit is refused when the value around it does not encode back to them, as with an addition of a later release. The value of an IE or of a transfer is not taken out: the IE is removed, or its `octets` are set. |
| An IE that is written in a view | `{"id", "criticality", "value"}`, or `octets` in place of `value`. The `id` is a number or a name, and the value has the type of the IE whatever JSON it is. |
| A transfer that is written | Its decoded value, alone or as `decoded`, or its octets, in hexadecimal or as `octets`. |
| A tree that is written | The name of the message gives the procedure code, the direction and the criticalities. An IE says its `criticality` where no table has it: when the message does not have it, in a message that a transfer contains, and in an extension container. |
| What does not decode | An IE or a transfer that is unknown, does not decode, has a value that JER does not represent or is nested deeper than 64 levels stays octets beside a `_decode_error`, and the rest of the PDU is read. |
| [Octets that carry a type] | The container of a handover is written as `{NAME: VALUE}` by the name of its type, and read once [`open`] decoded it in the view. |
| [`check_path`], [`enumerated_at`] | Read a path against the types of a message of a name, without a view: under `/ngap`, the IE is one of the object set of the message. |

### Values

A tree and a view show the values of some types as they are usually
written, and take them so.

| Type | Notation |
|------|----------|
| `PLMNIdentity` | `"208-93"`, the MCC and the MNC |
| `TransportLayerAddress` | `"10.0.0.1"`, `"2001:db8::1"`, or the two with a comma |
| `TAC`, `EPS-TAC`, `LAC`, `GTP-TEID`, `FiveG-TMSI`, `SST`, `PortNumber`, `AMFRegionID`, `AMFSetID`, `AMFPointer`, `NRCellIdentity`, `EUTRACellIdentity`, `UL-NAS-Count` | a number, also read as a `"0x…"` string |
| `ENUMERATED` | the name of the value, read in any case |
| a string of bits whose size varies, as the identifier of a gNB | `{"value": "000004", "length": 22}`: the octets that hold it, in hexadecimal, and its length in bits |
| the other strings: an SD, keys and algorithm masks, the NAS-PDU and the other containers | hexadecimal, read in either case |

Each is also read as JER writes it, and a value that does not fit its
notation, such as a transport layer address of another length, stays as JER
writes it. The octets of a string of bits start with its first bit: `"C000"`
is the first two bits of sixteen, and `"000004"` the number 1 in 22 bits.

## Examples

| Example | Shows | Feature |
|---------|-------|---------|
| `build_pdu` | Initial Context Setup request and response, UE Context Release, Handover Required and Acknowledge, NG Setup Failure, and one IE built alone | |
| `decode_manually` | An NG Setup Response built and read through rasn's APER API | |
| `extract_ies` | UE release, handover, and nested PDU session and NAS data read from decoded messages | |
| `inspect` | An NG Setup Request: its values with their paths, one IE edited, one added by its value and one by its octets, and the PDU encoded again | `inspect` |

```bash
cargo run --example build_pdu
cargo run --example inspect --features inspect
```

Each has its counterpart in `oxirush-s1ap`, with the messages and the IEs of
S1AP.

## Specifications

| Specification | Version | For |
|---------------|---------|-----|
| 3GPP TS 38.413 | V19.4.0 (Release 19), `38413-j40.zip` | NGAP: the six ASN.1 modules of clause 9.4, and clause 9.3 for what the ASN.1 leaves to prose |
| ITU-T X.691 | (02/2021) | Aligned PER |

## Tests

| Test | What it holds |
|------|---------------|
| `tests/aper_vectors.rs` | The 144 messages of `tests/fixtures/messages.tsv`, one for each message of each procedure, were encoded by pycrate 0.8.1. Each decodes to its type and encodes to the same octets, and each of its truncations and one more octet are refused. |
| `tests/aper_round_trip.rs` | The macros, and the encodings where the codec departs from rasn's: constrained lists, `UTF8String`, long strings of bits. |
| `tests/unknown_extensions.rs` | The additions of a later release to an extensible `SEQUENCE` are skipped, and what follows them is read. |
| `tests/allocation.rs` | Decoding reserves memory for the elements that it reads, not for the count that a length determinant claims. |
| `tests/inspection.rs` | Each of the 144 messages is the same octets through its view, and through its tree where it has IEs to name. Each path of each selects its value, with and without `value` and `decoded`, and is one that `check_path` takes. Edits, written trees, criticalities, the two IEs that have the type of what they extend, and the identifiers of the registry against the constants of the bindings. |
| `tests/shared_files.rs` | The files that this crate shares with `oxirush-s1ap` are as they were last mirrored. |
| The unit tests of the paths | No two names that a path takes can be taken for each other, and a value has no member that is named as one of its IE, but the `value` of a string of bits. |
| The Rust blocks of this page | They compile and run as documentation tests, with the `inspect` feature, and `tests/readme.rs` compares what each prints with the output shown under it. |

```bash
cargo test --all-features
cargo test
```

## Generated code

Cargo compiles three checked-in files. A build, docs.rs and the verification
of a package run no generator, and the published crate has neither the
generator nor the ASN.1.

| File | Contents |
|------|----------|
| `src/ngap.rs` | The bindings of `rasn-compiler`, with the flat API, the convenience methods and the `Display` implementation that the generator adds |
| `src/registry.rs` | The list of the procedures and of the IEs, one line for each: its code or its identifier, its names and its types, and for each message of a procedure the IEs of its object set. The names that the macros take, `NgapPduKind` and the names and the types of `inspect` all expand from it, so each is written once |
| `src/inspect_registry.rs` | What `inspect` alone needs, with the `inspect` feature: among it the list of the types, one line for each `SEQUENCE` and each `CHOICE` with its members, which the paths are read against, and the types that a handover's containers carry, which the generator has from the prose of the specification |

For maintainers, `build/` holds the generator, a package of its own whose
lockfile pins `rasn-compiler`. It reads the six `.asn` modules in `ngap/`,
which Git ignores: the ASN.1 of clause 9.4 of TS 38.413 V19.4.0, one file per
module, taken out of the document of the `38413-j40.zip` archive. The
repository has no step that extracts them. From the crate directory:

```sh
CARGO="$(command -v cargo)" cargo run --locked --manifest-path build/Cargo.toml
rustfmt --edition 2024 src/ngap.rs src/registry.rs src/inspect_registry.rs
```

`rasn-compiler` finds rustfmt through `CARGO`. Commit the three files after
regenerating them, and do not edit them by hand. They were last regenerated
on 2026-10-11, with the generator of that revision and the rustfmt of Rust
1.88: they came out as they are committed, octet for octet. No CI job
regenerates them, as the ASN.1 is not in the repository.

oxirush-ngap and oxirush-s1ap share, octet for octet, the code that does not
depend on the protocol: `build/aper_fix.rs`, `build/containers.rs`,
`build/inspection.rs`, `build/registry.rs`, `src/inspect_paths.rs`,
`src/per.rs`, `src/sized.rs` and the test `tests/readme.rs`.
`tests/shared_files.rs` holds a digest of each, so that an edit in one crate
fails its tests until the file is the same in the other and both record the
new digest.

### rasn integration

`rasn-compiler` runs in its stable mode, which leaves an open type opaque:
TS 38.413 relies on parameterized information object classes, and the
experimental typed mode emits unresolved object sets and bindings that do not
compile for these modules. `Any` stays inside the generated types, and the
macros give typed construction, extraction and mutation.

| Subject | What the crate does |
|---------|---------------------|
| Containers | The protocol IE and extension containers take an object set as a parameter, which an opaque open type does not use. Each is one type, `ProtocolIEContainer` of `ProtocolIEField` and `ProtocolExtensionContainer` of `ProtocolExtensionField`, in place of the one that `rasn-compiler` resolves for each use. |
| rasn features | `bytes` storage is on; `f32` and `f64`, which nothing uses, are off. |
| Complete values | `NGAP_PDU::decode`, `decode_value` and the extraction of a typed IE refuse incomplete values and trailing whole octets. |

rasn 0.28's APER codec departs from ITU-T X.691 where NGAP reaches it. There
the generator replaces the derived codec with `Encode` and `Decode`
implementations written against rasn's public codec traits; every other value
uses the derived codec unchanged.

| Value | Correction |
|-------|------------|
| Constrained `SEQUENCE OF` | rasn misaligns the length determinant and the components |
| Size-constrained `UTF8String` | Its size is not PER-visible, so its length is an unconstrained count of octets |
| Fixed-size `BIT STRING` longer than 16 bits | It is octet-aligned |
| `OCTET STRING` and `BIT STRING` with a two-octet length determinant | Components and alternatives take the types of the `sized` module |
| Extensible `SEQUENCE` | Its decoder consumes and skips the additions that it does not know |

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Sign off your commits (`git commit -s`)
4. Open a Pull Request

### Developer Certificate of Origin (DCO)

By contributing to this project, you agree to the [Developer Certificate of Origin (DCO)](https://developercertificate.org/). This means that you have the right to submit your contributions and you agree to license them according to the project's license.

All commits should be signed-off with `git commit -s` to indicate your agreement to the DCO.

## License

Copyright 2025-2026 Valentin D'Emmanuele

Licensed under the Apache License, Version 2.0. See [LICENSE](https://github.com/linouxis9/oxirush-ngap/blob/master/LICENSE) for details.

[`inspect`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/index.html
[Paths]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/index.html#paths
[Names]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/index.html#names
[What is not there]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/index.html#what-is-not-there
[Edits]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/index.html#edits
[Octets that carry a type]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/index.html#octets-that-carry-a-type
[`message_view`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.message_view.html
[`message_from_view`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.message_from_view.html
[`message_tree`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.message_tree.html
[`message_from_tree`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.message_from_tree.html
[`select`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.select.html
[`paths`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.paths.html
[`set`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.set.html
[`insert`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.insert.html
[`remove`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.remove.html
[`open`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.open.html
[`check_path`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.check_path.html
[`enumerated_at`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.enumerated_at.html
[`message_name`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.message_name.html
[`message_named`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.message_named.html
[`message_names`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.message_names.html
[`message_ies`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.message_ies.html
[`message_ie_criticalities`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.message_ie_criticalities.html
[`message_criticality`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.message_criticality.html
[`item_criticality`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.item_criticality.html
[`ie_names`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/inspect/fn.ie_names.html
