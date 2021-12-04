use core::ffi::c_void;

/// Represents a reference to any type.
#[repr(C)]
pub struct NSTDPointer {
    /// Raw pointer to the referenced object.
    pub ptr: *mut c_void,
    /// Size in bytes of the referenced object.
    pub size: usize,
}

/// Creates a new instance of `NSTDPointer`.
/// Parameters:
///     `void *const obj` - The object to reference.
///     `const NSTDUSize size` - The size in bytes of `obj`.
/// Returns: `NSTDPointer ptr` - The pointer type.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_pointer_new(obj: *mut c_void, size: usize) -> NSTDPointer {
    NSTDPointer { ptr: obj, size }
}
