use core::ops::Range;

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
