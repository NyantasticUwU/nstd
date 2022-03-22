//! Low level memory allocation for Linux.
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
pub unsafe extern "C" fn nstd_os_linux_alloc_allocate(size: usize) -> NSTDAny {
    libc::malloc(size)
}

/// Allocates a zero-initialized block of memory on the heap.
///
/// # Parameters
///
/// - `const NSTDUSize num` - The number of objects to allocate.
///
/// - `const NSTDUSize size` - The size of each object to allocate.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the heap block of memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_linux_alloc_allocate_zeroed(num: usize, size: usize) -> NSTDAny {
    libc::calloc(num, size)
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
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_linux_alloc_reallocate(
    ptr: &mut NSTDAny,
    new_size: usize,
) -> NSTDErrorCode {
    let rptr = libc::realloc(*ptr, new_size);
    if !rptr.is_null() {
        *ptr = rptr;
        return 0;
    }
    1
}

/// Deallocates a block of memory.
///
/// # Parameters
///
/// - `NSTDAny *const ptr` - Pointer to the block of memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_linux_alloc_deallocate(ptr: &mut NSTDAny) {
    libc::free(*ptr);
    *ptr = std::ptr::null_mut();
}
