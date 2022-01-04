use core::ops::Range;

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

/// Represents a byte.
pub type NSTDByte = NSTDUInt8;

/// A boolean type.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NSTDBool {
    NSTD_BOOL_FALSE,
    NSTD_BOOL_TRUE,
}
impl Default for NSTDBool {
    #[inline]
    fn default() -> Self {
        Self::NSTD_BOOL_FALSE
    }
}

/// Represents a signed range.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct NSTDIRange {
    pub start: i64,
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
    pub start: u64,
    pub end: u64,
}
impl Into<Range<u64>> for NSTDURange {
    #[inline]
    fn into(self) -> Range<u64> {
        self.start..self.end
    }
}
