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
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"radio-connection-with-ue-lost"}}]}
```

This page describes 0.5.0, which is not released: it needs `oxirush-aper` and
`oxirush-utils`, which are not on crates.io yet, from the repositories that
[Installation](#installation) names.
[`oxirush-s1ap`](https://crates.io/crates/oxirush-s1ap) is the same crate for
S1AP, TS 36.413.

## Installation

```toml
[dependencies]
oxirush-ngap = "0.5"
# With the JSON forms:
# oxirush-ngap = { version = "0.5", features = ["inspect"] }

# Until oxirush-aper 0.1.0 and oxirush-utils 0.1.0 are on crates.io, at the
# revisions that this revision of the crate is tested with
[patch.crates-io]
oxirush-aper = { git = "https://github.com/linouxis9/oxirush-aper", rev = "e1b9077b2b882b113a25207db612b4d30d357af6" }
oxirush-utils = { git = "https://github.com/linouxis9/oxirush-utils", rev = "5c47d25abdddf234d4e4bc95b6f77c41f9372bdc" }
```

| Feature | Adds |
|---------|------|
| `inspect` | A PDU as a `serde_json::Value`: its tree, its view and the paths of its values. It turns on `serde_json/preserve_order` for the whole build, which keeps the members of a tree in the order of the specification |
| `serde` | `Serialize` and `Deserialize` for every type of the bindings, through its tree. It turns on `inspect` |

The minimum supported Rust version is 1.88. A clean `cargo build -j 5` of
the crate and its dependencies on Rust 1.88, on five cores of an x86-64
Linux machine that runs other work:

| Build | Time | Peak memory |
|-------|------|-------------|
| No feature | 18 s | 1.3 GiB |
| `inspect` | 31 s | 1.9 GiB |
| `serde` | 28 s | 1.9 GiB |
| `inspect`, `--release` | 37 s | 1.5 GiB |

The crate, oxirush-s1ap and `oxirush-aper` are released in step: a breaking
release of `oxirush-aper` is one of both protocol crates. The values of the
bindings are types of `oxirush-aper`, such as `Octets`, `Bits`, `BitString`
and `Error`: a program names them from the crate, or from
`oxirush_ngap::aper`, so that it has one version of each.

## Typed API

### Decode a PDU

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

The crate has no limit on the octets of a PDU, and each call takes time and
memory in proportion to them: the caller bounds what a peer can make it spend
by what it reads from SCTP. The messages that the cores sent in the lab runs
are 238 octets at most, and the longest of the vectors, which have the longest
lists of the specification, 106,555: a receiver that reads at most 128 KiB for
a PDU takes them all. On a message of 65,535 IEs, 393 KB, the most memory for
an octet:

| Call | Memory at most, for an octet of the PDU | On that message |
|------|-----------------------------------------|-----------------|
| `decode`, `decode_borrowed` | 1.3; none for `decode_borrowed` below 16384 octets | 0.02 ms |
| `protocol_ies` | 5.3 | 0.5 ms |
| `check` | 22 | 8 ms |
| `to_view`, `to_tree` | 162 | 138 to 201 ms, 64 MB |
| `from_tree` | 90 | 128 ms, 36 MB |
| `from_view`, `with_view` | 238 | 176 to 181 ms, 94 MB |
| `inspect::paths` | 78 | 53 ms |
| `inspect::set`, `remove`, `insert`, `open` | None: the view is edited in its place | 0.02 ms |

A typed value takes what its type holds: the longest list of connections of
an NG RESET, 65,536 items in 32 KB, is 4.7 MB as its type. `cargo bench
--bench scale`, in `benches/`, measures the table.

### Keep a value after its PDU

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

A function that is given octets returns the message that borrows them,
`NGAP_PDU::decode_borrowed(octets)?.message()`; one whose PDU is a variable of
its own returns an owned value (E0515). A message may be held across an
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

### Build a PDU

`build_ngap!` takes the direction, the procedure, the criticality of the PDU,
the type of the message, then each IE as `CRITICALITY Name(value)`, as in the
first example. The name of an IE is its ASN.1 identifier without `id-` and
with `_` for `-`: `id-AMF-UE-NGAP-ID` is `AMF_UE_NGAP_ID`. A name that the
crate does not have, a message that the procedure does not have in that
direction and a missing criticality do not compile. The macro gives a
`Result`: a value that does not encode, as an integer outside its
constraint, is an error.

The macros write the criticalities that they are given. `check()` reports
one that is not the one that TS 38.413 assigns, and [`from_tree()`] writes
those that it assigns.

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
name for its decoded value; after `=>` comes what the field keeps of it.

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

`with_ngap_ie_mut!` gives `Ok(false)` for an IE that is not in the message;
`=> expression` in place of `= value` changes the decoded value and says
whether it did. Where an IE is in a message twice, it and
`extract_ngap_ies!` take the last one.

### The IEs of any message

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

Every message but the private one is a container of protocol IEs: these
give them in the order of the message, an IE that it cannot have or has
twice among them, and `set_protocol_ies` checks nothing.

### A transfer and a handover container, typed

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

### Check a message against its object set

The ASN.1 gives each message an object set: the IEs that it can have, in
their order, each with a criticality and a presence. `check()` reports what a
message has otherwise, and refuses nothing: the macros build such a message
and the codec encodes it, so that a test of a peer can send it.

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

`check()` decodes the value of each IE, not the IEs that a value holds as
octets, and does not judge a conditional IE: its condition is prose.

### Answer a faulty message

Clause 10 of TS 38.413 has what the receiver of a message that deviates
answers: `cause()` of a deviation is the protocol cause, and
`diagnostics_item()` the item of the Criticality Diagnostics, for an IE of
the criticality `reject` or `notify`.

```rust
use oxirush_ngap::{Deviation, build_ngap, ngap::*};

fn main() -> Result<(), oxirush_ngap::Error> {
    // An NG SETUP REQUEST with none of its mandatory IEs but the paging DRX
    let request = build_ngap!(InitiatingMessage, NGSetup, REJECT, NGSetupRequest,
        IGNORE DefaultPagingDRX(PagingDRX::v128),
    )?;
    let deviations = NGAP_PDU::decode(&request.encode()?)?.check()?;

    // Clause 10.3.5: the procedure is rejected, and the missing IEs are reported
    let cause = deviations.iter().find_map(Deviation::cause).unwrap();
    let items: Vec<_> = deviations.iter().filter_map(Deviation::diagnostics_item).collect();
    let diagnostics = CriticalityDiagnosticsIEList(items);
    let failure = build_ngap!(UnsuccessfulOutcome, NGSetup, REJECT, NGSetupFailure,
        IGNORE Cause(Cause::protocol(cause)),
        IGNORE CriticalityDiagnostics(CriticalityDiagnostics::new(
            None, None, None, Some(diagnostics), None,
        )),
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
40150015000002000f40016200134009080100001b40006640
```

### Identities and capabilities

```rust
use oxirush_ngap::helpers::*;

// A PLMN from its MCC and its MNC, and what is identified within it
let network = plmn("208", "93").unwrap();
let amf = guami(network, 1, 1, 0).unwrap(); // region, set, pointer
let tracking_area = tai(network, [0x00, 0x00, 0x01]); // TAC
let cell = nr_cgi(network, 0x123456, 1).unwrap(); // gNB, cell
let gnb = global_gnb_id(network, 0x123456).unwrap();

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
`13 00 14`. `plmn_from` reads the MCC and the MNC of a PLMN identity. No
helper panics, and none cuts a number short.

### Errors

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

### The names of the fields

A field has the words of the identifier that ASN.1 gives its member, in lower
case with `_` between them: `pLMNIdentity` is `plmn_identity`, `tAC` is
`tac`, `iE-Extensions` is `ie_extensions`, `nRCellIdentity` is
`nr_cell_identity`, `gNB-ID` is `gnb_id`, `gTP-TEID` is `gtp_teid`. The
documentation of the crate has the rules, and `build/fields.tsv` of the
repository each field with its name in 0.4.2. A `SEQUENCE` is a struct that a
literal builds, and its `new` takes the members in their order; an extensible
`CHOICE` or `ENUMERATED` is `#[non_exhaustive]`.

## JSON: trees, views and paths

With the `inspect` feature a PDU is a `serde_json::Value` in two forms: a
tree, the name of its message and its IEs by name, in which a message is
shown and written, and a view, which keeps what was received and is read and
edited by paths with the functions of the [`inspect`] module.

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
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
{"id":15,"criticality":"ignore","value":{"radioNetwork":"user-inactivity"},"_raw_value":"0500","_original_id":15,"_ie_name":"Cause"}
```

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
assert_eq!(octets, [&json!("002a")]);

// A name that the type does not have is refused, and the view is as it was
let wrong = inspect::set(&mut view, "/ngap/Cause/radioNetwrok", json!("unspecified"));
println!("{}", wrong.unwrap_err());
```

```text
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":42},{"RAN-UE-NGAP-ID":9},{"Cause":{"nas":"normal-release"}}]}
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions; an IE itself has id, criticality, value, octets, at /ngap/Cause/radioNetwrok
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
[{"RAN-UE-NGAP-ID":7},{"60000":{"octets":"c0ffee"},"criticality":"ignore"},{"Cause":{"radioNetwork":"user-inactivity"}},{"RANNodeName":"gnb-1","criticality":"ignore"}]
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
{"id":65000,"criticality":"reject","value":"c0ffee","_decode_error":"NGAP IE 65000 is unknown","_raw_value":"c0ffee","_original_id":65000}
{"message":"UEContextReleaseRequest","ies":[{"AMF-UE-NGAP-ID":255},{"65000":{"octets":"c0ffee"},"criticality":"reject"}]}
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
{"message":"InitialContextSetupRequest","ies":[{"AllowedNSSAI":[{"s-NSSAI":{"sST":2}},{"s-NSSAI":{"sST":3}}]}]}
```

### A transfer and its members

A view shows the decoded value of a transfer beside its octets, and a tree
its decoded value alone.

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
{"_raw_transfer":"0000010086000100","decoded":{"protocolIEs":[{"id":134,"criticality":"reject","value":"ipv4","_raw_value":"00","_original_id":134,"_ie_name":"PDUSessionType"}]}}
{"PDUSessionResourceSetupListSUReq":[{"pDUSessionID":5,"s-NSSAI":{"sST":1},"pDUSessionResourceSetupRequestTransfer":{"protocolIEs":[{"PDUSessionType":"ipv6","criticality":"reject"}]}}]}
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

A name is read in any case and with any separators, as `"Normal Release"`
above, and shown as the specification spells it.

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
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions; an IE itself has id, criticality, value, octets, at /ngap/Cause/radioNetwrok
"valeu" is not a member of this value, which has none; an IE itself has id, criticality, value, octets, at /ngap/RAN-UE-NGAP-ID/valeu
"GUAMI" is not an IE of UEContextReleaseRequest, which has AMF-UE-NGAP-ID, RAN-UE-NGAP-ID, PDUSessionResourceListCxtRelReq, Cause, GW-Context-Release-Indication, at /ngap/GUAMI
```

The functions `message_ies`, `message_ie_criticalities`, `item_criticality`
and the others of [`inspect`] give what the ASN.1 says of each message, which
`check_path` reads.

### The container of a handover

Three IEs carry octets that TS 38.413 gives by reference: between NG-RAN
nodes, a type of the specification, as `SourceToTarget-TransparentContainer`
carries a `SourceNGRANNode-ToTargetNGRANNode-TransparentContainer` (clause
9.3.1.20). Nothing in a message says which, so a tree and a view show the
octets, and the one that writes or reads them names the type.

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
{"SourceToTarget-TransparentContainer":"000201020002f8390000012340080100"}
{"rRCContainer":"0102","targetCell-ID":{"nR-CGI":{"pLMNIdentity":"208-93","nRCellIdentity":4660}},"uEHistoryInformation":[{"lastVisitedCellInformation":{"eUTRANCell":"00"}}]}
{"SourceToTarget-TransparentContainer":"000201020002f8390000000070080100"}
```

### Any format of serde

With the `serde` feature, a PDU is serialized as its tree and deserialized
from one, in any format that serde has, and each other type of the bindings
in the form of its ASN.1 type.

```rust
use oxirush_ngap::ngap::{NGAP_PDU, SNSSAI, SST};

let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
let pdu = NGAP_PDU::decode(&bytes).unwrap();
let text = serde_json::to_string(&pdu).unwrap();
let read: NGAP_PDU = serde_json::from_str(&text).unwrap();
assert_eq!(read.encode().unwrap(), bytes);

// A value alone has the form of its type: an SST is its octet
let slice = SNSSAI::new(SST([1]), None, None);
println!("{}", serde_json::to_string(&slice).unwrap());
```

```text
{"sST":"01"}
```

### Values

A tree and a view show the values of some types as they are usually
written, and take them so or in the form of their ASN.1 type.

| Type | Notation |
|------|----------|
| `PLMNIdentity` | `"208-93"`, the MCC and the MNC |
| `TransportLayerAddress` | `"10.0.0.1"`, `"2001:db8::1"`, or the two with a comma |
| `TAC`, `EPS-TAC`, `LAC`, `GTP-TEID`, `FiveG-TMSI`, `SST`, `PortNumber`, `AMFRegionID`, `AMFSetID`, `AMFPointer`, `NRCellIdentity`, `EUTRACellIdentity`, `UL-NAS-Count` | a number, also read as a `"0x…"` string |
| `ENUMERATED` | the name of the value, read in any case |
| `INTEGER` | a number, which 64 bits hold, as the bindings hold it |
| a bit string whose size varies, as the identifier of a gNB | `{"value": "000004", "length": 22}`: its octets in hexadecimal, from its first bit, and its length in bits |
| a bit string of another size than the one that its type gives beside an extension marker | `{"value": "c00000", "length": 24}`, where a mask of the 16 bits of its type is `"c000"` |
| the other strings: an SD, keys, the NAS-PDU and the other containers | hexadecimal, written in lower case and read in either |

The documentation of the [`inspect`] module has the rules of the paths, of
the edits and of the values, and the kinds of the errors of the JSON forms.

## Examples

| Example | Shows | Feature |
|---------|-------|---------|
| `ngap_build_pdu` | Initial Context Setup request and response, UE Context Release, Handover Required and Acknowledge, NG Setup Failure, and one IE built alone | |
| `ngap_decode_manually` | An NG Setup Response built and read without the macros: the identifier, the criticality and the octets of each IE | |
| `ngap_extract_ies` | UE release, handover, and nested PDU session and NAS data read from decoded messages | |
| `ngap_inspect` | An NG Setup Request: its values with their paths, one IE edited, one added by its value and one by its octets, and the PDU encoded again | `inspect` |

```bash
cargo run --example ngap_build_pdu
cargo run --example ngap_inspect --features inspect
```

## What the crate does not do

- Procedures, state and transport: it is a codec.
- Judge a conditional IE: `check()` reports what the object sets say, and a
  condition is prose.
- Keep what a later release added to a value that is decoded as its type and
  encoded again: a PDU keeps the octets of its IEs until one is decoded.
- Look into an RRC container, or into the container of a handover for
  another system than NG-RAN: they stay octets.

## Specifications

| Specification | Version | For |
|---------------|---------|-----|
| 3GPP TS 38.413 | V19.4.0 (Release 19), `38413-j40.zip` | The six ASN.1 modules of clause 9.4, and clause 9.3 for what the ASN.1 leaves to prose |
| ITU-T X.691 | (02/2021) | Aligned PER |

## Tests

```bash
cargo test --all-features
cargo test
cd oracle && cargo test   # against the codec of rasn
```

| Test | What it holds |
|------|---------------|
| `aper_vectors` | The 144 messages and the 479 vectors that pycrate 0.8.1 encoded from the ASN.1: each decodes to its type and encodes to the same octets, and a truncation or one more octet is refused |
| `vectors` | With `inspect`, each vector through its view, its values alone and its tree, with every IE, transfer and container read with its type |
| `captured` | 177 PDUs that the AMF of Open5GS 2.8.0, the AMF of free5GC 4.2.3 and PacketRusher as gNB sent in lab runs, read and written back as the same octets |
| `aper_round_trip`, `unknown_extensions`, `allocation` | The macros, the encodings that pycrate and rasn write otherwise than X.691, the additions of a later release, and memory reserved for what is read, not for a count that a length claims |
| `message`, `protocol_ies`, `check`, `diagnostics` | The lifetimes of a message, the IEs of any message, what `check()` reports and what a receiver answers |
| `errors`, `keys`, `readable`, `plmn_identity` | The errors with their kinds and words, `Debug` that prints no key, how an identity prints, and the octets of a PLMN identity, which tshark reads as the same network |
| `inspection`, `many_ies`, `serde` | The JSON forms of each message, a message of 65,535 IEs, and serde |
| `readme`, `shared_files`, `revisions` | The output of each block of this page, the files shared with oxirush-s1ap, and the pinned revisions |
| `oracle/tests/` | Each vector and each captured PDU, cut, lengthened and with changed bits, read by the crate and by rasn 0.29 as the same value, or by neither |

The package that crates.io holds has these tests and their fixtures, and
`tests/package.sh` runs them from the package alone. The vectors are
pycrate's and the captured PDUs come from two open-source cores and one
open-source simulator: 18 of the 144 messages were seen on a wire, and no PDU
of a commercial RAN or core is among them, so what such equipment sends that
these did not is untested. [CONTRIBUTING.md](CONTRIBUTING.md) says how the
vectors were made and the PDUs captured.

## Fuzzing

`fuzz/` has three [cargo-fuzz](https://crates.io/crates/cargo-fuzz) targets,
which CI runs for a minute each: `decode` compares the crate with the codec
of rasn on any octets, `view` the JSON forms, and `paths` the paths and the
edits.

```bash
./fuzz/seed.sh
RUSTFLAGS="--cfg oxirush_ngap_hooks" cargo +nightly fuzz run decode -- -max_total_time=60
```

## Benchmarks

`benches/` is a package of its own: criterion for the time, a counting
allocator for the memory, pprof for the profiles, and `build_cost.py` for a
clean build.

```bash
cd benches
cargo bench --bench codec -- decode/
cargo bench --bench allocations
python3 build_cost.py +1.88.0 --features inspect
```

## Contributing

The bindings are generated: [CONTRIBUTING.md](CONTRIBUTING.md) says how, and
what a change to them, to the vectors or to the pinned revisions takes. Each
commit is signed off (`git commit -s`), under the
[Developer Certificate of Origin](https://developercertificate.org/).

## License

Copyright 2025-2026 Valentin D'Emmanuele

Licensed under the Apache License, Version 2.0. See [LICENSE](https://github.com/linouxis9/oxirush-ngap/blob/master/LICENSE) for details.

[`inspect`]: https://docs.rs/oxirush-ngap/0.5.0/oxirush_ngap/inspect/index.html
[`from_tree()`]: https://docs.rs/oxirush-ngap/0.5.0/oxirush_ngap/ngap/ngap_pdu_descriptions/enum.NGAPPDU.html#method.from_tree
