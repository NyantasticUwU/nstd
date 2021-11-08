use core::ffi::c_void;

/// Returns the size (in bytes) of a pointer.
/// Returns: `NSTDCOREUSize size` - Size of a pointer.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_arch_ptr_size() -> usize {
    core::mem::size_of::<*const c_void>()
}