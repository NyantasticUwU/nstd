mod platform;
use nstd_core::pointer::NSTDPointer;
use std::{
    os::raw::{c_int, c_void},
    ptr::addr_of_mut,
};
#[cfg(feature = "deps")]
pub mod deps {
    #[cfg(target_os = "macos")]
    pub use core_foundation;
    #[cfg(target_os = "linux")]
    pub use libc;
    pub use nstd_core;
    #[cfg(target_os = "windows")]
    pub use windows;
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
    platform::allocate(size)
}

/// Allocates a new memory block with all bytes set to 0.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDByte *ptr` - The new memory block.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_alloc_allocate_zeroed(size: usize) -> *mut u8 {
    platform::allocate_zeroed(size)
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
    platform::reallocate(ptr, size, new_size)
}

/// Deallocates a memory block.
/// Parameters:
///     `NSTDByte **ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - Number of bytes to deallocate.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_alloc_deallocate(ptr: *mut *mut u8, size: usize) -> c_int {
    platform::deallocate(ptr, size)
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
