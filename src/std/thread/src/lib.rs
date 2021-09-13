use std::{
    os::raw::{c_double, c_int, c_void},
    ptr,
    thread::{self, JoinHandle},
    time::Duration,
};

/// Represents a thread handle
type NSTDThreadHandle = *mut c_void;
/// A thread function's return type.
type ThreadReturn = c_int;

/// Sleeps the current thread for `secs` seconds.
/// Parameters:
///     `const double secs` - Number of seconds to sleep for.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_thread_sleep(secs: c_double) {
    thread::sleep(Duration::from_secs_f64(secs));
}

/// Yields the current thread allowing other threads to have more CPU time.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_thread_yield() {
    thread::yield_now();
}

/// Spawns a new thread.
/// Failure to call `nstd_std_thread_join` or `nstd_std_thread_detach` will result in a memory leak.
/// Parameters:
///     `int(*thread_fn)()` - The function to be spawned as a new thread.
/// Returns: `void *handle` - The handle to the thread.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_thread_spawn(
    thread_fn: extern "C" fn() -> ThreadReturn,
) -> NSTDThreadHandle {
    Box::into_raw(Box::new(thread::spawn(move || thread_fn()))) as NSTDThreadHandle
}

/// Joins the given thread. Will set the thread handle to `NSTDC_NULL`.
/// Parameters:
///     `void **handle` - The handle to the thread.
/// Returns: `int ret` - The value that the thread returns with.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_thread_join(
    handle: *mut NSTDThreadHandle,
    errc: *mut c_int,
) -> c_int {
    let (err, ret) = match Box::from_raw(*handle as *mut JoinHandle<ThreadReturn>).join() {
        Ok(v) => (0, v),
        Err(_) => (1, 1),
    };
    *errc = err;
    *handle = ptr::null_mut();
    ret
}

/// Detaches the given thread. Will set the thread handle to `NSTDC_NULL`.
/// Parameters:
///     `void **handle` - The handle to the thread.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_thread_detach(handle: *mut NSTDThreadHandle) {
    Box::from_raw(*handle as *mut JoinHandle<ThreadReturn>);
    *handle = ptr::null_mut();
}
