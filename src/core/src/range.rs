use core::ops::Range;

/// Represents a signed range.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct NSTDIRange {
    /// Start of the range (included).
    pub start: isize,
    /// End of the range (exluded).
    pub end: isize,
}
impl Into<Range<isize>> for NSTDIRange {
    #[inline]
    fn into(self) -> Range<isize> {
        self.start..self.end
    }
}
/// Represents an unsigned range.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct NSTDURange {
    /// Start of the range (included).
    pub start: usize,
    /// End of the range (exluded).
    pub end: usize,
}
impl Into<Range<usize>> for NSTDURange {
    #[inline]
    fn into(self) -> Range<usize> {
        self.start..self.end
    }
}
