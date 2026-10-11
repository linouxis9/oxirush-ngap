//! What the ASN.1 assigns the message of a PDU: the object set of its IEs.

use crate::registry::{Criticality, OBJECT_SETS};

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
}
