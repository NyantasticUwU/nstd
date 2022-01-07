use core::ops::Range;
use cty::{c_double, c_float, c_int, c_void};

/// Represents a pointer to any type.
pub type NSTDAny = *mut c_void;

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

/// Represents a byte.
pub type NSTDByte = NSTDUInt8;

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
        self as c_int != 0
    }
}

/// Represents a signed range.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct NSTDIRange {
    /// Start of the range (included).
    pub start: i64,
    /// End of the range (exluded).
    pub end: i64,
}
impl Into<Range<i64>> for NSTDIRange {
    #[inline]
    fn into(self) -> Range<i64> {
        self.start..self.end
    }
}
/// Represents an unsigned range.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct NSTDURange {
    /// Start of the range (included).
    pub start: u64,
    /// End of the range (exluded).
    pub end: u64,
}
impl Into<Range<u64>> for NSTDURange {
    #[inline]
    fn into(self) -> Range<u64> {
        self.start..self.end
    }
}
