use core::ffi::c_void;

/// Represents a reference to any type.
#[repr(C)]
pub struct NSTDPointer {
    /// Raw pointer to the referenced object.
    pub ptr: *mut c_void,
    /// Size in bytes of the referenced object.
    pub size: usize,
}
/// Conversion methods.
impl NSTDPointer {
    /// Interprets an NSTDPointer as a byte slice.
    #[inline]
    pub unsafe fn as_byte_slice(&self) -> &[u8] {
        core::slice::from_raw_parts(self.ptr as *const u8, self.size)
    }

    /// Interprets an NSTDPointer as a mutable byte slice.
    #[inline]
    pub unsafe fn as_byte_slice_mut(&mut self) -> &mut [u8] {
        core::slice::from_raw_parts_mut(self.ptr as *mut u8, self.size)
    }
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

/// Overwrites the current referenced object's data with `obj`.
/// Parameters:
///     `NSTDPointer *const ptr` - The pointer.
///     `const void *const obj` - The object to overwrite with.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_pointer_write(ptr: &mut NSTDPointer, obj: *const c_void) {
    let obj_data = core::slice::from_raw_parts(obj.cast(), ptr.size);
    ptr.as_byte_slice_mut().copy_from_slice(obj_data);
}
