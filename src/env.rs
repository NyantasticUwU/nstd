use crate::{
    collections::vec::*,
    core::{def::NSTDChar, str::NSTDStr},
    string::NSTDString,
};
use std::{
    ffi::CStr,
    os::raw::{c_char, c_int},
    ptr::addr_of_mut,
};

#[allow(non_camel_case_types)]
type byte = u8;

/// Generates `nstd_env_path_to_exe` and `nstd_env_current_dir` functions.
macro_rules! nstd_path_fns {
    ($fn_name: ident, $env_fn: ident) => {
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $fn_name(errc: *mut c_int) -> NSTDString {
            match std::env::$env_fn() {
                Ok(path) => {
                    *errc = 0;
                    NSTDString::from(path.to_string_lossy().to_string().into_bytes())
                }
                _ => {
                    *errc = 1;
                    null_string()
                }
            }
        }
    };
}
nstd_path_fns!(nstd_env_path_to_exe, current_exe);
nstd_path_fns!(nstd_env_current_dir, current_dir);

/// Returns the path of a temporary directory.
/// Returns: `NSTDString path` - The path of the temp dir.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_temp_dir() -> NSTDString {
    match std::env::temp_dir().into_os_string().into_string() {
        Ok(path) => NSTDString::from(path.into_bytes()),
        _ => null_string(),
    }
}

/// Sets the current working directory.
/// Parameters:
///     `const NSTDStr *const path` - The new working directory.
/// Returns: `int errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_set_current_dir(path: &NSTDStr) -> c_int {
    match std::str::from_utf8(path.bytes.as_byte_slice()) {
        Ok(path) => match std::env::set_current_dir(path) {
            Ok(_) => 0,
            _ => 1,
        },
        _ => 1,
    }
}

/// Returns a vector of strings that contain the cmd args that the program was started with.
/// Returns: `NSTDVec args` - A vector of `NSTDString`.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_args() -> NSTDVec {
    const ELEMENT_SIZE: usize = std::mem::size_of::<NSTDString>();
    let args_iter = std::env::args().collect::<Vec<String>>();
    let mut args = nstd_collections_vec_new_with_capacity(ELEMENT_SIZE, args_iter.len());
    if !args.buffer.ptr.raw.is_null() {
        for arg in args_iter {
            let mut string = NSTDString::from(arg.into_bytes());
            let string_ptr = addr_of_mut!(string).cast();
            nstd_collections_vec_push(&mut args, string_ptr);
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
        let stringptr = &mut *(nstd_collections_vec_get(args, i) as *mut NSTDString);
        crate::string::nstd_string_free(stringptr);
    }
    nstd_collections_vec_free(args)
}

/// Sets an environment variable.
/// Parameters:
///     `const NSTDChar *const k` - The var key.
///     `const NSTDChar *const v` - The var value.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_set_var(k: *const NSTDChar, v: *const NSTDChar) {
    if let Ok(k) = CStr::from_ptr(k).to_str() {
        if let Ok(v) = CStr::from_ptr(v).to_str() {
            std::env::set_var(k, v);
        }
    }
}

/// Gets an environment variable.
/// Parameters:
///     `const NSTDChar *const k` - The var key.
/// Returns: `NSTDString v` - The value of the variable.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_get_var(k: *const NSTDChar) -> NSTDString {
    if let Ok(k) = CStr::from_ptr(k).to_str() {
        if let Ok(v) = std::env::var(k) {
            return NSTDString::from(v.into_bytes());
        }
    }
    null_string()
}

/// Removes an environment variable.
/// This will not free memory allocated by `nstd_env_get_var`.
/// Parameters:
///     `const char *const k` - The var key.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_remove_var(k: *const c_char) {
    if let Ok(k) = CStr::from_ptr(k).to_str() {
        std::env::remove_var(k);
    }
}

/// Returns an array of strings that contain the environment variables.
/// Parameters:
///     `NSTDUSize *size` - Number of variables.
/// Returns: `char *vars` - The environment variables keys.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_env_vars(size: *mut usize) -> *mut c_char {
    let vars = std::env::vars().collect::<Vec<(String, String)>>();
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
    *cstr = std::ptr::null_mut();
}

/// Creates a null `NSTDString`.
#[inline]
unsafe fn null_string() -> NSTDString {
    let null = crate::core::slice::nstd_core_slice_new(0, 0, std::ptr::null_mut());
    let null = nstd_collections_vec_from_existing(0, &null);
    crate::string::nstd_string_from_existing(&null)
}
