pub mod heap;
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
mod platform;
use nstd_core::def::NSTDAny;
use platform::*;
use std::os::raw::c_int;

/// Allocates a new memory block.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDAny ptr` - The new memory block.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocate(size: usize) -> NSTDAny {
    PlatformAlloc::allocate(size)
}

/// Allocates a new memory block with all bytes set to 0.
/// Parameters:
///     `const NSTDUSize size` - Number of bytes to allocate.
/// Returns: `NSTDAny ptr` - The new memory block.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocate_zeroed(size: usize) -> NSTDAny {
    PlatformAlloc::allocate_zeroed(size)
}

/// Reallocates a memory block.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - The current size of the memory block.
///     `const NSTDUSize new_size` - The new size of the memory block.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_reallocate(
    ptr: *mut NSTDAny,
    size: usize,
    new_size: usize,
) -> c_int {
    PlatformAlloc::reallocate(ptr, size, new_size)
}

/// Deallocates a memory block.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the memory block.
///     `const NSTDUSize size` - Number of bytes to deallocate.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_deallocate(ptr: *mut NSTDAny, size: usize) -> c_int {
    PlatformAlloc::deallocate(ptr, size)
}
