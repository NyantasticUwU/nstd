#ifndef NSTD_OS_WINDOWS_THREAD_H_INCLUDED
#define NSTD_OS_WINDOWS_THREAD_H_INCLUDED
#include "../../core/def.h"
#include "../../nstd.h"
NSTDCPPSTART

/// Puts the current thread to sleep for `ms` milliseconds.
///
/// # Parameters:
///
/// - `const NSTDUInt32 ms` - The number of milliseconds to sleep for.
NSTDAPI void nstd_os_windows_thread_sleep(const NSTDUInt32 ms);

NSTDCPPEND
#endif
