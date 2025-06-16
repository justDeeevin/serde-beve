use crate::ser::KeyType;

#[derive(Debug, thiserror::Error)]
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
    #[error("Mismatched key types. Expected {expected}, found {found}.")]
    MismatchedKeyType { expected: KeyType, found: KeyType },
    #[error("Object, array, or string too long")]
    TooLong,
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
