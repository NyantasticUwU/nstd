#ifndef NSTD_STD_IO_H_INCLUDED
#define NSTD_STD_IO_H_INCLUDED
#include "../core/def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Writes a single character to stdout.
/// Parameters:
///     `const char ch` - Character to write.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_io_writechar(const char ch);

/// Writes `str` to stdout.
/// Parameters:
///     `const char *const str` - String to write to stdout.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_io_write(const char *const str);

/// Writes `str` to stdout with an additional newline.
/// Parameters:
///     `const char *const str` - String to write to stdout.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_io_writeline(const char *const str);

/// Reads a single character from stdin.
/// Parameters:
///     `int *const errc` - Error code, returns as nonzero on error.
/// Returns: `char ch` - Character read from stdin.
NSTDAPI char nstd_std_io_readchar(int *const errc);

/// Reads from stdin and returns the read string.
/// Parameters:
///     `int *const errc` - Error code, returns as nonzero on error.
/// Returns: `char *in` - String read from stdin.
NSTDAPI char *nstd_std_io_read(int *const errc);

/// Reads from stdin and returns the read string appending a newline to the end.
/// Parameters:
///     `int *const errc` - Error code, returns as nonzero on error.
/// Returns: `char *in` - String read from stdin.
NSTDAPI char *nstd_std_io_readline(int *const errc);

#ifdef __cplusplus
}
#endif
#endif
