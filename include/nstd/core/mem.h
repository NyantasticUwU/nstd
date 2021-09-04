#ifndef NSTD_CORE_MEM_H_INCLUDED
#define NSTD_CORE_MEM_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Allocates a block of memory with `size` bytes.
/// Parameters:
///     `const NSTDCSize size` - Size in bytes of memory to allocate.
/// Returns: `void *ptr` - Pointer to the newly allocated memory.
NSTDAPI void *nstd_core_mem_allocate(const NSTDCSize size);

/// Reallocates a block of memory with `size` bytes.
/// Parameters:
///     `const void **const ptr` - Pointer to the memory to be allocated.
///     `const NSTDCSize size` - Size in bytes of newly allocated memory.
/// Returns: `int success` - Nonzero if reallocation succeeds.
NSTDAPI int nstd_core_mem_reallocate(const void **const ptr, const NSTDCSize size);

/// Frees a block of memory. Will set `*ptr` to NULL.
/// Parameters:
///     `const void **const ptr` - Pointer to the pointer to memory to free.
NSTDAPI void nstd_core_mem_deallocate(const void **const ptr);

/// Copies bytes from `other` to `copycat`.
/// Parameters:
///     `void *const copycat` - Pointer to memory to be copied to.
///     `const void *const other` - Pointer to memory to be copied from.
///     `const NSTDCSize size` - Number of bytes to copy.
NSTDAPI void nstd_core_mem_copy(void *const copycat, const void *const other, const NSTDCSize size);

/// Moves bytes from `from` to `to`. Zeroes out `from`'s memory.
/// Parameters:
///     `void *const from` - Memory to be moved from.
///     `void *const to` - Memory to be moved to.
///     `const NSTDCSize size` - Number of bytes to move.
NSTDAPI void nstd_core_mem_move(void *const from, void *const to, const NSTDCSize size);

/// Moves memory from `*ptr1` to `*ptr2` and vice versa.
/// Parameters:
///     `const void **const ptr1` - Pointer to first pointer's memory location.
///     `const void **const ptr2` - Pointer to second pointer's memory location.
NSTDAPI void nstd_core_mem_switch(const void **const ptr1, const void **const ptr2);

/// Fills a block of memory with `byte`.
/// Parameters:
///     `void *const ptr` - Pointer to block of memory.
///     `const NSTDCSize size` - Size of block.
///     `const NSTDCByte byte` - Byte to fill with.
NSTDAPI void nstd_core_mem_fill(void *const ptr, const NSTDCSize size, const NSTDCByte byte);

/// Zeros a memory range pointed to by `ptr`.
/// Parameters:
///     `void *const ptr` - Pointer to memory to be zeroed.
///     `NSTDCSize start` - Starting index of memory to be zeroed.
///     `const NSTDCSize end` - Ending index of memory to be zeroed. (Excluded).
NSTDAPI void nstd_core_mem_zero(void *const ptr, NSTDCSize start, const NSTDCSize end);

#ifdef __cplusplus
}
#endif
#endif
