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
///     `void *ptr` - Pointer to the pointer to memory to free.
void nstd_deallocate(void **ptr);

#ifdef __cplusplus
}
#endif
#endif
