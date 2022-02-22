#ifndef NSTD_THREAD_H_INCLUDED
#define NSTD_THREAD_H_INCLUDED
#include "core/def.h"
#include "nstd.h"
NSTDCPPSTART

/// Represents a thread handle.
typedef void *NSTDThreadHandle;

/// The return type of a thread function.
typedef NSTDErrorCode NSTDThreadReturn;

/// Sleeps the current thread for `secs` seconds.
/// Parameters:
///     `const NSTDFloat64 secs` - Number of seconds to sleep for.
NSTDAPI void nstd_thread_sleep(const NSTDFloat64 secs);

/// Yields the current thread allowing other threads to have more CPU time.
NSTDAPI void nstd_thread_yield();

/// Spawns a new thread.
/// Failure to call `nstd_thread_join` or `nstd_thread_detach` will result in a memory leak.
/// Parameters:
///     `NSTDThreadReturn(*thread_fn)()` - The function to be spawned as a new thread.
/// Returns: `NSTDThreadHandle handle` - The handle to the thread.
NSTDAPI NSTDThreadHandle nstd_thread_spawn(NSTDThreadReturn(*thread_fn)());

/// Joins the given thread. Will set the thread handle to `NSTDC_NULL`.
/// Parameters:
///     `NSTDThreadHandle *const handle` - The handle to the thread.
///     `NSTDErrorCode *const errc` - Returns as nonzero on error.
/// Returns: `NSTDThreadReturn ret` - The value that the thread returns with.
NSTDAPI NSTDThreadReturn nstd_thread_join(
    NSTDThreadHandle *const handle,
    NSTDErrorCode *const errc);

/// Detaches the given thread. Will set the thread handle to `NSTDC_NULL`.
/// Parameters:
///     `NSTDThreadHandle *const handle` - The handle to the thread.
NSTDAPI void nstd_thread_detach(NSTDThreadHandle *const handle);

NSTDCPPEND
#endif
