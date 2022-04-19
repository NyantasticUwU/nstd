//! Threading API.
use crate::core::def::{NSTDAny, NSTDErrorCode};
use std::{thread::JoinHandle, time::Duration};

/// Represents a thread handle
pub type NSTDThreadHandle = *mut JoinHandle<NSTDThreadReturn>;

/// The return type of a thread function.
pub type NSTDThreadReturn = NSTDErrorCode;

/// Sleeps the current thread for `secs` seconds.
///
/// # Parameters
///
/// - `const NSTDFloat64 secs` - Number of seconds to sleep for.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_sleep(secs: f64) {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[cfg_attr(not(target_has_atomic = "ptr"), allow(unused_variables))]
pub unsafe extern "C" fn nstd_thread_spawn(
    thread_fn: unsafe extern "C" fn(NSTDAny) -> NSTDThreadReturn,
    data: NSTDAny,
) -> NSTDThreadHandle {
    #[cfg(target_has_atomic = "ptr")]
    {
        use std::sync::atomic::AtomicPtr;
        let data = AtomicPtr::new(data);
        let thread = std::thread::spawn(move || thread_fn(data.into_inner()));
        Box::into_raw(Box::new(thread))
    }
    #[cfg(not(target_has_atomic = "ptr"))]
    {
        let thread = std::thread::spawn(move || thread_fn(std::ptr::null_mut()));
        Box::into_raw(Box::new(thread))
    }
}

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
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_join(
    handle: *mut NSTDThreadHandle,
    errc: *mut NSTDErrorCode,
) -> NSTDThreadReturn {
    let (err, ret) = match Box::from_raw(*handle).join() {
        Ok(v) => (0, v),
        Err(_) => (1, 1),
    };
    *errc = err;
    *handle = std::ptr::null_mut();
    ret
}

/// Detaches the given thread. Will set the thread handle to `NSTD_CORE_NULL`.
///
/// # Parameters
///
/// - `NSTDThreadHandle *const handle` - The handle to the thread.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_thread_detach(handle: *mut NSTDThreadHandle) {
    Box::from_raw(*handle);
    *handle = std::ptr::null_mut();
}
