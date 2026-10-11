//! The files that oxirush-ngap and oxirush-s1ap have in common, octet for octet: the
//! codec helpers, the paths of the inspection, the generators that both protocols run
//! and the test of the README. Each crate holds the digest of each file, so that an
//! edit in one fails here until the file is the same in the other and both record its
//! new digest. This file is one of them, and the two crates keep it the same too.

use std::path::Path;

/// The shared files, and the FNV-1a digest of each as it was last mirrored.
const SHARED: &[(&str, u64)] = &[
    ("build/aper_fix.rs", 0x0468956d2aad0ba6),
    ("build/containers.rs", 0x9d134011ce89fec9),
    ("build/inspection.rs", 0xe3950d08e5de3193),
    ("build/registry.rs", 0x8963c2b3e689ff5a),
    ("src/inspect_paths.rs", 0x815f2980e815b73e),
    ("src/per.rs", 0x80552549e724045f),
    ("src/sized.rs", 0xacd754552266ad78),
    ("tests/readme.rs", 0xe02499cb40541d3c),
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
        // The published crate has the sources and not the generator.
        let Ok(octets) = std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)) else {
            assert!(path.starts_with("build/"), "{path} is missing");
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
