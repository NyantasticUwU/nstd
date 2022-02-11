#ifndef NSTD_IO_STDERR_H_INCLUDED
#define NSTD_IO_STDERR_H_INCLUDED
#include "../nstd.h"
#include "output_stream.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a handle to the standard error stream.
typedef NSTDOutputStream NSTDStandardError;

/// Frees a handle to stderr.
/// Parameters:
///     `NSTDStandardError stderr` - The standard error stream.
NSTDAPI void nstd_io_stderr_free(NSTDStandardError stderr);

#ifdef __cplusplus
}
#endif
#endif
