use crate::core::def::NSTDErrorCode;
use std::{
    os::raw::{c_double, c_int},
    thread::JoinHandle,
    time::Duration,
};

/// Represents a thread handle
pub type NSTDThreadHandle = *mut JoinHandle<NSTDThreadReturn>;

/// The return type of a thread function.
pub type NSTDThreadReturn = NSTDErrorCode;

/// Sleeps the current thread for `secs` seconds.
/// Parameters:
///     `const double secs` - Number of seconds to sleep for.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_sleep(secs: c_double) {
    std::thread::sleep(Duration::from_secs_f64(secs));
}

/// Yields the current thread allowing other threads to have more CPU time.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_yield() {
    std::thread::yield_now();
}

/// Spawns a new thread.
/// Failure to call `nstd_thread_join` or `nstd_thread_detach` will result in a memory leak.
/// Parameters:
///     `NSTDThreadReturn(*thread_fn)()` - The function to be spawned as a new thread.
/// Returns: `void *handle` - The handle to the thread.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_spawn(
    thread_fn: extern "C" fn() -> NSTDThreadReturn,
) -> NSTDThreadHandle {
    Box::into_raw(Box::new(std::thread::spawn(move || thread_fn())))
}

/// Joins the given thread. Will set the thread handle to `NSTDC_NULL`.
/// Parameters:
///     `void **handle` - The handle to the thread.
/// Returns: `NSTDThreadReturn ret` - The value that the thread returns with.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_join(
    handle: *mut NSTDThreadHandle,
    errc: *mut c_int,
) -> NSTDThreadReturn {
    let (err, ret) = match Box::from_raw(*handle).join() {
        Ok(v) => (0, v),
        Err(_) => (1, 1),
    };
    *errc = err;
    *handle = std::ptr::null_mut();
    ret
}

/// Detaches the given thread. Will set the thread handle to `NSTDC_NULL`.
/// Parameters:
///     `void **handle` - The handle to the thread.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_detach(handle: *mut NSTDThreadHandle) {
    Box::from_raw(*handle);
    *handle = std::ptr::null_mut();
}
