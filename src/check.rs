//! How a PDU compares with what the ASN.1 assigns its message: the criticality of its
//! procedure, and the object set of the IEs of the message.

use std::fmt;

use crate::registry::{Criticality, IE_NAMES, OBJECT_SETS};

/// What the registry has of a message: the direction and the procedure code of its PDU,
/// the name that ASN.1 gives it, and the IEs of its object set in the order of the set,
/// each as its identifier, the criticality that the set assigns it and whether its
/// presence is mandatory. A message that has no container of protocol IEs, as the
/// private message, has no object set.
pub(crate) struct ObjectSet {
    pub(crate) direction: &'static str,
    pub(crate) procedure_code: u8,
    pub(crate) name: &'static str,
    pub(crate) ies: Option<&'static [(u16, Criticality, bool)]>,
}

/// What the registry has of the message of a direction and a procedure code.
pub(crate) fn object_set(direction: &str, code: u8) -> Option<&'static ObjectSet> {
    let mut sets = OBJECT_SETS.iter();
    sets.find(|set| (set.direction, set.procedure_code) == (direction, code))
}

/// The name that ASN.1 gives the IE of an identifier, after `id-`.
fn name(id: u16) -> Option<&'static str> {
    let mut names = IE_NAMES.iter();
    names.find(|(known, _)| *known == id).map(|(_, name)| *name)
}

/// One way in which a PDU is not as the specification has a sender construct it: what
/// `check()` of a PDU reports.
///
/// The check compares the criticality of the PDU with the one that the specification
/// assigns to its procedure, and the IEs of its message with the object set that the
/// ASN.1 gives the message. An IE is told by its identifier, by the name that ASN.1
/// gives it after `id-` and by its position in the message, which starts at 0, and
/// `Display` says each deviation in a sentence.
///
/// More variants may come with a later version: leave the others to `_`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Deviation {
    /// The criticality of the PDU is not the one that the specification assigns to its
    /// procedure.
    PduCriticality {
        /// The criticality that the PDU has.
        criticality: Criticality,
        /// The criticality of the procedure.
        assigned: Criticality,
    },
    /// An IE that the message cannot have: the object set of the message does not have
    /// its identifier.
    UnexpectedIe {
        /// The identifier of the IE.
        id: u16,
        /// Its name; `None` for an identifier that no object set of the protocol has.
        name: Option<&'static str>,
        /// Its position in the message.
        position: usize,
    },
    /// An IE that the message has already: an IE is in a message once.
    RepeatedIe {
        /// The identifier of the IE.
        id: u16,
        /// Its name.
        name: &'static str,
        /// The position of this one.
        position: usize,
        /// The position of the first one.
        first: usize,
    },
    /// An IE that comes after one that the object set has after it. The IEs that the
    /// object set does not have are left out of the order, and so is an IE that is
    /// repeated, after its first.
    OutOfOrder {
        /// The identifier of the IE.
        id: u16,
        /// Its name.
        name: &'static str,
        /// Its position in the message.
        position: usize,
        /// The identifier of an IE before it in the message that the object set has
        /// after it: the one that the object set has last.
        after: u16,
        /// The name of that IE.
        after_name: &'static str,
    },
    /// An IE whose criticality is not the one that the object set assigns it.
    IeCriticality {
        /// The identifier of the IE.
        id: u16,
        /// Its name.
        name: &'static str,
        /// Its position in the message.
        position: usize,
        /// The criticality that the IE has.
        criticality: Criticality,
        /// The criticality that the object set assigns it.
        assigned: Criticality,
    },
    /// An IE whose presence is mandatory, and that the message does not have.
    MissingIe {
        /// The identifier of the IE.
        id: u16,
        /// Its name.
        name: &'static str,
    },
}

impl fmt::Display for Deviation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PduCriticality {
                criticality,
                assigned,
            } => write!(
                formatter,
                "the criticality of the PDU is {criticality:?}, and that of its procedure \
                 is {assigned:?}"
            ),
            Self::UnexpectedIe { id, name, position } => match name {
                Some(name) => write!(
                    formatter,
                    "{name} (IE {id}), at position {position}, is not an IE of the message"
                ),
                None => write!(
                    formatter,
                    "IE {id}, at position {position}, is not an IE of the message, nor of \
                     the protocol"
                ),
            },
            Self::RepeatedIe {
                id,
                name,
                position,
                first,
            } => write!(
                formatter,
                "{name} (IE {id}), at position {position}, is in the message already, at \
                 position {first}"
            ),
            Self::OutOfOrder {
                id,
                name,
                position,
                after,
                after_name,
            } => write!(
                formatter,
                "{name} (IE {id}), at position {position}, comes after {after_name} (IE \
                 {after}), which the object set of the message has after it"
            ),
            Self::IeCriticality {
                id,
                name,
                position,
                criticality,
                assigned,
            } => write!(
                formatter,
                "the criticality of {name} (IE {id}), at position {position}, is \
                 {criticality:?}, and the message assigns it {assigned:?}"
            ),
            Self::MissingIe { id, name } => write!(
                formatter,
                "{name} (IE {id}) is mandatory, and the message does not have it"
            ),
        }
    }
}

/// Add to `found` how `ies`, the IEs of a message in their order, each as its identifier
/// and its criticality, deviate from `set`, the IEs of the object set of that message:
/// those of the IEs in the order of the message, then the mandatory IEs that are
/// missing in the order of the set.
pub(crate) fn deviations(
    set: &[(u16, Criticality, bool)],
    ies: &[(u16, Criticality)],
    found: &mut Vec<Deviation>,
) {
    // Where the message has each IE of the set first, and the IE that the set has last
    // among those that the message had so far.
    let mut first: Vec<Option<usize>> = vec![None; set.len()];
    let mut last: Option<usize> = None;
    for (position, (id, criticality)) in ies.iter().enumerate() {
        let (id, criticality) = (*id, *criticality);
        let Some(index) = set.iter().position(|(known, ..)| *known == id) else {
            let name = name(id);
            found.push(Deviation::UnexpectedIe { id, name, position });
            continue;
        };
        let name = name(id).unwrap_or_default();
        match (first[index], last) {
            (Some(first), _) => found.push(Deviation::RepeatedIe {
                id,
                name,
                position,
                first,
            }),
            (None, Some(last)) if index < last => {
                first[index] = Some(position);
                let (after, ..) = set[last];
                found.push(Deviation::OutOfOrder {
                    id,
                    name,
                    position,
                    after,
                    after_name: self::name(after).unwrap_or_default(),
                });
            }
            (None, _) => {
                first[index] = Some(position);
                last = Some(index);
            }
        }
        let (_, assigned, _) = set[index];
        if criticality != assigned {
            found.push(Deviation::IeCriticality {
                id,
                name,
                position,
                criticality,
                assigned,
            });
        }
    }
    for ((id, _, mandatory), first) in set.iter().zip(first) {
        if *mandatory && first.is_none() {
            let name = name(*id).unwrap_or_default();
            found.push(Deviation::MissingIe { id: *id, name });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_message_but_the_private_one_has_an_object_set() {
        let without = OBJECT_SETS.iter().filter(|set| set.ies.is_none());
        let without: Vec<_> = without.map(|set| set.name).collect();
        assert_eq!(without, ["PrivateMessage"]);
        // A message is found by its direction and its procedure code.
        for message in OBJECT_SETS {
            let found = object_set(message.direction, message.procedure_code);
            assert_eq!(found.map(|found| found.name), Some(message.name));
        }
        assert!(object_set("InitiatingMessage", 200).is_none());
        assert!(OBJECT_SETS.len() > 100, "{}", OBJECT_SETS.len());
    }

    /// An object set of four IEs: the first and the third are mandatory, and the
    /// identifiers are not in their own order.
    fn set() -> Vec<(u16, Criticality, bool)> {
        vec![
            (3, Criticality::reject, true),
            (1, Criticality::ignore, false),
            (2, Criticality::reject, true),
            (4, Criticality::notify, false),
        ]
    }

    /// What a message of these IEs deviates in.
    fn found(ies: &[(u16, Criticality)]) -> Vec<Deviation> {
        let mut found = Vec::new();
        deviations(&set(), ies, &mut found);
        found
    }

    const REJECT: Criticality = Criticality::reject;
    const IGNORE: Criticality = Criticality::ignore;
    const NOTIFY: Criticality = Criticality::notify;

    #[test]
    fn a_message_as_its_object_set_has_it_deviates_in_nothing() {
        assert_eq!(found(&[(3, REJECT), (2, REJECT)]), []);
        assert_eq!(
            found(&[(3, REJECT), (1, IGNORE), (2, REJECT), (4, NOTIFY)]),
            []
        );
    }

    #[test]
    fn each_deviation_is_told_with_its_ie_and_its_position() {
        // A mandatory IE that is missing, and an optional one is not.
        let missing = found(&[(3, REJECT)]);
        assert!(matches!(missing[..], [Deviation::MissingIe { id: 2, .. }]));
        // An IE that the set does not have: one of the protocol or not.
        let other = found(&[(3, REJECT), (65000, IGNORE), (2, REJECT)]);
        let unexpected = Deviation::UnexpectedIe {
            id: 65000,
            name: None,
            position: 1,
        };
        assert_eq!(other, [unexpected]);
        // An IE that is there twice, whatever is between the two.
        let twice = found(&[(3, REJECT), (2, REJECT), (3, REJECT)]);
        assert!(matches!(
            twice[..],
            [Deviation::RepeatedIe {
                id: 3,
                position: 2,
                first: 0,
                ..
            }]
        ));
        // Two IEs in the other order: the second is after one that the set has after it.
        let swapped = found(&[(2, REJECT), (3, REJECT)]);
        assert!(matches!(
            swapped[..],
            [Deviation::OutOfOrder {
                id: 3,
                position: 1,
                after: 2,
                ..
            }]
        ));
        // A criticality that is not the one of the set.
        let criticality = found(&[(3, REJECT), (2, IGNORE)]);
        assert!(matches!(
            criticality[..],
            [Deviation::IeCriticality {
                id: 2,
                position: 1,
                criticality: IGNORE,
                assigned: REJECT,
                ..
            }]
        ));
    }

    #[test]
    fn the_order_is_that_of_the_set_and_leaves_out_what_the_set_does_not_have() {
        // The last IE of the set first: each of the others comes after it.
        let late = found(&[(4, NOTIFY), (3, REJECT), (1, IGNORE), (2, REJECT)]);
        let after: Vec<_> = late
            .iter()
            .map(|deviation| match deviation {
                Deviation::OutOfOrder {
                    id,
                    position,
                    after,
                    ..
                } => (*id, *position, *after),
                other => panic!("{other}"),
            })
            .collect();
        assert_eq!(after, [(3, 1, 4), (1, 2, 4), (2, 3, 4)]);
        // An IE that the set does not have is between two that are in order.
        let between = found(&[(3, REJECT), (65000, REJECT), (2, REJECT)]);
        assert!(matches!(between[..], [Deviation::UnexpectedIe { .. }]));
        // A repeated IE is told once, as repeated, and the missing ones come last.
        let all = found(&[(1, REJECT), (1, IGNORE), (3, REJECT)]);
        assert!(matches!(
            all[..],
            [
                Deviation::IeCriticality {
                    id: 1,
                    position: 0,
                    ..
                },
                Deviation::RepeatedIe {
                    id: 1,
                    position: 1,
                    first: 0,
                    ..
                },
                Deviation::OutOfOrder {
                    id: 3,
                    position: 2,
                    after: 1,
                    ..
                },
                Deviation::MissingIe { id: 2, .. },
            ]
        ));
    }

    #[test]
    fn every_ie_of_an_object_set_has_a_name_and_is_there_once() {
        for message in OBJECT_SETS {
            for (index, (id, ..)) in message.ies.into_iter().flatten().enumerate() {
                assert!(name(*id).is_some(), "IE {id} of {}", message.name);
                let before = &message.ies.unwrap_or_default()[..index];
                let again = before.iter().any(|(known, ..)| known == id);
                assert!(!again, "IE {id} is twice in the set of {}", message.name);
            }
        }
    }
}
