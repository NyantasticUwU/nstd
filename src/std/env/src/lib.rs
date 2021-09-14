use std::{
    env,
    ffi::CString,
    os::raw::{c_char, c_int},
    ptr,
};
#[allow(non_camel_case_types)]
type byte = u8;

/// Generates `nstd_std_env_path_to_exe` and `nstd_std_env_current_dir` functions.
macro_rules! nstd_path_fns {
    ($fn_name: ident, $env_fn: ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $fn_name(errc: *mut c_int) -> *mut c_char {
            match env::$env_fn() {
                Ok(path) => {
                    *errc = 0;
                    let mut path = path.to_string_lossy().to_string();
                    path.push('\0');
                    CString::from_vec_unchecked(path.into_bytes()).into_raw()
                }
                _ => {
                    *errc = 1;
                    ptr::null_mut()
                }
            }
        }
    };
}
nstd_path_fns!(nstd_std_env_path_to_exe, current_exe);
nstd_path_fns!(nstd_std_env_current_dir, current_dir);

/// Frees memory allocated by `nstd_std_env_path_to_exe` or `nstd_std_env_current_dir`.
/// Parameters:
///     `char **path` - String from `nstd_std_env_path_to_exe` or `nstd_std_env_current_dir`.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_env_free_path(path: *mut *mut c_char) {
    CString::from_raw(*path);
    *path = ptr::null_mut();
}

/// Returns an array of strings that contain the cmd args that the program was started with.
/// Parameters:
///     `NSTDSize *size` - Number of args.
/// Returns: `char *args` - The command line arguments.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_env_args(size: *mut usize) -> *mut c_char {
    let args = env::args().collect::<Vec<String>>();
    let mut bytes = Vec::<byte>::new();
    *size = args.len();
    for arg in args {
        bytes.extend(arg.into_bytes());
        bytes.push(0);
    }
    Box::<[byte]>::into_raw(bytes.into_boxed_slice()) as *mut c_char
}

/// Frees memory allocated by `nstd_std_env_args`.
/// Parameters:
///     `char **args` - Returned from `nstd_std_env_args`.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_env_free_args(args: *mut *mut c_char) {
    Box::from_raw(*args as *mut byte);
    *args = ptr::null_mut();
}
