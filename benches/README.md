# Benchmarks of oxirush-ngap

A package of its own, which the published crate does not have. Its programs
are those of `protocols/benches/` of
[oxirush-aper](https://github.com/linouxis9/oxirush-aper), in the checkout
beside this repository, which oxirush-ngap and oxirush-s1ap share; what is of
the protocol is in `src/` and `compare/` here.

The README sums up the comparison with the other implementations of NGAP,
and [The six PDUs](#the-six-pdus) has its figures. The figures below were taken on the same
machine, an AMD Ryzen 7 5700G (16 threads, Linux 7.0, Rust 1.98.1), measured
on a quiet machine on 2026-10-18, at aad0ccf.

## Reading the IEs, and checking a PDU

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
| NGSetupResponse (58) | 914 ns, 12 allocations | 83 ns, 2 allocations | 11x |
| PDUSessionResourceSetupRequest (192) | 715 ns, 11 allocations | 73 ns, 2 allocations | 9.8x |
| HandoverRequest (238) | 2,182 ns, 24 allocations | 198 ns, 2 allocations | 11x |

Each program ran on one core, and each figure is the median of three runs.

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

## The six PDUs

The comparison of the [README](../README.md#against-other-implementations),
on the six PDUs of `compare/pdus.tsv`, which each implementation decodes and
encodes. The envelope alone is `NGAP_PDU::decode`, which leaves the value of
each IE in its octets; "this crate" decodes the PDU and then every IE of its
message as its type, which is what the others do; and encoding writes the PDU
from the typed values of its IEs, as `build_ngap!` does.

Decoding, ns per call:

| PDU (octets) | this crate, the envelope alone | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | free5GC ngap 1.2.0 | pycrate 0.8.1 |
|---|---|---|---|---|---|---|---|
| UEContextReleaseRequest (25) | 8 | 88 | 636 | 594 | 1,670 | 1,150 | 68,916 |
| InitialUEMessage (70) | 8 | 151 | 1,389 | 1,706 | 2,797 | 1,878 | 127,498 |
| InitialContextSetupRequest (212) | 10 | 495 | 3,628 | 4,364 | 6,486 | 4,484 | 431,384 |
| NGSetupResponse (58) | 8 | 316 | 1,881 | 2,324 | 3,532 | 2,354 | 172,873 |
| PDUSessionResourceSetupRequest (192) | 10 | 180 | 1,230 | 1,805 | 3,177 | 2,099 | 267,008 |
| HandoverRequest (238) | 8 | 525 | 3,948 | 5,017 | 7,324 | 4,993 | 465,894 |

The time of each other implementation over this crate's, decoding:

| PDU (octets) | rasn / this crate | hampi / this crate | asn1c / this crate | free5GC / this crate | pycrate / this crate |
|---|---|---|---|---|---|
| UEContextReleaseRequest (25) | 7.2x | 6.7x | 19x | 13x | 780x |
| InitialUEMessage (70) | 9.2x | 11x | 18x | 12x | 840x |
| InitialContextSetupRequest (212) | 7.3x | 8.8x | 13x | 9.1x | 870x |
| NGSetupResponse (58) | 6.0x | 7.4x | 11x | 7.5x | 550x |
| PDUSessionResourceSetupRequest (192) | 6.8x | 10x | 18x | 12x | 1,500x |
| HandoverRequest (238) | 7.5x | 9.5x | 14x | 9.5x | 890x |

Encoding, ns per call:

| PDU (octets) | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | free5GC ngap 1.2.0 | pycrate 0.8.1 |
|---|---|---|---|---|---|---|
| UEContextReleaseRequest (25) | 63 | 889 | 1,314 | 898 | 578 | 45,156 |
| InitialUEMessage (70) | 99 | 1,265 | 2,640 | 1,474 | 888 | 77,259 |
| InitialContextSetupRequest (212) | 251 | 2,638 | 6,159 | 3,352 | 2,021 | 288,997 |
| NGSetupResponse (58) | 128 | 1,453 | 3,048 | 1,492 | 964 | 106,966 |
| PDUSessionResourceSetupRequest (192) | 119 | 1,347 | 3,054 | 1,987 | 1,128 | 192,710 |
| HandoverRequest (238) | 277 | 3,053 | 6,941 | 3,819 | 2,295 | 309,128 |

The time of each other implementation over this crate's, encoding:

| PDU (octets) | rasn / this crate | hampi / this crate | asn1c / this crate | free5GC / this crate | pycrate / this crate |
|---|---|---|---|---|---|
| UEContextReleaseRequest (25) | 14x | 21x | 14x | 9.1x | 710x |
| InitialUEMessage (70) | 13x | 27x | 15x | 9.0x | 780x |
| InitialContextSetupRequest (212) | 10x | 24x | 13x | 8.0x | 1,200x |
| NGSetupResponse (58) | 11x | 24x | 12x | 7.5x | 840x |
| PDUSessionResourceSetupRequest (192) | 11x | 26x | 17x | 9.5x | 1,600x |
| HandoverRequest (238) | 11x | 25x | 14x | 8.3x | 1,100x |

The figures were taken at aad0ccf, by [the method of the comparison](#the-method-of-the-comparison).

## Every figure of the comparison for HandoverRequest

The programs, the machine and the method of the comparison of the
[README](../README.md#against-other-implementations), per call:

|  | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | free5GC ngap 1.2.0 | pycrate 0.8.1 |
|---|---|---|---|---|---|---|
| decode ns | 525 | 3,948 | 5,017 | 7,324 | 4,993 | 465,894 |
| decode cycles | 2,327 | 17,973 | 23,137 | 33,063 | 28,849 | 2,141,808 |
| decode instructions | 6,573 | 41,848 | 69,702 | 118,711 | 95,642 | 4,442,005 |
| decode allocations | 3 | 35 | 76 | 121 | 179 |  |
| decode octets allocated | 584 | 2,168 | 7,641 | 15,532 | 2,952 |  |
| encode ns | 277 | 3,053 | 6,941 | 3,819 | 2,295 | 309,128 |
| encode cycles | 1,251 | 13,886 | 31,925 | 17,399 | 13,008 | 1,437,455 |
| encode instructions | 4,376 | 46,672 | 89,343 | 65,546 | 41,964 | 2,975,625 |
| encode allocations | 2 | 53 | 27 | 32 | 36 |  |
| encode octets allocated | 495 | 2,171 | 2,315 | 4,504 | 1,184 |  |
| most memory held, KiB | 3,884 | 4,652 | 3,440 | 6,336 | 9,436 | 37,416 |

The time, the cycles and the instructions of each other implementation over this crate's:

|  | rasn / this crate | hampi / this crate | asn1c / this crate | free5GC / this crate | pycrate / this crate |
|---|---|---|---|---|---|
| decode ns | 7.5x | 9.5x | 14x | 9.5x | 890x |
| decode cycles | 7.7x | 9.9x | 14x | 12x | 920x |
| decode instructions | 6.4x | 11x | 18x | 14x | 680x |
| encode ns | 11x | 25x | 14x | 8.3x | 1,100x |
| encode cycles | 11x | 26x | 14x | 10x | 1,100x |
| encode instructions | 11x | 20x | 15x | 9.6x | 680x |

## The method of the comparison

A time is the fastest of three runs of a call repeated for about 0.2 s. The
cycles and the instructions are those that `perf stat` counts for the calls
alone: the difference between a run of twice the calls and one of the calls.
The allocations and the octets that they ask for are counted by an allocator
that counts in Rust, by `malloc`, `calloc` and `realloc` for asn1c, by
`runtime.MemStats` for Go; pycrate's are not counted. The most memory held is
the high-water mark of the resident memory of the program (`VmHWM`), its runtime
included. The figures were taken at aad0ccf on an AMD Ryzen 7 5700G (16 threads, Linux
7.0, Rust 1.98.1, Go 1.27, Python 3.14), each program on one core, measured on
a quiet machine on 2026-10-18: each is the median of three runs of the script,
which fetches each implementation at a pinned version: hampi 0.7.2 and rasn
0.29.0 from crates.io, Open5GS v2.8.0 from GitHub, the ngap package v1.2.0 of
free5GC through its go.mod, pycrate 0.8.1 from PyPI, and the ASN.1 from the
archive of 3GPP.
The [README](../README.md#against-other-implementations) has the command.
