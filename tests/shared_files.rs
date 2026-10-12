//! The files that oxirush-ngap and oxirush-s1ap have in common, octet for octet: the
//! error, the codec helpers, the paths of the inspection, the generators that both
//! protocols run, the test of the README and the benchmarks. Each crate holds the digest
//! of each file, so that an edit in one fails here until the file is the same in the
//! other and both record its new digest. This file is one of them, and the two crates
//! keep it the same too.

use std::path::Path;

/// The shared files, and the FNV-1a digest of each as it was last mirrored.
const SHARED: &[(&str, u64)] = &[
    ("benches/benches/allocations.rs", 0x7e9caf0d00bb13de),
    ("benches/benches/codec.rs", 0x904d8a5909d190c1),
    ("benches/benches/heap.rs", 0xffc0849cd2e0625d),
    ("benches/build_cost.py", 0x84c70b1ce3c14ed6),
    ("benches/src/lib.rs", 0x296993cddf8c05c2),
    ("build/aper_fix.rs", 0x00b83a662f2e1d4c),
    ("build/containers.rs", 0x9d134011ce89fec9),
    ("build/inspection.rs", 0xcc54553d89f026e5),
    ("build/registry.rs", 0xd1c828575adadf7c),
    ("src/check.rs", 0xa22672c09f0b0090),
    ("src/error.rs", 0xe3604d5e30f8758a),
    ("src/inspect_paths.rs", 0x92f5cef9146d9104),
    ("src/message.rs", 0x95e7ef29b6bc50cf),
    ("src/per.rs", 0x3ccde6c46ad3f8b6),
    ("src/sized.rs", 0x05d05e62217fc26a),
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
