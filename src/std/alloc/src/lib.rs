use nstd_core::pointer::NSTDPointer;
use std::{
    alloc::Layout,
    os::raw::{c_int, c_void},
    ptr::addr_of_mut,
};
#[cfg(feature = "deps")]
pub mod deps {
    pub use nstd_core;
}

/// Represents a heap allocated object.
#[repr(C)]
pub struct NSTDHeap {
    /// Raw pointer to heap allocated object.
    pub ptr: NSTDPointer,
}

/// Allocates a new memory block.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDByte *ptr` - The new memory block.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_alloc_allocate(size: usize) -> *mut u8 {
    match Layout::array::<u8>(size) {
        Ok(layout) => std::alloc::alloc(layout),
        _ => std::ptr::null_mut(),
    }
}

/// Allocates a new memory block with all bytes set to 0.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDByte *ptr` - The new memory block.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_alloc_allocate_zeroed(size: usize) -> *mut u8 {
    match Layout::array::<u8>(size) {
        Ok(layout) => std::alloc::alloc_zeroed(layout),
        _ => std::ptr::null_mut(),
    }
}

/// Reallocates a memory block.
/// Parameters:
///     `NSTDByte **ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - The current size of the memory block.
///     `const NSTDUSize new_size` - The new size of the memory block.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_alloc_reallocate(
    ptr: *mut *mut u8,
    size: usize,
    new_size: usize,
) -> c_int {
    match Layout::array::<u8>(size) {
        Ok(layout) => {
            let new_mem = std::alloc::realloc(*ptr, layout, new_size);
            match new_mem.is_null() {
                false => {
                    *ptr = new_mem;
                    0
                }
                true => 1,
            }
        }
        _ => 1,
    }
}

/// Deallocates a memory block.
/// Parameters:
///     `NSTDByte **ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - Number of bytes to deallocate.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_alloc_deallocate(ptr: *mut *mut u8, size: usize) -> c_int {
    match Layout::array::<u8>(size) {
        Ok(layout) => {
            std::alloc::dealloc(*ptr, layout);
            *ptr = std::ptr::null_mut();
            0
        }
        _ => 1,
    }
}

/// Creates a new heap allocated object.
/// Parameters:
///     `const NSTDPointer *const ptr` - Pointer to an object to be copied to the heap.
/// Returns: `NSTDHeap obj` - The new heap allocated object.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_alloc_heap_new(ptr: &NSTDPointer) -> NSTDHeap {
    let ptr_slice = core::slice::from_raw_parts(ptr.ptr as *const u8, ptr.size);
    let alloc = nstd_std_alloc_allocate(ptr.size);
    if !alloc.is_null() {
        let alloc_slice = core::slice::from_raw_parts_mut(alloc, ptr.size);
        alloc_slice.copy_from_slice(ptr_slice);
    }
    NSTDHeap {
        ptr: nstd_core::pointer::nstd_core_pointer_new(alloc as *mut c_void, ptr.size),
    }
}

/// Frees a heap allocated object.
/// Parameters:
///     `NSTDHeap *const obj` - The heap allocated object.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_alloc_heap_free(obj: &mut NSTDHeap) -> c_int {
    let ptr = addr_of_mut!(obj.ptr.ptr) as *mut *mut u8;
    nstd_std_alloc_deallocate(ptr, obj.ptr.size)
}
