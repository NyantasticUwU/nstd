#ifndef NSTD_IO_STDIN_H_INCLUDED
#define NSTD_IO_STDIN_H_INCLUDED
#include "../nstd.h"
#include "input_stream.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a handle to the standard input stream.
typedef NSTDInputStream NSTDStandardInput;

/// Frees a handle to stdin.
/// Parameters:
///     `NSTDStandardInput stdin` - The standard input stream.
NSTDAPI void nstd_io_stdin_free(NSTDStandardInput stdin);

#ifdef __cplusplus
}
#endif
#endif
