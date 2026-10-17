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
    ("benches/build.rs", 0x1b8b69137f40e100),
    ("benches/compare/asn1c.c", 0xe697c79e2a3e3d14),
    ("benches/compare/compare.py", 0xe54e220932f31dbf),
    ("benches/compare/hampi/Cargo.toml", 0x0d133edcad0a9e76),
    ("benches/compare/hampi/build.rs", 0x3d8c517628ca6864),
    ("benches/compare/hampi/src/main.rs", 0x3b86b18642820924),
    ("benches/compare/pycrate.py", 0x717bcb5fbe26ff78),
    ("benches/build_cost.py", 0x3ab37d9a89423f0c),
    ("benches/src/counting.rs", 0x613f75b3dd6a75fd),
    ("benches/src/bin/compare.rs", 0xaaa259d1b03a4e1f),
    ("benches/src/lib.rs", 0x806745e55dbb55ae),
    ("build/extract_asn1.py", 0xd5c10df676fed9aa),
    ("build/pin.py", 0x7c9f2c635006e491),
    ("fuzz/fuzz_targets/decode.rs", 0x45a0d53107995c8a),
    ("fuzz/fuzz_targets/paths.rs", 0x68daa1c78c9c02da),
    ("fuzz/fuzz_targets/view.rs", 0xa0e7481987d00ee0),
    ("fuzz/seed.sh", 0xbd4472a68d451bd2),
    ("fuzz/src/lib.rs", 0x3a30c38edb7b8632),
    ("oracle/src/lib.rs", 0x7a288146067c4ab3),
    ("oracle/src/per.rs", 0xdca5ab28574941f0),
    ("oracle/src/sized.rs", 0x65f4c1e8ee1593db),
    ("oracle/tests/codec.rs", 0x5ead793cb4ab1fce),
    ("oracle/tests/compare.rs", 0xe55a4f29880c055e),
    ("src/error.rs", 0x5e4af975e69a1a55),
    ("src/inspect_paths.rs", 0x142ccd6e3e5ccae5),
    ("src/message.rs", 0x352ebc559fb8e992),
    ("tests/fixtures/dissect.py", 0xdda0b5e438b19794),
    ("tests/fixtures/generate.py", 0x313265374d7367d9),
    ("tests/package.sh", 0xf01928838141a32f),
    ("tests/readme.rs", 0x3e39e3418a7e4743),
    ("tests/revisions.rs", 0x73ebe128c418bd47),
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
