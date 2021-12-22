#ifndef NSTD_STD_PROC_H_INCLUDED
#define NSTD_STD_PROC_H_INCLUDED
#include "../core/def.h"
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a process ID.
typedef unsigned long NSTDProcessID;

/// Represents a process handle returned by `nstd_std_proc_spawn`.
typedef void *NSTDChildProcess;

/// Terminates the program in an abnormal fashion.
NSTDAPI void nstd_std_proc_abort();

/// Exits the program with the specified exit code.
/// Parameters:
///     `const int code` - The exit code.
NSTDAPI void nstd_std_proc_exit(const int code);

/// Gets the current process' ID.
/// Returns: `NSTDProcessID id` - The process ID.
NSTDAPI NSTDProcessID nstd_std_proc_id();

/// Starts a new process.
/// Parameters:
///     `const char *const name` - The name of the process.
///     `const char *args` - String array of arguments to pass to the process.
///     `const NSTDUSize size` - The number of args to pass.
/// Returns: `NSTDChildProcess handle` - The handle to the new process, null on error.
NSTDAPI NSTDChildProcess nstd_std_proc_spawn(
    const char *const name,
    const char *args,
    const NSTDUSize size);

/// Gets the ID of a process by handle.
/// Parameters:
///     `NSTDChildProcess handle` - The handle to the process.
/// Returns: `NSTDProcessID id` - The process ID.
NSTDAPI NSTDProcessID nstd_std_proc_pid(NSTDChildProcess handle);

/// Waits for a process to finish.
/// Does not free memory allocated by `nstd_std_proc_spawn`.
/// Parameters:
///     `NSTDChildProcess handle` - The handle to the process.
///     `int *code` - The exit code from the process, set to null if there was none specified.
NSTDAPI void nstd_std_proc_wait(NSTDChildProcess handle, int *code);

/// Kills a process by it's handle.
/// Does not free memory allocated by `nstd_std_proc_spawn`.
/// Parameters:
///     `NSTDChildProcess handle` - The handle to the process.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_proc_kill(NSTDChildProcess handle);

/// Frees memory allocated by `nstd_std_proc_spawn`.
/// Parameters:
///     `NSTDChildProcess *handle` - Pointer to a process handle.
NSTDAPI void nstd_std_proc_free(NSTDChildProcess *handle);

#ifdef __cplusplus
}
#endif
#endif
