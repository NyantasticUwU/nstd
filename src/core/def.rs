use cty::{c_char, c_double, c_float, c_void};

/// Represents a pointer to any type.
pub type NSTDAny = *mut c_void;
/// Represents a const pointer to any type.
pub type NSTDAnyConst = *const c_void;

/// Represents a size of any type, such as a memory block.
pub type NSTDUSize = usize;
/// Represents a signed size.
pub type NSTDISize = isize;

/// 8-bit unsigned int.
pub type NSTDUInt8 = u8;
/// 8-bit signed int.
pub type NSTDInt8 = i8;
/// 16-bit unsigned int.
pub type NSTDUInt16 = u16;
/// 16-bit signed int.
pub type NSTDInt16 = i16;
/// 32-bit unsigned int.
pub type NSTDUInt32 = u32;
/// 32-bit signed int.
pub type NSTDInt32 = i32;
/// 64-bit unsigned int.
pub type NSTDUInt64 = u64;
/// 64-bit signed int.
pub type NSTDInt64 = i64;

/// 32-bit float.
pub type NSTDFloat32 = c_float;
/// 64-bit float.
pub type NSTDFloat64 = c_double;

/// Alias for C's char primitive.
pub type NSTDChar = c_char;
/// Represents a 8-bit char.
pub type NSTDChar8 = u8;
/// Represents a 16-bit char.
pub type NSTDChar16 = u16;
/// Represents a 32-bit char.
pub type NSTDChar32 = u32;
/// Represents a unicode char type.
pub type NSTDUnichar = NSTDChar32;

/// Represents a byte.
pub type NSTDByte = NSTDUInt8;

/// Represents an error code. An error code of 0 usually means success, while anything else
/// indicates a failure.
pub type NSTDErrorCode = i32;

/// A boolean type.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NSTDBool {
    /// Boolean false or 0.
    NSTD_BOOL_FALSE,
    /// Boolean true or 1.
    NSTD_BOOL_TRUE,
}
impl Default for NSTDBool {
    #[inline]
    fn default() -> Self {
        Self::NSTD_BOOL_FALSE
    }
}
impl From<bool> for NSTDBool {
    #[inline]
    fn from(b: bool) -> Self {
        match b {
            false => Self::NSTD_BOOL_FALSE,
            true => Self::NSTD_BOOL_TRUE,
        }
    }
}
impl Into<bool> for NSTDBool {
    #[inline]
    fn into(self) -> bool {
        self == Self::NSTD_BOOL_TRUE
    }
}

/// The value of a single bit, either on or off.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NSTDBitValue {
    /// The bit is off (0).
    NSTD_BIT_VALUE_OFF,
    /// The bit is on (1).
    NSTD_BIT_VALUE_ON,
}
impl Default for NSTDBitValue {
    /// Returns the default bit value.
    #[inline]
    fn default() -> Self {
        Self::NSTD_BIT_VALUE_OFF
    }
}
