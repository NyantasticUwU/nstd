//! Process management.
use crate::core::{def::NSTDErrorCode, slice::NSTDSlice, str::NSTDStr};
use std::process::{Child, Command};

/// Represents a process ID.
pub type NSTDProcessID = u32;

/// Represents a process handle returned by `nstd_proc_spawn`.
pub type NSTDChildProcess = *mut Child;

/// An error code to be returned by a process.
pub type NSTDExitCode = i32;

/// Terminates the program in an abnormal fashion.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_proc_abort() {
    std::process::abort();
}

/// Exits the program with the specified exit code.
///
/// # Parameters
///
/// - `const NSTDExitCode code` - The exit code.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_proc_exit(code: NSTDExitCode) {
    std::process::exit(code);
}

/// Gets the current process' ID.
///
/// # Returns
///
/// `NSTDProcessID id` - The process ID.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_proc_id() -> NSTDProcessID {
    std::process::id()
}

/// Starts a new process.
///
/// # Parameters
///
/// - `const NSTDStr *const name` - The name of the process.
///
/// - `const NSTDSlice *const args` - Slice of `NSTDStr` arguments to pass to the process.
///
/// # Returns
///
/// `NSTDChildProcess handle` - The handle to the new process, null on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_proc_spawn(name: &NSTDStr, args: &NSTDSlice) -> NSTDChildProcess {
    if let Ok(name) = std::str::from_utf8(name.bytes.as_byte_slice()) {
        let mut rargs = Vec::<&str>::new();
        for i in 0..args.size {
            let arg = crate::core::slice::nstd_core_slice_get(args, i) as *mut NSTDStr;
            if let Ok(arg) = std::str::from_utf8((*arg).bytes.as_byte_slice()) {
                rargs.push(arg);
            }
        }
        let mut command = Command::new(name);
        command.args(rargs);
        if let Ok(child) = command.spawn() {
            return Box::into_raw(Box::new(child));
        }
    }
    std::ptr::null_mut()
}

/// Gets the ID of a process by handle.
///
/// # Parameters
///
/// - `const NSTDChildProcess handle` - The handle to the process.
///
/// # Returns
///
/// `NSTDProcessID id` - The process ID.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_proc_pid(handle: NSTDChildProcess) -> NSTDProcessID {
    (*handle).id()
}

/// Waits for a process to finish.
/// Does not free memory allocated by `nstd_proc_spawn`.
///
/// # Parameters
///
/// - `const NSTDChildProcess handle` - The handle to the process.
///
/// - `NSTDExitCode **const code` - The exit code from the process, null if none specified.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_proc_wait(handle: NSTDChildProcess, code: *mut *mut NSTDExitCode) {
    if let Ok(es) = (*handle).wait() {
        match es.code() {
            Some(ec) => **code = ec,
            _ => *code = std::ptr::null_mut(),
        }
    }
}

/// Kills a process by it's handle.
/// Does not free memory allocated by `nstd_proc_spawn`.
///
/// # Parameters
///
/// - `const NSTDChildProcess handle` - The handle to the process.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_proc_kill(handle: NSTDChildProcess) -> NSTDErrorCode {
    if (*handle).kill().is_ok() {
        return 0;
    }
    1
}

/// Frees memory allocated by `nstd_proc_spawn`.
///
/// # Parameters
///
/// - `NSTDChildProcess *const handle` - Pointer to a process handle.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_proc_free(handle: *mut NSTDChildProcess) {
    Box::from_raw(*handle);
    *handle = std::ptr::null_mut();
}
