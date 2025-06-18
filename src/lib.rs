//! # Serde BEVE
//!
//! A Serde data format for [BEVE](https://github.com/beve-org/beve).
//!
//! ## 16-bit floats
//!
//! BEVE supports both [`f16`](half::f16)s and [brain floats](https://en.wikipedia.org/wiki/Bfloat16_floating-point_format). This crate supports the deserialization of both, converting them into [`f32`]s. However, similar to [extension types](https://github.com/beve-org/beve/blob/main/extensions.md), Serde provides no means of serializing these types, and as such **their deserialization is gated by the `half` feature**, which enables the use of the [`half`] crate for working with them.
//!
//! ## Notes
//!
//! Since BEVE is a binary format, this crate doesn't provide any tools for serializing to or deserializing from strings.
//!
//! BEVE collections (arrays, object, and strings) store their lengths as compressed integers[^1]. The compression method uses the first two bits to indicate the number of bytes in the integer, and as such, the maximum size is 62 bits[^2]. If, for some reason, you have a string with more than that many bytes, an array with more than that many items, or (heaven forbid) a struct or map with more than that many fields, serialization will fail.
//!
//! BEVE is a little-endian format, and for the sake of simplicity, this crate assumes it is being used on a little-endian system.
//!
//! [^1]: <https://github.com/beve-org/beve?tab=readme-ov-file#compressed-unsigned-integer>
//!
//! [^2]: (2^62) - 1 = 4611686018427387904

/// Deserialization logic.
pub mod de;
/// Errors that can occur during serialization or deserialization.
pub mod error;
/// Serialization logic.
pub mod ser;
pub mod value;

mod headers;

pub use de::from_reader;
pub use error::{Error, Result};
pub use headers::{ArrayKind, ObjectKind};
pub use ser::to_writer;
pub use value::Value;
