//! The files that oxirush-ngap and oxirush-s1ap have in common, octet for octet: the
//! check of a message against its object set, the paths of the inspection, what names
//! a message and its errors, the oracle of the tests and the comparison with it, the
//! test of the README, the scripts of the vectors and of the ASN.1, the scripts that
//! make a workspace of the crate and that test its package, the test and the script of
//! the revisions of the crates that are not published, the benchmarks and the fuzz
//! targets. Each crate holds the digest of each file, so that an edit in one fails here
//! until the file is the same in the other and both record its new digest. This file
//! is one of them, and the two crates keep it the same too.
//!
//! The codec, its errors, the JSON forms and the generator of the bindings are not
//! copied: both crates take them from oxirush-aper.

use std::path::Path;

/// The shared files, and the FNV-1a digest of each as it was last mirrored.
const SHARED: &[(&str, u64)] = &[
    ("benches/benches/allocations.rs", 0x228fe2c8ba50d435),
    ("benches/benches/codec.rs", 0x904d8a5909d190c1),
    ("benches/benches/heap.rs", 0xffc0849cd2e0625d),
    ("benches/benches/scale.rs", 0xb584eff01e6248fe),
    ("benches/build_cost.py", 0x3ab37d9a89423f0c),
    ("benches/src/counting.rs", 0x613f75b3dd6a75fd),
    ("benches/src/lib.rs", 0x59c963921d8a9737),
    ("build/extract_asn1.py", 0xd5c10df676fed9aa),
    ("build/pin.py", 0x7c9f2c635006e491),
    ("fuzz/fuzz_targets/decode.rs", 0x45a0d53107995c8a),
    ("fuzz/fuzz_targets/paths.rs", 0x68daa1c78c9c02da),
    ("fuzz/fuzz_targets/view.rs", 0xa0e7481987d00ee0),
    ("fuzz/seed.sh", 0x45e3d85c62151ab4),
    ("fuzz/src/lib.rs", 0xf381badf0f82d147),
    ("oracle/src/lib.rs", 0x7418ce8b2e48d747),
    ("oracle/src/per.rs", 0xdca5ab28574941f0),
    ("oracle/src/sized.rs", 0x65f4c1e8ee1593db),
    ("oracle/tests/codec.rs", 0x5ead793cb4ab1fce),
    ("oracle/tests/compare.rs", 0x036cd0bea2502576),
    ("src/check.rs", 0x899169819fd24325),
    ("src/error.rs", 0x25ef2a01e092bf28),
    ("src/inspect_paths.rs", 0x1205807881dc5778),
    ("src/message.rs", 0x1cdcc3d36746fb8d),
    ("tests/fixtures/dissect.py", 0xdda0b5e438b19794),
    ("tests/fixtures/generate.py", 0x313265374d7367d9),
    ("tests/package.sh", 0xf01928838141a32f),
    ("tests/readme.rs", 0x8649417b32dae0f4),
    ("tests/revisions.rs", 0x1cfbf3fec493dc4d),
    ("tests/workspace_member.sh", 0xed255d2d6316a65a),
];

/// The 64-bit FNV-1a digest of `octets`, without the carriage returns that a checkout
/// may add to the ends of the lines.
fn digest(octets: &[u8]) -> u64 {
    let octets = octets.iter().filter(|octet| **octet != b'\r');
    octets.fold(0xcbf2_9ce4_8422_2325, |digest, octet| {
        (digest ^ u64::from(*octet)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

#[test]
fn the_files_shared_with_the_other_protocol_are_as_they_were_mirrored() {
    let mut read = 0;
    for (path, recorded) in SHARED {
        // The published crate has the Rust files of `src` and of `tests`, and neither
        // the generator, the benchmarks, the fuzz targets, the oracle nor the scripts.
        let Ok(octets) = std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)) else {
            let sources = path.starts_with("src/") || path.starts_with("tests/");
            let published = sources && path.ends_with(".rs");
            assert!(!published, "{path} is missing");
            continue;
        };
        read += 1;
        assert_eq!(
            digest(&octets),
            *recorded,
            "{path} changed: make it the same in oxirush-ngap and oxirush-s1ap, then record \
             {:#018x} for it in tests/shared_files.rs of both",
            digest(&octets)
        );
    }
    assert!(read >= 3, "the shared sources are read");
}
