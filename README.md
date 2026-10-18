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

With the `inspect` feature, a PDU is also JSON, read, edited and written by
the names of the specification:

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use oxirush_ngap::tree;

// The same message as a tree: its name and its IEs by name
let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
println!("{}", pdu.to_tree());

// Its view is edited at a path, and is a PDU again
let mut view = pdu.to_view().unwrap();
let lost = tree!("radio-connection-with-ue-lost");
inspect::set(&mut view, "/ngap/Cause/radioNetwork", lost).unwrap();
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree());

// Its octets are those that the typed example built
let octets = hex::encode(edited.encode().unwrap());
assert_eq!(octets, "002a4015000003000a0002002a005500020007000f40020540");
```

```json
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"radio-connection-with-ue-lost"}}]}
```

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

## Keep a value after its PDU

A type that holds octets has a lifetime, as `NASPDU<'a>`: a value that is
decoded borrows them from the PDU. A type that holds none, as `Cause`, `TAI`,
`NRCGI` or `GUAMI`, has no lifetime, and the extension container of a value
owns its octets so that it gives it none. `into_owned()` gives the value with
the lifetime `'static`, which is the type to write in a struct.

```rust
use oxirush_ngap::ngap::{Cause, GUAMI, NRCGI, TAI};

// What holds no octets of its own is kept as it is decoded
struct Cell {
    tai: TAI,
    cgi: NRCGI,
    guami: GUAMI,
    last_cause: Option<Cause>,
}
```

```rust
use oxirush_ngap::{Error, build_ngap, extract_ngap_ies, ngap::*};

// What a node keeps of a UE after its first message
struct Ue {
    ran_id: u32,
    nas: NASPDU<'static>,
}

fn admit(pdu: &NGAP_PDU) -> Result<Ue, Error> {
    // `message` borrows from `pdu`, and `nas` from `message`
    let message: InitialUEMessage = pdu.decode_value()?;
    extract_ngap_ies!(message, InitialUEMessage,
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
        req nas: NASPDU = NAS_PDU(nas) => nas,
    );
    // Kept beyond the PDU: the octets are copied here, once
    Ok(Ue { ran_id, nas: nas.into_owned() })
}

fn main() -> Result<(), Error> {
    let pdu = build_ngap!(InitiatingMessage, InitialUEMessage, IGNORE, InitialUEMessage,
        REJECT RAN_UE_NGAP_ID(7u32),
        REJECT NAS_PDU(vec![0x7e, 0x00, 0x41]),
    )?;
    let ue = admit(&pdu)?;
    drop(pdu);
    println!("UE {}: a NAS-PDU of {} octets", ue.ran_id, ue.nas.0.len());
    Ok(())
}
```

```text
UE 7: a NAS-PDU of 3 octets
```

To return a message that borrows the caller's buffer, use
`decode_borrowed(octets)?.message()`. A message may be held across an
`.await`, and every value of the bindings is `Send`:

```rust
use oxirush_ngap::{Error, build_ngap, extract_ngap_ies, ngap::*};

async fn deliver(nas: &[u8]) {
    println!("a NAS-PDU of {} octets", nas.len());
}

async fn handle(received: Vec<u8>) -> Result<(), Error> {
    // Nothing of `received` is copied
    let pdu = NGAP_PDU::decode_borrowed(&received)?;
    let message: UplinkNASTransport = pdu.decode_value()?;
    extract_ngap_ies!(message, UplinkNASTransport,
        req nas: NASPDU = NAS_PDU(nas) => nas,
    );
    deliver(&nas.0).await;
    Ok(())
}

fn main() -> Result<(), Error> {
    let transport = build_ngap!(InitiatingMessage, UplinkNASTransport, IGNORE, UplinkNASTransport,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        REJECT NAS_PDU(vec![0x7e, 0x00, 0x5d]),
    )?;
    let handler = handle(transport.encode()?);

    // A runtime may move the handler to another thread
    fn is_send(_: &impl Send) {}
    is_send(&handler);

    // Polled here without a runtime: this handler never waits
    let mut handler = std::pin::pin!(handler);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    assert!(handler.as_mut().poll(&mut context).is_ready());
    Ok(())
}
```

```text
a NAS-PDU of 3 octets
```

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
A character string is checked when it is decoded, so a RAN node name with a
character that a PrintableString does not have, as `_`, is `None` for a `lax`
field and an error for an `opt` one.

```rust
use oxirush_ngap::{extract_ngap_ies, ngap::*};

// The cause of a request, when it has one that is read
fn cause(request: &UEContextReleaseRequest<'_>) -> Option<Cause> {
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

## Change or build one IE

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

`with_ngap_ie_mut!` gives `Ok(false)` for an IE that is not in the message;
`IE(value) => { … }` in place of `= value` changes the decoded value, mutable
and its own, in a block that says whether it did: its documentation has an
example. Where an IE is in a message twice, it and `extract_ngap_ies!` take
the last one.

## The IEs of any message

```rust
use oxirush_ngap::{build_ngap_ie, ngap::*};

fn main() -> Result<(), oxirush_ngap::Error> {
    // An NG SETUP FAILURE, as any PDU that was received
    let bytes = hex::decode("40150008000001000f400188").unwrap();
    let mut pdu = NGAP_PDU::decode(&bytes)?;

    // Its IEs as they were received: identifier, criticality and octets. They
    // borrow from the PDU, and are owned here, as the PDU is changed below
    let mut ies = pdu.protocol_ies()?.into_owned();
    for ie in &ies.0 {
        let octets = hex::encode(&ie.value);
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

`protocol_ies()` gives the IEs of any message but the private one in their
order, including those that the message cannot have or has twice, and
`set_protocol_ies` writes any list without checking it.

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

## Check a message against its object set

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
([Performance](#performance)).

## Answer a faulty message

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

## Identities and capabilities

```rust
use oxirush_ngap::helpers::*;

// A PLMN from its MCC and its MNC, and what is identified within it
let network = plmn("208", "93").unwrap();
let amf = guami(network, 1, 1, 0).unwrap(); // region, set, pointer
let tracking_area = tai(network, [0x00, 0x00, 0x01]); // TAC
let cell = nr_cgi(network, 0x123456, 24, 1).unwrap(); // gNB of 24 bits, cell
let gnb = global_gnb_id(network, 0x123456, 24).unwrap();
let ng_enb = global_ng_enb_id(network, NgEnbKind::Macro, 0x12345).unwrap();
let lte_cell = eutra_cgi(network, NgEnbKind::Macro, 0x12345, 7).unwrap(); // ng-eNB, cell

// A bit string from octets, and the NR algorithms of a UE: the first
// octet of its encryption bitmap, then of its integrity bitmap
let algorithm_mask = oxirush_ngap::BitString::from([0xe0]);
let capabilities = ue_security_capabilities(0xe0, 0xe0);

// What is no MCC, and an AMF Set ID that does not fit in its 10 bits
println!("{}", plmn("20", "93").unwrap_err());
println!("{}", guami(network, 1, 1024, 0).unwrap_err());
```

```text
"20" is not an MCC: it has three digits, 0 to 9
the AMF Set ID 1024 does not fit in 10 bits
```

A PLMN identity has its digits in the order of TS 38.413 clause 9.3.3.5:
`plmn("310", "410")` is the octets `13 40 01`, where a NAS message has
`13 00 14`. `plmn_from` reads the MCC and the MNC of a PLMN identity, and a
gNB ID has the length that it is given, 22 to 32 bits (clause 9.3.1.6), and
an ng-eNB ID that of its kind (clause 9.3.1.8).

## Errors

```rust
use oxirush_ngap::{ErrorKind, ngap::*};

// A UE CONTEXT RELEASE REQUEST is not decoded as an NG SETUP REQUEST
let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
let wrong = pdu.decode_value::<NGSetupRequest>().unwrap_err();
assert_eq!(wrong.kind(), ErrorKind::WrongMessage);
println!("{wrong}");

// Octets that end early
let short = NGAP_PDU::decode(&bytes[..10]).unwrap_err();
assert_eq!(short.kind(), ErrorKind::Decode);
println!("{short}");
```

```text
the message of this PDU is UEContextReleaseRequest, not NGSetupRequest: kind() says which message a PDU holds
these octets do not decode as NGAP-PDU: the octets end before the value does, in its field `value`
```

Every call that can fail returns an `oxirush_ngap::Error`. Its `kind()` is
what a caller matches on, and the documentation of `ErrorKind` lists the
kinds; `error.ie()` names the IE of an error of the macros.

## The tree and the view of a PDU

With the `inspect` feature a PDU is JSON in two forms: a tree, in which a
message is shown and written, and a view, which keeps what was received and is
read and edited by paths with the functions of the [`inspect`] module. Every
PDU that decodes has a tree, which is the same PDU again.

```rust
use oxirush_ngap::ngap::NGAP_PDU;

// UE CONTEXT RELEASE REQUEST
let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();

// Its tree, then its third IE as the view has it
println!("{}", pdu.to_tree());
let view = pdu.to_view().unwrap();
println!("{}", view["message"]["protocolIEs"][2]);
```

```json
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
{"id":15,"criticality":"ignore","value":{"radioNetwork":"user-inactivity"},"_raw_value":"0500","_original_id":15,"_ie_name":"Cause"}
```

## Read an IE

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use oxirush_ngap::tree;

let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
let view = pdu.to_view().unwrap();

// `/ngap` is the IEs of the message, and an IE goes by its name
let select = |path| inspect::select(&view, path).unwrap();
assert_eq!(select("/ngap/RAN-UE-NGAP-ID/value"), [&tree!(7)]);
assert_eq!(select("/ngap/Cause/radioNetwork"), [&tree!("user-inactivity")]);
assert_eq!(select("/ngap/Cause/octets"), [&tree!("0500")]);

// By its position, by its name whatever its case and its separators, by its
// identifier, and each IE with `*`
assert_eq!(select("/ngap/1/value"), select("/NGAP/ran_ue_ngap_id/value"));
assert_eq!(select("/ngap/@id=85/value"), [&tree!(7)]);
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
```

## Change a value

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use oxirush_ngap::tree;

let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
let mut view = pdu.to_view().unwrap();

// A value, and another alternative of a CHOICE in place of the one that is there
inspect::set(&mut view, "/ngap/RAN-UE-NGAP-ID/value", tree!(9)).unwrap();
inspect::set(&mut view, "/ngap/Cause/nas", tree!("Normal Release")).unwrap();
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree());

// What was not edited is the octets that were received
let after = edited.to_view().unwrap();
let octets = inspect::select(&after, "/ngap/AMF-UE-NGAP-ID/octets").unwrap();
assert_eq!(octets, [&tree!("002a")]);

// A name that the type does not have is refused, and the view is as it was
let wrong = inspect::set(&mut view, "/ngap/Cause/radioNetwrok", tree!("unspecified"));
println!("{}", wrong.unwrap_err());
```

```text
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":9},{"Cause":{"nas":"normal-release"}}]}
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions; an IE itself has id, criticality, value, octets, at /ngap/Cause/radioNetwrok
```

## Add or remove an IE

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use oxirush_ngap::tree;

let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
let mut view = pdu.to_view().unwrap();

// An IE is added at the end, by its name and its typed value
let name = tree!({"id": "RANNodeName", "criticality": "ignore", "value": "gnb-1"});
inspect::insert(&mut view, "/ngap/-", name).unwrap();

// Before another IE, by its number and its octets
let unknown = tree!({"id": 60000, "criticality": "ignore", "octets": "c0ffee"});
inspect::insert(&mut view, "/ngap/Cause", unknown).unwrap();

// An IE is taken out by its name
inspect::remove(&mut view, "/ngap/AMF-UE-NGAP-ID").unwrap();

let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree()["ies"]);
```

```json
[{"RAN-UE-NGAP-ID":7},{"60000":{"octets":"c0ffee"},"criticality":"ignore"},{"Cause":{"radioNetwork":"user-inactivity"}},{"RANNodeName":"gnb-1","criticality":"ignore"}]
```

## Write a message from its name and its IEs

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

## Send octets as they are

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

## A repeated IE

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use oxirush_ngap::tree;

// A message that has one IE twice
let twice = tree!({"message": "UEContextReleaseRequest", "ies": [
    {"AMF-UE-NGAP-ID": 1},
    {"RAN-UE-NGAP-ID": 7},
    {"AMF-UE-NGAP-ID": 2},
]});
let pdu = NGAP_PDU::from_tree(&twice).unwrap();
let mut view = pdu.to_view().unwrap();

// The name selects both, in the order of the message
let both = inspect::select(&view, "/ngap/AMF-UE-NGAP-ID/value").unwrap();
assert_eq!(both, [&tree!(1), &tree!(2)]);

// Each has its position, which is how `paths` names it, and its name may follow
inspect::set(&mut view, "/ngap/2/AMF-UE-NGAP-ID/value", tree!(3)).unwrap();

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

## An item of a list

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use oxirush_ngap::{tree, tree::Value};

// A message whose one IE is a list of two items
let request = tree!({"message": "InitialContextSetupRequest", "ies": [
    {"AllowedNSSAI": [{"s-NSSAI": {"sST": 1}}, {"s-NSSAI": {"sST": 2, "sD": "000001"}}]},
]});
let pdu = NGAP_PDU::from_tree(&request).unwrap();
let mut view = pdu.to_view().unwrap();

// An item by its position, and each of them after `value`
let select = |view, path| -> Vec<Value> {
    let selected = inspect::select(view, path).unwrap();
    selected.into_iter().cloned().collect()
};
assert_eq!(select(&view, "/ngap/AllowedNSSAI/1/s-NSSAI/sD"), [tree!("000001")]);
assert_eq!(select(&view, "/ngap/AllowedNSSAI/value/*/s-NSSAI/sST"), [tree!(1), tree!(2)]);

// An optional member that an item does not have selects nothing
assert!(select(&view, "/ngap/AllowedNSSAI/0/s-NSSAI/sD").is_empty());

// An item is added at the end, and one is taken out
inspect::insert(&mut view, "/ngap/AllowedNSSAI/-", tree!({"s-NSSAI": {"sST": 3}})).unwrap();
inspect::remove(&mut view, "/ngap/AllowedNSSAI/0").unwrap();

// `null` takes an optional member out
inspect::set(&mut view, "/ngap/AllowedNSSAI/0/s-NSSAI/sD", Value::Null).unwrap();

let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree());
```

```json
{"message":"InitialContextSetupRequest","ies":[{"AllowedNSSAI":[{"s-NSSAI":{"sST":2}},{"s-NSSAI":{"sST":3}}]}]}
```

## A transfer and its members

A view shows the decoded value of a transfer beside its octets, and a tree
its decoded value alone.

```rust
use oxirush_ngap::{inspect, ngap::NGAP_PDU};
use oxirush_ngap::tree;

// A message with a transfer, written as the message that it contains
let request = tree!({"message": "PDUSessionResourceSetupRequest", "ies": [
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
assert_eq!(inspect::select(&view, &kind).unwrap(), [&tree!("ipv4")]);

// The same value by the path that says `value` and `decoded`
let long = "/ngap/PDUSessionResourceSetupListSUReq/value/0\
    /pDUSessionResourceSetupRequestTransfer/decoded/PDUSessionType/value";
assert_eq!(inspect::select(&view, long), inspect::select(&view, &kind));

// And by the names that the tree shows, joined: the IE by its position, then its name
let joined = format!("{transfer}/protocolIEs/0/PDUSessionType/value");
assert_eq!(inspect::select(&view, &joined), inspect::select(&view, &kind));

// An edit of the message that it contains is encoded into the transfer
inspect::set(&mut view, &kind, tree!("ipv6")).unwrap();
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree()["ies"][2]);
```

```json
{"_raw_transfer":"0000010086000100","decoded":{"protocolIEs":[{"id":134,"criticality":"reject","value":"ipv4","_raw_value":"00","_original_id":134,"_ie_name":"PDUSessionType"}]}}
{"PDUSessionResourceSetupListSUReq":[{"pDUSessionID":5,"s-NSSAI":{"sST":1},"pDUSessionResourceSetupRequestTransfer":{"protocolIEs":[{"PDUSessionType":"ipv6","criticality":"reject"}]}}]}
```

## The names of an ENUMERATED

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

A name is read in any case and with any separators, as `"Normal Release"`
above, and shown as the specification spells it.

## Check a path before use

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
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions; an IE itself has id, criticality, value, octets, at /ngap/Cause/radioNetwrok
"valeu" is not a member of this value, which has none; an IE itself has id, criticality, value, octets, at /ngap/RAN-UE-NGAP-ID/valeu
"GUAMI" is not an IE of UEContextReleaseRequest, which has AMF-UE-NGAP-ID, RAN-UE-NGAP-ID, PDUSessionResourceListCxtRelReq, Cause, GW-Context-Release-Indication, at /ngap/GUAMI
```

The functions `message_ies`, `message_ie_criticalities`, `item_criticality`
and the others of [`inspect`] give what the ASN.1 says of each message, which
`check_path` reads.

## The container of a handover

Three IEs carry octets that TS 38.413 gives by reference: between NG-RAN
nodes, a type of the specification, as `SourceToTarget-TransparentContainer`
carries a `SourceNGRANNode-ToTargetNGRANNode-TransparentContainer` (clause
9.3.1.20). Nothing in a message says which, so a tree and a view show the
octets, and the one that writes or reads them names the type.

```rust
use oxirush_ngap::{ErrorKind, inspect, ngap::NGAP_PDU};
use oxirush_ngap::tree;

// Written: the name of the type with its value, in place of the octets
let name = "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer";
let required = tree!({"message": "HandoverRequired", "ies": [
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
println!("{}", pdu.to_tree()["ies"][2]);

// Read: a path goes through the name of the type
let mut view = pdu.to_view().unwrap();
let container = format!("/ngap/SourceToTarget-TransparentContainer/{name}");
let cell = format!("{container}/targetCell-ID/nR-CGI/nRCellIdentity");
assert_eq!(inspect::check_path("HandoverRequired", &cell), Ok(()));

// The path is an error until `open` has decoded the octets, and selects the value after that
assert_eq!(inspect::select(&view, &cell).unwrap_err().kind(), ErrorKind::NotOpened);
inspect::open(&mut view, &cell).unwrap();
assert_eq!(inspect::select(&view, &cell).unwrap(), [&tree!(4660)]);

// What the octets carry, decoded
let decoded = format!("{container}/decoded");
println!("{}", inspect::select(&view, &decoded).unwrap()[0]);

// An edit goes into it, and what is not edited is sent as it was received
inspect::set(&mut view, &cell, tree!(7)).unwrap();
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree()["ies"][2]);
```

```text
{"SourceToTarget-TransparentContainer":"000201020002f8390000012340080100"}
{"rRCContainer":"0102","targetCell-ID":{"nR-CGI":{"pLMNIdentity":"208-93","nRCellIdentity":4660}},"uEHistoryInformation":[{"lastVisitedCellInformation":{"eUTRANCell":"00"}}]}
{"SourceToTarget-TransparentContainer":"000201020002f8390000000070080100"}
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

## Write values as they are usually written

A tree, a view and a value alone show the values of some types as they are
usually written, and take them so or in the form of their ASN.1 type. A string
of decimal digits is never read as octets: octets whose hexadecimal has no
letter are written after `0x`.

| Type | Notation |
|------|----------|
| `PLMNIdentity` | `"208-93"`, the MCC and the MNC, or its octets: `"02f839"`, `"0x134001"`. `"310410"` is refused, as it is read as 310-410 and as the octets 0x310410 |
| `TransportLayerAddress` | `"10.0.0.1"`, `"2001:db8::1"`, or the two with a comma |
| `TAC`, `EPS-TAC`, `LAC`, `GTP-TEID`, `FiveG-TMSI`, `SST`, `PortNumber`, `AMFRegionID`, `AMFSetID`, `AMFPointer`, `NRCellIdentity`, `EUTRACellIdentity`, `UL-NAS-Count` | a number, also read in hexadecimal as `"0x…"`. A string of digits, as `"123456"`, is refused: it is the number 123456 and the octets 0x123456 |
| `ENUMERATED` | the name of the value, read in any case |
| `INTEGER` | a number, which 64 bits hold, as the bindings hold it |
| a bit string whose size varies, as the identifier of a gNB | `{"value": "000004", "length": 22}`: its octets in hexadecimal, from its first bit, and its length in bits |
| a bit string of another size than the one that its type gives beside an extension marker | `{"value": "c00000", "length": 24}`, where a mask of the 16 bits of its type is `"c000"` |
| the other strings: an SD, keys, the NAS-PDU and the other containers | hexadecimal, written in lower case and read in either |

The documentation of the [`inspect`] module has the rules of the paths, of
the edits and of the values, and the kinds of the errors of the JSON forms.

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

Every figure for HandoverRequest, per call:

|  | decode ns | cycles | instructions | allocations | octets allocated | encode ns | cycles | instructions | allocations | octets allocated | most memory held, KiB |
|---|---|---|---|---|---|---|---|---|---|---|---|
| this crate | 525 | 2,327 | 6,573 | 3 | 584 | 277 | 1,251 | 4,376 | 2 | 495 | 3,884 |
| rasn 0.29 | 3,948 (7.5x) | 17,973 (7.7x) | 41,848 (6.4x) | 35 | 2,168 | 3,053 (11.0x) | 13,886 (11.1x) | 46,672 (10.7x) | 53 | 2,171 | 4,652 |
| hampi 0.7.2 | 5,017 (9.5x) | 23,137 (9.9x) | 69,702 (10.6x) | 76 | 7,641 | 6,941 (25.0x) | 31,925 (25.5x) | 89,343 (20.4x) | 27 | 2,315 | 3,440 |
| asn1c (Open5GS 2.8.0) | 7,324 (13.9x) | 33,063 (14.2x) | 118,711 (18.1x) | 121 | 15,532 | 3,819 (13.8x) | 17,399 (13.9x) | 65,546 (15.0x) | 32 | 4,504 | 6,336 |
| free5GC ngap 1.2.0 | 4,993 (9.5x) | 28,849 (12.4x) | 95,642 (14.5x) | 179 | 2,952 | 2,295 (8.3x) | 13,008 (10.4x) | 41,964 (9.6x) | 36 | 1,184 | 9,436 |
| pycrate 0.8.1 | 465,894 (886.7x) | 2,141,808 (920.6x) | 4,442,005 (675.8x) |  |  | 309,128 (1114.8x) | 1,437,455 (1148.7x) | 2,975,625 (679.9x) |  |  | 37,416 |

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

Reading the IEs of a message and checking a PDU, per call, on the four
messages that `benches/` writes: `decode_value()` and then the macro on the
message, which decodes the container of the message first; the macro on the
PDU (`extract_ngap_ies!(pdu, ...)`), which walks the octets of its message once and
decodes the IEs that its fields ask for; and `ie::<T>()` of the PDU, which
decodes one IE. In `benches/`, `cargo run --release --bin readers --features
readers` measures them:

| Message | the first IE: `decode_value()`, then the macro | the macro on the PDU | `ie::<T>()` | every IE: `decode_value()`, then the macro | the macro on the PDU |
|---|---|---|---|---|---|
| An INITIAL CONTEXT SETUP REQUEST of 2 PDU sessions and a NAS-PDU of 1,500 octets | 113 ns, 1 allocation | 38 ns, no allocation | 27 ns, no allocation | 579 ns, 3 allocations | 500 ns, 2 allocations |
| A PDU SESSION RESOURCE SETUP REQUEST of 8 PDU sessions | 66 ns, 1 allocation | 24 ns, no allocation | 27 ns, no allocation | 471 ns, 2 allocations | 425 ns, 1 allocation |
| A HANDOVER REQUIRED of a container of 4,000 octets | 86 ns, 1 allocation | 30 ns, no allocation | 27 ns, no allocation | 290 ns, 2 allocations | 248 ns, 1 allocation |
| A PAGING of 16 tracking areas | 99 ns, 1 allocation | 52 ns, no allocation | 64 ns, no allocation | 565 ns, 2 allocations | 522 ns, 1 allocation |

`check()`, which decodes the value of each IE, and `check_structure()`, which
compares the container alone with the object set of its message, on the PDUs
of the comparison (`cargo run --release --bin checks` in `benches/`):

| PDU (octets) | `check()` | `check_structure()` | `check()` / `check_structure()` |
|---|---|---|---|
| UEContextReleaseRequest (25) | 392 ns, 6 allocations | 64 ns, 2 allocations | 6.1x |
| InitialUEMessage (70) | 777 ns, 10 allocations | 104 ns, 2 allocations | 7.5x |
| InitialContextSetupRequest (212) | 1,926 ns, 21 allocations | 196 ns, 2 allocations | 9.8x |
| NGSetupResponse (58) | 914 ns, 12 allocations | 83 ns, 2 allocations | 11.0x |
| PDUSessionResourceSetupRequest (192) | 715 ns, 11 allocations | 73 ns, 2 allocations | 9.8x |
| HandoverRequest (238) | 2,182 ns, 24 allocations | 198 ns, 2 allocations | 11.0x |

These were taken on the same machine, each program on one core, as the median
of three runs.

A clean `cargo build -j 5` of the crate and its dependencies on Rust 1.88, on
five cores of the same machine, the median of three builds, with the debug
information of the default profile and without it:

| Build | Time | Peak memory | Peak memory, `CARGO_PROFILE_DEV_DEBUG=0` |
|-------|------|-------------|------------------------------------------|
| No feature | 15 s | 1.36 GiB | 1.09 GiB |
| `inspect` | 24 s | 2.02 GiB | 1.57 GiB |
| `serde` | 24 s | 2.11 GiB | 1.60 GiB |
| `inspect`, `--release` | 35 s | 1.60 GiB | 1.59 GiB |

On a message of 65,535 IEs of one name, 393 KB, `decode` takes 0.01 ms, `check`
6.40 ms, `to_view` 69 ms and 37 MB, `with_view` 120 ms and 48 MB,
`inspect::paths` 25 ms, and an edit at a path 0.01 ms, on one core of the same
machine, the median of three runs. In `benches/`, `cargo bench --bench scale`
measures them, and `cargo bench --bench codec` and `--bench allocations` the
codec on the PDUs of the tests; from the root of the repository,
`python3 ../oxirush-aper/protocols/benches/build_cost.py +1.88.0 --features inspect`
measures a clean build.

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
from this page). Sign off your commits
(`git commit -s`): by contributing you agree to the
[Developer Certificate of Origin](https://developercertificate.org/).

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

[`inspect`]: https://docs.rs/oxirush-ngap/0.5.0/oxirush_ngap/inspect/index.html
[`from_tree()`]: https://docs.rs/oxirush-ngap/0.5.0/oxirush_ngap/ngap/ngap_pdu_descriptions/enum.NGAPPDU.html#method.from_tree
[`to_view`]: https://docs.rs/oxirush-ngap/0.5.0/oxirush_ngap/ngap/ngap_pdu_descriptions/enum.NGAPPDU.html#method.to_view
