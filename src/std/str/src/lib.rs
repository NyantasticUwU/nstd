use nstd_collections::{
    deps::nstd_alloc::deps::nstd_core::{char_types::NSTDUnichar, slice::NSTDSlice},
    vec::NSTDVec,
};
use std::{
    ffi::{CStr, CString},
    os::raw::*,
    ptr,
};
#[cfg(feature = "deps")]
pub mod deps {
    pub use nstd_collections;
}

/// Represents a dynamic-sized array of UTF-8 chars.
#[repr(C)]
#[derive(Default)]
pub struct NSTDString {
    pub bytes: NSTDVec,
}
impl<T> From<Vec<T>> for NSTDString {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        Self {
            bytes: NSTDVec::from(vec),
        }
    }
}

/// Creates a new `NSTDString` instance.
/// Returns: `NSTDString string` - The new string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_str_string_new() -> NSTDString {
    const BYTE_SIZE: usize = std::mem::size_of::<u8>();
    let bytes = nstd_collections::vec::nstd_std_collections_vec_new(BYTE_SIZE);
    NSTDString { bytes }
}

/// Creates a new `NSTDString` from a raw C string.
/// Parameters:
///     `const char *const cstr` - The C string.
/// Returns: `NSTDString string` - The new NSTD string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_str_string_from_cstring(cstr: *const c_char) -> NSTDString {
    NSTDString::from(CStr::from_ptr(cstr).to_bytes().to_vec())
}

/// Creates an `NSTDSlice` from an `NSTDString`.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDSlice slice` - The new slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_str_string_as_slice(string: &NSTDString) -> NSTDSlice {
    nstd_collections::vec::nstd_std_collections_vec_as_slice(&string.bytes)
}

/// Gets the length of a string.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDUSize len` - The length of the UTF-8 encoded string, -1 on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_str_string_len(string: &NSTDString) -> usize {
    let bytes = nstd_std_str_string_as_slice(string);
    match std::str::from_utf8(bytes.as_byte_slice()) {
        Ok(string) => string.chars().count(),
        _ => usize::MAX,
    }
}

/// Returns the number of bytes used by this string.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDUSize len` - The number of bytes in the string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_str_string_byte_len(string: &NSTDString) -> usize {
    string.bytes.size
}

/// Pushes an `NSTDUnichar` to an `NSTDString`.
/// Parameters:
///     `NSTDString *const string` - The string.
///     `const NSTDUnichar chr` - The unicode character to push to the string.
/// Returns: `int errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_str_string_push(
    string: &mut NSTDString,
    chr: NSTDUnichar,
) -> c_int {
    match char::from_u32(chr) {
        Some(chr) => {
            let mut bytes = [0u8; 4];
            chr.encode_utf8(&mut bytes);
            for i in 0..chr.len_utf8() {
                let byteptr = &bytes[i] as *const u8 as *const c_void;
                nstd_collections::vec::nstd_std_collections_vec_push(&mut string.bytes, byteptr);
            }
            0
        }
        _ => 1,
    }
}

/// Removes an `NSTDUnichar` from the end of an `NSTDString`.
/// Parameters:
///     `NSTDString *const string` - The string.
/// Returns: `NSTDUnichar chr` - The unichar that was popped off the string, fill char on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_str_string_pop(string: &mut NSTDString) -> NSTDUnichar {
    let bytes = nstd_collections::vec::nstd_std_collections_vec_as_slice(&string.bytes);
    match std::str::from_utf8(bytes.as_byte_slice()) {
        Ok(str) => match str.chars().rev().next() {
            Some(chr) => {
                for _ in 0..chr.len_utf8() {
                    nstd_collections::vec::nstd_std_collections_vec_pop(&mut string.bytes);
                }
                chr as NSTDUnichar
            }
            _ => char::REPLACEMENT_CHARACTER as NSTDUnichar,
        },
        _ => char::REPLACEMENT_CHARACTER as NSTDUnichar,
    }
}

/// Extends an `NSTDString` by an `NSTDSlice` of `NSTDUnichar`s.
/// Parameters:
///     `NSTDString *const string` - The string.
///     `const NSTDSlice chars` - `NSTDSlice` of `NSTDUnichar`s.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_str_string_extend(string: &mut NSTDString, chars: &NSTDSlice) {
    let mut ptr = chars.data;
    for _ in 0..chars.size {
        let chr = ptr as *const NSTDUnichar;
        nstd_std_str_string_push(string, *chr);
        ptr = ptr.add(chars.element_size);
    }
}

/// Frees an `NSTDString` instance.
/// Parameters:
///     `NSTDString *const string` - Pointer to a string.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_str_string_free(string: &mut NSTDString) -> c_int {
    nstd_collections::vec::nstd_std_collections_vec_free(&mut string.bytes)
}

/// Calculates a string's length.
/// Parameters:
///     `const char *const str` - The string.
/// Returns: `NSTDUSize len` - The length of the string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_str_len(str: *const c_char) -> usize {
    CStr::from_ptr(str).to_bytes().len()
}

/// Concatenates two strings.
/// Parameters:
///     `const char *const str1` - The first string.
///     `const char *const str2` - The second string.
/// Returns: `char *str` - The new string, null on error.
#[cfg_attr(feature = "clib", no_mangle)]
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
///     `const char **str` - The string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_str_free_concat(str: *mut *mut c_char) {
    drop(CString::from_raw(*str));
    *str = ptr::null_mut();
}

/// Compares two strings.
/// Parameters:
///     `const char *const str1` - The first string to compare.
///     `const char *const str2` - The second string to compare.
/// Returns: `int e` - Nonzero if the two strings are lexicographically equal.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_str_compare(str1: *const c_char, str2: *const c_char) -> c_int {
    (CStr::from_ptr(str1) == CStr::from_ptr(str2)) as c_int
}

/// Generates a function that checks a cstring pattern with another cstring.
macro_rules! nstd_pattern_check_cstr_cstr {
    ($name: ident, $method_name: ident) => {
        #[cfg_attr(feature = "clib", no_mangle)]
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
        #[cfg_attr(feature = "clib", no_mangle)]
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
        #[cfg_attr(feature = "clib", no_mangle)]
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
        #[cfg_attr(feature = "clib", no_mangle)]
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
        #[cfg_attr(feature = "clib", no_mangle)]
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
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(num: $type) -> NSTDString {
            NSTDString::from(num.to_string().into_bytes())
        }
    };
}
nstd_from_ctype!(nstd_std_str_string_from_float, c_float);
nstd_from_ctype!(nstd_std_str_string_from_double, c_double);
nstd_from_ctype!(nstd_std_str_string_from_schar, c_schar);
nstd_from_ctype!(nstd_std_str_string_from_uchar, c_uchar);
nstd_from_ctype!(nstd_std_str_string_from_short, c_short);
nstd_from_ctype!(nstd_std_str_string_from_ushort, c_ushort);
nstd_from_ctype!(nstd_std_str_string_from_int, c_int);
nstd_from_ctype!(nstd_std_str_string_from_uint, c_uint);
nstd_from_ctype!(nstd_std_str_string_from_long, c_long);
nstd_from_ctype!(nstd_std_str_string_from_ulong, c_ulong);
nstd_from_ctype!(nstd_std_str_string_from_longlong, c_longlong);
nstd_from_ctype!(nstd_std_str_string_from_ulonglong, c_ulonglong);
nstd_from_ctype!(nstd_std_str_string_from_isize, isize);
nstd_from_ctype!(nstd_std_str_string_from_usize, usize);
