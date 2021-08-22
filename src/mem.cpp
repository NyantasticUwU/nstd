#include <nstd/def.h>
#include <nstd/mem.h>

extern "C"
{
    /// Allocates a block of memory with `size` bytes.
    /// Parameters:
    ///     `const NSTDSize size` - Size in bytes of memory to allocate.
    /// Returns: `void *ptr` - Pointer to the newly allocated memory.
    void *nstd_allocate(const NSTDSize size)
    {
        return new NSTDByte[size];
    }

    /// Frees a block of memory. Will set `*ptr` to NULL.
    /// Parameters:
    ///     `const void **const ptr` - Pointer to the pointer to memory to free.
    void nstd_deallocate(const void **const ptr)
    {
        const NSTDByte *const rawptr{static_cast<const NSTDByte *const>(*ptr)};
        delete[] rawptr;
        *ptr = NSTD_NULL;
    }

    /// Copies bytes from `*other` to `*copycat`.
    /// Parameters:
    ///     `void *const copycat` - Pointer to memory to be copied to.
    ///     `const void *const other` - Pointer to memory to be copied from.
    ///     `const NSTDSize size` - Number of bytes to copy.
    void nstd_memCopy(void *const copycat, const void *const other, const NSTDSize size)
    {
        NSTDByte *const copier{static_cast<NSTDByte *const>(copycat)};
        const NSTDByte *const copied{static_cast<const NSTDByte *const>(other)};
        for (NSTDSize i{}; i < size; ++i)
            copier[i] = copied[i];
    }
}
