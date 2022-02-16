#ifndef NSTD_PROC_H_INCLUDED
#define NSTD_PROC_H_INCLUDED
#include "core/def.h"
#include "nstd.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a process ID.
typedef NSTDUInt32 NSTDProcessID;

/// Represents a process handle returned by `nstd_proc_spawn`.
typedef NSTDAny NSTDChildProcess;

/// An error code to be returned by a process.
typedef NSTDInt32 NSTDExitCode;

/// Terminates the program in an abnormal fashion.
NSTDAPI void nstd_proc_abort();

/// Exits the program with the specified exit code.
/// Parameters:
///     `const NSTDExitCode code` - The exit code.
NSTDAPI void nstd_proc_exit(const NSTDExitCode code);

/// Gets the current process' ID.
/// Returns: `NSTDProcessID id` - The process ID.
NSTDAPI NSTDProcessID nstd_proc_id();

/// Starts a new process.
/// Parameters:
///     `const char *const name` - The name of the process.
///     `const char *args` - String array of arguments to pass to the process.
///     `const NSTDUSize size` - The number of args to pass.
/// Returns: `NSTDChildProcess handle` - The handle to the new process, null on error.
NSTDAPI NSTDChildProcess nstd_proc_spawn(
    const char *const name,
    const char *args,
    const NSTDUSize size);

/// Gets the ID of a process by handle.
/// Parameters:
///     `NSTDChildProcess handle` - The handle to the process.
/// Returns: `NSTDProcessID id` - The process ID.
NSTDAPI NSTDProcessID nstd_proc_pid(NSTDChildProcess handle);

/// Waits for a process to finish.
/// Does not free memory allocated by `nstd_proc_spawn`.
/// Parameters:
///     `NSTDChildProcess handle` - The handle to the process.
///     `NSTDExitCode **const code` - The exit code from the process, null if none specified.
NSTDAPI void nstd_proc_wait(NSTDChildProcess handle, NSTDExitCode **const code);

/// Kills a process by it's handle.
/// Does not free memory allocated by `nstd_proc_spawn`.
/// Parameters:
///     `NSTDChildProcess handle` - The handle to the process.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_proc_kill(NSTDChildProcess handle);

/// Frees memory allocated by `nstd_proc_spawn`.
/// Parameters:
///     `NSTDChildProcess *handle` - Pointer to a process handle.
NSTDAPI void nstd_proc_free(NSTDChildProcess *handle);

#ifdef __cplusplus
}
#endif
#endif
