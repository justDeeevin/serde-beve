pub const NULL: u8 = 0;
pub const FALSE: u8 = 0b00001000;
pub const TRUE: u8 = 0b00011000;

pub const BF16: u8 = 0b00000001;
pub const F16: u8 = 0b000100001;
pub const F32: u8 = 0b01000001;
pub const F64: u8 = 0b01100001;
pub const F128: u8 = 0b10000001;

pub const I8: u8 = 0b00001001;
pub const I16: u8 = 0b00101001;
pub const I32: u8 = 0b01001001;
pub const I64: u8 = 0b01101001;
pub const I128: u8 = 0b10001001;

pub const U8: u8 = 0b00010001;
pub const U16: u8 = 0b00110001;
pub const U32: u8 = 0b01010001;
pub const U64: u8 = 0b01110001;
pub const U128: u8 = 0b10010001;

pub const STRING: u8 = 0b00000010;

pub const STRING_OBJECT: u8 = 0b00000011;

pub const I8_OBJECT: u8 = 0b00001011;
pub const I16_OBJECT: u8 = 0b00101011;
pub const I32_OBJECT: u8 = 0b01001011;
pub const I64_OBJECT: u8 = 0b01101011;
pub const I128_OBJECT: u8 = 0b10001011;

pub const U8_OBJECT: u8 = 0b00010011;
pub const U16_OBJECT: u8 = 0b00110011;
pub const U32_OBJECT: u8 = 0b01010011;
pub const U64_OBJECT: u8 = 0b01110011;
pub const U128_OBJECT: u8 = 0b10010011;

pub const BF16_ARRAY: u8 = 0b00000100;
pub const F16_ARRAY: u8 = 0b00100100;
pub const F32_ARRAY: u8 = 0b01000100;
pub const F64_ARRAY: u8 = 0b01100100;
pub const F128_ARRAY: u8 = 0b10000100;

pub const I8_ARRAY: u8 = 0b00001100;
pub const I16_ARRAY: u8 = 0b00101100;
pub const I32_ARRAY: u8 = 0b01001100;
pub const I64_ARRAY: u8 = 0b01101100;
pub const I128_ARRAY: u8 = 0b10001100;

pub const U8_ARRAY: u8 = 0b00010100;
pub const U16_ARRAY: u8 = 0b00110100;
pub const U32_ARRAY: u8 = 0b01010100;
pub const U64_ARRAY: u8 = 0b01110100;
pub const U128_ARRAY: u8 = 0b10010100;

pub const BOOL_ARRAY: u8 = 0b00011100;
pub const STRING_ARRAY: u8 = 0b00111100;
pub const GENERIC_ARRAY: u8 = 0b00000101;

pub const fn header_name(header: u8) -> &'static str {
    match header {
        NULL => "null",
        FALSE | TRUE => "boolean",

        BF16 => "brain float",
        F16 => "16-bit float",
        F32 => "32-bit float",
        F64 => "64-bit float",
        F128 => "128-bit float",

        I8 => "8-bit integer",
        I16 => "16-bit integer",
        I32 => "32-bit integer",
        I64 => "64-bit integer",
        I128 => "128-bit integer",

        U8 => "8-bit unsigned integer",
        U16 => "16-bit unsigned integer",
        U32 => "32-bit unsigned integer",
        U64 => "64-bit unsigned integer",
        U128 => "128-bit unsigned integer",

        STRING => "string",

        STRING_OBJECT => "string-keyed object",

        I8_OBJECT => "8-bit integer-keyed object",
        I16_OBJECT => "16-bit integer-keyed object",
        I32_OBJECT => "32-bit integer-keyed object",
        I64_OBJECT => "64-bit integer-keyed object",
        I128_OBJECT => "128-bit integer-keyed object",

        U8_OBJECT => "unsigned 8-bit integer-keyed object",
        U16_OBJECT => "unsigned 16-bit integer-keyed object",
        U32_OBJECT => "unsigned 32-bit integer-keyed object",
        U64_OBJECT => "unsigned 64-bit integer-keyed object",
        U128_OBJECT => "unsigned 128-bit integer-keyed object",

        BF16_ARRAY => "array of brain floats",
        F16_ARRAY => "array of 16-bit floats",
        F32_ARRAY => "array of 32-bit floats",
        F64_ARRAY => "array of 64-bit floats",
        F128_ARRAY => "array of 128-bit floats",

        I8_ARRAY => "array of 8-bit integers",
        I16_ARRAY => "array of 16-bit integers",
        I32_ARRAY => "array of 32-bit integers",
        I64_ARRAY => "array of 64-bit integers",
        I128_ARRAY => "array of 128-bit integers",

        U8_ARRAY => "array of 8-bit unsigned integers",
        U16_ARRAY => "array of 16-bit unsigned integers",
        U32_ARRAY => "array of 32-bit unsigned integers",
        U64_ARRAY => "array of 64-bit unsigned integers",
        U128_ARRAY => "array of 128-bit unsigned integers",

        BOOL_ARRAY => "array of booleans",
        STRING_ARRAY => "array of strings",
        GENERIC_ARRAY => "generic array",
        _ => "unknown type",
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ArrayKind {
    Generic,
    String,
    Boolean,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    BF16,
    F16,
    F32,
    F64,
}

impl ArrayKind {
    pub const fn header(self) -> u8 {
        match self {
            Self::Generic => GENERIC_ARRAY,
            Self::String => STRING,
            Self::Boolean => TRUE,
            Self::I8 => I8,
            Self::I16 => I16,
            Self::I32 => I32,
            Self::I64 => I64,
            Self::I128 => I128,
            Self::U8 => U8,
            Self::U16 => U16,
            Self::U32 => U32,
            Self::U64 => U64,
            Self::U128 => U128,
            Self::BF16 => BF16,
            Self::F16 => F16,
            Self::F32 => F32,
            Self::F64 => F64,
        }
    }
}

impl std::fmt::Display for ArrayKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic => write!(f, "{}", header_name(GENERIC_ARRAY)),
            Self::String => write!(f, "{}", header_name(STRING)),
            Self::Boolean => write!(f, "{}", header_name(TRUE)),
            Self::I8 => write!(f, "{}", header_name(I8)),
            Self::I16 => write!(f, "{}", header_name(I16)),
            Self::I32 => write!(f, "{}", header_name(I32)),
            Self::I64 => write!(f, "{}", header_name(I64)),
            Self::I128 => write!(f, "{}", header_name(I128)),
            Self::U8 => write!(f, "{}", header_name(U8)),
            Self::U16 => write!(f, "{}", header_name(U16)),
            Self::U32 => write!(f, "{}", header_name(U32)),
            Self::U64 => write!(f, "{}", header_name(U64)),
            Self::U128 => write!(f, "{}", header_name(U128)),
            Self::BF16 => write!(f, "{}", header_name(BF16)),
            Self::F16 => write!(f, "{}", header_name(F16)),
            Self::F32 => write!(f, "{}", header_name(F32)),
            Self::F64 => write!(f, "{}", header_name(F64)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ObjectKind {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    String,
}

impl ObjectKind {
    pub fn header(self) -> u8 {
        match self {
            Self::U8 => U8_OBJECT,
            Self::U16 => U16_OBJECT,
            Self::U32 => U32_OBJECT,
            Self::U64 => U64_OBJECT,
            Self::U128 => U128_OBJECT,
            Self::I8 => I8_OBJECT,
            Self::I16 => I16_OBJECT,
            Self::I32 => I32_OBJECT,
            Self::I64 => I64_OBJECT,
            Self::I128 => I128_OBJECT,
            Self::String => STRING_OBJECT,
        }
    }
}

impl std::fmt::Display for ObjectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8 => write!(f, "{}", header_name(U8)),
            Self::U16 => write!(f, "{}", header_name(U16)),
            Self::U32 => write!(f, "{}", header_name(U32)),
            Self::U64 => write!(f, "{}", header_name(U64)),
            Self::U128 => write!(f, "{}", header_name(U128)),
            Self::I8 => write!(f, "{}", header_name(I8)),
            Self::I16 => write!(f, "{}", header_name(I16)),
            Self::I32 => write!(f, "{}", header_name(I32)),
            Self::I64 => write!(f, "{}", header_name(I64)),
            Self::I128 => write!(f, "{}", header_name(I128)),
            Self::String => write!(f, "{}", header_name(STRING)),
        }
    }
}
