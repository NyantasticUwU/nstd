use core::{ffi::c_void, ops::Range};

/// Returns a null pointer.
/// Returns: `void *null` - A null pointer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_def_null() -> *mut c_void {
    core::ptr::null_mut()
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
