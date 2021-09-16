#ifndef NSTD_STD_PROC_H_INCLUDED
#define NSTD_STD_PROC_H_INCLUDED
#include "../core/def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a process ID.
typedef unsigned long NSTDProcessID;

/// Represents a process handle returned by `nstd_std_proc_spawn`.
typedef void *NSTDProcessHandle;

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
///     `const NSTDSize size` - The number of args to pass.
/// Returns: `NSTDProcessHandle handle` - The handle to the new process, null on error.
NSTDAPI NSTDProcessHandle nstd_std_proc_spawn(
    const char *const name,
    const char *args,
    const NSTDSize size);

/// Gets the ID of a process by handle.
/// Parameters:
///     `NSTDProcessHandle handle` - The handle to the process.
/// Returns: `NSTDProcessID id` - The process ID.
NSTDAPI NSTDProcessID nstd_std_proc_pid(NSTDProcessHandle handle);

/// Waits for a process to finish.
/// Does not free memory allocated by `nstd_std_proc_spawn`.
/// Parameters:
///     `NSTDProcessHandle handle` - The handle to the process.
///     `int *code` - The exit code from the process, set to null if there was none specified.
NSTDAPI void nstd_std_proc_wait(NSTDProcessHandle handle, int *code);

/// Kills a process by it's handle.
/// Does not free memory allocated by `nstd_std_proc_spawn`.
/// Parameters:
///     `NSTDProcessHandle handle` - The handle to the process.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_proc_kill(NSTDProcessHandle handle);

/// Frees memory allocated by `nstd_std_proc_spawn`.
/// Parameters:
///     `NSTDProcessHandle *handle` - Pointer to a process handle.
NSTDAPI void nstd_std_proc_free(NSTDProcessHandle *handle);

#ifdef __cplusplus
}
#endif
#endif
