pub mod heap;
use crate::core::def::NSTDAny;
use std::alloc::Layout;

/// Allocates a new memory block.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDAny ptr` - The new memory block.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocate(size: usize) -> NSTDAny {
    match Layout::array::<u8>(size) {
        Ok(layout) => std::alloc::alloc(layout).cast(),
        _ => std::ptr::null_mut(),
    }
}

/// Allocates a new memory block with all bytes set to 0.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDAny ptr` - The new memory block.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocate_zeroed(size: usize) -> NSTDAny {
    match Layout::array::<u8>(size) {
        Ok(layout) => std::alloc::alloc_zeroed(layout).cast(),
        _ => std::ptr::null_mut(),
    }
}

/// Reallocates a memory block.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - The current size of the memory block.
///     `const NSTDUSize new_size` - The new size of the memory block.
/// Returns: `NSTDInt32 errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_reallocate(
    ptr: *mut NSTDAny,
    size: usize,
    new_size: usize,
) -> i32 {
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

/// Deallocates a memory block.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - Number of bytes to deallocate.
/// Returns: `NSTDInt32 errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_deallocate(ptr: *mut NSTDAny, size: usize) -> i32 {
    match Layout::array::<u8>(size) {
        Ok(layout) => {
            std::alloc::dealloc((*ptr).cast(), layout);
            *ptr = std::ptr::null_mut();
            0
        }
        _ => 1,
    }
}
