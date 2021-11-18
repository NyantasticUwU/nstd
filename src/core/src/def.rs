use core::ffi::c_void;

/// Returns a null pointer.
/// Returns: `void *null` - A null pointer.
#[inline]
#[no_mangle]
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
/// Represents an unsigned range.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct NSTDURange {
    pub start: u64,
    pub end: u64,
}
