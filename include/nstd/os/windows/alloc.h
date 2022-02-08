#ifndef NSTD_OS_WINDOWS_ALLOC_H_INCLUDED
#define NSTD_OS_WINDOWS_ALLOC_H_INCLUDED
#include "../../core/def.h"
#include "../../nstd.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a handle to a heap.
typedef NSTDISize NSTDOSWindowsHeapHandle;

/// Returns a handle to this process's heap.
/// Returns: `NSTDOSWindowsHeapHandle heap` - A handle to this process's heap.
NSTDAPI NSTDOSWindowsHeapHandle nstd_os_windows_alloc_get_process_heap();

/// Allocates a block of memory on the heap.
/// Parameters:
///     `const NSTDUSize size` - The number of bytes to allocate.
/// Returns: `NSTDAny ptr` - A pointer to the heap block of memory.
NSTDAPI NSTDAny nstd_os_windows_alloc_allocate(const NSTDUSize size);

/// Allocates a zero-initialized block of memory on the heap.
/// Parameters:
///     `const NSTDUSize size` - The number of bytes to allocate.
/// Returns: `NSTDAny ptr` - A pointer to the heap block of memory.
NSTDAPI NSTDAny nstd_os_windows_alloc_allocate_zeroed(const NSTDUSize size);

/// Reallocates a memory block with a new size.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the pointer to the memory block to reallocate.
///     `const NSTDUSize new_size` - The number of bytes the new memory block will have.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_alloc_reallocate(
    NSTDAny *const ptr,
    const NSTDUSize new_size);

/// Deallocates a block of memory.
/// Parameters:
///     `NSTDAny *const ptr` - Pointer to the block of memory.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_alloc_deallocate(NSTDAny *const ptr);

/// Creates a new private heap for this process.
/// Returns: `NSTDOSWindowsHeapHandle heap` - A handle to the new heap.
NSTDAPI NSTDOSWindowsHeapHandle nstd_os_windows_alloc_heap_new();

/// Allocates a block of memory on the specified heap.
/// Parameters:
///     `const NSTDOSWindowsHeapHandle heap` - The heap to allocate memory on.
///     `const NSTDUSize size` - The number of bytes to allocate.
/// Returns: `NSTDAny ptr` - A pointer to the heap block of memory.
NSTDAPI NSTDAny nstd_os_windows_alloc_heap_allocate(
    const NSTDOSWindowsHeapHandle heap,
    const NSTDUSize size);

/// Allocates a zero-initialized block of memory on the specified heap.
/// Parameters:
///     `const NSTDOSWindowsHeapHandle heap` - The heap to allocate memory on.
///     `const NSTDUSize size` - The number of bytes to allocate.
/// Returns: `NSTDAny ptr` - A pointer to the heap block of memory.
NSTDAPI NSTDAny nstd_os_windows_alloc_heap_allocate_zeroed(
    const NSTDOSWindowsHeapHandle heap,
    const NSTDUSize size);

/// Reallocates a memory block with a new size.
/// Parameters:
///     `const NSTDOSWindowsHeapHandle heap` - The heap to allocate memory on.
///     `NSTDAny *const ptr` - Pointer to the pointer to the memory block to reallocate.
///     `const NSTDUSize new_size` - The number of bytes the new memory block will have.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_alloc_heap_reallocate(
    const NSTDOSWindowsHeapHandle heap,
    NSTDAny *const ptr,
    const NSTDUSize new_size);

/// Deallocates a block of memory.
/// Parameters:
///     `const NSTDOSWindowsHeapHandle heap` - The heap to allocate memory on.
///     `NSTDAny *const ptr` - Pointer to the block of memory.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_alloc_heap_deallocate(
    const NSTDOSWindowsHeapHandle heap,
    NSTDAny *const ptr);

/// Destroys a heap created by `nstd_os_windows_alloc_heap_new`.
/// Parameters:
///     `NSTDOSWindowsHeapHandle *const heap` - A pointer to a heap handle.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_os_windows_alloc_heap_free(NSTDOSWindowsHeapHandle *const heap);

#ifdef __cplusplus
}
#endif
#endif
