#ifndef NSTD_CORE_MEM_H_INCLUDED
#define NSTD_CORE_MEM_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Copies bytes from `other` to `copycat`.
/// Parameters:
///     `NSTDByte *const copycat` - Pointer to memory to be copied to.
///     `const NSTDByte *const other` - Pointer to memory to be copied from.
///     `const NSTDUSize size` - Number of bytes to copy.
NSTDAPI void nstd_core_mem_copy(
    NSTDByte *const copycat,
    const NSTDByte *const other,
    const NSTDUSize size);

/// Moves bytes from `from` to `to`. Zeroes out `from`'s memory.
/// Parameters:
///     `NSTDByte *const from` - Memory to be moved from.
///     `NSTDByte *const to` - Memory to be moved to.
///     `const NSTDUSize size` - Number of bytes to move.
NSTDAPI void nstd_core_mem_move(
    NSTDByte *const from,
    NSTDByte *const to,
    const NSTDUSize size);

/// Moves memory from `*ptr1` to `*ptr2` and vice versa.
/// Parameters:
///     `const void **const ptr1` - Pointer to first pointer's memory location.
///     `const void **const ptr2` - Pointer to second pointer's memory location.
NSTDAPI void nstd_core_mem_switch(const void **const ptr1, const void **const ptr2);

/// Fills a block of memory with `byte`.
/// Parameters:
///     `NSTDByte *const ptr` - Pointer to block of memory.
///     `const NSTDUSize size` - Size of block.
///     `const NSTDByte byte` - Byte to fill with.
NSTDAPI void nstd_core_mem_fill(
    NSTDByte *const ptr,
    const NSTDUSize size,
    const NSTDByte byte);

/// Zeros a memory range pointed to by `ptr`.
/// Parameters:
///     `NSTDByte *const ptr` - Pointer to memory to be zeroed.
///     `NSTDUSize start` - Starting index of memory to be zeroed.
///     `const NSTDUSize end` - Ending index of memory to be zeroed. (Excluded).
NSTDAPI void nstd_core_mem_zero(
    NSTDByte *const ptr,
    NSTDUSize start, const
    NSTDUSize end);

#ifdef __cplusplus
}
#endif
#endif
