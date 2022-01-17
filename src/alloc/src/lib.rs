pub mod heap;
mod platform;
use std::os::raw::c_int;
#[cfg(feature = "deps")]
pub mod deps {
    #[cfg(target_os = "macos")]
    pub use core_foundation;
    #[cfg(target_os = "linux")]
    pub use libc;
    pub use nstd_core;
    #[cfg(target_os = "windows")]
    pub use windows;
}

/// Allocates a new memory block.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDByte *ptr` - The new memory block.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocate(size: usize) -> *mut u8 {
    platform::allocate(size)
}

/// Allocates a new memory block with all bytes set to 0.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDByte *ptr` - The new memory block.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocate_zeroed(size: usize) -> *mut u8 {
    platform::allocate_zeroed(size)
}

/// Reallocates a memory block.
/// Parameters:
///     `NSTDByte **ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - The current size of the memory block.
///     `const NSTDUSize new_size` - The new size of the memory block.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_reallocate(
    ptr: *mut *mut u8,
    size: usize,
    new_size: usize,
) -> c_int {
    platform::reallocate(ptr, size, new_size)
}

/// Deallocates a memory block.
/// Parameters:
///     `NSTDByte **ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - Number of bytes to deallocate.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_deallocate(ptr: *mut *mut u8, size: usize) -> c_int {
    platform::deallocate(ptr, size)
}
