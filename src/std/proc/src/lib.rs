use std::{
    ffi::CStr,
    os::raw::*,
    process::{self, Child, Command},
    ptr,
};

/// Represents a process ID.
type NSTDProcessID = c_ulong;

/// Represents a process handle returned by `nstd_std_proc_spawn`.
type NSTDProcessHandle = *mut c_void;

/// Terminates the program in an abnormal fashion.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_proc_abort() {
    process::abort();
}

/// Exits the program with the specified exit code.
/// Parameters:
///     `const int code` - The exit code.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_proc_exit(code: c_int) {
    process::exit(code as i32);
}

/// Gets the current process' ID.
/// Returns: `NSTDProcessID id` - The process ID.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_proc_id() -> NSTDProcessID {
    process::id() as NSTDProcessID
}

/// Starts a new process.
/// Parameters:
///     `const char *const name` - The name of the process.
///     `const char *args` - String array of arguments to pass to the process.
///     `const NSTDSize size` - The number of args to pass.
/// Returns: `NSTDProcessHandle handle` - The handle to the new process, null on error.
#[no_mangle]
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
            Ok(child) => return Box::into_raw(Box::<Child>::new(child)) as NSTDProcessHandle,
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
#[no_mangle]
pub unsafe extern "C" fn nstd_std_proc_pid(handle: *mut NSTDProcessHandle) -> NSTDProcessID {
    (*(handle as *mut Child)).id() as NSTDProcessID
}

/// Waits for a process to finish.
/// Does not free memory allocated by `nstd_std_proc_spawn`.
/// Parameters:
///     `NSTDProcessHandle handle` - The handle to the process.
///     `int *code` - The exit code from the process, set to null if there was none specified.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_proc_wait(handle: *mut NSTDProcessHandle, code: *mut c_int) {
    let child = &mut *(handle as *mut Child);
    if let Ok(es) = child.wait() {
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
#[no_mangle]
pub unsafe extern "C" fn nstd_std_proc_kill(handle: *mut NSTDProcessHandle) -> c_int {
    match (*(handle as *mut Child)).kill() {
        Ok(_) => 0,
        _ => 1,
    }
}

/// Frees memory allocated by `nstd_std_proc_spawn`.
/// Parameters:
///     `NSTDProcessHandle *handle` - Pointer to a process handle.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_proc_free(handle: *mut NSTDProcessHandle) {
    Box::from_raw(*handle as *mut Child);
    *handle = ptr::null_mut();
}
