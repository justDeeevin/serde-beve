//! # Serde BEVE
//!
//! A Serde data format for [BEVE](https://github.com/beve-org/beve).
//!
//! ## Notes
//! Since BEVE is a binary format, this crate doesn't provide any tools for serializing to or
//! deserializing from strings.
//!
//! BEVE collections (arrays, object, and strings) store their lengths as compressed integers[^1]. The compression method uses the first two bits to indicate the number of bytes in the integer, and as such, the maximum size is 62 bits[^2]. If, for some reason, you have a string with more than that many characters, an array with more than that many items, or (heaven forbid) a struct or map with more than that many fields, serialization will fail.
//!
//! BEVE is a little-endian format, and for the sake of simplicity, this crate assumes it is being
//! used on a little-endian system.
//!
//! [^1]: <https://github.com/beve-org/beve?tab=readme-ov-file#compressed-unsigned-integer>
//! [^2]: (2^62) - 1 = 4611686018427387904

pub mod de;
pub mod error;
pub mod ser;

pub use error::{Error, Result};
pub use ser::to_writer;

mod headers;
