#include <nstd/core/def.h>
#include <nstd/core/mem.h>
#include <nstd/std/io.h>
#include <iostream>

/// Reads from stdin and returns the read string.
/// Parameters:
///     `const bool appendNewline` - Whether or not to append a newline to the end.
/// Returns: `char *in` - String read from stdin.
static char *static_nstd_read_stdin(const bool appendNewline)
{
    NSTDCSize currentSize{8};
    char *in{static_cast<char *const>(nstd_core_mem_allocate(currentSize))};
    char *pos{in};
    long long offset;
    int next;
    while (true)
    {
        offset = pos - in;
        if (currentSize - offset <= 3)
        {
            currentSize *= 2;
            nstd_core_mem_reallocate((const void **)&in, currentSize);
            pos = in + offset;
        }
        next = std::cin.get();
        if (next == EOF || next == '\n')
            break;
        *(pos++) = next;
    }
    if (appendNewline)
        *(pos++) = '\n';
    *pos = '\0';
    std::cin.clear();
    return in;
}

extern "C"
{
    /// Writes a single character to stdout.
    /// Parameters:
    ///     `const char ch` - Character to write.
    NSTDAPI void nstd_std_io_writechar(const char ch)
    {
        std::cout.put(ch);
        std::cout.clear();
    }

    /// Writes `str` to stdout.
    /// Parameters:
    ///     `const char *const str` - String to write to stdout.
    NSTDAPI void nstd_std_io_write(const char *const str)
    {
        std::cout << str;
        std::cout.clear();
    }

    /// Writes `str` to stdout with an additional newline.
    /// Parameters:
    ///     `const char *const str` - String to write to stdout.
    NSTDAPI void nstd_std_io_writeline(const char *const str)
    {
        std::cout << str << '\n';
        std::cout.clear();
    }

    /// Reads a single character from stdin.
    /// Returns: `char ch` - Character read from stdin.
    NSTDAPI char nstd_std_io_readchar()
    {
        char ch;
        std::cin.get(ch);
        std::cin.clear();
        return ch;
    }

    /// Reads from stdin and returns the read string.
    /// Returns: `char *in` - String read from stdin.
    NSTDAPI char *nstd_std_io_read()
    {
        return static_nstd_read_stdin(false);
    }

    /// Reads from stdin and returns the read string appending a newline to the end.
    /// Returns: `char *in` - String read from stdin.
    NSTDAPI char *nstd_std_io_readline()
    {
        return static_nstd_read_stdin(true);
    }
}
