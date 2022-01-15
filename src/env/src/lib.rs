use nstd_collections::vec::*;
use std::{
    env,
    ffi::{CStr, CString},
    os::raw::{c_char, c_int, c_void},
    ptr,
};
#[cfg(feature = "deps")]
pub mod deps {
    pub use nstd_collections;
}

#[allow(non_camel_case_types)]
type byte = u8;

/// Generates `nstd_env_path_to_exe` and `nstd_env_current_dir` functions.
macro_rules! nstd_path_fns {
    ($fn_name: ident, $env_fn: ident) => {
        #[cfg_attr(feature = "clib", no_mangle)]
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
nstd_path_fns!(nstd_env_path_to_exe, current_exe);
nstd_path_fns!(nstd_env_current_dir, current_dir);

/// Returns the path of a temporary directory.
/// Use `nstd_env_free_path` to free memory allocated by this function.
/// Returns: `char *path` - The path of the temp dir.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_temp_dir() -> *mut c_char {
    match env::temp_dir().into_os_string().into_string() {
        Ok(path) => CString::from_vec_unchecked(path.into_bytes()).into_raw(),
        _ => ptr::null_mut(),
    }
}

/// Frees memory allocated by `nstd_env_path_to_exe`,  `nstd_env_current_dir` or
/// `nstd_env_temp_dir`.
/// Parameters:
///     `char **path` - String from `nstd_env_path_to_exe` or `nstd_env_current_dir`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_free_path(path: *mut *mut c_char) {
    static_nstd_free_cstring(path);
}

/// Sets the current working directory.
/// Parameters:
///     `const char *const path` - The new working directory.
/// Returns: `int errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_set_current_dir(path: *const c_char) -> c_int {
    match CStr::from_ptr(path).to_str() {
        Ok(path) => match env::set_current_dir(path) {
            Ok(_) => 0,
            _ => 1,
        },
        _ => 1,
    }
}

/// Returns a vector of strings that contain the cmd args that the program was started with.
/// Returns: `NSTDVec args` - The command line arguments.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_args() -> NSTDVec {
    const ELEMENT_SIZE: usize = std::mem::size_of::<*mut c_char>();
    let args_iter = env::args().collect::<Vec<String>>();
    let mut args = nstd_collections_vec_new_with_capacity(ELEMENT_SIZE, args_iter.len());
    if !args.data.is_null() {
        for arg in args_iter {
            let mut bytes = arg.into_bytes();
            bytes.push(0);
            let cstr = CString::from_vec_unchecked(bytes).into_raw();
            let cstrptr = &cstr as *const *mut c_char as *const c_void;
            nstd_collections_vec_push(&mut args, cstrptr);
        }
    }
    args
}

/// Frees memory allocated by `nstd_env_args`.
/// Parameters:
///     `NSTDVec *const args` - Returned from `nstd_env_args`.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_free_args(args: &mut NSTDVec) -> c_int {
    for i in 0..args.size {
        let cstrptr = nstd_collections_vec_get(args, i) as *const *mut c_char;
        if !cstrptr.is_null() {
            drop(CString::from_raw(*cstrptr));
        }
    }
    nstd_collections_vec_free(args)
}

/// Sets an environment variable.
/// Parameters:
///     `const char *const k` - The var key.
///     `const char *const v` - The var value.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_set_var(k: *const c_char, v: *const c_char) {
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
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_get_var(k: *const c_char) -> *mut c_char {
    if let Ok(k) = CStr::from_ptr(k).to_str() {
        if let Ok(v) = env::var(k) {
            return CString::from_vec_unchecked(v.into_bytes()).into_raw();
        }
    }
    ptr::null_mut()
}

/// Removes an environment variable.
/// This will not free memory allocated by `nstd_env_get_var`.
/// Parameters:
///     `const char *const k` - The var key.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_remove_var(k: *const c_char) {
    if let Ok(k) = CStr::from_ptr(k).to_str() {
        env::remove_var(k);
    }
}

/// Frees memory allocated by `nstd_env_get_var`.
/// Parameters:
///     `char **v` - The value returned from `nstd_env_get_var`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_free_var(k: *mut *mut c_char) {
    static_nstd_free_cstring(k);
}

/// Returns an array of strings that contain the environment variables.
/// Parameters:
///     `NSTDUSize *size` - Number of variables.
/// Returns: `char *vars` - The environment variables keys.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_vars(size: *mut usize) -> *mut c_char {
    let vars = env::vars().collect::<Vec<(String, String)>>();
    let mut bytes = Vec::<byte>::new();
    *size = vars.len();
    for var in vars {
        bytes.extend(var.0.into_bytes());
        bytes.push(0);
    }
    Box::<[byte]>::into_raw(bytes.into_boxed_slice()) as *mut c_char
}

/// Frees memory allocated by `nstd_env_vars`.
/// Parameters:
///     `char **vars` - Returned from `nstd_env_vars`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_free_vars(vars: *mut *mut c_char) {
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
