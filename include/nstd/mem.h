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
void *nstd_allocate(const NSTDSize size);

/// Frees a block of memory. Will set `*ptr` to NULL.
/// Parameters:
///     `const void **const ptr` - Pointer to the pointer to memory to free.
void nstd_deallocate(const void **const ptr);

/// Copies bytes from `*other` to `*copycat`.
/// Parameters:
///     `void *const copycat` - Pointer to memory to be copied to.
///     `const void *const other` - Pointer to memory to be copied from.
///     `const NSTDSize size` - Number of bytes to copy.
void nstd_memCopy(void *const copycat, const void *const other, const NSTDSize size);

#ifdef __cplusplus
}
#endif
#endif
