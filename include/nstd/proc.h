#ifndef NSTD_PROC_H_INCLUDED
#define NSTD_PROC_H_INCLUDED
#include "core/def.h"
#include "core/slice.h"
#include "core/str.h"
#include "nstd.h"
NSTDCPPSTART

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
///     `const NSTDStr *const name` - The name of the process.
///     `const NSTDSlice *const args` - Slice of `NSTDStr` arguments to pass to the process.
/// Returns: `NSTDChildProcess handle` - The handle to the new process, null on error.
NSTDAPI NSTDChildProcess nstd_proc_spawn(const NSTDStr *const name, const NSTDSlice *const args);

/// Gets the ID of a process by handle.
/// Parameters:
///     `const NSTDChildProcess handle` - The handle to the process.
/// Returns: `NSTDProcessID id` - The process ID.
NSTDAPI NSTDProcessID nstd_proc_pid(const NSTDChildProcess handle);

/// Waits for a process to finish.
/// Does not free memory allocated by `nstd_proc_spawn`.
/// Parameters:
///     `const NSTDChildProcess handle` - The handle to the process.
///     `NSTDExitCode **const code` - The exit code from the process, null if none specified.
NSTDAPI void nstd_proc_wait(const NSTDChildProcess handle, NSTDExitCode **const code);

/// Kills a process by it's handle.
/// Does not free memory allocated by `nstd_proc_spawn`.
/// Parameters:
///     `const NSTDChildProcess handle` - The handle to the process.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_proc_kill(const NSTDChildProcess handle);

/// Frees memory allocated by `nstd_proc_spawn`.
/// Parameters:
///     `NSTDChildProcess *const handle` - Pointer to a process handle.
NSTDAPI void nstd_proc_free(NSTDChildProcess *const handle);

NSTDCPPEND
#endif
