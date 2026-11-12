//! The trees of the PDUs of the fixtures, as `tests/fixtures/trees.tsv` records
//! them: for each PDU of `captured.tsv`, `fuzzed.tsv`, `later.tsv`,
//! `messages.tsv` and `vectors.tsv`, the length and the FNV-1a digest of the text
//! of its tree, and the digest of the octets of the PDU that the tree writes (the
//! trees themselves are 6 MB). A change of how a PDU becomes a tree, or a tree a PDU, keeps every
//! line. `OXIRUSH_BLESS=1 cargo test --features inspect --test trees` writes the
//! file again from the code.
#![cfg(feature = "inspect")]

use oxirush_ngap::ngap::NGAPPDU;

/// The 64-bit FNV-1a digest of `text`, in hexadecimal.
fn digest(text: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for octet in text.bytes() {
        hash = (hash ^ u64::from(octet)).wrapping_mul(0x0100_0000_01b3);
    }
    format!("{hash:016x}")
}

/// The line of the PDU `octets`, named `name`.
fn line(name: &str, octets: &[u8]) -> String {
    let pdu = match NGAPPDU::decode(octets) {
        Ok(pdu) => pdu,
        Err(error) => return format!("{name}\tdecode: {error}"),
    };
    let tree = pdu.to_tree();
    let written = match NGAPPDU::from_tree(&tree).and_then(|pdu| pdu.encode()) {
        Ok(octets) => hex::encode(octets),
        Err(error) => format!("error: {error}"),
    };
    let tree = tree.to_string();
    format!(
        "{name}\t{}\t{}\t{}",
        tree.len(),
        digest(&tree),
        digest(&written)
    )
}

fn lines() -> Vec<String> {
    oxirush_s1ap::inspect::register();
    oxirush_ngap::inspect::register();
    let files = [
        ("captured", include_str!("fixtures/captured.tsv")),
        ("fuzzed", include_str!("fixtures/fuzzed.tsv")),
        ("later", include_str!("fixtures/later.tsv")),
        ("messages", include_str!("fixtures/messages.tsv")),
        ("vectors", include_str!("fixtures/vectors.tsv")),
    ];
    let mut lines = Vec::new();
    for (file, text) in files {
        for row in text
            .lines()
            .filter(|row| !row.starts_with('#') && !row.is_empty())
        {
            let columns: Vec<&str> = row.split('\t').collect();
            let octets = hex::decode(columns[1]).unwrap();
            lines.push(line(&format!("{file}/{}", columns[0]), &octets));
        }
    }
    lines
}

/// Every PDU of the fixtures has the tree and the written octets that the file
/// records.
#[test]
fn the_trees_of_the_fixtures_are_those_recorded() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/trees.tsv");
    let made = lines();
    if std::env::var_os("OXIRUSH_BLESS").is_some() {
        let header =
            "# name\tlength of the tree\tdigest of the tree\tdigest of the octets that it writes\n";
        std::fs::write(path, header.to_owned() + &made.join("\n") + "\n").unwrap();
        return;
    }
    let recorded = std::fs::read_to_string(path).unwrap();
    let recorded: Vec<&str> = recorded
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect();
    assert_eq!(recorded.len(), made.len(), "the number of PDUs");
    for (recorded, made) in recorded.iter().zip(&made) {
        assert_eq!(recorded, made);
    }
}
