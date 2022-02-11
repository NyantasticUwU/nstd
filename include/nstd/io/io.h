#ifndef NSTD_IO_IO_H_INCLUDED
#define NSTD_IO_IO_H_INCLUDED
#include "../nstd.h"
#include "stdin.h"
#include "stdout.h"
#include "stderr.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Returns a handle to stdin.
/// Returns: `NSTDStandardInput stdin` - The standard input stream.
NSTDAPI NSTDStandardInput nstd_io_stdin();

/// Returns a handle to stdout.
/// Returns: `NSTDStandardOutput stdout` - The standard output stream.
NSTDAPI NSTDStandardOutput nstd_io_stdout();

/// Returns a handle to stderr.
/// Returns: `NSTDStandardError stderr` - The standard error stream.
NSTDAPI NSTDStandardError nstd_io_stderr();

#ifdef __cplusplus
}
#endif
#endif
