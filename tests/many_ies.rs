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

//! What the crate costs on a message of the most IEs that a container holds: 65,535 of
//! them, of one name. Each call takes a time that grows as the number of IEs does, so
//! each is done within a bound that a call whose time grows as the square of it cannot
//! meet: the listing of the paths once took nine minutes on this message.

#![cfg(feature = "inspect")]

use oxirush_ngap::inspect;
use oxirush_ngap::ngap::*;
use oxirush_ngap::{tree, tree::Value};
use std::sync::mpsc;
use std::time::Duration;

/// The most IEs of a container: `maxProtocolIEs` of TS 38.413 clause 9.4.7.
const MOST: usize = 65_535;

/// What a call is given at most, on a build without optimisation: the calls of this
/// file take two seconds together there.
const BOUND: Duration = Duration::from_secs(60);

/// An UPLINK NAS TRANSPORT of `count` IEs of the identifier `id`, each of `value`.
fn message_of(count: usize, id: u16, value: &[u8]) -> NGAP_PDU {
    let ie = ProtocolIEField::new(ProtocolIEID(id), Criticality::reject, value.to_vec().into());
    let mut pdu = NGAP_PDU::decode(&hex::decode("002e4003000000").unwrap()).unwrap();
    pdu.set_protocol_ies(ProtocolIEContainer(vec![ie; count]))
        .unwrap();
    pdu
}

/// What `call` gives, when it gives it within the bound. The test fails at the bound,
/// without waiting for a call that is not done.
fn within<T: Send + 'static>(what: &str, call: impl FnOnce() -> T + Send + 'static) -> T {
    let (done, result) = mpsc::channel();
    std::thread::spawn(move || done.send(call()));
    match result.recv_timeout(BOUND) {
        Ok(result) => result,
        Err(_) => panic!("{what} is not done after {BOUND:?} on {MOST} IEs"),
    }
}

#[test]
fn each_call_on_the_most_ies_of_one_name_is_done_within_a_bound() {
    // AMF-UE-NGAP-ID, the smallest IE that has a name and a value.
    let pdu = message_of(MOST, 10, &[0x00, 0x01]);
    let octets = pdu.encode().unwrap();
    // Six octets an IE, and the lengths of the message and of its container.
    assert_eq!(octets.len(), 393_224);

    let checked = pdu.clone();
    let deviations = within("check", move || checked.check().unwrap());
    assert!(!deviations.is_empty());

    let viewed = pdu.clone();
    let view = within("to_view", move || viewed.to_view().unwrap());
    assert_eq!(
        view["message"]["protocolIEs"].as_array().unwrap().len(),
        MOST
    );

    let written = pdu.clone();
    let tree = within("to_tree", move || written.to_tree());
    assert_eq!(tree["ies"].as_array().unwrap().len(), MOST);

    let listed = view.clone();
    let paths = within("paths", move || inspect::paths(&listed));
    // An IE whose identifier is there twice goes by its position.
    let last = format!("/ngap/{}/value", MOST - 1);
    // Three values of the PDU, and three of each IE.
    assert_eq!(paths.len(), 3 + 3 * MOST);
    assert!(paths.contains(&(last.clone(), tree!(1))));

    let (selected, path) = (view.clone(), last.clone());
    let one = within("select", move || {
        inspect::select(&selected, &path).map(|values| values.len())
    });
    assert_eq!(one.unwrap(), 1);
    // A path selects every IE of a message, which has 65,535 at most (the review of the
    // move of the paths to oxirush-utils: it selected 4,096).
    let selected = view.clone();
    let all = within("select by name", move || {
        inspect::select(&selected, "/ngap/AMF-UE-NGAP-ID/value").map(|values| values.len())
    });
    assert_eq!(all.unwrap(), MOST);

    let (mut edited, path) = (view.clone(), last);
    let edited = within("set", move || {
        inspect::set(&mut edited, &path, tree!(2)).map(|()| edited)
    })
    .unwrap();
    let (unedited, same) = (pdu.clone(), view.clone());
    let again = within("with_view", move || unedited.with_view(&same).unwrap());
    assert_eq!(again.encode().unwrap(), octets);
    let other = within("with_view of an edit", move || {
        pdu.with_view(&edited).unwrap()
    });
    let other = other.encode().unwrap();
    assert_eq!(other.len(), octets.len());
    assert_eq!((octets[octets.len() - 1], other[other.len() - 1]), (1, 2));
}

#[test]
fn the_paths_of_ies_without_a_name_are_listed_within_the_bound_too() {
    let pdu = message_of(MOST, 60_000, &[0x00]);
    let view = within("to_view", move || pdu.to_view().unwrap());
    let paths: Vec<(String, Value)> = within("paths", move || inspect::paths(&view));
    assert!(paths.len() >= MOST);
}

/// The fourth finding of the review of the move of the paths to oxirush-utils: `open`
/// took the path again from the view for each value that it opened, ten seconds for the
/// 4,096 containers of a HANDOVER REQUIRED. It opens those of a segment in one walk, by
/// the name of the IE and with `*`, which takes the IEs that have the name of the type
/// and leaves the others.
#[test]
fn open_on_the_containers_of_a_handover_is_done_within_a_bound() {
    const CONTAINERS: usize = 8192;
    let name = "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer";
    let container = tree!({name: {
        "rRCContainer": "0102",
        "targetCell-ID": {"nR-CGI": {"pLMNIdentity": "208-93", "nRCellIdentity": 4660}},
        "uEHistoryInformation": [{"lastVisitedCellInformation": {"eUTRANCell": "00"}}],
    }});
    let mut ies = vec![tree!({"AMF-UE-NGAP-ID": 42}), tree!({"RAN-UE-NGAP-ID": 7})];
    let containers =
        (0..CONTAINERS).map(|_| tree!({"SourceToTarget-TransparentContainer": &container}));
    ies.extend(containers);
    let required = tree!({"message": "HandoverRequired", "ies": ies});
    let pdu = NGAP_PDU::from_tree(&required).unwrap();
    let view = within("to_view", move || pdu.to_view().unwrap());
    let cell = format!(
        "/ngap/SourceToTarget-TransparentContainer/{name}/targetCell-ID/nR-CGI/nRCellIdentity"
    );
    for path in [cell.clone(), format!("/ngap/*/{name}/targetCell-ID")] {
        let mut opened = view.clone();
        let opened = within("open", move || {
            inspect::open(&mut opened, &path).unwrap();
            opened
        });
        let cells = inspect::select(&opened, &cell).unwrap();
        assert_eq!(cells, vec![&tree!(4660); CONTAINERS]);
    }
}
