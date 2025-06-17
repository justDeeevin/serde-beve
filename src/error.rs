use crate::headers::{ArrayKind, ObjectKind};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("{0}")]
    Custom(String),
    #[error("IO error")]
    Io(
        #[from]
        #[source]
        std::io::Error,
    ),
    #[error("Arrays and objects must have a length")]
    MissingLength,
    #[error("Keys must be strings or integers")]
    InvalidKey,
    #[error("Unsupported data type: {0}")]
    UnsupportedDataType(SpecialType),
    #[error("Invalid type. Expected {expected}, found {found}.")]
    WrongType {
        expected: &'static str,
        found: &'static str,
    },
    #[error("Mismatched key types. Expected {expected}, found {found}.")]
    MismatchedKeyType {
        expected: ObjectKind,
        found: ObjectKind,
    },
    #[error("Mismatched array type. Expected {expected}, found {found}.")]
    MismatchedArrayType {
        expected: ArrayKind,
        found: ArrayKind,
    },
    #[error("Object, array, or string too long")]
    /// This can occur either when trying to serialize data whose size is greater than (2^62)-1
    /// bits or when trying to deserialize a size that is larger than the platform's pointer width
    /// (e.g. trying to deserialize a 62-bit size on a 32-bit platform).
    TooLong,
    #[error("Invalid header: {0:08b}")]
    InvalidHeader(u8),
    #[error("Invalid UTF-8 sequence")]
    Utf8(
        #[from]
        #[source]
        std::str::Utf8Error,
    ),
    #[error("Invalid UTF-8 sequence")]
    FromUtf8(
        #[from]
        #[source]
        std::string::FromUtf8Error,
    ),
    #[error("Cannot deserialize reserved")]
    Reserved,
    #[error("Variants are tagged by index in BEVE")]
    Variant,
    #[error("Variant index out of range")]
    VariantOutOfRange,
}

#[derive(Debug)]
pub enum SpecialType {
    HalfFloat,
    BrainFloat,
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
