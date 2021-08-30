#include <nstd/core/def.h>
#include <nstd/core/mem.h>

extern "C"
{
    /// Allocates a block of memory with `size` bytes.
    /// Parameters:
    ///     `const NSTDCSize size` - Size in bytes of memory to allocate.
    /// Returns: `void *ptr` - Pointer to the newly allocated memory.
    void *nstd_core_mem_allocate(const NSTDCSize size)
    {
        return new NSTDCByte[size];
    }

    /// Reallocates a block of memory with `size` bytes.
    /// Parameters:
    ///     `const void **const ptr` - Pointer to the memory to be allocated.
    ///     `const NSTDCSize size` - Size in bytes of newly allocated memory.
    void nstd_core_mem_reallocate(const void **const ptr, const NSTDCSize size)
    {
        NSTDCByte *newMem{static_cast<NSTDCByte *>(nstd_core_mem_allocate(size))};
        nstd_core_mem_copy(newMem, *ptr, size);
        nstd_core_mem_deallocate(ptr);
        *ptr = newMem;
    }

    /// Frees a block of memory. Will set `*ptr` to NULL.
    /// Parameters:
    ///     `const void **const ptr` - Pointer to the pointer to memory to free.
    void nstd_core_mem_deallocate(const void **const ptr)
    {
        delete[] static_cast<const NSTDCByte *const>(*ptr);
        *ptr = NSTDC_NULL;
    }

    /// Copies bytes from `other` to `copycat`.
    /// Parameters:
    ///     `void *const copycat` - Pointer to memory to be copied to.
    ///     `const void *const other` - Pointer to memory to be copied from.
    ///     `const NSTDCSize size` - Number of bytes to copy.
    void nstd_core_mem_copy(void *const copycat, const void *const other, const NSTDCSize size)
    {
        NSTDCByte *const copier{static_cast<NSTDCByte *const>(copycat)};
        const NSTDCByte *const copied{static_cast<const NSTDCByte *const>(other)};
        for (NSTDCSize i{}; i < size; ++i)
            copier[i] = copied[i];
    }

    /// Moves bytes from `from` to `to`. Zeroes out `from`'s memory.
    /// Parameters:
    ///     `void *const from` - Memory to be moved from.
    ///     `void *const to` - Memory to be moved to.
    ///     `const NSTDCSize size` - Number of bytes to move.
    void nstd_core_mem_move(void *const from, void *const to, const NSTDCSize size)
    {
        NSTDCByte *const frm{static_cast<NSTDCByte *const>(from)};
        NSTDCByte *const t{static_cast<NSTDCByte *const>(to)};
        for (NSTDCSize i{}; i < size; ++i)
        {
            t[i] = frm[i];
            frm[i] = 0;
        }
    }

    /// Moves memory from `*ptr1` to `*ptr2` and vice versa.
    /// Parameters:
    ///     `const void **const ptr1` - Pointer to first pointer's memory location.
    ///     `const void **const ptr2` - Pointer to second pointer's memory location.
    void nstd_core_mem_switch(const void **const ptr1, const void **const ptr2)
    {
        const void *const ptr3 = *ptr1;
        *ptr1 = *ptr2;
        *ptr2 = ptr3;
    }

    /// Zeros a memory range pointed to by `ptr`.
    /// Parameters:
    ///     `void *const ptr` - Pointer to memory to be zeroed.
    ///     `NSTDCSize start` - Starting index of memory to be zeroed.
    ///     `const NSTDCSize end` - Ending index of memory to be zeroed. (Excluded).
    void nstd_core_mem_zero(void *const ptr, NSTDCSize start, const NSTDCSize end)
    {
        NSTDCByte *const mem{static_cast<NSTDCByte *const>(ptr)};
        while (start < end)
        {
            mem[start] = 0;
            ++start;
        }
    }
}
