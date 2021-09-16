#ifndef NSTD_STD_PROC_H_INCLUDED
#define NSTD_STD_PROC_H_INCLUDED
#include "../core/def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a process ID.
typedef unsigned long NSTDProcessID;

/// Terminates the program in an abnormal fashion.
NSTDAPI void nstd_std_proc_abort();

/// Exits the program with the specified exit code.
/// Parameters:
///     `const int code` - The exit code.
NSTDAPI void nstd_std_proc_exit(const int code);

/// Gets the current process' ID.
/// Returns: `NSTDProcessID id` - The process ID.
NSTDAPI NSTDProcessID nstd_std_proc_id();

#ifdef __cplusplus
}
#endif
#endif
