#ifndef NSTD_THREAD_H_INCLUDED
#define NSTD_THREAD_H_INCLUDED
#include "core/def.h"
#include "nstd.h"
NSTDCPPSTART

/// Represents a thread handle.
typedef NSTDAny NSTDThreadHandle;

/// The return type of a thread function.
typedef NSTDErrorCode NSTDThreadReturn;

/// Sleeps the current thread for `secs` seconds.
///
/// # Parameters
///
/// - `const NSTDFloat64 secs` - Number of seconds to sleep for.
NSTDAPI void nstd_thread_sleep(const NSTDFloat64 secs);

/// Yields the current thread allowing other threads to have more CPU time.
NSTDAPI void nstd_thread_yield();

/// Spawns a new thread with `data`.
///
/// # Notes
/// - `data` will always be null on platforms that do not support atomic pointers.
/// - Failure to call `nstd_thread_join` or `nstd_thread_detach` will result in a memory leak.
///
/// # Parameters
///
/// - `NSTDThreadReturn(*thread_fn)(NSTDAny)` - The function to be spawned as a new thread.
///
/// - `NSTDAny data` - Custom data to send to the thread.
///
/// # Returns
///
/// `NSTDThreadHandle handle` - The handle to the thread.
NSTDAPI NSTDThreadHandle nstd_thread_spawn(NSTDThreadReturn(*thread_fn)(NSTDAny), NSTDAny data);

/// Joins the given thread. Will set the thread handle to `NSTD_CORE_NULL`.
///
/// # Parameters
///
/// - `NSTDThreadHandle *const handle` - The handle to the thread.
///
/// - `NSTDErrorCode *const errc` - Returns as nonzero on error.
///
/// # Returns
///
/// `NSTDThreadReturn ret` - The value that the thread returns with.
NSTDAPI NSTDThreadReturn nstd_thread_join(
    NSTDThreadHandle *const handle,
    NSTDErrorCode *const errc);

/// Detaches the given thread. Will set the thread handle to `NSTD_CORE_NULL`.
///
/// # Parameters
///
/// - `NSTDThreadHandle *const handle` - The handle to the thread.
NSTDAPI void nstd_thread_detach(NSTDThreadHandle *const handle);

NSTDCPPEND
#endif
