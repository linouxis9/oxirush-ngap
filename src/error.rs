//! The error of the crate: a kind that a caller matches on, and a message for the one
//! who reads it.

use std::fmt;
use std::sync::Arc;

use rasn::error::{DecodeError, DecodeErrorKind, EncodeError, EncodeErrorKind};

/// What went wrong: the part of an [`Error`] that a caller matches on.
///
/// More kinds may come with a later version: match the ones that matter, and leave the
/// others to `_`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Octets are not the value that they are decoded as, in aligned PER: they end
    /// before the value does, go on after it, or hold a value outside its constraint.
    Decode,
    /// A value does not encode in aligned PER, as an integer outside its constraint.
    Encode,
    /// `decode_value::<T>()` on a PDU that holds another message than `T`, and the
    /// protocol IEs of a PDU whose message has none, as the private message.
    WrongMessage,
    /// A required IE that the message does not have: a `req` field of the macro that
    /// extracts IEs.
    MissingIe,
    /// An IE, or a transfer, that is there and does not decode as its type.
    UndecodableIe,
    /// An argument that is not what a helper takes, as an MCC of two digits.
    Argument,
    /// A name of a message, or a direction and a procedure code, that the protocol
    /// does not have.
    UnknownMessage,
    /// A name or an identifier of an IE that the protocol does not have, or has no
    /// type for where it is.
    UnknownIe,
    /// A path that is not well formed.
    Path,
    /// A name that the type at that place cannot have: a member of a value, an IE of
    /// a message, or a name where a list goes by position.
    NoSuchMember,
    /// An edit whose path selects nothing.
    NothingSelected,
    /// A path through the type that octets carry, before `open` decoded them.
    NotOpened,
    /// A JSON value that is not a value of its ASN.1 type: of another form, with a
    /// name that an ENUMERATED does not have, or outside a constraint.
    Value,
    /// JSON that is not written as a tree or as a view.
    Form,
    /// An edit that is refused: a member that its type always has is not taken out,
    /// and the octets of a value that was edited since are not read.
    Edit,
    /// Nesting or a number of occurrences over the limits of the inspection, or
    /// what it does not represent.
    Limit,
}

/// The error of every function and macro of the crate that can fail.
///
/// [`kind`](Self::kind) is what a caller matches on. `Display` says what is wrong, in
/// the words of the specification where it has them, and what to do about it where
/// something can be done. `source()` is the error of `rasn` where the codec gave one.
///
/// Two errors are equal when they have the same kind and the same message.
#[derive(Clone)]
pub struct Error {
    kind: ErrorKind,
    /// What is wrong, and after it the reason, where the message has one apart: what
    /// the codec says of octets or of a value.
    what: Box<str>,
    why: Option<Box<str>>,
    source: Option<Arc<dyn std::error::Error + Send + Sync + 'static>>,
}

impl Error {
    /// What went wrong, as a caller matches on it.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub(crate) fn new(kind: ErrorKind, what: impl Into<String>) -> Self {
        Self {
            kind,
            what: what.into().into_boxed_str(),
            why: None,
            source: None,
        }
    }

    /// The reason of what is wrong, which the message has after it.
    fn because(mut self, why: impl Into<String>) -> Self {
        self.why = Some(why.into().into_boxed_str());
        self
    }

    /// The error of rasn that this one tells of.
    fn caused_by(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.source = Some(Arc::new(source));
        self
    }

    /// Octets that are not a value of the type `T`, and why.
    pub(crate) fn not_a<T: rasn::AsnType>(why: impl Into<String>) -> Self {
        let what = format!("these octets do not decode as {}", name::<T>());
        Self::new(ErrorKind::Decode, what).because(why)
    }

    /// Octets that rasn does not decode as a value of the type `T`.
    pub(crate) fn decoding<T: rasn::AsnType>(error: DecodeError) -> Self {
        Self::not_a::<T>(in_its_field(&error)).caused_by(error)
    }

    /// A value of the type `T` that rasn does not encode.
    pub(crate) fn encoding<T: rasn::AsnType>(error: EncodeError) -> Self {
        let name = T::IDENTIFIER.0.unwrap_or("this type");
        let what = format!("a value of {name} does not encode");
        let why = encode_reason(&error);
        Self::new(ErrorKind::Encode, what)
            .because(why)
            .caused_by(error)
    }

    /// A PDU of the message of `direction` and `code`, which is decoded as the
    /// message that ASN.1 names `expected`.
    pub(crate) fn wrong_message(expected: &str, direction: &str, code: u8) -> Self {
        let message = match crate::registry::message_name(direction, code) {
            Some(held) => format!(
                "the message of this PDU is {held}, not {expected}: kind() says which \
                 message a PDU holds"
            ),
            None => format!(
                "the message of this PDU is not {expected}: the specification has no \
                 {direction} of procedure {code}"
            ),
        };
        Self::new(ErrorKind::WrongMessage, message)
    }

    /// A PDU of the message that ASN.1 names `name`, which has no container of
    /// protocol IEs.
    pub(crate) fn no_protocol_ies(name: &str) -> Self {
        let what =
            format!("the message of this PDU is {name}, which has no container of protocol IEs");
        Self::new(ErrorKind::WrongMessage, what)
    }

    /// A PDU whose direction and procedure code the specification has no message for,
    /// and so no object set.
    pub(crate) fn no_message_of(direction: &str, code: u8) -> Self {
        let what = format!(
            "the specification has no {direction} of procedure {code}, and so no object set \
             for its IEs"
        );
        Self::new(ErrorKind::UnknownMessage, what)
    }

    /// This error is why the message of a PDU does not decode as a container of
    /// protocol IEs: the message that ASN.1 names `name`, when the specification has
    /// one for the PDU.
    pub(crate) fn of_message(self, name: Option<&str>) -> Self {
        const UNNAMED: &str =
            "the message of this PDU does not decode as a container of protocol IEs";
        self.told_of(match name {
            Some(name) => format!("the {name} of this PDU does not decode"),
            None => UNNAMED.to_string(),
        })
    }

    /// This error is why `count` protocol IEs do not encode as a message.
    pub(crate) fn of_protocol_ies(self, count: usize) -> Self {
        self.told_of(format!(
            "these {count} protocol IEs do not encode as a message"
        ))
    }

    /// The same error, of `what`: what it said is wrong is the reason, where it gave
    /// none apart.
    fn told_of(self, what: String) -> Self {
        Self {
            kind: self.kind,
            what: what.into_boxed_str(),
            why: Some(self.why.unwrap_or(self.what)),
            source: self.source,
        }
    }

    /// An argument that a helper does not take, and what it takes.
    pub(crate) fn argument(what: impl Into<String>) -> Self {
        Self::new(ErrorKind::Argument, what)
    }

    /// The same error, with the message that `reword` makes of the one that it has.
    #[cfg(feature = "inspect")]
    pub(crate) fn reworded(self, reword: impl FnOnce(&str) -> String) -> Self {
        Self {
            kind: self.kind,
            what: reword(&self.to_string()).into_boxed_str(),
            why: None,
            source: self.source,
        }
    }

    /// A value that the JSON encoding rules of rasn do not write.
    #[cfg(feature = "inspect")]
    pub(crate) fn unwritten(error: EncodeError) -> Self {
        let why = encode_reason(&error);
        Self::unrepresented(why).caused_by(error)
    }

    /// A value that the JSON encoding rules of rasn do not write, or do not read back
    /// as it is: a limit of the inspection, and no fault of the value.
    #[cfg(feature = "inspect")]
    pub(crate) fn unrepresented(why: impl Into<String>) -> Self {
        Self::new(ErrorKind::Limit, "JER does not represent this value").because(why)
    }

    /// JSON that rasn does not read as a value of the type `T`: the members on the way
    /// to where it stopped, by the names that ASN.1 gives them, and what is wrong there.
    #[cfg(feature = "inspect")]
    pub(crate) fn not_a_value<T: rasn::AsnType>(
        at: &[String],
        why: String,
        source: DecodeError,
    ) -> Self {
        let what = match at.is_empty() {
            true => format!("this is not a value of {}", name::<T>()),
            false => format!(
                "this is not a value of {}, at {}",
                name::<T>(),
                at.join("/")
            ),
        };
        Self::new(ErrorKind::Value, what)
            .because(why)
            .caused_by(source)
    }

    /// JSON that is read as a value of the type `T` and is not what that value is
    /// written as: the kind of the error, the members on the way and what is wrong.
    #[cfg(feature = "inspect")]
    pub(crate) fn not_written_as<T: rasn::AsnType>(
        kind: ErrorKind,
        at: &[String],
        why: String,
    ) -> Self {
        let what = match at.is_empty() {
            true => format!("this is not a value of {}", name::<T>()),
            false => format!(
                "this is not a value of {}, at {}",
                name::<T>(),
                at.join("/")
            ),
        };
        Self::new(kind, what).because(why)
    }

    /// A value of the type `T` that is written in JSON and does not encode: it is
    /// outside a constraint of its type, and so no value of that type. This error is
    /// what the codec said of it.
    #[cfg(feature = "inspect")]
    pub(crate) fn outside_its_type<T: rasn::AsnType>(self) -> Self {
        Self {
            kind: ErrorKind::Value,
            what: format!("this is not a value of {}", name::<T>()).into_boxed_str(),
            why: Some(self.why.unwrap_or(self.what)),
            source: self.source,
        }
    }

    /// For the macro that extracts IEs: the IE `id`, which the field `field` requires,
    /// is not in a message of the type `message`.
    #[doc(hidden)]
    pub fn __missing_ie(message: &str, id: u16, field: &str) -> Self {
        let ie = ie(id);
        let what = format!("this {message} has no {ie}, which the field `{field}` requires");
        Self::new(ErrorKind::MissingIe, what)
    }

    /// For the macros that read an IE: this error is why the IE `id` of a message of
    /// the type `message` does not decode.
    #[doc(hidden)]
    pub fn __undecodable_ie(self, message: &str, id: u16) -> Self {
        let what = format!("the {} of this {message} does not decode", ie(id));
        Self {
            kind: ErrorKind::UndecodableIe,
            what: what.into_boxed_str(),
            // What the codec said of the octets, without the type that it took them for.
            why: Some(self.why.unwrap_or(self.what)),
            source: self.source,
        }
    }
}

/// The errors of the `inspect` feature, each by its kind and with what is wrong.
macro_rules! kinds {
    ($($(#[$doc:meta])* $name:ident $kind:ident;)*) => {
        #[cfg(feature = "inspect")]
        impl Error {
            $(
                $(#[$doc])*
                pub(crate) fn $name(what: impl Into<String>) -> Self {
                    Self::new(ErrorKind::$kind, what)
                }
            )*
        }
    };
}

kinds! {
    /// A name of a message, or a direction and a procedure code, that the protocol
    /// does not have.
    unknown_message UnknownMessage;
    /// A name or an identifier of an IE that the protocol does not have, or has no
    /// type for where it is.
    unknown_ie UnknownIe;
    /// A path that is not well formed.
    path Path;
    /// A name that the type at that place cannot have.
    no_such_member NoSuchMember;
    /// An edit whose path selects nothing.
    nothing_selected NothingSelected;
    /// A path through the type that octets carry, before they are opened.
    not_opened NotOpened;
    /// A path into an IE or a transfer that is there and did not decode.
    undecodable UndecodableIe;
    /// A JSON value that is not a value of its ASN.1 type.
    value Value;
    /// JSON that is not written as a tree or as a view.
    form Form;
    /// An edit that is refused.
    edit Edit;
    /// Nesting or a number of occurrences over the limits of the inspection.
    limit Limit;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.what)?;
        match &self.why {
            Some(why) => write!(formatter, ": {why}"),
            None => Ok(()),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The source is the same error as rasn tells it, and is reached as `source()`.
        let mut error = formatter.debug_struct("Error");
        error.field("kind", &self.kind);
        error.field("message", &self.to_string());
        error.finish()
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        let source = self.source.as_deref()?;
        Some(source)
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.to_string() == other.to_string()
    }
}

impl Eq for Error {}

/// Octets that do not decode, where nothing says what they were decoded as.
impl From<DecodeError> for Error {
    fn from(error: DecodeError) -> Self {
        let why = in_its_field(&error);
        Self::new(ErrorKind::Decode, "the octets do not decode")
            .because(why)
            .caused_by(error)
    }
}

/// A value that does not encode, where nothing says what its type is.
impl From<EncodeError> for Error {
    fn from(error: EncodeError) -> Self {
        let why = encode_reason(&error);
        Self::new(ErrorKind::Encode, "the value does not encode")
            .because(why)
            .caused_by(error)
    }
}

/// The IE of an identifier, as a message names it: the name that ASN.1 gives it after
/// `id-`, or its number for one that the protocol does not have.
fn ie(id: u16) -> String {
    let mut names = crate::registry::IE_NAMES.iter();
    match names.find(|(known, _)| *known == id) {
        Some((_, name)) => name.to_string(),
        None => format!("IE {id}"),
    }
}

/// The name that ASN.1 gives the type `T`. The sentences that have it need no article:
/// the names of a protocol are read letter by letter as often as not.
fn name<T: rasn::AsnType>() -> &'static str {
    T::IDENTIFIER.0.unwrap_or("their type")
}

/// Why rasn does not decode octets, and the field of the bindings where it stopped: the
/// typed API names a member as the bindings do.
fn in_its_field(error: &DecodeError) -> String {
    let (fields, why) = decode_reason(error);
    match fields.is_empty() {
        true => why,
        false => format!("{why}, in its field `{}`", fields.join(".")),
    }
}

/// Why rasn does not decode a value, without the name of its codec: the members on the
/// way to where the decoding stopped, each as the bindings name the field, as
/// `g_nb_id`, then what is wrong there.
pub(crate) fn decode_reason(error: &DecodeError) -> (Vec<&'static str>, String) {
    // rasn names a field after the type that has it: `GlobalGNBID.g_nb_id`.
    let field = |name: &'static str| name.rsplit('.').next().unwrap_or(name);
    let mut members = Vec::new();
    let mut error = error;
    while let DecodeErrorKind::FieldError { name, nested } = &*error.kind {
        members.push(field(name));
        error = nested;
    }
    let why = match &*error.kind {
        DecodeErrorKind::Incomplete { .. } | DecodeErrorKind::Eof => {
            "the octets end before the value does".to_string()
        }
        DecodeErrorKind::UnexpectedEmptyInput => "there are no octets where a value is".to_string(),
        DecodeErrorKind::UnexpectedExtraData { length } => {
            format!("{length} octets follow the value")
        }
        DecodeErrorKind::ValueConstraintNotSatisfied { value, expected } => {
            format!("{value} is outside its constraint ({expected})")
        }
        DecodeErrorKind::SizeConstraintNotSatisfied { size, expected } => match size {
            Some(size) => format!("a size of {size} is outside its constraint ({expected})"),
            None => format!("the size is outside its constraint ({expected})"),
        },
        DecodeErrorKind::IntegerOverflow { max_width } => {
            format!("the number does not fit in {max_width} bits")
        }
        DecodeErrorKind::EnumerationIndexNotFound { index, .. } => {
            format!("{index} is no value of the ENUMERATED")
        }
        DecodeErrorKind::ChoiceIndexNotFound { index, .. } => {
            format!("{index} is no alternative of the CHOICE")
        }
        DecodeErrorKind::MissingField { name } => {
            members.push(field(name));
            "the member is missing".to_string()
        }
        DecodeErrorKind::DuplicateField { name } => {
            members.push(field(name));
            "the member is there twice".to_string()
        }
        DecodeErrorKind::Custom { msg } => msg.clone(),
        DecodeErrorKind::CodecSpecific { inner } => codec_reason(inner),
        other => other.to_string(),
    };
    (members, why)
}

/// Why the JSON encoding rules do not read a value, and what another codec says.
fn codec_reason(error: &rasn::error::CodecDecodeError) -> String {
    use rasn::error::{CodecDecodeError, JerDecodeErrorKind};
    let CodecDecodeError::Jer(error) = error else {
        return error.to_string();
    };
    match error {
        JerDecodeErrorKind::EndOfInput {} => "the value is incomplete".to_string(),
        // A bit string whose size varies is the one type that rasn names so.
        JerDecodeErrorKind::TypeMismatch { needed, .. } if needed.contains("'length'") => {
            "a bit string whose size varies is written as {\"value\": HEXADECIMAL, \"length\": BITS}"
                .to_string()
        }
        JerDecodeErrorKind::TypeMismatch { needed, found } => {
            let needed = match *needed {
                "enumerated item as string" => "the name of a value of the ENUMERATED",
                "number (supported range -2^63..2^63)" => "a number below 2^63",
                "hex string" => "octets in hexadecimal",
                "object" => "an object",
                "array" => "a list",
                "string" => "a string",
                "boolean" => "true or false",
                other => other,
            };
            match found.as_str() {
                "unknown" => format!("{needed} is expected"),
                found => format!("{needed} is expected, and {found} is written"),
            }
        }
        JerDecodeErrorKind::InvalidJerBitstring { .. } => {
            "a bit string is written in hexadecimal".to_string()
        }
        JerDecodeErrorKind::InvalidJerOctetString {} => {
            "octets are written in hexadecimal, two digits for each".to_string()
        }
        JerDecodeErrorKind::InvalidEnumDiscriminant { discriminant } => {
            format!("{discriminant:?} is no value of the ENUMERATED")
        }
        other => other.to_string(),
    }
}

/// Why rasn does not encode a value, without the name of its codec.
fn encode_reason(error: &EncodeError) -> String {
    match &*error.kind {
        EncodeErrorKind::ValueConstraintNotSatisfied { value, expected } => {
            format!("{value} is outside its constraint ({expected})")
        }
        EncodeErrorKind::SizeConstraintNotSatisfied { size, expected }
        | EncodeErrorKind::InvalidLength {
            length: size,
            expected,
        } => format!("a size of {size} is outside its constraint ({expected})"),
        EncodeErrorKind::AlphabetConstraintNotSatisfied { reason } => {
            format!("a character is outside its alphabet: {reason}")
        }
        EncodeErrorKind::Custom { msg } => msg.clone(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_error_is_cloned_sent_and_compared_by_its_kind_and_its_message() {
        fn sent<T: Clone + Send + Sync + std::error::Error + 'static>() {}
        sent::<Error>();
        let error = Error::new(ErrorKind::Argument, "an MCC has three digits");
        assert_eq!(error.kind(), ErrorKind::Argument);
        assert_eq!(error.to_string(), "an MCC has three digits");
        assert_eq!(error, error.clone());
        assert_ne!(
            error,
            Error::new(ErrorKind::Value, "an MCC has three digits")
        );
        assert_ne!(error, Error::new(ErrorKind::Argument, "an MNC has two"));
        // The source is not compared: it is the same error, as rasn tells it.
        let caused = error.clone().caused_by(std::fmt::Error);
        assert_eq!(caused, error);
        assert!(std::error::Error::source(&error).is_none());
        assert!(std::error::Error::source(&caused).is_some());
        let shown = format!("{error:?}");
        assert_eq!(
            shown,
            "Error { kind: Argument, message: \"an MCC has three digits\" }"
        );
    }

    #[test]
    fn an_error_of_rasn_is_told_without_the_name_of_its_codec() {
        use rasn::types::Integer;
        // An INTEGER that has no octets, and one whose octets end too early.
        let empty = rasn::aper::decode::<Integer>(&[]).unwrap_err();
        let text = empty.to_string();
        let error = Error::from(empty);
        assert_eq!(error.kind(), ErrorKind::Decode);
        assert!(error.to_string().starts_with("the octets do not decode: "));
        assert!(!error.to_string().contains("Codec"), "{error}");
        // What rasn says is the source, as it says it.
        let source = std::error::Error::source(&error).expect("the error of rasn");
        assert_eq!(source.to_string(), text);
        let short = rasn::aper::decode::<Integer>(&[2, 1]).unwrap_err();
        let named = Error::decoding::<Integer>(short);
        assert_eq!(
            named.to_string(),
            "these octets do not decode as INTEGER: the octets end before the value does"
        );
    }
}
