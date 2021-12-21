use core::ops::Range;

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
