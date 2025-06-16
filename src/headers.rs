pub const NULL: u8 = 0;
pub const FALSE: u8 = 0b00001000;
pub const TRUE: u8 = 0b00011000;

pub const BRAIN_FLOAT: u8 = 0b00000001;
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
pub const INT_OBJECT: u8 = 0b00001011;
pub const UINT_OBJECT: u8 = 0b00010011;
pub const TYPED_ARRAY: u8 = 0b00000100;
pub const GENERIC_ARRAY: u8 = 0b00000101;
