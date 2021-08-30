#include <nstd/def.h>
#include <nstd/mem.h>

extern "C"
{
    /// Allocates a block of memory with `size` bytes.
    /// Parameters:
    ///     `const NSTDSize size` - Size in bytes of memory to allocate.
    /// Returns: `void *ptr` - Pointer to the newly allocated memory.
    void *nstd_mem_allocate(const NSTDSize size)
    {
        return new NSTDByte[size];
    }

    /// Reallocates a block of memory with `size` bytes.
    /// Parameters:
    ///     `const void **const ptr` - Pointer to the memory to be allocated.
    ///     `const NSTDSize size` - Size in bytes of newly allocated memory.
    void nstd_mem_reallocate(const void **const ptr, const NSTDSize size)
    {
        NSTDByte *newMem{static_cast<NSTDByte *>(nstd_mem_allocate(size))};
        nstd_mem_copy(newMem, *ptr, size);
        nstd_mem_deallocate(ptr);
        *ptr = newMem;
    }

    /// Frees a block of memory. Will set `*ptr` to NULL.
    /// Parameters:
    ///     `const void **const ptr` - Pointer to the pointer to memory to free.
    void nstd_mem_deallocate(const void **const ptr)
    {
        delete[] static_cast<const NSTDByte *const>(*ptr);
        *ptr = NSTD_NULL;
    }

    /// Copies bytes from `other` to `copycat`.
    /// Parameters:
    ///     `void *const copycat` - Pointer to memory to be copied to.
    ///     `const void *const other` - Pointer to memory to be copied from.
    ///     `const NSTDSize size` - Number of bytes to copy.
    void nstd_mem_copy(void *const copycat, const void *const other, const NSTDSize size)
    {
        NSTDByte *const copier{static_cast<NSTDByte *const>(copycat)};
        const NSTDByte *const copied{static_cast<const NSTDByte *const>(other)};
        for (NSTDSize i{}; i < size; ++i)
            copier[i] = copied[i];
    }

    /// Moves bytes from `from` to `to`. Zeroes out `from`'s memory.
    /// Parameters:
    ///     `void *const from` - Memory to be moved from.
    ///     `void *const to` - Memory to be moved to.
    ///     `const NSTDSize size` - Number of bytes to move.
    void nstd_mem_move(void *const from, void *const to, const NSTDSize size)
    {
        NSTDByte *const frm{static_cast<NSTDByte *const>(from)};
        NSTDByte *const t{static_cast<NSTDByte *const>(to)};
        for (NSTDSize i{}; i < size; ++i)
        {
            t[i] = frm[i];
            frm[i] = 0;
        }
    }

    /// Moves memory from `*ptr1` to `*ptr2` and vice versa.
    /// Parameters:
    ///     `const void **const ptr1` - Pointer to first pointer's memory location.
    ///     `const void **const ptr2` - Pointer to second pointer's memory location.
    void nstd_mem_switch(const void **const ptr1, const void **const ptr2)
    {
        const void *const ptr3 = *ptr1;
        *ptr1 = *ptr2;
        *ptr2 = ptr3;
    }

    /// Zeros a memory range pointed to by `ptr`.
    /// Parameters:
    ///     `void *const ptr` - Pointer to memory to be zeroed.
    ///     `NSTDSize start` - Starting index of memory to be zeroed.
    ///     `const NSTDSize end` - Ending index of memory to be zeroed. (Excluded).
    void nstd_mem_zero(void *const ptr, NSTDSize start, const NSTDSize end)
    {
        NSTDByte *const mem{static_cast<NSTDByte *const>(ptr)};
        while (start < end)
        {
            mem[start] = 0;
            ++start;
        }
    }
}
