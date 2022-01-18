#ifndef NSTD_IO_H_INCLUDED
#define NSTD_IO_H_INCLUDED
#include "core/def.h"
#include "core/slice.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Attempts to flush stdout.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_io_flush();

/// Writes a single character to stdout.
/// Parameters:
///     `const char ch` - Character to write.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_io_write_char(const char ch);

/// Writes `str` to stdout.
/// Parameters:
///     `const char *const str` - String to write to stdout.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_io_write(const char *const str);

/// Writes `str` to stdout with an additional newline.
/// Parameters:
///     `const char *const str` - String to write to stdout.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_io_write_line(const char *const str);

/// Writes a raw byte slice to stdout.
/// Parameters:
///     `const NSTDSlice *const bytes` - The byte slice to write to stdout.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_io_write_raw(const NSTDSlice *const bytes);

/// Attempts to flush stderr.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_io_flush_err();

/// Writes a single character to stderr.
/// Parameters:
///     `const char ch` - Character to write.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_io_write_char_err(const char ch);

/// Writes `str` to stderr.
/// Parameters:
///     `const char *const str` - String to write to stderr.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_io_write_err(const char *const str);

/// Writes `str` to stderr with an additional newline.
/// Parameters:
///     `const char *const str` - String to write to stderr.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_io_write_line_err(const char *const str);

/// Writes a raw byte slice to stderr.
/// Parameters:
///     `const NSTDSlice *const bytes` - The byte slice to write to stderr.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_io_write_raw_err(const NSTDSlice *const bytes);

/// Reads a single character from stdin.
/// Parameters:
///     `int *errc` - Error code, returns as nonzero on error.
/// Returns: `char ch` - Character read from stdin.
NSTDAPI char nstd_io_read_char(int *errc);

/// Reads from stdin and returns the read string.
/// Parameters:
///     `int *errc` - Error code, returns as nonzero on error.
/// Returns: `char *in` - String read from stdin.
NSTDAPI char *nstd_io_read(int *errc);

/// Reads from stdin and returns the read string appending a newline to the end.
/// Parameters:
///     `int *errc` - Error code, returns as nonzero on error.
/// Returns: `char *in` - String read from stdin.
NSTDAPI char *nstd_io_read_line(int *errc);

/// Frees memory allocated by `nstd_io_read` and `nstd_io_readline`.
/// Parameters:
///     `const char **str` - Pointer to the string returned from the read functions.
NSTDAPI void nstd_io_free_read(const char **str);

#ifdef __cplusplus
}
#endif
#endif