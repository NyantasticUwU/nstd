#ifndef NSTD_MEM_H_INCLUDED
#define NSTD_MEM_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Allocates a block of memory with `size` bytes.
/// Parameters:
///     `const NSTDSize size` - Size in bytes of memory to allocate.
/// Returns: `void *ptr` - Pointer to the newly allocated memory.
void *nstd_mem_allocate(const NSTDSize size);

/// Reallocates a block of memory with `size` bytes.
/// Parameters:
///     `const void **const ptr` - Pointer to the memory to be allocated.
///     `const NSTDSize size` - Size in bytes of newly allocated memory.
void nstd_mem_reallocate(const void **const ptr, const NSTDSize size);

/// Frees a block of memory. Will set `*ptr` to NULL.
/// Parameters:
///     `const void **const ptr` - Pointer to the pointer to memory to free.
void nstd_mem_deallocate(const void **const ptr);

/// Copies bytes from `other` to `copycat`.
/// Parameters:
///     `void *const copycat` - Pointer to memory to be copied to.
///     `const void *const other` - Pointer to memory to be copied from.
///     `const NSTDSize size` - Number of bytes to copy.
void nstd_mem_copy(void *const copycat, const void *const other, const NSTDSize size);

/// Moves bytes from `from` to `to`. Zeroes out `from`'s memory.
/// Parameters:
///     `void *const from` - Memory to be moved from.
///     `void *const to` - Memory to be moved to.
///     `const NSTDSize size` - Number of bytes to move.
void nstd_mem_move(void *const from, void *const to, const NSTDSize size);

/// Moves memory from `*ptr1` to `*ptr2` and vice versa.
/// Parameters:
///     `const void **const ptr1` - Pointer to first pointer's memory location.
///     `const void **const ptr2` - Pointer to second pointer's memory location.
void nstd_mem_switch(const void **const ptr1, const void **const ptr2);

/// Zeros a memory range pointed to by `ptr`.
/// Parameters:
///     `void *const ptr` - Pointer to memory to be zeroed.
///     `NSTDSize start` - Starting index of memory to be zeroed.
///     `const NSTDSize end` - Ending index of memory to be zeroed. (Excluded).
void nstd_mem_zero(void *const ptr, NSTDSize start, const NSTDSize end);

#ifdef __cplusplus
}
#endif
#endif
