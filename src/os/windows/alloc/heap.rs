use crate::core::def::{NSTDAny, NSTDErrorCode};
use windows::Win32::System::Memory::{
    HeapAlloc, HeapCreate, HeapDestroy, HeapFree, HeapHandle, HeapReAlloc, HEAP_FLAGS,
    HEAP_ZERO_MEMORY,
};

/// Represents a handle to a heap.
pub type NSTDOSWindowsHeapHandle = isize;

/// Creates a new private heap for this process.
/// Returns: `NSTDOSWindowsHeapHandle heap` - A handle to the new heap.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_new() -> NSTDOSWindowsHeapHandle {
    HeapCreate(HEAP_FLAGS::default(), 0, 0).0
}

/// Allocates a block of memory on the specified heap.
/// Parameters:
///     `const NSTDOSWindowsHeapHandle heap` - The heap to allocate memory on.
///     `const NSTDUSize size` - The number of bytes to allocate.
/// Returns: `NSTDAny ptr` - A pointer to the heap block of memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_allocate(
    heap: NSTDOSWindowsHeapHandle,
    size: usize,
) -> NSTDAny {
    HeapAlloc(HeapHandle(heap), HEAP_FLAGS::default(), size)
}

/// Allocates a zero-initialized block of memory on the specified heap.
/// Parameters:
///     `const NSTDOSWindowsHeapHandle heap` - The heap to allocate memory on.
///     `const NSTDUSize size` - The number of bytes to allocate.
/// Returns: `NSTDAny ptr` - A pointer to the heap block of memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_allocate_zeroed(
    heap: NSTDOSWindowsHeapHandle,
    size: usize,
) -> NSTDAny {
    HeapAlloc(HeapHandle(heap), HEAP_ZERO_MEMORY, size)
}

/// Reallocates a memory block with a new size.
/// Parameters:
///     `const NSTDOSWindowsHeapHandle heap` - The heap to allocate memory on.
///     `NSTDAny *const ptr` - Pointer to the pointer to the memory block to reallocate.
///     `const NSTDUSize new_size` - The number of bytes the new memory block will have.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_reallocate(
    heap: NSTDOSWindowsHeapHandle,
    ptr: &mut NSTDAny,
    new_size: usize,
) -> NSTDErrorCode {
    let rptr = HeapReAlloc(HeapHandle(heap), HEAP_FLAGS::default(), *ptr, new_size);
    if !rptr.is_null() {
        *ptr = rptr;
        return 0;
    }
    1
}

/// Deallocates a block of memory.
/// Parameters:
///     `const NSTDOSWindowsHeapHandle heap` - The heap to allocate memory on.
///     `NSTDAny *const ptr` - Pointer to the block of memory.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_deallocate(
    heap: NSTDOSWindowsHeapHandle,
    ptr: &mut NSTDAny,
) -> NSTDErrorCode {
    let hptr = *ptr;
    *ptr = std::ptr::null_mut();
    (HeapFree(HeapHandle(heap), HEAP_FLAGS::default(), hptr).0 == 0) as NSTDErrorCode
}

/// Destroys a heap created by `nstd_os_windows_alloc_heap_new`.
/// Parameters:
///     `NSTDOSWindowsHeapHandle *const heap` - A pointer to a heap handle.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_free(
    heap: &mut NSTDOSWindowsHeapHandle,
) -> NSTDErrorCode {
    let heap_handle = HeapHandle(*heap);
    *heap = 0;
    (HeapDestroy(heap_handle).0 == 0) as NSTDErrorCode
}