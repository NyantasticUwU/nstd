#include <nstd/core/def.h>
#include <nstd/core/mem.h>

extern "C"
{
    /// Allocates a block of memory with `size` bytes.
    /// Parameters:
    ///     `const NSTDCORESize size` - Size in bytes of memory to allocate.
    /// Returns: `void *ptr` - Pointer to the newly allocated memory.
    NSTDAPI void *nstd_core_mem_allocate(const NSTDCORESize size)
    {
        return new NSTDCOREByte[size];
    }

    /// Reallocates a block of memory with `size` bytes.
    /// Parameters:
    ///     `const void **const ptr` - Pointer to the memory to be allocated.
    ///     `const NSTDCORESize size` - Size in bytes of newly allocated memory.
    /// Returns: `int errc` - Nonzero if reallocation succeeds.
    NSTDAPI int nstd_core_mem_reallocate(const void **const ptr, const NSTDCORESize size)
    {
        NSTDCOREByte *newMem{static_cast<NSTDCOREByte *>(nstd_core_mem_allocate(size))};
        if (newMem)
        {
            nstd_core_mem_copy(newMem, *ptr, size);
            nstd_core_mem_deallocate(ptr);
            *ptr = newMem;
        }
        return newMem == nullptr;
    }

    /// Frees a block of memory. Will set `*ptr` to NULL.
    /// Parameters:
    ///     `const void **const ptr` - Pointer to the pointer to memory to free.
    NSTDAPI void nstd_core_mem_deallocate(const void **const ptr)
    {
        delete[] static_cast<const NSTDCOREByte *const>(*ptr);
        *ptr = NSTDC_NULL;
    }

    /// Copies bytes from `other` to `copycat`.
    /// Parameters:
    ///     `void *const copycat` - Pointer to memory to be copied to.
    ///     `const void *const other` - Pointer to memory to be copied from.
    ///     `const NSTDCORESize size` - Number of bytes to copy.
    NSTDAPI void nstd_core_mem_copy(
        void *const copycat,
        const void *const other,
        const NSTDCORESize size)
    {
        NSTDCOREByte *copier{static_cast<NSTDCOREByte *>(copycat)};
        const NSTDCOREByte *copied{static_cast<const NSTDCOREByte *>(other)};
        for (NSTDCORESize i{}; i < size; ++i, ++copier, ++copied)
            *copier = *copied;
    }

    /// Moves bytes from `from` to `to`. Zeroes out `from`'s memory.
    /// Parameters:
    ///     `void *const from` - Memory to be moved from.
    ///     `void *const to` - Memory to be moved to.
    ///     `const NSTDCORESize size` - Number of bytes to move.
    NSTDAPI void nstd_core_mem_move(void *const from, void *const to, const NSTDCORESize size)
    {
        NSTDCOREByte *frm{static_cast<NSTDCOREByte *>(from)};
        NSTDCOREByte *t{static_cast<NSTDCOREByte *>(to)};
        for (NSTDCORESize i{}; i < size; ++i, ++frm, ++t)
        {
            *t = *frm;
            *frm = 0;
        }
    }

    /// Moves memory from `*ptr1` to `*ptr2` and vice versa.
    /// Parameters:
    ///     `const void **const ptr1` - Pointer to first pointer's memory location.
    ///     `const void **const ptr2` - Pointer to second pointer's memory location.
    NSTDAPI void nstd_core_mem_switch(const void **const ptr1, const void **const ptr2)
    {
        const void *const ptr3 = *ptr1;
        *ptr1 = *ptr2;
        *ptr2 = ptr3;
    }

    /// Fills a block of memory with `byte`.
    /// Parameters:
    ///     `void *const ptr` - Pointer to block of memory.
    ///     `const NSTDCORESize size` - Size of block.
    ///     `const NSTDCOREByte byte` - Byte to fill with.
    NSTDAPI void nstd_core_mem_fill(void *const ptr, const NSTDCORESize size, const NSTDCOREByte byte)
    {
        NSTDCOREByte *mem{static_cast<NSTDCOREByte *>(ptr)};
        const NSTDCOREByte *const last{static_cast<NSTDCOREByte *>(ptr) + size};
        while (mem < last)
            *(mem++) = byte;
    }

    /// Zeros a memory range pointed to by `ptr`.
    /// Parameters:
    ///     `void *const ptr` - Pointer to memory to be zeroed.
    ///     `NSTDCORESize start` - Starting index of memory to be zeroed.
    ///     `const NSTDCORESize end` - Ending index of memory to be zeroed. (Excluded).
    NSTDAPI void nstd_core_mem_zero(void *const ptr, NSTDCORESize start, const NSTDCORESize end)
    {
        NSTDCOREByte *mem{static_cast<NSTDCOREByte *>(ptr) + start};
        while (start < end)
        {
            *(mem++) = 0;
            ++start;
        }
    }
}
