use crate::core::def::{NSTDAny, NSTDErrorCode};
use windows::Win32::System::Memory::{
    GetProcessHeap, HeapAlloc, HeapFree, HeapReAlloc, HEAP_FLAGS, HEAP_ZERO_MEMORY,
};

/// Allocates a block of memory on the heap.
/// Parameters:
///     `const NSTDUSize size` - The number of bytes to allocate.
/// Returns: `NSTDAny ptr` - A pointer to the heap block of memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_allocate(size: usize) -> NSTDAny {
    HeapAlloc(GetProcessHeap(), HEAP_FLAGS::default(), size)
}

/// Allocates a zero-initialized block of memory on the heap.
/// Parameters:
///     `const NSTDUSize size` - The number of bytes to allocate.
/// Returns: `NSTDAny ptr` - A pointer to the heap block of memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_allocate_zeroed(size: usize) -> NSTDAny {
    HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, size)
}

/// Reallocates a memory block with a new size.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the pointer to the memory block to reallocate.
///     `const NSTDUSize new_size` - The number of bytes the new memory block will have.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_reallocate(
    ptr: &mut NSTDAny,
    new_size: usize,
) -> NSTDErrorCode {
    let rptr = HeapReAlloc(GetProcessHeap(), HEAP_FLAGS::default(), *ptr, new_size);
    if !rptr.is_null() {
        *ptr = rptr;
        return 0;
    }
    1
}

/// Deallocates a block of memory.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the block of memory.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_deallocate(ptr: &mut NSTDAny) -> NSTDErrorCode {
    let hptr = *ptr;
    *ptr = std::ptr::null_mut();
    (HeapFree(GetProcessHeap(), HEAP_FLAGS::default(), hptr).0 == 0) as NSTDErrorCode
}
