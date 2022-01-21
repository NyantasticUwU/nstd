#![cfg(target_os = "macos")]
use super::PlatformImpl;
use core_foundation::base::{CFAllocatorAllocate, CFAllocatorDeallocate, CFAllocatorReallocate};
use nstd_core::def::NSTDAny;

/// Windows platform allocation.
pub struct PlatformAlloc;
impl PlatformImpl for PlatformAlloc {
    /// MacOS implementation of allocating memory on the heap.
    #[inline]
    unsafe fn allocate(size: usize) -> NSTDAny {
        CFAllocatorAllocate(std::ptr::null_mut(), size as isize, 0)
    }

    /// MacOS implementation of allocating zeroed memory on the heap.
    unsafe fn allocate_zeroed(size: usize) -> NSTDAny {
        let ptr = Self::allocate(size);
        if !ptr.is_null() {
            let ptr = ptr as *mut u8;
            for i in 0..size {
                *ptr.add(i) = 0;
            }
        }
        ptr
    }

    /// MacOS implementation of reallocating memory on the heap.
    unsafe fn reallocate(ptr: *mut NSTDAny, _: usize, new_size: usize) -> i32 {
        let new_mem = CFAllocatorReallocate(std::ptr::null_mut(), *ptr, new_size as isize, 0);
        match new_mem.is_null() {
            false => {
                *ptr = new_mem;
                0
            }
            true => 1,
        }
    }

    /// MacOS implementation of deallocating memory on the heap.
    #[inline]
    unsafe fn deallocate(ptr: *mut NSTDAny, _: usize) -> i32 {
        CFAllocatorDeallocate(std::ptr::null_mut(), *ptr);
        *ptr = std::ptr::null_mut();
        0
    }
}
