use std::{
    os::raw::{c_int, c_ulong},
    process,
};

/// Represents a process ID.
type NSTDProcessID = c_ulong;

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
