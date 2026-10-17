//! How a PDU compares with what the ASN.1 assigns its message: the criticality of its
//! procedure, and the object set of the IEs of the message. What does not depend on the
//! protocol is `oxirush_aper::check`, which oxirush-ngap and oxirush-s1ap share: here are
//! the object sets, the names of the IEs and the types that a deviation is told with.

use crate::registry::{
    CauseProtocol, Criticality, CriticalityDiagnosticsIEItem, IE_NAMES, OBJECT_SETS, ProtocolIEID,
    TypeOfError,
};
use oxirush_aper::check::{Diagnostics, ProtocolCause};
pub(crate) use oxirush_aper::check::{Wrong, deviations};

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

/// NGAP, as the types that its deviations are told with: a [`Deviation`] has the
/// [`Criticality`] of the protocol, and its answer the [`TypeOfError`], the
/// [`CauseProtocol`] and the [`CriticalityDiagnosticsIEItem`] of the protocol.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ngap;

impl Diagnostics for Ngap {
    type Criticality = Criticality;
    type TypeOfError = TypeOfError;
    type Cause = CauseProtocol;
    type Item = CriticalityDiagnosticsIEItem;

    fn name(id: u16) -> Option<&'static str> {
        name(id)
    }

    fn type_of_error(missing: bool) -> TypeOfError {
        match missing {
            true => TypeOfError::missing,
            false => TypeOfError::not_understood,
        }
    }

    fn cause(cause: ProtocolCause) -> CauseProtocol {
        match cause {
            ProtocolCause::TransferSyntaxError => CauseProtocol::transfer_syntax_error,
            ProtocolCause::AbstractSyntaxErrorReject => CauseProtocol::abstract_syntax_error_reject,
            ProtocolCause::AbstractSyntaxErrorIgnoreAndNotify => {
                CauseProtocol::abstract_syntax_error_ignore_and_notify
            }
            ProtocolCause::AbstractSyntaxErrorFalselyConstructedMessage => {
                CauseProtocol::abstract_syntax_error_falsely_constructed_message
            }
        }
    }

    fn item(
        criticality: Criticality,
        id: u16,
        type_of_error: TypeOfError,
    ) -> CriticalityDiagnosticsIEItem {
        CriticalityDiagnosticsIEItem::new(criticality, ProtocolIEID(id), type_of_error, None)
    }
}

/// One way in which a PDU is not as the specification has a sender construct it: what
/// `check()` of a PDU reports, in the types of NGAP. See
/// [`oxirush_aper::check::Deviation`], which oxirush-ngap and oxirush-s1ap share, for its
/// variants and what each says.
pub type Deviation = oxirush_aper::check::Deviation<Ngap>;

#[cfg(test)]
mod tests {
    use super::*;
    use oxirush_aper::Faults;

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
        deviations::<Ngap>(&set(), ies, &|_| (None, Faults::default()), &mut found);
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
