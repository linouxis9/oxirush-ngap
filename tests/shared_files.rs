//! The files that oxirush-ngap and oxirush-s1ap have in common, octet for octet: the
//! error, the codec helpers, the JSON codec and the paths of the inspection, the
//! generators that both protocols run, the test of the README and the benchmarks. Each
//! crate holds the digest of each file, so that an edit in one fails here until the
//! file is the same in the other and both record its new digest. This file is one of
//! them, and the two crates keep it the same too.

use std::path::Path;

/// The shared files, and the FNV-1a digest of each as it was last mirrored.
const SHARED: &[(&str, u64)] = &[
    ("benches/benches/allocations.rs", 0x7e9caf0d00bb13de),
    ("benches/benches/codec.rs", 0x904d8a5909d190c1),
    ("benches/benches/heap.rs", 0xffc0849cd2e0625d),
    ("benches/build_cost.py", 0x84c70b1ce3c14ed6),
    ("benches/src/lib.rs", 0x296993cddf8c05c2),
    ("build/aper_fix.rs", 0x6f465c49ce231fc8),
    ("build/containers.rs", 0x9d134011ce89fec9),
    ("build/inspection.rs", 0xc1a15755d6ee7e94),
    ("build/registry.rs", 0xd1c828575adadf7c),
    ("src/check.rs", 0x6a14609dae13ca66),
    ("src/error.rs", 0x5b938dac1af61881),
    ("src/inspect_paths.rs", 0x7e8563ca61dc25f0),
    ("src/json.rs", 0x94b43ab85ed2ca05),
    ("src/message.rs", 0x80820382c540a0cc),
    ("src/per.rs", 0x8c08042e9798a7f9),
    ("src/sized.rs", 0x05d05e62217fc26a),
    ("src/wire.rs", 0x6e7764143db2ce5e),
    ("tests/readme.rs", 0x8649417b32dae0f4),
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
        // The published crate has the sources, and neither the generator nor the
        // benchmarks.
        let Ok(octets) = std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)) else {
            let published = !path.starts_with("build/") && !path.starts_with("benches/");
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
