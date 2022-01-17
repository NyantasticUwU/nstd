#![cfg(target_os = "linux")]
use super::PlatformImpl;
use std::os::raw::{c_int, c_void};

/// Windows platform allocation.
pub struct PlatformAlloc;
impl PlatformImpl for PlatformAlloc {
    /// Linux implementation of allocating memory on the heap.
    #[inline]
    unsafe fn allocate(size: usize) -> *mut u8 {
        libc::malloc(size).cast()
    }

    /// Linux implementation of allocating zeroed memory on the heap.
    #[inline]
    unsafe fn allocate_zeroed(size: usize) -> *mut u8 {
        const BYTE_SIZE: usize = std::mem::size_of::<u8>();
        libc::calloc(size, BYTE_SIZE).cast()
    }

    /// Linux implementation of reallocating memory on the heap.
    unsafe fn reallocate(ptr: *mut *mut u8, _: usize, new_size: usize) -> c_int {
        let new_mem = libc::realloc((*ptr).cast(), new_size);
        match new_mem.is_null() {
            false => {
                *ptr = new_mem.cast();
                0
            }
            true => 1,
        }
    }

    /// Linux implementation of deallocating memory on the heap.
    #[inline]
    unsafe fn deallocate(ptr: *mut *mut u8, _: usize) -> c_int {
        let hptr = *ptr as *mut c_void;
        *ptr = std::ptr::null_mut();
        libc::free(hptr);
        0
    }
}
