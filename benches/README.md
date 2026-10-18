# Benchmarks of oxirush-ngap

A package of its own, which the published crate does not have. Its programs
are those of `protocols/benches/` of
[oxirush-aper](https://github.com/linouxis9/oxirush-aper), in the checkout
beside this repository, which oxirush-ngap and oxirush-s1ap share; what is of
the protocol is in `src/` and `compare/` here.

The comparison with the other implementations of NGAP is in the
[README](../README.md#performance). The figures below were taken on the same
machine, an AMD Ryzen 7 5700G (16 threads, Linux 7.0, Rust 1.98.1), measured
on a quiet machine on 2026-10-18.

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
| NGSetupResponse (58) | 914 ns, 12 allocations | 83 ns, 2 allocations | 11.0x |
| PDUSessionResourceSetupRequest (192) | 715 ns, 11 allocations | 73 ns, 2 allocations | 9.8x |
| HandoverRequest (238) | 2,182 ns, 24 allocations | 198 ns, 2 allocations | 11.0x |

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

## Every figure of the comparison for HandoverRequest

The programs, the machine and the method of the comparison of the
[README](../README.md#against-other-implementations), per call:

|  | decode ns | cycles | instructions | allocations | octets allocated | encode ns | cycles | instructions | allocations | octets allocated | most memory held, KiB |
|---|---|---|---|---|---|---|---|---|---|---|---|
| this crate | 525 | 2,327 | 6,573 | 3 | 584 | 277 | 1,251 | 4,376 | 2 | 495 | 3,884 |
| rasn 0.29 | 3,948 (7.5x) | 17,973 (7.7x) | 41,848 (6.4x) | 35 | 2,168 | 3,053 (11.0x) | 13,886 (11.1x) | 46,672 (10.7x) | 53 | 2,171 | 4,652 |
| hampi 0.7.2 | 5,017 (9.5x) | 23,137 (9.9x) | 69,702 (10.6x) | 76 | 7,641 | 6,941 (25.0x) | 31,925 (25.5x) | 89,343 (20.4x) | 27 | 2,315 | 3,440 |
| asn1c (Open5GS 2.8.0) | 7,324 (13.9x) | 33,063 (14.2x) | 118,711 (18.1x) | 121 | 15,532 | 3,819 (13.8x) | 17,399 (13.9x) | 65,546 (15.0x) | 32 | 4,504 | 6,336 |
| free5GC ngap 1.2.0 | 4,993 (9.5x) | 28,849 (12.4x) | 95,642 (14.5x) | 179 | 2,952 | 2,295 (8.3x) | 13,008 (10.4x) | 41,964 (9.6x) | 36 | 1,184 | 9,436 |
| pycrate 0.8.1 | 465,894 (886.7x) | 2,141,808 (920.6x) | 4,442,005 (675.8x) |  |  | 309,128 (1114.8x) | 1,437,455 (1148.7x) | 2,975,625 (679.9x) |  |  | 37,416 |
