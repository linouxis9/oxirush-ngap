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

That needs no feature, only the `hex` crate for the octets of this page. With
the optional `inspect` feature, and `serde_json` beside it, a PDU is also
JSON, which is read, edited and written by the names of the specification:

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use serde_json::json;

// The same message as a tree: its name and its IEs by name
let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
println!("{}", pdu.to_tree().unwrap());

// Its view is edited at a path, and is a PDU again
let mut view = pdu.to_view().unwrap();
let lost = json!("radio-connection-with-ue-lost");
inspect::set(&mut view, "/ngap/Cause/radioNetwork", lost).unwrap();
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree().unwrap());

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
[Benchmarks](#benchmarks) ·
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

No feature is on by default. The examples of this page also use the `hex`
crate, and `serde_json` for JSON. The values of the bindings are those of
`rasn`, such as `Integer`, `FixedBitString`, `OctetString` and `Any`. The
crate re-exports it as `oxirush_ngap::rasn`, which is where the examples take
these names from: a crate that names them needs no dependency on `rasn` of
its own. The errors of the codec are the crate's own: see [Errors](#errors).

The minimum supported Rust version is 1.88. A project that is built with
Rust 1.88 needs a resolver that looks at the Rust version, which edition
2024 has and `resolver = "3"` in its `Cargo.toml` gives an older edition:
the default resolver of edition 2021 takes a `uuid` that needs Rust 1.89,
through the macros of `rasn`, and the build stops with
`uuid@1.28.0 requires rustc 1.89.0`.

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
| Any message | `protocol_ies()` and `set_protocol_ies()` read and replace the IEs of a PDU whatever its message is, and `check()` reports how a message deviates from its object set: a mandatory IE that is missing, an IE that it cannot have or has twice, the order of its IEs and the criticalities |
| Helpers | PLMN, core network, tracking area, cell, radio node and slice identities, bit strings and UE security capabilities |
| `inspect` | A decoded PDU as a JSON tree, in which a message is shown and written as its name and its IEs, and as a view that is read and edited by paths and encoded again |

## What the crate does not cover

| Subject | Limit |
|---------|-------|
| Procedures | It is a codec: no procedure, no state and no transport. `check()` reports a mandatory IE that a message lacks, an IE that it cannot have or has twice, the order of its IEs and a criticality that is not the assigned one ([below](#check-a-message-against-its-object-set)), and nothing is refused for them. It does not know when a conditional IE is due, and it does not look into the values of the IEs. |
| Unknown additions | A PDU keeps the octets of its open types. A typed value that is decoded and encoded again loses the additions of a later release. |
| RRC, other systems | An RRC container stays octets. The container of a handover is read as the NG-RAN type of TS 38.413 alone ([below](#the-container-of-a-handover)). |
| `inspect`: two values | An IE with an integer of 2^63 or more stays octets beside a `_decode_error`, and so does one with a `BIT STRING` of another size than the one that its extensible type gives, as an algorithm mask of 24 bits: rasn's JER does not represent them. |
| `inspect`: criticalities | The tables have those of the object set of each message of a procedure, and of each single container. Anywhere else an IE says its own, as the [rules](#rules) list. |
| `inspect`: `check_path` | What the types do not say is not refused: the value of an IE selected by `@id=N` whose identifier is unknown or has no type where it is, the entries of a container of a value when they are selected by position, and a segment under such an IE. |

## Typed API

### Decode a PDU

| Of an `NGAP_PDU` | Gives |
|------------------|-------|
| `NGAP_PDU::decode(&bytes)` | The PDU of the octets of one message. Octets that are incomplete, and octets after the message, are an error of the kind `Decode` |
| `encode()` | Its octets, a `Vec<u8>` |
| `procedure_code()`, `procedure_name()` | The code and the name of its procedure: `42` and `"UEContextReleaseRequest"` |
| `direction()` | `"InitiatingMessage"`, `"SuccessfulOutcome"` or `"UnsuccessfulOutcome"`; `is_initiating()`, `is_successful()` and `is_unsuccessful()` say which |
| `kind()` | An `NgapPduKind`: one variant for each message, and `Other` for a procedure that the specification does not have in that direction |
| `decode_value()` | The message, as its type, which the caller names. The type of another message is an error of the kind `WrongMessage`, and a type that is no message does not compile |
| `protocol_ies()`, `set_protocol_ies(ies)` | The IEs of its message, whatever the message is, and the same PDU with other IEs: [below](#the-ies-of-any-message) |
| `check()` | How it deviates from the object set of its message, a `Vec<Deviation>`: [below](#check-a-message-against-its-object-set) |

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
`RAN_UE_NGAP_ID`. A name that the crate does not have does not compile, and
neither does a call that leaves out a criticality: the error says which one
is missing. The macro gives a `Result`: a value that does not encode, such
as an integer outside its constraint, is an error of the kind `Encode`.

A criticality says what a receiver does with what it does not comprehend
(TS 38.413 clause 10.3.2). The specification assigns one to each procedure
and to each IE of a message, in the tables of clause 9.2. The macros write
the one that they are given and check none: `check()` reports one that is
not the assigned one. With the `inspect` feature, [`from_tree()`] writes a
message with those that are assigned.

### Read the IEs of a message

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
name for its decoded value. After `=>` comes what the field keeps of that
value; without it, the field is the `.0` of the generated newtype.

| Field | Is | When the IE is not in the message | When it is there and does not decode |
|-------|----|-----------------------------------|--------------------------------------|
| `req` | A `T` | An error of the kind `MissingIe`, which names the IE and the field | An error of the kind `UndecodableIe` |
| `opt` | An `Option<T>` | `None` | The same error |
| `lax` | An `Option<T>` | `None` | `None` |

The error is that of the enclosing function, converted as `?` converts one:
the function returns a `Result` whose error is `Error`,
`Box<dyn std::error::Error>` or any type with a `From<Error>`. A call with
`lax` fields alone has no error, and its function returns what it likes.

A field is `req` when the receiver cannot go on without the IE, and `opt`
when it goes on without an IE that was not sent, and not with one that it
could not read. A `lax` field takes the IE if it decodes: the receiver goes on
as if the IE had not been received. TS 38.413 clause 10.3.4.2 has a receiver
ignore the content of an IE that it does not comprehend and that was received
with the criticality `ignore`, and go on so; with `notify` it also reports the
IE, and with `reject` it rejects the procedure. The macro does not look at the
criticality of an IE: the caller reads it in `protocol_ies`, and takes with
`lax` what it may ignore.

```rust
use oxirush_ngap::{extract_ngap_ies, ngap::*};

// The cause of a request, when it has one that is read
fn cause(request: &UEContextReleaseRequest) -> Option<Cause> {
    extract_ngap_ies!(request, UEContextReleaseRequest,
        lax cause: Cause = Cause(value) => value,
    );
    cause
}

// A UE CONTEXT RELEASE REQUEST, then the same with an octet of its Cause spoiled
for octets in [
    "002a4015000003000a0002002a005500020007000f40020500",
    "002a4015000003000a0002002a005500020007000f4002e500",
] {
    let pdu = NGAP_PDU::decode(&hex::decode(octets).unwrap()).unwrap();
    println!("{:?}", cause(&pdu.decode_value().unwrap()));
}
```

```text
Some(radioNetwork(user_inactivity))
None
```

### Change or build one IE

```rust
use oxirush_ngap::{build_ngap_ie, ngap::*, with_ngap_ie_mut};

// A message with one IE, which is built alone
let mut message = UplinkNASTransport::new(ProtocolIEContainer(vec![
    build_ngap_ie!(UplinkNASTransport, REJECT AMF_UE_NGAP_ID(1u64)).unwrap(),
]));

// The IE is decoded, changed and encoded again in its place
let updated = with_ngap_ie_mut!(message, UplinkNASTransport,
    AMF_UE_NGAP_ID(id) = 42u64
);
assert_eq!(updated, Ok(true));
```

`build_ngap_ie!` builds one entry of the IEs of a message, a
`ProtocolIEField`, and gives a `Result` as `build_ngap!` does.
`with_ngap_ie_mut!` decodes one IE, changes it and encodes it again: `= value`
sets the `.0` of a newtype, and `=> expression` changes the decoded value and
says `true` or `false`. It gives a `Result<bool, Error>`: what the expression
says, `Ok(false)` for an IE that is not in the message, and an error for an
IE that does not decode or a changed value that does not encode. When a
message has the same IE twice, `with_ngap_ie_mut!` and `extract_ngap_ies!` both
use the last one, and one of them that does not decode is an error for both.

### The IEs of any message

Every message of TS 38.413 but the private one is a container of protocol
IEs, so a PDU gives the IEs of its message, and takes them back, without the
type of the message. `decode_value` takes the type of the message that the
PDU holds, and refuses another one.

```rust
use oxirush_ngap::{build_ngap_ie, ngap::*};

fn main() -> Result<(), oxirush_ngap::Error> {
    // An NG SETUP FAILURE, as any PDU that was received
    let bytes = hex::decode("40150008000001000f400188").unwrap();
    let mut pdu = NGAP_PDU::decode(&bytes)?;

    // Its IEs as they were received: identifier, criticality and octets
    let mut ies = pdu.protocol_ies()?;
    for ie in &ies.0 {
        let octets = hex::encode(ie.value.as_bytes());
        println!("{} {:?} {octets}", ie.id.0, ie.criticality);
    }

    // One more IE, and the container is the message of the same PDU again
    let wait = build_ngap_ie!(NGSetupFailure, IGNORE TimeToWait(TimeToWait::v5s))?;
    ies.0.push(wait);
    pdu.set_protocol_ies(ies)?;
    let octets = hex::encode(pdu.encode()?);
    println!("{} {octets}", pdu.procedure_name());
    Ok(())
}
```

```text
15 ignore 88
NGSetup 4015000d000002000f400188006b400120
```

`protocol_ies()` gives a `ProtocolIEContainer`, the IEs in the order of the
message: one that the message cannot have, one that the protocol does not
have and one that is there twice are among them. `set_protocol_ies` checks
nothing, and the PDU keeps its direction, its procedure code and its
criticality. Both are an error of the kind `WrongMessage` for the private
message, which has private IEs. The message of a procedure that the
specification does not have in that direction is read as a container of
protocol IEs too.

### Check a message against its object set

The ASN.1 gives each message an object set: the IEs that it can have, in
their order, each with a criticality and a presence. A sender constructs a
message so (TS 38.413 clause 9.4.1), and `check()` of a PDU reports what is
not so. It refuses nothing. The macros still build such a message, a tree
writes it and the codec encodes and decodes it, because a test of how a peer
handles it must be able to send it.

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

`check()` gives a `Vec<Deviation>`, empty for a message that is as its object
set has it. A `Deviation` has the identifier of its IE, the name that ASN.1
gives it and its position in the message, from 0, and `Display` says it in a
sentence.

| `Deviation` | Is |
|-------------|----|
| `PduCriticality` | A criticality of the PDU that is not that of its procedure |
| `UnexpectedIe` | An IE that the object set of the message does not have: one of another message, or one that the protocol does not have |
| `RepeatedIe` | An IE that the message has already |
| `OutOfOrder` | An IE that comes after one that the object set has after it. The IEs that the set does not have are left out of the order |
| `IeCriticality` | A criticality of an IE that is not the one that the object set assigns it |
| `MissingIe` | An IE whose presence is mandatory, and that the message does not have |

Clause 10.3 of TS 38.413 says what a receiver does. An IE that it does not
comprehend is handled by the criticality that the IE carries (10.3.4), a
missing IE by the criticality that the specification assigns it (10.3.5),
and IEs in a wrong order or with too many occurrences reject the procedure
(10.3.6).

`check()` does not report a conditional IE, missing or present: its
condition is prose. It looks at the IEs of the message itself, not at their
values or at the IEs inside a value. The private message has no object set,
so only the criticality of its PDU is compared, and a procedure that the
specification does not have in that direction is an error of the kind
`UnknownMessage`.

### Identities and capabilities

```rust
use oxirush_ngap::helpers::*;

// A PLMN from its MCC and its MNC, and what is identified within it
let network = plmn("208", "93").unwrap();
let amf = guami(network.clone(), 1, 1, 0).unwrap(); // region, set, pointer
let tracking_area = tai(network.clone(), [0x00, 0x00, 0x01]); // TAC
let cell = nr_cgi(network.clone(), 0x123456, 1).unwrap(); // gNB, cell
let gnb = global_gnb_id(network.clone(), 0x123456).unwrap();

// A bit string from octets, and the NR algorithms of a UE: the first
// octet of its encryption bitmap, then of its integrity bitmap
let algorithm_mask = bytes_to_bitvec(&[0xe0]);
let capabilities = ue_security_capabilities(0xe0, 0xe0);

// What is no MCC, and an AMF Set ID that does not fit in its 10 bits
println!("{}", plmn("20", "93").unwrap_err());
println!("{}", guami(network, 1, 1024, 0).unwrap_err());
```

```text
"20" is not an MCC: it has three digits, 0 to 9
the AMF Set ID 1024 does not fit in 10 bits
```

A PLMN identity has its digits in the order of TS 38.413 clause 9.3.3.5: the
three of the MCC, then the two or three of the MNC, two in each octet, the
second over the first. `plmn("310", "410")` is the octets `13 40 01`. A NAS
message has another order for an MNC of three digits, that of TS 24.008,
where the same network is `13 00 14`. `plmn_from` reads the MCC and the MNC
of a PLMN identity, and has `None` for octets that are not one, as
`ab cd ef`.

No helper panics, and none cuts a number short. `plmn` gives an error of the
kind `Argument` for what is no MCC or no MNC, and so do `guami`, `nr_cgi`,
`global_gnb_id` and `int_to_bitvec` for a number that does not fit in the
bits of its field. `tai` takes the three octets of a TAC as an array.

### Errors

Every function and macro that can fail returns an `oxirush_ngap::Error`: the
codec, the four macros, the helpers and, with the `inspect` feature, the
methods and the functions of the JSON forms.

| Of an `Error` | Gives |
|---------------|-------|
| `kind()` | An `ErrorKind`, which is what a caller matches on. More kinds may come with a later version: leave the others to `_` |
| `Display` | What is wrong, with the names of the specification, and what to do about it where something can be done |
| `source()` | The error of `rasn`, where the codec gave one: a `rasn::error::DecodeError` or an `EncodeError` |

It is `Clone`, `Send` and `Sync`, two errors are equal when their kind and
their message are, and `?` converts one into a `Box<dyn std::error::Error>`.

| Kind | When |
|------|------|
| `Decode` | Octets are not the value that they are decoded as: they end before it does, go on after it, or hold a value outside its constraint |
| `Encode` | A value does not encode, as an integer outside its constraint |
| `WrongMessage` | `decode_value::<T>()` on a PDU that holds another message than `T`, and the protocol IEs of the private message, which has none |
| `MissingIe` | A `req` field of `extract_ngap_ies!` whose IE is not in the message |
| `UndecodableIe` | An IE that is in the message and does not decode as its type: a `req` or an `opt` field of `extract_ngap_ies!`, where a `lax` field is `None`, and the IE of `with_ngap_ie_mut!`. With `inspect`, a path into an IE or a transfer that did not decode |
| `Argument` | An argument that a helper does not take: what is no MCC or no MNC, and a number that does not fit in the bits of its field |
| `UnknownMessage` | `check()` on a PDU whose procedure the specification does not have in that direction: it has no object set. With `inspect`, also a name that is no message |

```rust
use oxirush_ngap::{ErrorKind, ngap::*};

// A UE CONTEXT RELEASE REQUEST is not decoded as an NG SETUP REQUEST
let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
let wrong = pdu.decode_value::<NGSetupRequest>().unwrap_err();
assert_eq!(wrong.kind(), ErrorKind::WrongMessage);
println!("{wrong}");

// Octets that end early: the message, and what rasn said
let short = NGAP_PDU::decode(&bytes[..10]).unwrap_err();
assert_eq!(short.kind(), ErrorKind::Decode);
println!("{short}");
println!("{}", std::error::Error::source(&short).unwrap());
```

```text
the message of this PDU is UEContextReleaseRequest, not NGSetupRequest: kind() says which message a PDU holds
these octets do not decode as NGAP-PDU: the octets end before the value does, in its field `value`
Error when decoding field `InitiatingMessage.value`: Need more data to continue: Size(120) (Codec: APER) (Codec: APER)
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
| `PLMNIdentity` | Three octets: the digits of the MCC, then those of the MNC, two in each octet |
| `TAI` | PLMN and a TAC of three octets |
| `NRCGI` | PLMN and a 36-bit NR cell identity |
| `SNSSAI` | Slice/service type and optional slice differentiator |
| `NASPDU` | A 5GS NAS message, as octets |

## JSON: trees, views and paths

With the `inspect` feature a PDU is a `serde_json::Value` in two forms,
which are methods of `NGAP_PDU`. A tree is the short form, in which a
message is shown and written: its name and its IEs by name. A view keeps
what was received, and is read and edited by paths, with the functions of
the [`inspect`] module. Each of them that can fail returns an `Error`, whose
kind says what went wrong: see
[Errors of the JSON forms](#errors-of-the-json-forms).

| | Tree | View |
|---|------|------|
| From a PDU | [`to_tree()`] | [`to_view()`] |
| An IE is | its name with its value | an entry: its `id`, its `criticality`, its typed `value`, and what was received |
| It has | the name of the message and its IEs, without what the specification assigns | the procedure code, the direction, the criticality and the message, in the ASN.1 JSON encoding |
| Read and edited | as JSON | by paths: [`select`], [`paths`], [`set`], [`insert`], [`remove`], [`open`] |
| Back to a PDU | [`from_tree()`]: a message written from the tree alone | [`with_view()`]: this PDU, edited; [`from_view()`]: the PDU of any view. What was not edited keeps the octets received |

```rust
use oxirush_ngap::ngap::NGAP_PDU;

// UE CONTEXT RELEASE REQUEST
let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();

// Its tree, then its third IE as the view has it
println!("{}", pdu.to_tree().unwrap());
let view = pdu.to_view().unwrap();
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
let view = pdu.to_view().unwrap();

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
let mut view = pdu.to_view().unwrap();

// A value, and another alternative of a CHOICE in place of the one that is there
inspect::set(&mut view, "/ngap/RAN-UE-NGAP-ID/value", json!(9)).unwrap();
inspect::set(&mut view, "/ngap/Cause/nas", json!("Normal Release")).unwrap();
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree().unwrap());

// What was not edited is the octets that were received
let after = edited.to_view().unwrap();
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
let mut view = pdu.to_view().unwrap();

// An IE is added at the end, by its name and its typed value
let name = json!({"id": "RANNodeName", "criticality": "ignore", "value": "gnb-1"});
inspect::insert(&mut view, "/ngap/-", name).unwrap();

// Before another IE, by its number and its octets
let unknown = json!({"id": 60000, "criticality": "ignore", "octets": "c0ffee"});
inspect::insert(&mut view, "/ngap/Cause", unknown).unwrap();

// An IE is taken out by its name
inspect::remove(&mut view, "/ngap/AMF-UE-NGAP-ID").unwrap();

let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree().unwrap()["ies"]);
```

```json
[{"RAN-UE-NGAP-ID":7},{"60000":{"octets":"C0FFEE"},"criticality":"ignore"},{"Cause":{"radioNetwork":"user-inactivity"}},{"RANNodeName":"gnb-1","criticality":"ignore"}]
```

### Write a message from its name and its IEs

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
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

let pdu = NGAP_PDU::from_tree(&request).unwrap();
println!("{}", hex::encode(pdu.encode().unwrap()));

// The name gave the direction, the procedure code and the criticalities
let view = pdu.to_view().unwrap();
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
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use serde_json::json;

// `{"octets": "…"}` in place of a value is the octets of the IE, whatever they are
let request = json!({"message": "UEContextReleaseRequest", "ies": [
    {"AMF-UE-NGAP-ID": 42},
    {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
]});
let pdu = NGAP_PDU::from_tree(&request).unwrap();
println!("{}", hex::encode(pdu.encode().unwrap()));

// A view shows an IE that does not decode by its octets, and says why
let mut view = pdu.to_view().unwrap();
println!("{}", view["message"]["protocolIEs"][1]);

// In a view, the `octets` of an IE are set in place of its value
inspect::set(&mut view, "/ngap/AMF-UE-NGAP-ID/octets", json!("00ff")).unwrap();
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree().unwrap());
```

```text
002a4010000002000a0002002afde80003c0ffee
{"_decode_error":"NGAP IE 65000 is unknown","_original_id":65000,"_raw_value":"C0FFEE","criticality":"reject","id":65000,"value":"C0FFEE"}
{"ies":[{"AMF-UE-NGAP-ID":255},{"65000":{"octets":"C0FFEE"},"criticality":"reject"}],"message":"UEContextReleaseRequest"}
```

### A repeated IE

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use serde_json::json;

// A message that has one IE twice
let twice = json!({"message": "UEContextReleaseRequest", "ies": [
    {"AMF-UE-NGAP-ID": 1},
    {"RAN-UE-NGAP-ID": 7},
    {"AMF-UE-NGAP-ID": 2},
]});
let pdu = NGAP_PDU::from_tree(&twice).unwrap();
let mut view = pdu.to_view().unwrap();

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
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use serde_json::{Value, json};

// A message whose one IE is a list of two items
let request = json!({"message": "InitialContextSetupRequest", "ies": [
    {"AllowedNSSAI": [{"s-NSSAI": {"sST": 1}}, {"s-NSSAI": {"sST": 2, "sD": "000001"}}]},
]});
let pdu = NGAP_PDU::from_tree(&request).unwrap();
let mut view = pdu.to_view().unwrap();

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

let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree().unwrap());
```

```json
{"ies":[{"AllowedNSSAI":[{"s-NSSAI":{"sST":2}},{"s-NSSAI":{"sST":3}}]}],"message":"InitialContextSetupRequest"}
```

### A transfer and its members

A transfer is an `OCTET STRING` that contains a type. A view shows its decoded
value beside its octets, and a tree its decoded value alone.

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
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
let pdu = NGAP_PDU::from_tree(&request).unwrap();
let mut view = pdu.to_view().unwrap();

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
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree().unwrap()["ies"][2]);
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
use oxirush_ngap::{ErrorKind, inspect, ngap::NGAP_PDU};
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
let pdu = NGAP_PDU::from_tree(&required).unwrap();
println!("{}", pdu.to_tree().unwrap()["ies"][2]);

// Read: a path goes through the name of the type
let mut view = pdu.to_view().unwrap();
let container = format!("/ngap/SourceToTarget-TransparentContainer/{name}");
let cell = format!("{container}/targetCell-ID/nR-CGI/nRCellIdentity");
assert_eq!(inspect::check_path("HandoverRequired", &cell), Ok(()));

// The path is an error until `open` has decoded the octets, and selects the value after that
assert_eq!(inspect::select(&view, &cell).unwrap_err().kind(), ErrorKind::NotOpened);
inspect::open(&mut view, &cell).unwrap();
assert_eq!(inspect::select(&view, &cell).unwrap(), [&json!(4660)]);

// What the octets carry, decoded
let decoded = format!("{container}/decoded");
println!("{}", inspect::select(&view, &decoded).unwrap()[0]);

// An edit goes into it, and what is not edited is sent as it was received
inspect::set(&mut view, &cell, json!(7)).unwrap();
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree().unwrap()["ies"][2]);
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
| `value`, `decoded` | May be left out of a path. A segment is first a member of the IE or of the transfer itself: `/ngap/Cause/*` is the members of the IE. If a value has a member with one of those names, write `value` before it. Only a bit string whose size varies has such a member, its own `value`, so its path ends in `value/value`. |
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
| a bit string whose size varies, as the identifier of a gNB | `{"value": "000004", "length": 22}`: the octets that hold it, in hexadecimal, and its length in bits |
| the other strings: an SD, keys and algorithm masks, the NAS-PDU and the other containers | hexadecimal, read in either case |

Each is also read as JER writes it, and a value that does not fit its
notation, such as a transport layer address of another length, stays as JER
writes it. The octets of a bit string start with its first bit: `"C000"`
is the first two bits of sixteen, and `"000004"` the number 1 in 22 bits.

### Errors of the JSON forms

The methods and the functions of the JSON forms have these kinds, and two of the
[typed API](#errors): `Decode` for a PDU whose message does not decode, and
`UndecodableIe` for a path into an IE or a transfer that did not decode.

| Kind | When |
|------|------|
| `UnknownMessage` | A name of a message, or a direction and a procedure code, that the protocol does not have |
| `UnknownIe` | A name or an identifier of an IE that the protocol does not have, or has no type for where it is: its octets are written in its place |
| `Path` | A path that is not well formed: no `/` before its first segment, `~` before what is no `0` and no `1`, `@id=` before what is no identifier |
| `NoSuchMember` | A name that the type at that place cannot have: in a path, a member of a value, an IE of the object set of a message, a name where a list goes by position; in a value that is written, a member that its type does not have |
| `NothingSelected` | An edit whose path selects nothing: an IE that the message does not have, or a position after the end of a list |
| `NotOpened` | A path through the type that octets carry, before [`open`] decoded them: open, then select again |
| `Value` | A JSON value that is not a value of its ASN.1 type: of another form, a name that an `ENUMERATED` does not have, a number outside its constraint, octets that are no hexadecimal. The message says at which member, by the names of the value |
| `Form` | JSON that is not written as a tree or as a view: no name of a message, a member that neither has, an IE with its value and its octets, an IE that says no criticality where no table has one |
| `Edit` | An edit that is refused: a member that its type always has, the value of an IE or the alternative of a `CHOICE` taken out, an insertion where there is no list, and the octets of a value that was edited since |
| `Limit` | A path of more than 64 segments or 4096 octets, a selection of more than 4096 values, a tree nested deeper than 256 levels, a value that JER does not represent, and the tree of a message without a list of IEs |

A kind says what a caller can do: write octets for an `UnknownIe`, call
[`open`] for a `NotOpened`, show the octets of a PDU for a `Limit`. The view
is as it was after an edit that returns an error.

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
| `tests/aper_round_trip.rs` | The macros, and the encodings where the codec departs from rasn's: constrained lists, `UTF8String`, long bit strings. |
| `tests/errors.rs` | The errors of the typed API, each with its kind and its message: a PDU that is decoded as another message, octets that are no PDU, a value outside its constraint in a function and in each macro, an IE that is absent or does not decode, for a `req` and for an `opt` field, and what each helper refuses. With `inspect`, a test for each kind of error of the JSON forms. |
| `tests/protocol_ies.rs` | The protocol IEs of each of the 144 messages but the private one are read from its PDU and put back as the same octets. An IE that the protocol does not have and one that is there twice are kept with their criticality, a message that the macros built has the IEs that it was built with, and the private message is an error. |
| `tests/check.rs` | What `check()` reports: a message without a mandatory IE, with an IE twice, with an IE of another message, with its IEs out of order and with another criticality is each built by the macros, written from a tree, encoded and decoded, and reported as exactly that. Each of the 144 messages of the fixtures against its object set. |
| `tests/unknown_extensions.rs` | The additions of a later release to an extensible `SEQUENCE` are skipped, and what follows them is read. |
| `tests/allocation.rs` | Decoding reserves memory for the elements that it reads, not for the count that a length determinant claims. |
| `tests/inspection.rs` | Each of the 144 messages is the same octets through its view, and through its tree where it has IEs to name. Each path of each selects its value, with and without `value` and `decoded`, and is one that `check_path` takes. Edits, written trees, criticalities, the two IEs that have the type of what they extend, and the identifiers of the registry against the constants of the bindings. |
| `tests/shared_files.rs` | The files that this crate shares with `oxirush-s1ap` are as they were last mirrored. |
| The unit tests of the JSON codec | Each ASN.1 construct of the protocol has its JSON form and is read back from it, with what is refused and where. Each value of each message of the fixtures has the form that the JSON Encoding Rules codec of rasn gives it. |
| The unit tests of the paths | No two names that a path takes can be mistaken for each other. No value has a member with the same name as a member of its IE, except the `value` of a bit string. |
| The Rust blocks of this page | They compile and run as documentation tests, with the `inspect` feature, and `tests/readme.rs` compares what each prints with the output shown under it. |

```bash
cargo test --all-features
cargo test
```

## Benchmarks

The benchmarks and their profilers are a package of their own, `benches/`,
so that criterion and pprof are no dependencies of the crate. They measure
the codec, the macros and the JSON forms on the messages of
`tests/fixtures/messages.tsv` and on four messages of the size that a
network sends.

```bash
cd benches
cargo bench --bench codec -- decode/                   # the time, with criterion
cargo bench --bench allocations                        # the allocations of each operation
cargo bench --bench heap -- to-view/fixtures           # where they are made
cargo bench --bench codec -- --profile-time 10 to-view/fixtures
go tool pprof -top target/criterion/to-view/fixtures/profile/profile.pb
python3 build_cost.py +1.88.0 --features inspect       # a clean build: its time and its memory
```

`--profile-time` runs the benchmark under [pprof](https://crates.io/crates/pprof)
and writes a CPU profile that `go tool pprof` reads; with `PROFILE=flamegraph`
in the environment it writes `flamegraph.svg` beside it instead.

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
on 2026-10-12, with the generator of that revision and the rustfmt of Rust
1.88: they came out as they are committed, octet for octet. No CI job
regenerates them, as the ASN.1 is not in the repository.

oxirush-ngap and oxirush-s1ap share, octet for octet, the code that does not
depend on the protocol: `build/aper_fix.rs`, `build/containers.rs`,
`build/inspection.rs`, `build/registry.rs`, `src/check.rs`, `src/error.rs`,
`src/inspect_paths.rs`, `src/json.rs`, `src/message.rs`, `src/per.rs`,
`src/sized.rs`, the test `tests/readme.rs`, and the benchmarks but for their
`protocol.rs`.
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

rasn 0.29's APER codec departs from ITU-T X.691 where NGAP reaches it. There
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
[`to_tree()`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/ngap/ngap_pdu_descriptions/enum.NGAPPDU.html#method.to_tree
[`from_tree()`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/ngap/ngap_pdu_descriptions/enum.NGAPPDU.html#method.from_tree
[`to_view()`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/ngap/ngap_pdu_descriptions/enum.NGAPPDU.html#method.to_view
[`from_view()`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/ngap/ngap_pdu_descriptions/enum.NGAPPDU.html#method.from_view
[`with_view()`]: https://docs.rs/oxirush-ngap/latest/oxirush_ngap/ngap/ngap_pdu_descriptions/enum.NGAPPDU.html#method.with_view
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
