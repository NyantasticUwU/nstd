#include <nstd/core/def.h>
#include <nstd/core/mem.h>

extern "C"
{
    /// Copies bytes from `other` to `copycat`.
    /// Parameters:
    ///     `void *const copycat` - Pointer to memory to be copied to.
    ///     `const void *const other` - Pointer to memory to be copied from.
    ///     `const NSTDCOREUSize size` - Number of bytes to copy.
    NSTDAPI void nstd_core_mem_copy(
        void *const copycat,
        const void *const other,
        const NSTDCOREUSize size)
    {
        NSTDCOREByte *copier{static_cast<NSTDCOREByte *>(copycat)};
        const NSTDCOREByte *copied{static_cast<const NSTDCOREByte *>(other)};
        for (NSTDCOREUSize i{}; i < size; ++i, ++copier, ++copied)
            *copier = *copied;
    }

    /// Moves bytes from `from` to `to`. Zeroes out `from`'s memory.
    /// Parameters:
    ///     `void *const from` - Memory to be moved from.
    ///     `void *const to` - Memory to be moved to.
    ///     `const NSTDCOREUSize size` - Number of bytes to move.
    NSTDAPI void nstd_core_mem_move(void *const from, void *const to, const NSTDCOREUSize size)
    {
        NSTDCOREByte *frm{static_cast<NSTDCOREByte *>(from)};
        NSTDCOREByte *t{static_cast<NSTDCOREByte *>(to)};
        for (NSTDCOREUSize i{}; i < size; ++i, ++frm, ++t)
        {
            *t = *frm;
            *frm = 0;
        }
    }

    /// Moves memory from `*ptr1` to `*ptr2` and vice versa.
    /// Parameters:
    ///     `const void **const ptr1` - Pointer to first pointer's memory location.
    ///     `const void **const ptr2` - Pointer to second pointer's memory location.
    NSTDAPI inline void nstd_core_mem_switch(const void **const ptr1, const void **const ptr2)
    {
        const void *const ptr3 = *ptr1;
        *ptr1 = *ptr2;
        *ptr2 = ptr3;
    }

    /// Fills a block of memory with `byte`.
    /// Parameters:
    ///     `void *const ptr` - Pointer to block of memory.
    ///     `const NSTDCOREUSize size` - Size of block.
    ///     `const NSTDCOREByte byte` - Byte to fill with.
    NSTDAPI void nstd_core_mem_fill(
        void *const ptr,
        const NSTDCOREUSize size,
        const NSTDCOREByte byte)
    {
        NSTDCOREByte *mem{static_cast<NSTDCOREByte *>(ptr)};
        const NSTDCOREByte *const last{static_cast<NSTDCOREByte *>(ptr) + size};
        while (mem < last)
            *(mem++) = byte;
    }

    /// Zeros a memory range pointed to by `ptr`.
    /// Parameters:
    ///     `void *const ptr` - Pointer to memory to be zeroed.
    ///     `NSTDCOREUSize start` - Starting index of memory to be zeroed.
    ///     `const NSTDCOREUSize end` - Ending index of memory to be zeroed. (Excluded).
    NSTDAPI void nstd_core_mem_zero(void *const ptr, NSTDCOREUSize start, const NSTDCOREUSize end)
    {
        NSTDCOREByte *mem{static_cast<NSTDCOREByte *>(ptr) + start};
        while (start < end)
        {
            *(mem++) = 0;
            ++start;
        }
    }
}
