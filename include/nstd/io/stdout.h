#ifndef NSTD_IO_STDOUT_H_INCLUDED
#define NSTD_IO_STDOUT_H_INCLUDED
#include "../nstd.h"
#include "output_stream.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a handle to the standard output stream.
typedef NSTDOutputStream NSTDStandardOutput;

/// Frees a handle to stdout.
/// Parameters:
///     `NSTDStandardOutput stdout` - The standard output stream.
NSTDAPI void nstd_io_stdout_free(NSTDStandardOutput stdout);

#ifdef __cplusplus
}
#endif
#endif
