#ifndef NSTD_STD_IO_H_INCLUDED
#define NSTD_STD_IO_H_INCLUDED
#include "../core/def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Writes `str` to stdout.
/// Parameters:
///     `const char *const str` - String to write to stdout.
NSTDAPI void nstd_std_io_write(const char *const str);

/// Writes `str` to stdout with an additional newline.
/// Parameters:
///     `const char *const str` - String to write to stdout.
NSTDAPI void nstd_std_io_writeline(const char *const str);

/// Reads from stdin and returns the read string.
/// Returns: `char *in` - String read from stdin.
NSTDAPI char *nstd_std_io_read();

/// Reads from stdin and returns the read string appending a newline to the end.
/// Returns: `char *in` - String read from stdin.
NSTDAPI char *nstd_std_io_readline();

#ifdef __cplusplus
}
#endif
#endif
