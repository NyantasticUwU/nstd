use std::{
    env::consts::{ARCH, OS},
    ffi::CString,
    os::raw::c_char,
    ptr,
};

/// Returns a string describing the specific operating system in use.
/// `nstd_os_free_name` must be called to free memory allocated by this function.
/// Returns: `char *os_name` - The os's name as a string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_name() -> *mut c_char {
    static_nstd_create_cstr(OS)
}

/// Frees memory allocated by `nstd_os_name`.
/// Parameters:
///     `const char **os_name` - Pointer to the os name cstr.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_free_name(os_name: *mut *mut c_char) {
    static_nstd_deallocate_cstr(os_name);
}

/// Returns a string describing the specific cpu architecture in use.
/// `nstd_os_free_arch_name` must be called to free memory allocated by this function.
/// Returns: `char *arch_name` - The cpu architecture's name as a string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_arch_name() -> *mut c_char {
    static_nstd_create_cstr(ARCH)
}

/// Frees memory allocated by `nstd_os_arch_name`.
/// Parameters:
///     `const char **arch_name` - Pointer to the arch name cstr.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_free_arch_name(arch_name: *mut *mut c_char) {
    static_nstd_deallocate_cstr(arch_name);
}

/// Creates a raw `*mut c_char` from a `&str`.
/// Parameters:
///     `rstr: &str` - Rust string slice to convert.
/// Returns: `cstr: *mut c_char` - The cstring version of `rstr`.
#[inline]
unsafe fn static_nstd_create_cstr(rstr: &str) -> *mut c_char {
    let mut bytes = String::from(rstr).into_bytes();
    bytes.push(0);
    CString::from_vec_unchecked(bytes).into_raw() as *mut c_char
}

/// Frees heap allocated rust c-string.
/// Parameters:
///     `cstr: *mut *mut c_char` - C-string to deallocate.
#[inline]
unsafe fn static_nstd_deallocate_cstr(cstr: *mut *mut c_char) {
    drop(CString::from_raw(*cstr));
    *cstr = ptr::null_mut();
}
