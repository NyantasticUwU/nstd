#![cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
use super::PlatformImpl;
use std::{alloc::Layout, os::raw::c_int};

/// Cross platform allocation.
pub struct PlatformAlloc;
impl PlatformImpl for PlatformAlloc {
    /// Cross platform implementation of allocating memory on the heap.
    #[inline]
    unsafe fn allocate(size: usize) -> *mut u8 {
        match Layout::array::<u8>(size) {
            Ok(layout) => std::alloc::alloc(layout),
            _ => std::ptr::null_mut(),
        }
    }

    /// Cross platform implementation of allocating zeroed memory on the heap.
    #[inline]
    unsafe fn allocate_zeroed(size: usize) -> *mut u8 {
        match Layout::array::<u8>(size) {
            Ok(layout) => std::alloc::alloc_zeroed(layout),
            _ => std::ptr::null_mut(),
        }
    }

    /// Cross platform implementation of reallocating memory on the heap.
    unsafe fn reallocate(ptr: *mut *mut u8, size: usize, new_size: usize) -> c_int {
        let new_mem = match Layout::array::<u8>(size) {
            Ok(layout) => std::alloc::realloc(*ptr, layout, new_size),
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
    unsafe fn deallocate(ptr: *mut *mut u8, size: usize) -> c_int {
        match Layout::array::<u8>(size) {
            Ok(layout) => {
                std::alloc::dealloc(*ptr, layout);
                *ptr = std::ptr::null_mut();
                0
            }
            _ => 1,
        }
    }
}
