use core::ffi::c_void;

/// Returns a null pointer.
/// Returns: `void *null` - A null pointer.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_def_null() -> *mut c_void {
    core::ptr::null_mut()
}
