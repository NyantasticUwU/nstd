use crate::core::{def::NSTDErrorCode, pointer::NSTDPointer};

/// Represents a heap allocated object.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NSTDHeap {
    /// Raw pointer to heap allocated object.
    pub ptr: NSTDPointer,
}
impl Clone for NSTDHeap {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { nstd_alloc_heap_new(&self.ptr) }
    }
}

/// Creates a new heap allocated object.
/// Parameters:
///     `const NSTDPointer *const ptr` - Pointer to an object to be copied to the heap.
/// Returns: `NSTDHeap obj` - The new heap allocated object.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_heap_new(ptr: &NSTDPointer) -> NSTDHeap {
    let alloc = crate::alloc::nstd_alloc_allocate(ptr.size);
    if !alloc.is_null() {
        let ptr_slice = core::slice::from_raw_parts(ptr.raw.cast(), ptr.size);
        let alloc_slice = core::slice::from_raw_parts_mut(alloc as *mut u8, ptr.size);
        alloc_slice.copy_from_slice(ptr_slice);
    }
    NSTDHeap {
        ptr: crate::core::pointer::nstd_core_pointer_new(alloc, ptr.size),
    }
}

/// Frees a heap allocated object.
/// Parameters:
///     `NSTDHeap *const obj` - The heap allocated object.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_heap_free(obj: &mut NSTDHeap) -> NSTDErrorCode {
    let ptr = &mut obj.ptr.raw;
    crate::alloc::nstd_alloc_deallocate(ptr, obj.ptr.size)
}
