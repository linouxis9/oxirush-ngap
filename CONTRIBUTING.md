# Contributing to oxirush-ngap

Each commit is signed off (`git commit -s`): a contribution is under the
[Developer Certificate of Origin](https://developercertificate.org/) and the
licence of the crate. This page is for those who change the generated code,
the fixtures of the tests or the pinned revisions; the README is for those
who use the crate.

## Generated code

Cargo compiles three checked-in files, and two more for the tests. A build,
docs.rs and the verification of a package run no generator, and the published
crate has neither the generator nor the ASN.1.

| File | Contents |
|------|----------|
| `src/ngap.rs` | The bindings: each type of the ASN.1 with its codec on `oxirush-aper`, its conversions and, with `inspect`, its JSON form, then the flat API, the convenience methods and the `Display` implementation |
| `src/registry.rs` | The list of the procedures and of the IEs, one line for each: its code or its identifier, its names and its types, and for each message of a procedure the IEs of its object set. The names that the macros take, `NgapPduKind` and the names and the types of `inspect` all expand from it, so each is written once |
| `src/inspect_registry.rs` | What `inspect` alone needs, with the `inspect` feature: among it the list of the types, one line for each `SEQUENCE` and each `CHOICE` with its members, which the paths are read against, and the types that a handover's containers carry, which the generator has from the prose of the specification |
| `oracle/src/bindings.rs`, `oracle/src/types.rs` | The oracle, a package of its own that its tests and the fuzz targets build, and the crate does not: the same types with the codec of rasn 0.29, and the list of those that the crate decodes by a name or by an identifier |

For maintainers, `build/` holds the generator, a package of its own whose
lockfile pins `rasn-compiler`. What the generators of the two protocols share
is `generator/` of [oxirush-aper](https://github.com/linouxis9/oxirush-aper),
which `build/` takes from a checkout of that repository beside this one. It
reads the six `.asn` modules in `ngap/`:
the ASN.1 of clause 9.4 of TS 38.413 V19.4.0, one file per module. Their
text is 3GPP's, and the repository does not have it. `build/asn1.sha256`
records the SHA-256 of each module, `build/extract_asn1.py` of oxirush-aper
takes them out of the archive that 3GPP publishes and compares them with it,
and the generator reads no module that has another digest. From the crate
directory, with oxirush-aper beside it:

```sh
curl -A "Mozilla/5.0" -O https://www.3gpp.org/ftp/Specs/archive/38_series/38.413/38413-j40.zip
python3 ../oxirush-aper/protocols/build/extract_asn1.py 38413-j40.zip
CARGO="$(command -v cargo)" cargo run --locked --manifest-path build/Cargo.toml
rustfmt --edition 2024 src/ngap.rs src/registry.rs src/inspect_registry.rs \
    oracle/src/bindings.rs oracle/src/types.rs
```

`rasn-compiler` finds rustfmt through `CARGO`. Commit the five files after
regenerating them, and do not edit them by hand. They were last regenerated
on 2026-10-16, with the generator of that revision and the rustfmt of Rust
1.88: they came out as they are committed, octet for octet. The `Reproduce`
workflow does it again from the archive of 3GPP when the generator, the
bindings or the vectors change, and on demand: it extracts the modules, runs
the generator and compares what it writes with the committed files, then
has pycrate write [the vectors](#the-vectors) and the crate write the values
of pycrate.

### What oxirush-ngap and oxirush-s1ap share

What the two protocol crates do alike is written once, in
[oxirush-aper](https://github.com/linouxis9/oxirush-aper): the library code in
the crate, generic, with what is of the protocol generated from the registry
of each, and the tools, the programs and the jobs in its repository, which
this repository takes from a checkout of oxirush-aper beside it, at the
revision of `build/revisions`, as `build/` takes the generator. A file of
`protocols/` there has the path that it had here.

| What | Where it lives |
|------|----------------|
| The traits of a protocol, the readers `ies::<T>()`, `ie::<T>()` and `items::<L>()`, `check()` and `check_structure()`, the errors that name an IE, and the bodies of the macros, which `src/macros.rs` and `src/pdu.rs` here forward to | `src/protocol/` of oxirush-aper, which the registry of each protocol, `src/registry.rs`, invokes; the paths of `inspect` in `src/inspect/` there |
| The generator of the bindings, of the registry and of the oracle | `generator/` of oxirush-aper; `build/main.rs` here says what is of NGAP |
| `extract_asn1.py`, `pin.py` | `protocols/build/` |
| The README test: `readme.py` writes `tests/readme.rs` from the README | `protocols/tests/readme.py` |
| `package.sh`, `workspace_member.sh` | `protocols/tests/` |
| The writer of the vectors and their reading by tshark: `generate.py`, `dissect.py` | `protocols/tests/fixtures/` |
| The oracle and its tests `codec.rs` and `compare.rs` | `protocols/oracle/`, which `oracle/Cargo.toml` builds with the bindings of `oracle/src/` and `oracle/tests/allocation.rs` here |
| The fuzz targets and `seed.sh` | `protocols/fuzz/`, which `fuzz/Cargo.toml` builds with the crate, which it names `codec`: its module `hooks` names what is of the protocol |
| The benchmarks, their programs, `build_cost.py` and the comparison with other implementations | `protocols/benches/`, which `benches/Cargo.toml` builds with `benches/src/protocol.rs`, `benches/src/readers.rs` and `benches/compare/pdus.tsv` here |
| The jobs of CI and of the Reproduce workflow | `.github/workflows/protocol.yml` and `protocol-reproduce.yml` of oxirush-aper, which the two workflows here call |

A change to a shared piece is a commit of oxirush-aper, which each protocol
crate takes when `build/revisions` records it.

### Until oxirush-aper and oxirush-utils are published

Neither is on crates.io yet, so the repository takes both from their
repositories. `build/revisions` records the commit of each, and every place
that names one names that commit and no branch: the patch of the README, the
workflows of oxirush-aper that the two workflows call, and the patch of
`oxirush-utils` in the manifests of `benches/`, `fuzz/` and `oracle/`, which
take `oxirush-aper` from the checkout beside the repository. The jobs, the
scripts and the Reproduce workflow, which checks oxirush-aper out beside the
repository, read the file. `python3 ../oxirush-aper/protocols/build/pin.py
--check`, which CI runs, fails when a place names another commit, so a
change of `oxirush-aper` or of `oxirush-utils` changes no result here before
a commit of this repository says so.

To move to another revision, write it in `build/revisions`, check
oxirush-aper out at it beside this repository, run
`python3 ../oxirush-aper/protocols/build/pin.py`, which writes it in each
place, generate the bindings again, and commit the three together.

When `oxirush-aper` 0.1.0 and `oxirush-utils` 0.1.0 are on crates.io, and
before this crate is published:

1. Delete the line of `oxirush-utils` from `[patch.crates-io]` of
   `benches/Cargo.toml`, `fuzz/Cargo.toml` and `oracle/Cargo.toml`. The line
   of `oxirush-aper` stays: those packages build the programs of the
   checkout beside the repository, with the crate of that checkout.
2. In oxirush-aper, delete `unpublished` and each `$UNPUBLISHED` from
   `.github/workflows/protocol.yml`, and what `protocols/tests/package.sh`
   and `protocols/tests/workspace_member.sh` write for the two crates when no
   directory is given for them.
3. Delete the patch and its lines from the Installation of the README, this
   section but for its last paragraph, and the line of `oxirush-utils` in
   `build/revisions`.
4. `cargo publish --dry-run` then builds the package against crates.io
   alone, and fails while a dependency is not there.

The line of `oxirush-aper` stays in `build/revisions`, with the commit of the
release: its `generator/`, its `protocols/` and its workflows are not
published, and the jobs check it out at that commit.

`oxirush-aper`, oxirush-ngap and oxirush-s1ap are released in step. Both
protocol crates depend on one version of `oxirush-aper` exactly (`=0.1.0` in
`Cargo.toml`): they build on its module `inspect`, which is hidden from its
documentation and changes with them, so a release of `oxirush-aper` comes with
a release of each, which takes that version.

### rasn-compiler and the oracle

`rasn-compiler` runs in its stable mode, which leaves an open type opaque:
TS 38.413 relies on parameterized information object classes, and the
experimental typed mode emits unresolved object sets and bindings that do not
compile for these modules. The value of an IE stays octets inside the
generated types, and the macros give typed construction, extraction and
mutation.

What `rasn-compiler` resolves is written twice: as the bindings of the crate,
and as bindings for rasn 0.29, in `oracle/`, which its tests and the fuzz
targets build and the crate does not. Both have the same types by
construction.

| Subject | What the generator does |
|---------|-------------------------|
| Containers | The protocol IE and extension containers take an object set as a parameter, which an opaque open type does not use. Each is one type, `ProtocolIEContainer` of `ProtocolIEField` and `ProtocolExtensionContainer` of `ProtocolExtensionField`, in place of the one that `rasn-compiler` resolves for each use. |
| Complete values | `NGAP_PDU::decode`, `decode_value` and the extraction of a typed IE refuse incomplete values and trailing whole octets. |

rasn 0.29's APER codec departs from ITU-T X.691 where NGAP reaches it. There
the oracle has a codec of its own in the place of the derived one, so that
the vectors, which pycrate wrote and tshark reads, hold for both:

| Value | Correction of the oracle |
|-------|--------------------------|
| Constrained `SEQUENCE OF` | rasn misaligns the length determinant and the components |
| Size-constrained `UTF8String` | Its size is not PER-visible, so its length is an unconstrained count of octets |
| Fixed-size `BIT STRING` longer than 16 bits | It is octet-aligned |
| `OCTET STRING` and `BIT STRING` with a two-octet length determinant | The length is octet-aligned wherever the string starts |
| Extensible `SEQUENCE` | Its decoder consumes and skips the additions that it does not know |


## The fixtures of the tests

### The vectors

`generate.py` of `protocols/tests/fixtures/` of oxirush-aper writes the three
files of vectors of `tests/fixtures/`, and with
`--check` it compares them with what is committed. It needs pycrate 0.8.1 and
the ASN.1 modules of [Generated code](#generated-code), and nothing of this
crate: pycrate compiles the modules with its own compiler and encodes, with
its own codec, values that the script makes from the compiled types.

| Vectors | What they hold |
|---------|----------------|
| `messages.tsv`, 144 | Each message with the IEs and the members that are mandatory, at their lower bounds |
| `vectors.tsv`, 447 `cover` | Each message with every IE of its object set and every member, in passes that take each alternative of each `CHOICE`, each value of each `ENUMERATED` but those of the type of handover, the bounds of each number and each size, the value after them where a constraint is extensible, and that leave members out: 1142 IEs of object sets, 181 alternatives and 839 values |
| 12 `length`, 6 `count`, 11 `bits` | A NAS-PDU, a list and a bit string around each form of the length determinant: 127 and 128, 16383 and 16384, and fragments, up to 65537 octets, 65536 entries and 131072 bits. The lengths are those of the string, of the open type of its IE and of the open type of the message |
| 3 `largest` | A list of 65535 entries in an extension container, a list of 4096, and a string of 9600 octets |
| `later.tsv`, 9 | Messages of a later release: `TAI`, `UserLocationInformationEUTRA` and `GUAMI` have additions after their extension marker, up to 64 of them, and each message is there without them too, as pycrate encodes it with the modules as they are |

Three readings of the vectors do not depend on each other:

- The crate reads each vector and writes the same octets, in `tests/aper_vectors.rs`.
  It reads a message of `later.tsv` as the message without the additions.
- pycrate has the value of each vector, and `generate.py --values FILE` writes
  them in the JSON encoding of ASN.1. An ignored test of `tests/vectors.rs`
  writes a PDU from each, and its octets are those of the vector, for each of
  the 479: the crate gives each name and each number of the reference the bits
  that the reference gave it.
- `dissect.py`, beside `generate.py`, gives each vector to tshark, which must read
  the procedure, the alternative of the PDU, the IEs of the message and the
  size that a vector is made for. Wireshark 4.6.4 reads them as the vectors
  have them, and 418 of the 479 vectors without a remark. In the others it
  stops in octets that hold a value of another specification, which are
  random in the vectors, or at one of the three lists in fragments, which it
  does not read: the IEs after that place are not compared. Of a message of
  `later.tsv` it says that it does not know the additions, and it stops at a
  type that has 64 of them, which are more than it takes.

```bash
python3 -m venv venv && venv/bin/pip install pycrate==0.8.1
venv/bin/python ../oxirush-aper/protocols/tests/fixtures/generate.py --check
venv/bin/python ../oxirush-aper/protocols/tests/fixtures/generate.py --values values.jsonl
REFERENCE_VALUES=values.jsonl cargo test --features inspect --test vectors -- --ignored
python3 ../oxirush-aper/protocols/tests/fixtures/dissect.py
```

`generate.py` repairs two defects of pycrate 0.8.1, which does not write two
values in fragments: for a list of 16384 entries or more it calls a function
that it does not have, and for a bit string it takes the octets of each
fragment after the first from the wrong place. Wireshark reads the bit
strings in fragments as the repaired pycrate writes them, and pycrate itself
reads back the lists, which Wireshark does not read. Two more are left alone,
and the types of `later.tsv` are chosen for what pycrate writes as X.691
(02/2021) has it. pycrate writes the count of more than 64 additions as a
normally small number, with the octets of its length, where 11.9.3.4 has a
length determinant: a type has 64 at most, and `tests/unknown_extensions.rs`
has more, written by hand. And pycrate pads before the first addition from
the place before the count and the bitmap of the additions, which gives an
octet too many where the members before them do not end an octet: Wireshark
does not read that octet either, and the types that have additions end one.

The crate and pycrate write one kind of value differently. A number that is
in the range of the root of an extensible constraint and is none of its
values, as 96 for `INTEGER (1..30|40|50, ...)`, is written in the root by the
crate, as X.691 (02/2021) 13.1 has a value "within the range of the extension
root", and after the extension bit by pycrate. Each reads what the other
writes, and the vectors have no such number.

### The captured PDUs

`tests/fixtures/captured.tsv` has 177 PDUs that other implementations sent,
where the vectors are what pycrate wrote: each distinct PDU of a sender,
captured with tcpdump on the SCTP association of lab runs, in which each core
ran from the sources of its release. Nothing of this crate wrote or chose
them. Each line has the octets, the procedure code and the alternative of the
PDU that their first two octets have, the sender with its revision, the
direction, and what tshark 4.6.4 says the frame is. The head of the file has
the full revisions.

| Sender | PDUs |
|--------|------|
| Open5GS v2.8.0 (`157f611a`), AMF | 85, to the RAN |
| free5GC v4.2.3 (`3b34a08e`), AMF v1.4.5 with free5gc/ngap v1.1.3 | 79, to the RAN |
| PacketRusher 20250225 (`3210a263`), with free5gc/ngap v1.0.8, as gNB | 13, to the core |

The procedures are registrations with sessions, deregistrations, service
requests, N2 and Xn handovers, and what a core answers to messages that are
deliberately wrong. The subscribers are test values, 208-93 0000000001 to
0000000008 with the key of the sample configuration of PacketRusher.

`tests/captured.rs` holds each of them, the comparison with the oracle of
`oracle/tests/compare.rs` gives each to both codecs with its truncations and its
changed bits, as it does the vectors, and the corpus of the fuzz targets
starts from them too. `dissect.py` gives them to tshark with
the vectors: Wireshark 4.6.4 reads the procedure, the alternative of the PDU
and the IEs of each as the file has them, and all 177 without a remark.

They are what these runs had: two open-source cores and one open-source
simulator, of which free5GC and PacketRusher have two versions of one NGAP
library, no commercial RAN or core, and 18 of the 144 messages, most often a
Downlink NAS Transport.

## Fuzzing

`fuzz/` has three [cargo-fuzz](https://crates.io/crates/cargo-fuzz) targets,
a package of its own that is not published. A target does not only look for a
panic: it gives its octets to the crate and to its oracle, the same types
with the codec of rasn, and compares what the two make of them.
CI runs each for a minute, from the vectors and the captured PDUs.

| Target | What holds for any octets |
|--------|---------------------------|
| `decode` | A PDU, the protocol IEs of its message and the message as its type are read by the crate and by rasn as the same value, which both write as the same octets, or by neither, and what was read is written and read again as the same |
| `view` | The view of a PDU gives that PDU back, a view and a tree are written to a PDU that has the same view and the same tree, each path of a view selects its value, and each value that a view decodes has the JSON form that the JSON Encoding Rules of rasn give the same octets |
| `paths` | `select`, `set`, `insert`, `remove`, `open`, `check_path` and `enumerated_at` take any path and any value without a panic, and a view that one of them changed is a PDU or an error |

```bash
cargo install cargo-fuzz
../oxirush-aper/protocols/fuzz/seed.sh
# What the targets take from the inside of the crate is behind its own name
export RUSTFLAGS="--cfg oxirush_ngap_hooks"
cargo +nightly fuzz run decode -- -max_total_time=60 -max_len=4096
```

## The tests that compare

The comparison with another implementation of aligned PER is in a package of
its own, `oracle/`, which depends on the crate: the crate does not depend on
it, so no build and no test of the crate builds rasn, and the published crate
does not have it. Its tests take what they need from the inside of the crate
through the module `hooks`, which the crate has with
`--cfg oxirush_ngap_hooks` and not without: `oracle/.cargo/config.toml` sets it
for a cargo that is run from that directory.

| Test | What it holds |
|------|---------------|
| `oracle/tests/compare.rs` | The crate against the oracle of `oracle/`, which has the same types with the codec of rasn 0.29. Each vector and each captured PDU, its truncations, it with one more octet and with each bit of chosen octets changed is given to both, as a PDU and as its message: both read it and write the same octets, or neither reads it. With `inspect`, each IE and each transfer that a view decodes has the JSON form that the JSON Encoding Rules of rasn give it, but for two bit strings that rasn writes otherwise than X.697 has them. The bindings hold an `INTEGER` in 64 bits, which is the one difference that the comparison takes. |
| `oracle/tests/codec.rs` | The codecs that the oracle has in the place of those that rasn 0.29 derives: the `SEQUENCE`, the list of a size constraint, and the strings whose length takes two octets. |
| `oracle/tests/allocation.rs` | The list codec that rasn derives reserves memory for the count that a length determinant claims, where the crate reserves for the elements that it reads. |

`../oxirush-aper/protocols/tests/package.sh DIRECTORY` has cargo make the
package, unpacks it and runs its tests there, with every feature and with
none. `../oxirush-aper/protocols/tests/workspace_member.sh DIRECTORY
[CRATE...]` makes a workspace of this crate and of the others that
it is given, as `oxirush-s1ap` and `oxirush-aper`: cargo loads it, clippy
accepts every target of every member, and rustdoc documents them. CI runs
both.

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
cd .. && python3 ../oxirush-aper/protocols/benches/build_cost.py +1.88.0 --features inspect  # a clean build
```

Until `oxirush-aper` and `oxirush-utils` are on crates.io, `build_cost.py` is
told where each is, before the other arguments: the `--config` options that
`UNPUBLISHED` of `.github/workflows/ci.yml` has.

`--profile-time` runs the benchmark under [pprof](https://crates.io/crates/pprof)
and writes a CPU profile that `go tool pprof` reads; with `PROFILE=flamegraph`
in the environment it writes `flamegraph.svg` beside it instead.
