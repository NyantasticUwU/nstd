use std::{
    ffi::CStr,
    os::raw::{c_char, c_int, c_ulong},
    process::{self, Child, Command},
    ptr,
};
#[cfg(feature = "deps")]
pub mod deps {}

/// Represents a process ID.
pub type NSTDProcessID = c_ulong;

/// Represents a process handle returned by `nstd_std_proc_spawn`.
pub type NSTDProcessHandle = *mut Child;

/// Terminates the program in an abnormal fashion.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_proc_abort() {
    process::abort();
}

/// Exits the program with the specified exit code.
/// Parameters:
///     `const int code` - The exit code.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_proc_exit(code: c_int) {
    process::exit(code as i32);
}

/// Gets the current process' ID.
/// Returns: `NSTDProcessID id` - The process ID.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_proc_id() -> NSTDProcessID {
    process::id() as NSTDProcessID
}

/// Starts a new process.
/// Parameters:
///     `const char *const name` - The name of the process.
///     `const char *args` - String array of arguments to pass to the process.
///     `const NSTDUSize size` - The number of args to pass.
/// Returns: `NSTDProcessHandle handle` - The handle to the new process, null on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_proc_spawn(
    name: *const c_char,
    mut args: *const c_char,
    size: usize,
) -> NSTDProcessHandle {
    let name = CStr::from_ptr(name);
    let mut rargs = Vec::<&CStr>::new();
    let mut cstr: &CStr;
    for _ in 0..size {
        cstr = CStr::from_ptr(args);
        args = args.offset(cstr.to_bytes_with_nul().len() as isize);
        rargs.push(cstr);
    }
    if let Ok(name) = name.to_str() {
        let mut command = Command::new(name);
        for arg in rargs {
            match arg.to_str() {
                Ok(arg) => command.arg(arg),
                _ => return ptr::null_mut(),
            };
        }
        match command.spawn() {
            Ok(child) => return Box::into_raw(Box::<Child>::new(child)),
            _ => return ptr::null_mut(),
        }
    }
    ptr::null_mut()
}

/// Gets the ID of a process by handle.
/// Parameters:
///     `NSTDProcessHandle handle` - The handle to the process.
/// Returns: `NSTDProcessID id` - The process ID.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_proc_pid(handle: NSTDProcessHandle) -> NSTDProcessID {
    (*handle).id() as NSTDProcessID
}

/// Waits for a process to finish.
/// Does not free memory allocated by `nstd_std_proc_spawn`.
/// Parameters:
///     `NSTDProcessHandle handle` - The handle to the process.
///     `int *code` - The exit code from the process, set to null if there was none specified.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_proc_wait(handle: NSTDProcessHandle, code: *mut c_int) {
    if let Ok(es) = (*handle).wait() {
        if let Some(ec) = es.code() {
            *code = ec as c_int;
        }
    }
}

/// Kills a process by it's handle.
/// Does not free memory allocated by `nstd_std_proc_spawn`.
/// Parameters:
///     `NSTDProcessHandle handle` - The handle to the process.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_proc_kill(handle: NSTDProcessHandle) -> c_int {
    match (*handle).kill() {
        Ok(_) => 0,
        _ => 1,
    }
}

/// Frees memory allocated by `nstd_std_proc_spawn`.
/// Parameters:
///     `NSTDProcessHandle *handle` - Pointer to a process handle.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_proc_free(handle: *mut NSTDProcessHandle) {
    Box::from_raw(*handle);
    *handle = ptr::null_mut();
}
