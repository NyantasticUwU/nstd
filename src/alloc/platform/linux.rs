#![cfg(target_os = "linux")]
use super::PlatformImpl;
use crate::core::def::NSTDAny;

/// Windows platform allocation.
pub struct PlatformAlloc;
impl PlatformImpl for PlatformAlloc {
    /// Linux implementation of allocating memory on the heap.
    #[inline]
    unsafe fn allocate(size: usize) -> NSTDAny {
        libc::malloc(size)
    }

    /// Linux implementation of allocating zeroed memory on the heap.
    #[inline]
    unsafe fn allocate_zeroed(size: usize) -> NSTDAny {
        const BYTE_SIZE: usize = std::mem::size_of::<u8>();
        libc::calloc(size, BYTE_SIZE)
    }

    /// Linux implementation of reallocating memory on the heap.
    unsafe fn reallocate(ptr: *mut NSTDAny, _: usize, new_size: usize) -> i32 {
        let new_mem = libc::realloc(*ptr, new_size);
        match new_mem.is_null() {
            false => {
                *ptr = new_mem;
                0
            }
            true => 1,
        }
    }

    /// Linux implementation of deallocating memory on the heap.
    #[inline]
    unsafe fn deallocate(ptr: *mut NSTDAny, _: usize) -> i32 {
        libc::free(*ptr);
        *ptr = std::ptr::null_mut();
        0
    }
}
