#![cfg(target_os = "windows")]
use super::PlatformImpl;
use crate::core::def::NSTDAny;
use windows::Win32::System::Memory::{
    GetProcessHeap, HeapAlloc, HeapFree, HeapReAlloc, HEAP_FLAGS, HEAP_ZERO_MEMORY,
};

/// Windows platform allocation.
pub struct PlatformAlloc;
impl PlatformImpl for PlatformAlloc {
    /// Windows implementation of allocating memory on the heap.
    #[inline]
    unsafe fn allocate(size: usize) -> NSTDAny {
        HeapAlloc(GetProcessHeap(), HEAP_FLAGS::default(), size)
    }

    /// Windows implementation of allocating zeroed memory on the heap.
    #[inline]
    unsafe fn allocate_zeroed(size: usize) -> NSTDAny {
        HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, size)
    }

    /// Windows implementation of reallocating memory on the heap.
    unsafe fn reallocate(ptr: *mut NSTDAny, _: usize, new_size: usize) -> i32 {
        let new_mem = HeapReAlloc(GetProcessHeap(), HEAP_FLAGS::default(), *ptr, new_size);
        match new_mem.is_null() {
            false => {
                *ptr = new_mem.cast();
                0
            }
            true => 1,
        }
    }

    /// Windows implementation of deallocating memory on the heap.
    #[inline]
    unsafe fn deallocate(ptr: *mut NSTDAny, _: usize) -> i32 {
        let hptr = *ptr;
        *ptr = std::ptr::null_mut();
        (HeapFree(GetProcessHeap(), HEAP_FLAGS::default(), hptr).0 == 0) as i32
    }
}
