#ifndef NSTD_STD_THREAD_H_INCLUDED
#define NSTD_STD_THREAD_H_INCLUDED
#include "../core/def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Sleeps the current thread for `secs` seconds.
/// Parameters:
///     `const double secs` - Number of seconds to sleep for.
NSTDAPI void nstd_std_thread_sleep(const double secs);

/// Spawns a new thread.
/// Parameters:
///     `int(*thread_fn)()` - The function to be spawned as a new thread.
/// Returns: `void *handle` - The handle to the thread.
NSTDAPI void *nstd_std_thread_spawn(int(*thread_fn)());

/// Joins the given thread. Will set the thread handle to `NSTDC_NULL`.
/// Parameters:
///     `void **handle` - The handle to the thread.
///     `int *errc` - Returns as nonzero on error.
/// Returns: `int ret` - The value that the thread returns with.
NSTDAPI int nstd_std_thread_join(void **handle, int *errc);

/// Detaches the given thread. Will set the thread handle to `NSTDC_NULL`.
/// Parameters:
///     `void **handle` - The handle to the thread.
NSTDAPI void nstd_std_thread_detach(void **handle);

#ifdef __cplusplus
}
#endif
#endif
