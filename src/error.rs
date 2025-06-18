use crate::headers::{ArrayKind, ObjectKind};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
/// Errors that can occur during serialization or deserialization.
pub enum Error {
    #[error("{0}")]
    /// A custom error message.
    ///
    /// This variant is never directly constructed by this library, but rather is used for serde's
    /// `Error` traits[^1].
    ///
    /// [^1]: [`serde::ser::Error`] and [`serde::de::Error`]
    Custom(String),

    #[error("IO error")]
    /// An IO error produced by an internal [`Read`](std::io::Read)er or [`Write`](std::io::Write)r.
    Io(
        #[from]
        #[source]
        std::io::Error,
    ),

    #[error("Keys must be strings or integers")]
    /// Returned when attempting to serialize an object key that is not a string or integer.
    InvalidKey,

    #[error("Unsupported data type: {0}")]
    /// Returned when trying to deserialize an unsupported data type.
    ///
    /// This can occur when trying to deserialize an [`f16`](half::f16) or a [`bf16`](half::bf16)
    /// without the `half` feature enabled. It can also occur when trying to deserialize a 128-bit
    /// float, as these are not yet stable in Rust.
    UnsupportedDataType(SpecialType),

    #[error("Invalid type. Expected {expected}, found {found}.")]
    /// Returned when a type is found during deserialization that doesn't match the expected type
    /// (e.g. [`deserialize_u8`](serde::Deserializer::deserialize_u8) is called but a `u16` header
    /// is encountered).
    WrongType {
        expected: &'static str,
        found: &'static str,
    },

    #[error("Mismatched key types. Expected {expected}, found {found}.")]
    /// Returned when, during the serialization or deserialization of an object, a key is
    /// encountered that doesn't match the first encountered key.
    /// Returned when, during the serialization or deserialization of an object, a key is
    ///
    /// Objects in BEVE can be keyed by strings or integers, but all fields must be of the same
    /// type.
    MismatchedKeyType {
        expected: ObjectKind,
        found: ObjectKind,
    },

    #[error("Mismatched array type. Expected {expected}, found {found}.")]
    MismatchedElementType {
        expected: ArrayKind,
        found: ArrayKind,
    },

    #[error("Object, array, or string too long")]
    /// Returned either when trying to serialize data whose size is greater than (2^62)-1
    /// bits or when trying to deserialize a size that is larger than the platform's pointer width
    /// (e.g. trying to deserialize a 62-bit size on a 32-bit platform).
    TooLong,

    #[error("Invalid header: {0:08b}")]
    /// Returned when a header is encountered that does not fit the BEVE format.
    InvalidHeader(u8),

    #[error("Invalid UTF-8 sequence")]
    /// Returned when the deserialization of a [`str`] fails.
    Utf8(
        #[from]
        #[source]
        std::str::Utf8Error,
    ),

    #[error("Invalid UTF-8 sequence")]
    /// Returned when the deserialization of a [`String`] fails.
    FromUtf8(
        #[from]
        #[source]
        std::string::FromUtf8Error,
    ),

    #[error("Cannot deserialize reserved")]
    /// Returned when attempting to deserialize a reserved header.
    Reserved,

    #[error("Enum variant tags must be deserialized as identifiers")]
    /// Returned when attempting to deserialize an enum tag as something other than an identifier.
    InvalidTag,

}

#[derive(Debug)]
pub enum SpecialType {
    /// A 16-bit float.
    HalfFloat,
    /// A [brain float](https://en.wikipedia.org/wiki/Bfloat16_floating-point_format).
    BrainFloat,
    /// A 128-bit float.
    F128,
}

impl std::fmt::Display for SpecialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecialType::HalfFloat => write!(f, "16-bit float"),
            SpecialType::F128 => write!(f, "128-bit float"),
            SpecialType::BrainFloat => write!(f, "brain float"),
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Custom(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Custom(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
