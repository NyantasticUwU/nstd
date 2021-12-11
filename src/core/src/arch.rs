use core::ffi::c_void;

/// Represents an endianness of a CPU.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NSTDEndian {
    NSTD_ENDIAN_LITTLE,
    NSTD_ENDIAN_BIG,
}

/// Returns the size (in bytes) of a pointer.
/// Returns: `NSTDUSize size` - Size of a pointer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_arch_ptr_size() -> usize {
    core::mem::size_of::<*const c_void>()
}

/// Returns the target CPU's endianness.
/// Returns: `NSTDEndian endian` - The target CPU endianness.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_arch_endian() -> NSTDEndian {
    #[cfg(target_endian = "little")]
    return NSTDEndian::NSTD_ENDIAN_LITTLE;
    #[cfg(target_endian = "big")]
    return NSTDEndian::NSTD_ENDIAN_BIG;
}
