//! Low level memory allocation for Windows.
pub mod heap;
use self::heap::*;
use crate::core::def::{NSTDAny, NSTDErrorCode};

/// Allocates a block of memory on the heap.
///
/// # Parameters
///
/// - `const NSTDUSize size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the heap block of memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_allocate(size: usize) -> NSTDAny {
    nstd_os_windows_alloc_heap_allocate(nstd_os_windows_alloc_heap_default(), size)
}

/// Allocates a zero-initialized block of memory on the heap.
///
/// # Parameters
///
/// - `const NSTDUSize size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the heap block of memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_allocate_zeroed(size: usize) -> NSTDAny {
    nstd_os_windows_alloc_heap_allocate_zeroed(nstd_os_windows_alloc_heap_default(), size)
}

/// Reallocates a memory block with a new size.
///
/// # Parameters
///
/// - `NSTDAny *const ptr` - Pointer to the pointer to the memory block to reallocate.
///
/// - `const NSTDUSize new_size` - The number of bytes the new memory block will have.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_reallocate(
    ptr: &mut NSTDAny,
    new_size: usize,
) -> NSTDErrorCode {
    nstd_os_windows_alloc_heap_reallocate(nstd_os_windows_alloc_heap_default(), ptr, new_size)
}

/// Deallocates a block of memory.
///
/// # Parameters
///
/// - `NSTDAny *const ptr` - Pointer to the block of memory.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_deallocate(ptr: &mut NSTDAny) -> NSTDErrorCode {
    nstd_os_windows_alloc_heap_deallocate(nstd_os_windows_alloc_heap_default(), ptr)
}
