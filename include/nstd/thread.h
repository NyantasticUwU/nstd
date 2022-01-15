#ifndef NSTD_THREAD_H_INCLUDED
#define NSTD_THREAD_H_INCLUDED
#include "core/def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a thread handle.
typedef void *NSTDThreadHandle;

/// Sleeps the current thread for `secs` seconds.
/// Parameters:
///     `const double secs` - Number of seconds to sleep for.
NSTDAPI void nstd_thread_sleep(const double secs);

/// Yields the current thread allowing other threads to have more CPU time.
NSTDAPI void nstd_thread_yield();

/// Spawns a new thread.
/// Failure to call `nstd_thread_join` or `nstd_thread_detach` will result in a memory leak.
/// Parameters:
///     `int(*thread_fn)()` - The function to be spawned as a new thread.
/// Returns: `NSTDThreadHandle handle` - The handle to the thread.
NSTDAPI NSTDThreadHandle nstd_thread_spawn(int(*thread_fn)());

/// Joins the given thread. Will set the thread handle to `NSTDC_NULL`.
/// Parameters:
///     `NSTDThreadHandle *handle` - The handle to the thread.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `int ret` - The value that the thread returns with.
NSTDAPI int nstd_thread_join(NSTDThreadHandle *handle, int *errc);

/// Detaches the given thread. Will set the thread handle to `NSTDC_NULL`.
/// Parameters:
///     `NSTDThreadHandle *handle` - The handle to the thread.
NSTDAPI void nstd_thread_detach(NSTDThreadHandle *handle);

#ifdef __cplusplus
}
#endif
#endif
