pub mod heap;
use self::heap::NSTDOSWindowsHeapHandle;
use crate::core::def::{NSTDAny, NSTDErrorCode};
use windows::Win32::System::Memory::GetProcessHeap;

/// Returns a handle to this process's heap.
/// Returns: `NSTDOSWindowsHeapHandle heap` - A handle to this process's heap.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_get_process_heap() -> NSTDOSWindowsHeapHandle {
    GetProcessHeap().0
}

/// Allocates a block of memory on the heap.
/// Parameters:
///     `const NSTDUSize size` - The number of bytes to allocate.
/// Returns: `NSTDAny ptr` - A pointer to the heap block of memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_allocate(size: usize) -> NSTDAny {
    self::heap::nstd_os_windows_alloc_heap_allocate(nstd_os_windows_alloc_get_process_heap(), size)
}

/// Allocates a zero-initialized block of memory on the heap.
/// Parameters:
///     `const NSTDUSize size` - The number of bytes to allocate.
/// Returns: `NSTDAny ptr` - A pointer to the heap block of memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_allocate_zeroed(size: usize) -> NSTDAny {
    self::heap::nstd_os_windows_alloc_heap_allocate_zeroed(
        nstd_os_windows_alloc_get_process_heap(),
        size,
    )
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
    self::heap::nstd_os_windows_alloc_heap_reallocate(
        nstd_os_windows_alloc_get_process_heap(),
        ptr,
        new_size,
    )
}

/// Deallocates a block of memory.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the block of memory.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_deallocate(ptr: &mut NSTDAny) -> NSTDErrorCode {
    self::heap::nstd_os_windows_alloc_heap_deallocate(nstd_os_windows_alloc_get_process_heap(), ptr)
}
