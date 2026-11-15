# Benchmarks of oxirush-ngap

A package of its own, which the published crate does not have. Its programs
are those of `protocols/benches/` of
[oxirush-aper](https://github.com/linouxis9/oxirush-aper), in the checkout
beside this repository, which oxirush-ngap and oxirush-s1ap share; what is of
the protocol is in `src/` and `compare/` here.

The README sums up the comparison with the other implementations of NGAP,
and [The six PDUs](#the-six-pdus) has its figures. Every figure here was taken
on one machine, an Intel Xeon E5-2680 v4 (Broadwell, 2.4 GHz; a KVM guest bound to
NUMA node 0; Linux 7.0, Rust 1.100.0), on 2026-11-15, at e09770b.

## The six PDUs

The comparison of the [README](../README.md#performance), on the six PDUs of `compare/pdus.tsv`, which each implementation decodes and
encodes. The envelope alone is `NGAPPDU::decode`, which leaves the value of
each IE in its octets; "this crate" decodes the PDU and then every IE of its
message as its type, which is what the others do; and encoding writes the PDU
from the typed values of its IEs, as `build_ngap!` does.

Decoding, ns per call:

| PDU (octets) | this crate, the envelope alone | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | free5GC ngap 1.2.0 | pycrate 0.8.1 |
|---|---|---|---|---|---|---|---|
| UEContextReleaseRequest (25) | 29 | 206 | 1,052 | 1,198 | 3,431 | 2,186 | 125,142 |
| InitialUEMessage (70) | 30 | 323 | 2,267 | 3,290 | 5,844 | 3,745 | 233,039 |
| InitialContextSetupRequest (212) | 30 | 899 | 6,073 | 8,437 | 13,452 | 8,669 | 812,835 |
| NGSetupResponse (58) | 30 | 633 | 3,194 | 4,650 | 7,124 | 4,701 | 322,949 |
| PDUSessionResourceSetupRequest (192) | 30 | 368 | 1,994 | 3,570 | 6,630 | 4,087 | 497,047 |
| HandoverRequest (238) | 31 | 973 | 6,664 | 9,681 | 15,316 | 9,552 | 873,394 |

The time of each other implementation over this crate's, decoding:

| PDU (octets) | rasn / this crate | hampi / this crate | asn1c / this crate | free5GC / this crate | pycrate / this crate |
|---|---|---|---|---|---|
| UEContextReleaseRequest (25) | 5.1x | 5.8x | 17x | 11x | 610x |
| InitialUEMessage (70) | 7.0x | 10x | 18x | 12x | 720x |
| InitialContextSetupRequest (212) | 6.8x | 9.4x | 15x | 9.6x | 900x |
| NGSetupResponse (58) | 5.0x | 7.3x | 11x | 7.4x | 510x |
| PDUSessionResourceSetupRequest (192) | 5.4x | 9.7x | 18x | 11x | 1,300x |
| HandoverRequest (238) | 6.8x | 10x | 16x | 9.8x | 900x |

Encoding, ns per call:

| PDU (octets) | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | free5GC ngap 1.2.0 | pycrate 0.8.1 |
|---|---|---|---|---|---|---|
| UEContextReleaseRequest (25) | 158 | 2,204 | 2,137 | 1,976 | 1,199 | 82,217 |
| InitialUEMessage (70) | 216 | 3,178 | 3,875 | 3,282 | 1,766 | 141,552 |
| InitialContextSetupRequest (212) | 573 | 6,633 | 9,388 | 7,505 | 3,953 | 557,557 |
| NGSetupResponse (58) | 302 | 3,646 | 4,697 | 3,412 | 2,085 | 199,897 |
| PDUSessionResourceSetupRequest (192) | 263 | 3,281 | 5,095 | 4,422 | 2,321 | 365,377 |
| HandoverRequest (238) | 615 | 7,664 | 10,663 | 8,608 | 4,487 | 601,733 |

The time of each other implementation over this crate's, encoding:

| PDU (octets) | rasn / this crate | hampi / this crate | asn1c / this crate | free5GC / this crate | pycrate / this crate |
|---|---|---|---|---|---|
| UEContextReleaseRequest (25) | 14x | 14x | 12x | 7.6x | 520x |
| InitialUEMessage (70) | 15x | 18x | 15x | 8.2x | 650x |
| InitialContextSetupRequest (212) | 12x | 16x | 13x | 6.9x | 970x |
| NGSetupResponse (58) | 12x | 16x | 11x | 6.9x | 660x |
| PDUSessionResourceSetupRequest (192) | 12x | 19x | 17x | 8.8x | 1,400x |
| HandoverRequest (238) | 12x | 17x | 14x | 7.3x | 980x |

The figures were taken at e09770b, by [the method of the comparison](#the-method-of-the-comparison).

## Every figure of the comparison for HandoverRequest

The programs, the machine and the method of [the six PDUs](#the-six-pdus), per call:

|  | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | free5GC ngap 1.2.0 | pycrate 0.8.1 |
|---|---|---|---|---|---|---|
| decode ns | 973 | 6,664 | 9,681 | 15,316 | 9,552 | 873,394 |
| decode cycles | 2,817 | 19,486 | 27,976 | 43,666 | 34,453 | 2,521,899 |
| decode instructions | 6,827 | 40,254 | 69,558 | 118,852 | 95,444 | 4,475,049 |
| decode allocations | 3 | 35 | 76 | 121 | 179 |  |
| decode octets allocated | 584 | 2,168 | 7,641 | 15,532 | 2,952 |  |
| encode ns | 615 | 7,664 | 10,663 | 8,608 | 4,487 | 601,733 |
| encode cycles | 1,759 | 22,195 | 30,608 | 24,359 | 16,432 | 1,730,210 |
| encode instructions | 4,436 | 47,035 | 89,813 | 65,578 | 42,703 | 2,984,052 |
| encode allocations | 2 | 53 | 27 | 32 | 36 |  |
| encode octets allocated | 497 | 2,171 | 2,315 | 4,504 | 1,184 |  |
| most memory held, KiB | 4,364 | 5,256 | 4,086 | 6,392 | 9,566 | 36,828 |

The time, the cycles and the instructions of each other implementation over this crate's:

|  | rasn / this crate | hampi / this crate | asn1c / this crate | free5GC / this crate | pycrate / this crate |
|---|---|---|---|---|---|
| decode ns | 6.8x | 10x | 16x | 9.8x | 900x |
| decode cycles | 6.9x | 9.9x | 16x | 12x | 900x |
| decode instructions | 5.9x | 10x | 17x | 14x | 660x |
| encode ns | 12x | 17x | 14x | 7.3x | 980x |
| encode cycles | 13x | 17x | 14x | 9.3x | 980x |
| encode instructions | 11x | 20x | 15x | 9.6x | 670x |

## The method of the comparison

Every operation of every implementation ran once a round, a call repeated
for about 0.2 s, in turns, ten rounds: a time is the median of the ten, and a
ratio of times is given where the ten times of the two do not overlap (they
overlap for none here). The cycles and the instructions, the median of the
rounds, are those that `perf stat` counts for the calls alone: the
difference between a run of twice the calls and one of the calls. The
allocations and the octets that they ask for are counted by an allocator
that counts in Rust, by `malloc`, `calloc` and `realloc` for asn1c, by
`runtime.MemStats` for Go; pycrate's are not counted. The most memory held is
the high-water mark of the resident memory of the program (`VmHWM`), its runtime
included. The figures were taken at e09770b on an Intel Xeon E5-2680 v4
(Broadwell, 2.4 GHz; a KVM guest bound to NUMA node 0; Linux 7.0, Rust
1.100.0, Go 1.27.4, Python 3.14), each program on core 3, on 2026-11-15, with the script, which fetches each implementation at a pinned version: hampi 0.7.2 and rasn
0.29.0 from crates.io, Open5GS v2.8.0 from GitHub, the ngap package v1.2.0 of
free5GC through its go.mod, pycrate 0.8.1 from PyPI, and the ASN.1 from the
archive of 3GPP.
From the root of the repository, with oxirush-aper beside it:

```sh
python3 ../oxirush-aper/protocols/benches/compare/compare.py --core 3 --perf "sudo -n perf"
```

## Reading the IEs, and checking a PDU

Reading the IEs of a message and checking a PDU, per call, on the four
messages that `benches/` writes: `decode_value()` and then the macro on the
message, which decodes the container of the message first; the macro on the
PDU (`extract_ngap_ies!(pdu, ...)`), which walks the octets of its message once and
decodes the IEs that its fields ask for; and `ie::<T>()` of the PDU, which
decodes one IE. In `benches/`, `cargo run --release --bin readers --features
readers` measures them:

| Message | The first IE: `decode_value()`, then the macro | Allocations | The macro on the PDU | Allocations | `ie::<T>()` | Allocations | Every IE: `decode_value()`, then the macro | Allocations | The macro on the PDU | Allocations |
|---|---|---|---|---|---|---|---|---|---|---|
| An INITIAL CONTEXT SETUP REQUEST of 2 PDU sessions and a NAS-PDU of 1,500 octets | 221 ns | 1 | 72 ns | 0 | 46 ns | 0 | 1,126 ns | 3 | 963 ns | 2 |
| A PDU SESSION RESOURCE SETUP REQUEST of 8 PDU sessions | 133 ns | 1 | 44 ns | 0 | 46 ns | 0 | 865 ns | 2 | 793 ns | 1 |
| A HANDOVER REQUIRED of a container of 4,000 octets | 176 ns | 1 | 56 ns | 0 | 46 ns | 0 | 598 ns | 2 | 474 ns | 1 |
| A PAGING of 16 tracking areas | 191 ns | 1 | 102 ns | 0 | 110 ns | 0 | 991 ns | 2 | 924 ns | 1 |

`check()`, which decodes the value of each IE, and `check_structure()`, which
compares the container alone with the object set of its message, on the PDUs
of the comparison (`cargo run --release --bin checks` in `benches/`):

| PDU (octets) | `check()` | Allocations | `check_structure()` | Allocations | `check()` / `check_structure()` |
|---|---|---|---|---|---|
| UEContextReleaseRequest (25) | 820 ns | 6 | 258 ns | 3 | 3.2x |
| InitialUEMessage (70) | 1,454 ns | 10 | 337 ns | 3 | 4.3x |
| InitialContextSetupRequest (212) | 3,624 ns | 21 | 508 ns | 3 | 7.1x |
| NGSetupResponse (58) | 1,792 ns | 12 | 304 ns | 3 | 5.9x |
| PDUSessionResourceSetupRequest (192) | 1,433 ns | 11 | 278 ns | 3 | 5.2x |
| HandoverRequest (238) | 4,146 ns | 24 | 526 ns | 3 | 7.9x |

Each program ran on one core, and each figure is the median of ten runs.

A clean `cargo build -j 5` of the crate and its dependencies on Rust 1.88, on
five cores of the same machine, the median of five builds, with the debug
information of the default profile and without it:

| Build | Time | Peak memory | Peak memory, `CARGO_PROFILE_DEV_DEBUG=0` |
|-------|------|-------------|------------------------------------------|
| No feature | 20 s | 1.05 GiB | 0.93 GiB |
| `inspect` | 30 s | 1.61 GiB | 1.36 GiB |
| `inspect`, `--release` | 57 s | 1.47 GiB | the same |

On a message of 65,535 IEs of one name, 393 KB, `decode` takes 0.04 ms, `check`
14.2 ms, `to_tree` 25 ms and 9.4 MB, `from_tree` 11.5 ms, `inspect::paths`
27 ms, and an edit at a path 0.006 ms, on one core of the same machine, the
median of ten runs. In `benches/`, `cargo bench --bench scale`
measures them, and `cargo bench --bench codec` and `--bench allocations` the
codec on the PDUs of the tests; from the root of the repository,
`python3 ../oxirush-aper/protocols/benches/build_cost.py +1.88.0 --features inspect`
measures a clean build.
