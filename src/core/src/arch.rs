/// Returns the size (in bytes) of a pointer.
/// Returns: `NSTDUSize size` - Size of a pointer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_arch_ptr_size() -> usize {
    core::mem::size_of::<&()>()
}
