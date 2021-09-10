#include <nstd/core/mem.h>
#include <nstd/std/def.h>
#include <nstd/std/io.h>
#include <ios>
#include <iostream>

/// Checks if the given stream's fail bit is set.
/// Parameters:
///     `const std::basic_ios &stream` - The stream to check.
/// Returns: `int errc` - 1 if the stream's fail bit is set.
template <typename T>
static inline int static_nstd_check_stream_fail(std::basic_ios<T> &stream)
{
    const int err{stream.fail()};
    stream.clear();
    return err;
}

/// Reads from stdin and returns the read string.
/// Parameters:
///     `const bool appendNewline` - Whether or not to append a newline to the end.
///     `int *const errc` - Returns as nonzero on error.
/// Returns: `char *in` - String read from stdin.
static char *static_nstd_read_stdin(const bool appendNewline, int *const errc)
{
    NSTDSize currentSize{8};
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
    *errc = static_nstd_check_stream_fail(std::cin);
    return in;
}

extern "C"
{
    /// Writes a single character to stdout.
    /// Parameters:
    ///     `const char ch` - Character to write.
    /// Returns: `int errc` - Nonzero on error.
    NSTDAPI int nstd_std_io_writechar(const char ch)
    {
        std::cout.put(ch);
        return static_nstd_check_stream_fail(std::cout);
    }

    /// Writes `str` to stdout.
    /// Parameters:
    ///     `const char *const str` - String to write to stdout.
    /// Returns: `int errc` - Nonzero on error.
    NSTDAPI int nstd_std_io_write(const char *const str)
    {
        std::cout << str;
        return static_nstd_check_stream_fail(std::cout);
    }

    /// Writes `str` to stdout with an additional newline.
    /// Parameters:
    ///     `const char *const str` - String to write to stdout.
    /// Returns: `int errc` - Nonzero on error.
    NSTDAPI int nstd_std_io_writeline(const char *const str)
    {
        std::cout << str << '\n';
        return static_nstd_check_stream_fail(std::cout);
    }

    /// Reads a single character from stdin.
    /// Parameters:
    ///     `int *const errc` - Error code, returns as nonzero on error.
    /// Returns: `char ch` - Character read from stdin.
    NSTDAPI char nstd_std_io_readchar(int *const errc)
    {
        char ch;
        std::cin.get(ch);
        *errc = static_nstd_check_stream_fail(std::cin);
        return ch;
    }

    /// Reads from stdin and returns the read string.
    /// Parameters:
    ///     `int *const errc` - Error code, returns as nonzero on error.
    /// Returns: `char *in` - String read from stdin.
    NSTDAPI char *nstd_std_io_read(int *const errc)
    {
        return static_nstd_read_stdin(false, errc);
    }

    /// Reads from stdin and returns the read string appending a newline to the end.
    /// Parameters:
    ///     `int *const errc` - Error code, returns as nonzero on error.
    /// Returns: `char *in` - String read from stdin.
    NSTDAPI char *nstd_std_io_readline(int *const errc)
    {
        return static_nstd_read_stdin(true, errc);
    }
}
