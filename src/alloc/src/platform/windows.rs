#![cfg(target_os = "windows")]
use super::PlatformImpl;
use std::os::raw::{c_int, c_void};
use windows::Win32::System::Memory::{
    GetProcessHeap, HeapAlloc, HeapFree, HeapReAlloc, HEAP_FLAGS, HEAP_ZERO_MEMORY,
};

/// Windows platform allocation.
pub struct PlatformAlloc;
impl PlatformImpl for PlatformAlloc {
    /// Windows implementation of allocating memory on the heap.
    #[inline]
    unsafe fn allocate(size: usize) -> *mut u8 {
        HeapAlloc(GetProcessHeap(), HEAP_FLAGS::default(), size).cast()
    }

    /// Windows implementation of allocating zeroed memory on the heap.
    #[inline]
    unsafe fn allocate_zeroed(size: usize) -> *mut u8 {
        HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, size).cast()
    }

    /// Windows implementation of reallocating memory on the heap.
    unsafe fn reallocate(ptr: *mut *mut u8, _: usize, new_size: usize) -> c_int {
        let new_mem = HeapReAlloc(
            GetProcessHeap(),
            HEAP_FLAGS::default(),
            (*ptr).cast(),
            new_size,
        );
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
    unsafe fn deallocate(ptr: *mut *mut u8, _: usize) -> c_int {
        let hptr = *ptr as *const c_void;
        *ptr = std::ptr::null_mut();
        (HeapFree(GetProcessHeap(), HEAP_FLAGS::default(), hptr).0 == 0) as c_int
    }
}
