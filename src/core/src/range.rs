use core::ops::Range;

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
