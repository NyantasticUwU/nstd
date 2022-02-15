#ifndef NSTD_IO_STDERR_H_INCLUDED
#define NSTD_IO_STDERR_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "output_stream.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// A raw handle to stdout.
typedef NSTDAny NSTDStandardErrorHandle;

/// Represents a handle to the standard error stream.
typedef struct
{
    /// The output stream.
    NSTDOutputStream output_stream;
    /// The raw handle to stderr.
    NSTDStandardErrorHandle handle;
} NSTDStandardError;

/// Frees a handle to stderr.
/// Parameters:
///     `NSTDStandardError *const stderr` - The standard error stream.
NSTDAPI void nstd_io_stderr_free(NSTDStandardError *const stderr);

#ifdef __cplusplus
}
#endif
#endif