use std::{alloc::Layout, os::raw::c_int, ptr};

/// Allocates a new memory block.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDByte *ptr` - The new memory block.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_alloc_allocate(size: usize) -> *mut u8 {
    match Layout::array::<u8>(size) {
        Ok(layout) => std::alloc::alloc(layout),
        _ => ptr::null_mut(),
    }
}

/// Allocates a new memory block with all bytes set to 0.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDByte *ptr` - The new memory block.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_alloc_allocate_zeroed(size: usize) -> *mut u8 {
    match Layout::array::<u8>(size) {
        Ok(layout) => std::alloc::alloc_zeroed(layout),
        _ => ptr::null_mut(),
    }
}

/// Reallocates a memory block.
/// Parameters:
///     `NSTDByte **ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - The current size of the memory block.
///     `const NSTDUSize new_size` - The new size of the memory block.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[no_mangle]
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
#[no_mangle]
pub unsafe extern "C" fn nstd_std_alloc_deallocate(ptr: *mut *mut u8, size: usize) -> c_int {
    match Layout::array::<u8>(size) {
        Ok(layout) => {
            std::alloc::dealloc(*ptr, layout);
            *ptr = ptr::null_mut();
            0
        }
        _ => 1,
    }
}
