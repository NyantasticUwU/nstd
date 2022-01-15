#![cfg(target_os = "macos")]
use core_foundation::base::{CFAllocatorAllocate, CFAllocatorDeallocate, CFAllocatorReallocate};
use std::os::raw::{c_int, c_void};

/// MacOS implementation of allocating memory on the heap.
#[inline]
pub unsafe fn allocate(size: usize) -> *mut u8 {
    CFAllocatorAllocate(std::ptr::null_mut(), size as isize, 0).cast()
}

/// MacOS implementation of allocating zeroed memory on the heap.
pub unsafe fn allocate_zeroed(size: usize) -> *mut u8 {
    let ptr = allocate(size);
    if !ptr.is_null() {
        for i in 0..size {
            *ptr.add(i) = 0;
        }
    }
    ptr
}

/// MacOS implementation of reallocating memory on the heap.
pub unsafe fn reallocate(ptr: *mut *mut u8, _: usize, new_size: usize) -> c_int {
    let new_mem = CFAllocatorReallocate(std::ptr::null_mut(), (*ptr).cast(), new_size as isize, 0);
    match new_mem.is_null() {
        false => {
            *ptr = new_mem.cast();
            0
        }
        true => 1,
    }
}

/// MacOS implementation of deallocating memory on the heap.
#[inline]
pub unsafe fn deallocate(ptr: *mut *mut u8, _: usize) -> c_int {
    let hptr = *ptr as *mut c_void;
    *ptr = std::ptr::null_mut();
    CFAllocatorDeallocate(std::ptr::null_mut(), hptr);
    0
}
