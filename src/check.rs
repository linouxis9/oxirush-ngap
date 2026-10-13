//! How a PDU compares with what the ASN.1 assigns its message: the criticality of its
//! procedure, and the object set of the IEs of the message.

use std::fmt;

use crate::registry::{
    CauseProtocol, Criticality, CriticalityDiagnosticsIEItem, IE_NAMES, OBJECT_SETS, ProtocolIEID,
    TypeOfError,
};
use crate::{Error, ErrorKind};

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
    // The code first: a number is compared sooner than the name of a direction.
    let mut sets = OBJECT_SETS.iter();
    sets.find(|set| set.procedure_code == code && set.direction == direction)
}

/// The name that ASN.1 gives the IE of an identifier, after `id-`. The registry lists
/// the IEs in the order of their identifiers.
pub(crate) fn name(id: u16) -> Option<&'static str> {
    let at = IE_NAMES.binary_search_by_key(&id, |(known, _)| *known);
    at.ok().map(|at| IE_NAMES[at].1)
}

/// One way in which a PDU is not as the specification has a sender construct it: what
/// `check()` of a PDU reports.
///
/// The check compares the criticality of the PDU with the one that the specification
/// assigns to its procedure, and the IEs of its message with the object set that the
/// ASN.1 gives the message: which IEs the message has, in which order, with which
/// criticality, and whether the value of each decodes as its type. An IE is told by
/// its identifier, by the name that ASN.1 gives it after `id-` and by its position in
/// the message, which starts at 0, and `Display` says each deviation in a sentence.
///
/// [`ie`](Self::ie), [`type_of_error`](Self::type_of_error),
/// [`diagnostics_item`](Self::diagnostics_item) and [`cause`](Self::cause) say what
/// the receiver of the message answers, as clause 10 of the specification has it.
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
        /// The criticality that the IE has.
        criticality: Criticality,
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
        /// The criticality that the object set assigns it.
        criticality: Criticality,
    },
    /// An IE of the message whose value does not decode as its type: its octets end
    /// before the value does, go on after it, or hold a value outside a constraint.
    UndecodableIe {
        /// The identifier of the IE.
        id: u16,
        /// Its name.
        name: &'static str,
        /// Its position in the message.
        position: usize,
        /// The criticality that the IE has.
        criticality: Criticality,
        /// Why its value does not decode: the error that the macro which extracts
        /// the IE returns.
        error: Error,
    },
}

impl Deviation {
    /// The IE that the deviation is about, as its identifier and the criticality that
    /// the receiver acts on: the one that the IE has, when it is in the message, and
    /// the one that the object set assigns it, when it is missing. `None` for the
    /// criticality of the PDU, and for an IE that is there twice or in a wrong order:
    /// the receiver rejects the procedure for it, whatever its criticality.
    pub fn ie(&self) -> Option<(u16, Criticality)> {
        match self {
            Self::PduCriticality { .. } => None,
            Self::UnexpectedIe {
                id, criticality, ..
            }
            | Self::IeCriticality {
                id, criticality, ..
            }
            | Self::MissingIe {
                id, criticality, ..
            }
            | Self::UndecodableIe {
                id, criticality, ..
            } => Some((*id, *criticality)),
            Self::RepeatedIe { .. } | Self::OutOfOrder { .. } => None,
        }
    }

    /// How the Criticality Diagnostics of an answer say what is wrong with the IE:
    /// `not_understood` for an IE that the message cannot have and for one whose
    /// value does not decode, and `missing` for one that is. `None` for a deviation
    /// that is no fault of one IE which a receiver reports so.
    pub fn type_of_error(&self) -> Option<TypeOfError> {
        match self {
            Self::UnexpectedIe { .. } | Self::UndecodableIe { .. } => {
                Some(TypeOfError::not_understood)
            }
            Self::MissingIe { .. } => Some(TypeOfError::missing),
            _ => None,
        }
    }

    /// The item of the Criticality Diagnostics IE that tells of this deviation: the
    /// criticality of [`ie`](Self::ie), its identifier and
    /// [`type_of_error`](Self::type_of_error). `None` where there is no type of error.
    ///
    /// The specification has a receiver report the IEs that it does not comprehend
    /// and those that are missing when their criticality is `reject` or `notify`, and
    /// ignore the others: the criticality of the item says which it is.
    pub fn diagnostics_item(&self) -> Option<CriticalityDiagnosticsIEItem<'static>> {
        let (id, criticality) = self.ie()?;
        let type_of_error = self.type_of_error()?;
        let id = ProtocolIEID(id);
        Some(CriticalityDiagnosticsIEItem::new(
            criticality,
            id,
            type_of_error,
            None,
        ))
    }

    /// The protocol cause that the specification has a receiver answer the message
    /// with, for this deviation alone:
    ///
    /// - `transfer_syntax_error` for an IE whose value does not decode;
    /// - `abstract_syntax_error_reject` and `abstract_syntax_error_ignore_and_notify`
    ///   for an IE that the message cannot have, by the criticality that it has, and
    ///   for an IE that is missing, by the criticality that the object set assigns it;
    /// - `abstract_syntax_error_falsely_constructed_message` for an IE that is there
    ///   twice or in a wrong order.
    ///
    /// `None` where the receiver goes on: for an IE of the criticality `ignore` that
    /// it does not comprehend or that is missing, and for a criticality that is not
    /// the assigned one.
    pub fn cause(&self) -> Option<CauseProtocol> {
        let by_criticality = |criticality: &Criticality| match criticality {
            Criticality::reject => Some(CauseProtocol::abstract_syntax_error_reject),
            Criticality::notify => Some(CauseProtocol::abstract_syntax_error_ignore_and_notify),
            Criticality::ignore => None,
        };
        match self {
            Self::UndecodableIe { .. } => Some(CauseProtocol::transfer_syntax_error),
            Self::UnexpectedIe { criticality, .. } | Self::MissingIe { criticality, .. } => {
                by_criticality(criticality)
            }
            Self::RepeatedIe { .. } | Self::OutOfOrder { .. } => {
                Some(CauseProtocol::abstract_syntax_error_falsely_constructed_message)
            }
            Self::PduCriticality { .. } | Self::IeCriticality { .. } => None,
        }
    }

    /// The deviation that an error of the macros tells of: the IE that a `req` field
    /// misses, with the criticality that the object set assigns it, and the IE whose
    /// value does not decode, with the criticality that it has. `None` for an error
    /// that is about no IE.
    pub fn of(error: &Error) -> Option<Self> {
        let ie = error.ie()?;
        let (id, criticality) = (ie.id, ie.criticality.into());
        let name = name(id).unwrap_or_default();
        match (error.kind(), ie.position) {
            (ErrorKind::MissingIe, None) => Some(Self::MissingIe {
                id,
                name,
                criticality,
            }),
            (ErrorKind::UndecodableIe, Some(position)) => Some(Self::UndecodableIe {
                id,
                name,
                position,
                criticality,
                error: error.clone(),
            }),
            _ => None,
        }
    }
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
            Self::UnexpectedIe {
                id, name, position, ..
            } => match name {
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
            Self::MissingIe { id, name, .. } => write!(
                formatter,
                "{name} (IE {id}) is mandatory, and the message does not have it"
            ),
            Self::UndecodableIe {
                id,
                name,
                position,
                error,
                ..
            } => write!(
                formatter,
                "{name} (IE {id}), at position {position}, does not decode: {}",
                error.reason()
            ),
        }
    }
}

/// Add to `found` how `ies`, the IEs of a message in their order, each as its identifier
/// and its criticality, deviate from `set`, the IEs of the object set of that message:
/// those of the IEs in the order of the message, then the mandatory IEs that are
/// missing in the order of the set. `undecodable` says why the value of the IE at a
/// position does not decode as its type, for an IE of the set whose value does not.
pub(crate) fn deviations(
    set: &[(u16, Criticality, bool)],
    ies: &[(u16, Criticality)],
    undecodable: &dyn Fn(usize) -> Option<Error>,
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
            found.push(Deviation::UnexpectedIe {
                id,
                name,
                position,
                criticality,
            });
            continue;
        };
        let name = || name(id).unwrap_or_default();
        match (first[index], last) {
            (Some(first), _) => found.push(Deviation::RepeatedIe {
                id,
                name: name(),
                position,
                first,
            }),
            (None, Some(last)) if index < last => {
                first[index] = Some(position);
                let (after, ..) = set[last];
                found.push(Deviation::OutOfOrder {
                    id,
                    name: name(),
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
                name: name(),
                position,
                criticality,
                assigned,
            });
        }
        if let Some(error) = undecodable(position) {
            found.push(Deviation::UndecodableIe {
                id,
                name: name(),
                position,
                criticality,
                error,
            });
        }
    }
    for ((id, criticality, mandatory), first) in set.iter().zip(first) {
        if *mandatory && first.is_none() {
            let name = name(*id).unwrap_or_default();
            found.push(Deviation::MissingIe {
                id: *id,
                name,
                criticality: *criticality,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ies_are_listed_in_the_order_of_their_identifiers() {
        // The name of an IE is found by a binary search of its identifier.
        assert!(IE_NAMES.windows(2).all(|pair| pair[0].0 < pair[1].0));
        for (id, known) in IE_NAMES {
            assert_eq!(name(*id), Some(*known));
        }
        assert_eq!(name(u16::MAX), None);
    }

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

    /// What a message of these IEs deviates in, when the value of each decodes.
    fn found(ies: &[(u16, Criticality)]) -> Vec<Deviation> {
        let mut found = Vec::new();
        deviations(&set(), ies, &|_| None, &mut found);
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
            criticality: IGNORE,
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
