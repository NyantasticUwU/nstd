use std::{
    env,
    ffi::{CStr, CString},
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

/// Returns the path of a temporary directory.
/// Use `nstd_std_env_free_path` to free memory allocated by this function.
/// Returns: `char *path` - The path of the temp dir.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_env_temp_dir() -> *mut c_char {
    match env::temp_dir().into_os_string().into_string() {
        Ok(path) => CString::from_vec_unchecked(path.into_bytes()).into_raw(),
        _ => ptr::null_mut(),
    }
}

/// Frees memory allocated by `nstd_std_env_path_to_exe`,  `nstd_std_env_current_dir` or
/// `nstd_std_env_temp_dir`.
/// Parameters:
///     `char **path` - String from `nstd_std_env_path_to_exe` or `nstd_std_env_current_dir`.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_env_free_path(path: *mut *mut c_char) {
    static_nstd_free_cstring(path);
}

/// Sets the current working directory.
/// Parameters:
///     `const char *const path` - The new working directory.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_env_set_current_dir(path: *const c_char) -> c_int {
    match CStr::from_ptr(path).to_str() {
        Ok(path) => match env::set_current_dir(path) {
            Ok(_) => 0,
            _ => 1,
        },
        _ => 1,
    }
}

/// Returns an array of strings that contain the cmd args that the program was started with.
/// Parameters:
///     `NSTDUSize *size` - Number of args.
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
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_env_free_args(args: *mut *mut c_char) {
    static_nstd_free_cstring(args);
}

/// Sets an environment variable.
/// Parameters:
///     `const char *const k` - The var key.
///     `const char *const v` - The var value.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_env_set_var(k: *const c_char, v: *const c_char) {
    if let Ok(k) = CStr::from_ptr(k).to_str() {
        if let Ok(v) = CStr::from_ptr(v).to_str() {
            env::set_var(k, v);
        }
    }
}

/// Gets an environment variable.
/// Parameters:
///     `const char *const k` - The var key.
/// Returns: `char *v` - The value of the variable.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_env_get_var(k: *const c_char) -> *mut c_char {
    if let Ok(k) = CStr::from_ptr(k).to_str() {
        if let Ok(v) = env::var(k) {
            return CString::from_vec_unchecked(v.into_bytes()).into_raw();
        }
    }
    ptr::null_mut()
}

/// Removes an environment variable.
/// This will not free memory allocated by `nstd_std_env_get_var`.
/// Parameters:
///     `const char *const k` - The var key.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_env_remove_var(k: *const c_char) {
    if let Ok(k) = CStr::from_ptr(k).to_str() {
        env::remove_var(k);
    }
}

/// Frees memory allocated by `nstd_std_env_get_var`.
/// Parameters:
///     `char **v` - The value returned from `nstd_std_env_get_var`.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_env_free_var(k: *mut *mut c_char) {
    static_nstd_free_cstring(k);
}

/// Returns an array of strings that contain the environment variables.
/// Parameters:
///     `NSTDUSize *size` - Number of variables.
/// Returns: `char *vars` - The environment variables keys.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_env_vars(size: *mut usize) -> *mut c_char {
    let vars = env::vars().collect::<Vec<(String, String)>>();
    let mut bytes = Vec::<byte>::new();
    *size = vars.len();
    for var in vars {
        bytes.extend(var.0.into_bytes());
        bytes.push(0);
    }
    Box::<[byte]>::into_raw(bytes.into_boxed_slice()) as *mut c_char
}

/// Frees memory allocated by `nstd_std_env_vars`.
/// Parameters:
///     `char **vars` - Returned from `nstd_std_env_vars`.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_env_free_vars(vars: *mut *mut c_char) {
    static_nstd_free_cstring(vars);
}

/// Frees a cstring.
/// Parameters:
///     `cstr: *mut *mut c_char` - The cstring.
#[inline]
unsafe fn static_nstd_free_cstring(cstr: *mut *mut c_char) {
    Box::from_raw(*cstr as *mut byte);
    *cstr = ptr::null_mut();
}
