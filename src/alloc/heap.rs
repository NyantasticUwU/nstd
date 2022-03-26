//! Similar to Rust's [Box] type.
use crate::core::{
    def::{NSTDAny, NSTDErrorCode},
    pointer::NSTDPointer,
};

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
impl Drop for NSTDHeap {
    #[inline]
    fn drop(self: &mut NSTDHeap) {
        unsafe { crate::alloc::nstd_alloc_deallocate(&mut self.ptr.raw, self.ptr.size) };
    }
}

/// Creates a new heap allocated object.
///
/// # Parameters
///
/// - `const NSTDPointer *const ptr` - Pointer to an object to be copied to the heap.
///
/// # Returns
///
/// `NSTDHeap obj` - The new heap allocated object.
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

/// Creates a new heap allocated object from a raw pointer.
///
/// # Parameters
///
/// - `const NSTDAny ptr` - A raw pointer to the object to copy to the heap.
///
/// - `const NSTDUSize size` - The size of the object.
///
/// # Returns
///
/// `NSTDHeap obj` - The new heap allocated object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_heap_from_raw(ptr: NSTDAny, size: usize) -> NSTDHeap {
    let ptr = crate::core::pointer::nstd_core_pointer_new(ptr, size);
    nstd_alloc_heap_new(&ptr)
}

/// Creates a new heap object from a raw pointer without making any allocations.
///
/// # Parameters
///
/// - `const NSTDAny ptr` - A raw pointer to the heap object.
///
/// - `const NSTDUSize size` - The size of the heap object.
///
/// # Returns
///
/// `NSTDHeap obj` - The new heap object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_heap_from_existing(ptr: NSTDAny, size: usize) -> NSTDHeap {
    NSTDHeap {
        ptr: crate::core::pointer::nstd_core_pointer_new(ptr, size),
    }
}

/// Frees a heap allocated object.
///
/// # Parameters
///
/// - `NSTDHeap *const obj` - The heap allocated object.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_heap_free(obj: &mut NSTDHeap) -> NSTDErrorCode {
    crate::alloc::nstd_alloc_deallocate(&mut obj.ptr.raw, obj.ptr.size)
}
