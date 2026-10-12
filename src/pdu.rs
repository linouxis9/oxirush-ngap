//! The protocol IEs of the message of a PDU, whatever the message: read, put back, and
//! compared with the object set that the ASN.1 gives the message.

use oxirush_aper::{Decode, Encode, Octets, Reader, Writer};

use crate::Error;
use crate::check::{Deviation, deviations, object_set};
use crate::ngap::{Criticality, NGAPPDU, ProtocolIEContainer};
use crate::registry::procedure_criticality;

/// The protocol IEs of a message, which the ASN.1 has as a SEQUENCE of one container
/// of protocol IEs, which a later release may add to: the additions are skipped.
fn message_ies<'a>(octets: &'a [u8]) -> Result<ProtocolIEContainer<'a>, Error> {
    let mut reader = Reader::new(octets);
    let extended = reader.bit()?;
    let ies = match ProtocolIEContainer::decode(&mut reader) {
        Ok(ies) => ies,
        Err(error) => return Err(error.in_field("protocol_ies")),
    };
    if extended {
        reader.skip_additions()?;
    }
    reader.finish()?;
    Ok(ies)
}

/// The octets of the message that has these protocol IEs.
pub(crate) fn message_octets(ies: &ProtocolIEContainer<'_>) -> Result<Vec<u8>, Error> {
    let values: usize = ies.0.iter().map(|ie| ie.value.len() + 5).sum();
    let mut octets = Vec::with_capacity(3 + values);
    let mut writer = Writer::new(&mut octets);
    writer.bit(false);
    ies.encode(&mut writer)?;
    Ok(octets)
}

impl<'a> NGAPPDU<'a> {
    /// The octets of the open type that holds the message of this PDU.
    pub(crate) fn value(&self) -> &Octets<'a> {
        match self {
            Self::initiatingMessage(message) => &message.value,
            Self::successfulOutcome(message) => &message.value,
            Self::unsuccessfulOutcome(message) => &message.value,
        }
    }

    /// The name that ASN.1 gives the message of this PDU, when the specification has
    /// its procedure in that direction. An error for a message that has no container
    /// of protocol IEs.
    fn message_of_protocol_ies(&self) -> Result<Option<&'static str>, Error> {
        match object_set(self.direction(), self.procedure_code()) {
            Some(set) if set.ies.is_none() => Err(Error::__no_protocol_ies(set.name)),
            set => Ok(set.map(|set| set.name)),
        }
    }

    /// The protocol IEs of the message of this PDU, whatever the message is: the
    /// container of the bindings, in which each IE is as it was received, with its
    /// identifier, its criticality and the octets of its value, which are borrowed
    /// from the PDU. An IE that the message cannot have, one that the protocol does not
    /// have and one that is there twice are all in it, in the order of the message.
    ///
    /// Every message of TS 38.413 but the private one is a container of protocol IEs,
    /// so this needs no type: `decode_value` takes the type of the message that the PDU
    /// holds, and refuses another one. The message of a procedure that the
    /// specification does not have in that direction is read as a container of
    /// protocol IEs too.
    ///
    /// ```
    /// use oxirush_ngap::ngap::*;
    ///
    /// // The RAN UE NGAP ID of a PDU, whatever its message, when it has one
    /// fn ran_ue_id(pdu: &NGAP_PDU) -> Result<Option<u32>, oxirush_ngap::Error> {
    ///     let ies = pdu.protocol_ies()?;
    ///     let Some(ie) = ies.0.iter().find(|ie| ie.id == ID_RAN_UE_NGAP_ID) else {
    ///         return Ok(None);
    ///     };
    ///     let id: RANUENGAPID = decode_open_type(&ie.value)?;
    ///     Ok(Some(id.0))
    /// }
    ///
    /// // A UE CONTEXT RELEASE REQUEST, then an NG SETUP FAILURE
    /// let request = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    /// assert_eq!(ran_ue_id(&NGAP_PDU::decode(&request)?)?, Some(7));
    /// let failure = hex::decode("40150008000001000f400188").unwrap();
    /// assert_eq!(ran_ue_id(&NGAP_PDU::decode(&failure)?)?, None);
    /// # Ok::<(), oxirush_ngap::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// An error of the kind `WrongMessage` for the private message, which has private
    /// IEs and no container of protocol IEs, and one of the kind `Decode` when the
    /// octets of the message are not a container of protocol IEs.
    pub fn protocol_ies(&self) -> Result<ProtocolIEContainer<'_>, Error> {
        let name = self.message_of_protocol_ies()?;
        if self.value().is_empty() {
            let none = Error::new(crate::ErrorKind::Decode, "there are no octets");
            return Err(none.__of_message(name));
        }
        message_ies(self.value()).map_err(|error| error.__of_message(name))
    }

    /// Make `protocol_ies` the protocol IEs of the message of this PDU: the PDU keeps
    /// its direction, its procedure code and its criticality, and its message is these
    /// IEs, in their order and as they are. With [`protocol_ies`](Self::protocol_ies),
    /// this amends a message of any type:
    ///
    /// ```
    /// use oxirush_ngap::{build_ngap_ie, ngap::*};
    ///
    /// // An NG SETUP FAILURE
    /// let failure = hex::decode("40150008000001000f400188").unwrap();
    /// let mut pdu = NGAP_PDU::decode(&failure)?;
    ///
    /// // Its IEs, with one more: they are owned, as the PDU is changed
    /// let mut ies = pdu.protocol_ies()?.into_owned();
    /// let wait = build_ngap_ie!(NGSetupFailure, IGNORE TimeToWait(TimeToWait::v5s))?;
    /// ies.0.push(wait);
    /// pdu.set_protocol_ies(ies)?;
    ///
    /// assert_eq!(pdu.kind(), NgapPduKind::Unsuccessful_NGSetup);
    /// let failure: NGSetupFailure = pdu.decode_value()?;
    /// assert_eq!(failure.protocol_ies.0.len(), 2);
    /// # Ok::<(), oxirush_ngap::Error>(())
    /// ```
    ///
    /// Nothing is checked: the IEs are those of any message, in any order.
    ///
    /// # Errors
    ///
    /// An error of the kind `WrongMessage` for the private message, which has no
    /// container of protocol IEs, and one of the kind `Encode` for more IEs than a
    /// container has, which is 65535. The PDU is as it was after an error.
    pub fn set_protocol_ies(&mut self, protocol_ies: ProtocolIEContainer<'_>) -> Result<(), Error> {
        self.message_of_protocol_ies()?;
        let count = protocol_ies.0.len();
        let octets = message_octets(&protocol_ies);
        let value = Octets::owned(octets.map_err(|error| error.__of_protocol_ies(count))?);
        match self {
            Self::initiatingMessage(message) => message.value = value,
            Self::successfulOutcome(message) => message.value = value,
            Self::unsuccessfulOutcome(message) => message.value = value,
        }
        Ok(())
    }

    /// The PDU of a direction, as `direction()` says it, with the code of its procedure,
    /// its criticality and the octets of the open type that holds its message; `None`
    /// for what is no direction.
    pub(crate) fn of(
        direction: &str,
        procedure_code: u8,
        criticality: Criticality,
        value: Octets<'a>,
    ) -> Option<Self> {
        use crate::ngap::{
            InitiatingMessage, ProcedureCode, SuccessfulOutcome, UnsuccessfulOutcome,
        };
        let procedure_code = ProcedureCode(procedure_code);
        Some(match direction {
            "InitiatingMessage" => Self::initiatingMessage(InitiatingMessage {
                procedure_code,
                criticality,
                value,
            }),
            "SuccessfulOutcome" => Self::successfulOutcome(SuccessfulOutcome {
                procedure_code,
                criticality,
                value,
            }),
            "UnsuccessfulOutcome" => Self::unsuccessfulOutcome(UnsuccessfulOutcome {
                procedure_code,
                criticality,
                value,
            }),
            _ => return None,
        })
    }

    /// The criticality of this PDU.
    pub(crate) fn criticality(&self) -> Criticality {
        match self {
            Self::initiatingMessage(message) => message.criticality,
            Self::successfulOutcome(message) => message.criticality,
            Self::unsuccessfulOutcome(message) => message.criticality,
        }
    }

    /// How this PDU deviates from what the ASN.1 assigns its message: the criticality of
    /// its procedure, and the object set of its IEs. The list is empty for a PDU that is
    /// constructed as TS 38.413 clause 9.4.1 has a sender construct it.
    ///
    /// | [`Deviation`] | Is |
    /// | --- | --- |
    /// | `PduCriticality` | A criticality of the PDU that is not that of its procedure |
    /// | `UnexpectedIe` | An IE that the object set of the message does not have |
    /// | `RepeatedIe` | An IE that the message has already |
    /// | `OutOfOrder` | An IE that comes after one that the object set has after it |
    /// | `IeCriticality` | A criticality of an IE that is not the one that the object set assigns it |
    /// | `MissingIe` | An IE whose presence is mandatory, and that the message does not have |
    ///
    /// The criticality of the PDU comes first, then what each IE deviates in, in the
    /// order of the message, then the IEs that are missing, in the order of the object
    /// set.
    ///
    /// ```
    /// use oxirush_ngap::{Deviation, build_ngap, ngap::*};
    ///
    /// // A UE CONTEXT RELEASE REQUEST without its Cause, and with its two identifiers
    /// // in the other order
    /// let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
    ///     IGNORE, UEContextReleaseRequest,
    ///     REJECT RAN_UE_NGAP_ID(7u32),
    ///     REJECT AMF_UE_NGAP_ID(42u64),
    /// )?;
    ///
    /// // It is built and encoded as any other, and `check` says what it deviates in
    /// let received = NGAP_PDU::decode(&pdu.encode()?)?;
    /// let deviations = received.check()?;
    /// assert_eq!(deviations.len(), 2);
    /// assert!(matches!(deviations[0], Deviation::OutOfOrder { id: 10, after: 85, .. }));
    /// assert_eq!(deviations[1], Deviation::MissingIe { id: 15, name: "Cause" });
    /// assert_eq!(
    ///     deviations[1].to_string(),
    ///     "Cause (IE 15) is mandatory, and the message does not have it"
    /// );
    /// # Ok::<(), oxirush_ngap::Error>(())
    /// ```
    ///
    /// It reports, and refuses nothing. The macros build a message whatever its IEs
    /// are, a tree writes one, and the codec encodes and decodes it: a test of what a
    /// peer does with such a message needs to send it. Clause 10.3 of TS 38.413 has what
    /// a receiver does: with an IE that it does not comprehend, by the criticality that
    /// the IE has (10.3.4), with one that is missing, by the criticality that the
    /// specification assigns it (10.3.5), and with IEs in a wrong order or with too many
    /// occurrences, which reject the procedure (10.3.6).
    ///
    /// What it does not look at:
    ///
    /// - a conditional IE, which is reported neither when it is missing nor when it is
    ///   there: its condition is in the prose of the specification;
    /// - the values of the IEs, and the IEs that a value holds, in a list, in a transfer
    ///   or in an extension container;
    /// - the IEs of the private message, which has no object set: the criticality of its
    ///   PDU alone is compared.
    ///
    /// # Errors
    ///
    /// An error of the kind `UnknownMessage` for a procedure that the specification does
    /// not have in that direction, which has no object set, and one of the kind `Decode`
    /// when the octets of the message are not a container of protocol IEs.
    pub fn check(&self) -> Result<Vec<Deviation>, Error> {
        let (direction, code) = (self.direction(), self.procedure_code());
        let Some(message) = object_set(direction, code) else {
            return Err(Error::__no_message_of(direction, code));
        };
        let mut found = Vec::new();
        let criticality = self.criticality();
        if let Some(assigned) = procedure_criticality(code)
            && criticality != assigned
        {
            found.push(Deviation::PduCriticality {
                criticality,
                assigned,
            });
        }
        if let Some(set) = message.ies {
            let ies = self.protocol_ies()?;
            let ies: Vec<_> = ies.0.iter().map(|ie| (ie.id.0, ie.criticality)).collect();
            deviations(set, &ies, &mut found);
        }
        Ok(found)
    }
}
