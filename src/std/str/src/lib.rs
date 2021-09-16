use std::{
    ffi::{CStr, CString},
    os::raw::*,
    ptr,
};

/// Calculates a string's length.
/// Parameters:
///     `const char *const str` - The string.
/// Returns: `NSTDSize len` - The length of the string.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_str_len(str: *const c_char) -> usize {
    CStr::from_ptr(str).to_bytes().len()
}

/// Concatenates two strings.
/// Parameters:
///     `const char *const str1` - The first string.
///     `const char *const str2` - The second string.
/// Returns: `char *str` - The new string, null on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_str_concat(
    str1: *const c_char,
    str2: *const c_char,
) -> *mut c_char {
    let mut bytes = Vec::<u8>::new();
    bytes.extend_from_slice(CStr::from_ptr(str1).to_bytes());
    bytes.extend_from_slice(CStr::from_ptr(str2).to_bytes());
    bytes.push(0);
    CString::from_vec_unchecked(bytes).into_raw()
}

/// Frees memory allocated by `nstd_std_str_concat`.
/// Parameters:
///     `char **str` - The string.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_str_free_concat(str: *mut *mut c_char) {
    CString::from_raw(*str);
    *str = ptr::null_mut();
}

/// Compares two strings.
/// Parameters:
///     `const char *const str1` - The first string to compare.
///     `const char *const str2` - The second string to compare.
/// Returns: `int e` - Nonzero if the two strings are lexicographically equal.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_str_compare(str1: *const c_char, str2: *const c_char) -> c_int {
    (CStr::from_ptr(str1) == CStr::from_ptr(str2)) as c_int
}

/// Generates a function that checks a cstring pattern with another cstring.
macro_rules! nstd_pattern_check_cstr_cstr {
    ($name: ident, $method_name: ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(str: *const c_char, pattern: *const c_char) -> c_int {
            if let Ok(str) = CStr::from_ptr(str).to_str() {
                if let Ok(pattern) = CStr::from_ptr(pattern).to_str() {
                    return str.$method_name(pattern) as c_int;
                }
            }
            0
        }
    };
}
nstd_pattern_check_cstr_cstr!(nstd_std_str_contains, contains);
nstd_pattern_check_cstr_cstr!(nstd_std_str_starts_with, starts_with);
nstd_pattern_check_cstr_cstr!(nstd_std_str_ends_with, ends_with);
/// Generates C string find/rfind functions.
macro_rules! nstd_find_cstr_cstr {
    ($name: ident, $method_name: ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(str: *const c_char, pattern: *const c_char) -> usize {
            if let Ok(str) = CStr::from_ptr(str).to_str() {
                if let Ok(pattern) = CStr::from_ptr(pattern).to_str() {
                    if let Some(pos) = str.$method_name(pattern) {
                        return pos;
                    }
                }
            }
            usize::MAX
        }
    };
}
nstd_find_cstr_cstr!(nstd_std_str_find, find);
nstd_find_cstr_cstr!(nstd_std_str_find_last, rfind);

/// Generates a function that checks a cstring pattern with a c char.
macro_rules! nstd_pattern_check_cstr_cchar {
    ($name: ident, $method_name: ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(str: *const c_char, pattern: c_char) -> c_int {
            if let Ok(str) = CStr::from_ptr(str).to_str() {
                return str.$method_name(pattern as u8 as char) as c_int;
            }
            0
        }
    };
}
nstd_pattern_check_cstr_cchar!(nstd_std_str_contains_char, contains);
nstd_pattern_check_cstr_cchar!(nstd_std_str_starts_with_char, starts_with);
nstd_pattern_check_cstr_cchar!(nstd_std_str_ends_with_char, ends_with);
/// Generates C string find/rfind functions.
macro_rules! nstd_find_cstr_cchar {
    ($name: ident, $method_name: ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(str: *const c_char, pattern: c_char) -> usize {
            if let Ok(str) = CStr::from_ptr(str).to_str() {
                if let Some(pos) = str.$method_name(pattern as u8 as char) {
                    return pos;
                }
            }
            usize::MAX
        }
    };
}
nstd_find_cstr_cchar!(nstd_std_str_find_char, find);
nstd_find_cstr_cchar!(nstd_std_str_find_last_char, rfind);

/// Generates string to ctype conversions.
macro_rules! nstd_to_ctype {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(str: *const c_char, errc: *mut c_int) -> $type {
            if let Ok(str) = CStr::from_ptr(str).to_str() {
                if let Ok(v) = str.parse::<$type>() {
                    *errc = 0;
                    return v;
                }
            }
            *errc = 1;
            <$type>::default()
        }
    };
}
nstd_to_ctype!(nstd_std_str_to_float, c_float);
nstd_to_ctype!(nstd_std_str_to_double, c_double);
nstd_to_ctype!(nstd_std_str_to_schar, c_schar);
nstd_to_ctype!(nstd_std_str_to_uchar, c_uchar);
nstd_to_ctype!(nstd_std_str_to_short, c_short);
nstd_to_ctype!(nstd_std_str_to_ushort, c_ushort);
nstd_to_ctype!(nstd_std_str_to_int, c_int);
nstd_to_ctype!(nstd_std_str_to_uint, c_uint);
nstd_to_ctype!(nstd_std_str_to_long, c_long);
nstd_to_ctype!(nstd_std_str_to_ulong, c_ulong);
nstd_to_ctype!(nstd_std_str_to_longlong, c_longlong);
nstd_to_ctype!(nstd_std_str_to_ulonglong, c_ulonglong);

/// Generates string to ctype conversions.
macro_rules! nstd_from_ctype {
    ($name: ident, $type: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(num: $type) -> *mut c_char {
            match CString::new(num.to_string().into_bytes()) {
                Ok(cstr) => cstr.into_raw(),
                _ => ptr::null_mut(),
            }
        }
    };
}
nstd_from_ctype!(nstd_std_str_from_float, c_float);
nstd_from_ctype!(nstd_std_str_from_double, c_double);
nstd_from_ctype!(nstd_std_str_from_schar, c_schar);
nstd_from_ctype!(nstd_std_str_from_uchar, c_uchar);
nstd_from_ctype!(nstd_std_str_from_short, c_short);
nstd_from_ctype!(nstd_std_str_from_ushort, c_ushort);
nstd_from_ctype!(nstd_std_str_from_int, c_int);
nstd_from_ctype!(nstd_std_str_from_uint, c_uint);
nstd_from_ctype!(nstd_std_str_from_long, c_long);
nstd_from_ctype!(nstd_std_str_from_ulong, c_ulong);
nstd_from_ctype!(nstd_std_str_from_longlong, c_longlong);
nstd_from_ctype!(nstd_std_str_from_ulonglong, c_ulonglong);

/// Frees a string allocated by `nstd_std_str_from_*`.
/// Parameters:
///     `const char **str` - Pointer to the character string.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_str_free_from(str: *mut *mut c_char) {
    CString::from_raw(*str);
    *str = ptr::null_mut();
}
