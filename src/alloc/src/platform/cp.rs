#![cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
use super::PlatformImpl;
use nstd_core::def::NSTDAny;
use std::alloc::Layout;

/// Cross platform allocation.
pub struct PlatformAlloc;
impl PlatformImpl for PlatformAlloc {
    /// Cross platform implementation of allocating memory on the heap.
    #[inline]
    unsafe fn allocate(size: usize) -> NSTDAny {
        match Layout::array::<u8>(size) {
            Ok(layout) => std::alloc::alloc(layout).cast(),
            _ => std::ptr::null_mut(),
        }
    }

    /// Cross platform implementation of allocating zeroed memory on the heap.
    #[inline]
    unsafe fn allocate_zeroed(size: usize) -> NSTDAny {
        match Layout::array::<u8>(size) {
            Ok(layout) => std::alloc::alloc_zeroed(layout).cast(),
            _ => std::ptr::null_mut(),
        }
    }

    /// Cross platform implementation of reallocating memory on the heap.
    unsafe fn reallocate(ptr: *mut NSTDAny, size: usize, new_size: usize) -> i32 {
        let new_mem = match Layout::array::<u8>(size) {
            Ok(layout) => std::alloc::realloc((*ptr).cast(), layout, new_size),
            _ => return 1,
        };
        match new_mem.is_null() {
            false => {
                *ptr = new_mem.cast();
                0
            }
            true => 1,
        }
    }

    /// Cross platform implementation of deallocating memory on the heap.
    unsafe fn deallocate(ptr: *mut NSTDAny, size: usize) -> i32 {
        match Layout::array::<u8>(size) {
            Ok(layout) => {
                std::alloc::dealloc((*ptr).cast(), layout);
                *ptr = std::ptr::null_mut();
                0
            }
            _ => 1,
        }
    }
}
