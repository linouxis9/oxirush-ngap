/*
   OxiRush
   Copyright 2025-2026 Valentin D'Emmanuele

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! oxirush-aper and oxirush-utils are not on crates.io yet, and the repository takes
//! them from their repositories. `build/revisions` records the revision of each, and
//! every place that takes one names that revision, and no branch: a change of either
//! repository changes no result here until the revision is changed here. The
//! generator of oxirush-aper at its revision wrote the bindings, which the Reproduce
//! workflow checks out by this file.
//!
//! The published crate has none of these files but the README: there the test holds the
//! README to the revisions that it names first. It is the same file in oxirush-ngap and
//! in oxirush-s1ap.

use std::path::Path;

/// The files that take a crate from its repository, on a line that has the address of
/// the repository between quotes. The scripts of `tests/` and the Reproduce workflow
/// read `build/revisions` themselves.
const PLACES: &[&str] = &[
    ".github/workflows/ci.yml",
    ".github/workflows/reproduce.yml",
    "README.md",
    "benches/Cargo.toml",
    "fuzz/Cargo.toml",
    "oracle/Cargo.toml",
];

/// The revision of each crate that the README takes from its repository first, as
/// `name revision` lines: what the published crate has of `build/revisions`.
fn named_in_the_readme(root: &Path) -> String {
    let readme = std::fs::read_to_string(root.join("README.md")).unwrap();
    let mut named = String::new();
    for name in ["oxirush-aper", "oxirush-utils"] {
        let place = format!("{name}.rev=\"");
        let at = readme
            .find(&place)
            .unwrap_or_else(|| panic!("the README takes no {name}"));
        let revision = &readme[at + place.len()..];
        let revision = &revision[..revision.find('"').unwrap()];
        named.push_str(&format!("{name} {revision}\n"));
    }
    named
}

#[test]
fn each_place_takes_the_unpublished_crates_at_the_revision_that_is_recorded() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    // The published crate has the README alone of these files.
    let (recorded, places) = match std::fs::read_to_string(root.join("build/revisions")) {
        Ok(recorded) => (recorded, PLACES),
        Err(_) => (named_in_the_readme(root), &["README.md"][..]),
    };
    let recorded = recorded.lines().filter(|line| !line.starts_with('#'));
    let recorded: Vec<(&str, &str)> = recorded.filter_map(|l| l.split_once(' ')).collect();
    let names: Vec<&str> = recorded.iter().map(|(name, _)| *name).collect();
    assert_eq!(names, ["oxirush-aper", "oxirush-utils"]);
    for (name, revision) in &recorded {
        // The name of a commit, whole: a branch or a tag may move.
        let digits = revision
            .bytes()
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'));
        assert!(revision.len() == 40 && digits, "{name} {revision}");
    }
    for place in places {
        let text = std::fs::read_to_string(root.join(place)).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        let mut taken = 0;
        for (number, line) in lines.iter().enumerate() {
            for (name, revision) in &recorded {
                if !line.contains(&format!("\"https://github.com/linouxis9/{name}\"")) {
                    continue;
                }
                taken += 1;
                // In a manifest the revision is on the line, and for cargo's `--config`
                // it is the next option, on the next line.
                let next = lines.get(number + 1).copied().unwrap_or_default();
                let named = [
                    format!("rev = \"{revision}\""),
                    format!("rev=\"{revision}\""),
                ];
                assert!(
                    named
                        .iter()
                        .any(|rev| line.contains(rev) || next.contains(rev)),
                    "{place}, line {}, takes {name} at another revision than {revision}, which \
                     build/revisions records: run build/pin.py",
                    number + 1
                );
            }
        }
        assert!(taken > 0, "{place} takes neither crate from its repository");
    }
}
