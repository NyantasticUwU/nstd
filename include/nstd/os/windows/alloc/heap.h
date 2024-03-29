#ifndef NSTD_OS_WINDOWS_ALLOC_HEAP_H_INCLUDED
#define NSTD_OS_WINDOWS_ALLOC_HEAP_H_INCLUDED
#include "../../../core/def.h"
#include "../../../nstd.h"
NSTDCPPSTART

/// Represents a handle to a heap.
typedef NSTDISize NSTDOSWindowsHeapHandle;

/// Returns a handle to this process's default heap.
///
/// # Note
///
/// DO NOT ATTEMPT TO FREE THE VALUE RETURNED FROM THIS FUNCTION.
///
/// # Returns
///
/// `NSTDOSWindowsHeapHandle heap` - A handle to this process's heap.
NSTDAPI NSTDOSWindowsHeapHandle nstd_os_windows_alloc_heap_default();

/// Creates a new private heap for this process.
///
/// # Returns
///
/// `NSTDOSWindowsHeapHandle heap` - A handle to the new heap.
NSTDAPI NSTDOSWindowsHeapHandle nstd_os_windows_alloc_heap_new();

/// Allocates a block of memory on the specified heap.
///
/// # Parameters
///
/// - `const NSTDOSWindowsHeapHandle heap` - The heap to allocate memory on.
///
/// - `const NSTDUSize size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the heap block of memory.
NSTDAPI NSTDAny nstd_os_windows_alloc_heap_allocate(
    const NSTDOSWindowsHeapHandle heap,
    const NSTDUSize size);

/// Allocates a zero-initialized block of memory on the specified heap.
///
/// # Parameters
///
/// - `const NSTDOSWindowsHeapHandle heap` - The heap to allocate memory on.
///
/// - `const NSTDUSize size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the heap block of memory.
NSTDAPI NSTDAny nstd_os_windows_alloc_heap_allocate_zeroed(
    const NSTDOSWindowsHeapHandle heap,
    const NSTDUSize size);

/// Reallocates a memory block with a new size.
///
/// # Parameters
///
/// - `const NSTDOSWindowsHeapHandle heap` - The heap to allocate memory on.
///
/// - `NSTDAny *const ptr` - Pointer to the pointer to the memory block to reallocate.
///
/// - `const NSTDUSize new_size` - The number of bytes the new memory block will have.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_alloc_heap_reallocate(
    const NSTDOSWindowsHeapHandle heap,
    NSTDAny *const ptr,
    const NSTDUSize new_size);

/// Deallocates a block of memory.
///
/// # Parameters
///
/// - `const NSTDOSWindowsHeapHandle heap` - The heap to allocate memory on.
///
/// - `NSTDAny *const ptr` - Pointer to the block of memory.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_alloc_heap_deallocate(
    const NSTDOSWindowsHeapHandle heap,
    NSTDAny *const ptr);

/// Gets the size of a memory block allocated on a heap.
///
/// # Parameters
///
/// - `const NSTDOSWindowsHeapHandle heap` - The heap the memory block was allocated on.
///
/// - `const NSTDAnyConst ptr` - A pointer to the memory block.
///
/// # Returns
///
/// `NSTDUSize size` - The size of the memory block that `ptr` points to.
NSTDAPI NSTDUSize nstd_os_windows_alloc_heap_allocation_size(
    const NSTDOSWindowsHeapHandle heap,
    const NSTDAnyConst ptr);

/// Destroys a heap created by `nstd_os_windows_alloc_heap_new`.
///
/// # Parameters
///
/// - `NSTDOSWindowsHeapHandle *const heap` - A pointer to a heap handle.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_alloc_heap_free(NSTDOSWindowsHeapHandle *const heap);

NSTDCPPEND
#endif
