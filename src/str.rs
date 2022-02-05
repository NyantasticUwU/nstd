use crate::{
    collections::vec::NSTDVec,
    core::{def::NSTDUnichar, slice::NSTDSlice, str::NSTDStr},
};
use std::{ffi::CStr, os::raw::*};

/// Represents a dynamic-sized array of UTF-8 chars.
#[repr(C)]
#[derive(Default)]
pub struct NSTDString {
    pub bytes: NSTDVec,
}
impl<T: Copy> From<Vec<T>> for NSTDString {
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
pub unsafe extern "C" fn nstd_str_string_new() -> NSTDString {
    const BYTE_SIZE: usize = std::mem::size_of::<u8>();
    let bytes = crate::collections::vec::nstd_collections_vec_new(BYTE_SIZE);
    NSTDString { bytes }
}

/// Creates a new `NSTDString` from a raw C string.
/// Parameters:
///     `const char *const cstr` - The C string.
/// Returns: `NSTDString string` - The new NSTD string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_str_string_from_cstring(cstr: *const c_char) -> NSTDString {
    NSTDString::from(CStr::from_ptr(cstr).to_bytes().to_vec())
}

/// Creates a string view from an `NSTDString`.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDStr str` - The new string view.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_str_string_as_str(string: &NSTDString) -> NSTDStr {
    crate::core::str::nstd_core_str_from_bytes(&nstd_str_string_as_slice(string))
}

/// Creates an `NSTDSlice` from an `NSTDString`.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDSlice slice` - The new slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_str_string_as_slice(string: &NSTDString) -> NSTDSlice {
    crate::collections::vec::nstd_collections_vec_as_slice(&string.bytes)
}

/// Gets the length of a string.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDUSize len` - The length of the UTF-8 encoded string, -1 on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_str_string_len(string: &NSTDString) -> usize {
    let bytes = nstd_str_string_as_slice(string);
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
pub unsafe extern "C" fn nstd_str_string_byte_len(string: &NSTDString) -> usize {
    string.bytes.size
}

/// Pushes an `NSTDUnichar` to an `NSTDString`.
/// Parameters:
///     `NSTDString *const string` - The string.
///     `const NSTDUnichar chr` - The unicode character to push to the string.
/// Returns: `int errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_str_string_push(string: &mut NSTDString, chr: NSTDUnichar) -> c_int {
    match char::from_u32(chr) {
        Some(chr) => {
            let mut bytes = [0u8; 4];
            chr.encode_utf8(&mut bytes);
            for i in 0..chr.len_utf8() {
                let byteptr = &bytes[i] as *const u8 as *const c_void;
                crate::collections::vec::nstd_collections_vec_push(&mut string.bytes, byteptr);
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
pub unsafe extern "C" fn nstd_str_string_pop(string: &mut NSTDString) -> NSTDUnichar {
    let bytes = crate::collections::vec::nstd_collections_vec_as_slice(&string.bytes);
    match std::str::from_utf8(bytes.as_byte_slice()) {
        Ok(str) => match str.chars().rev().next() {
            Some(chr) => {
                for _ in 0..chr.len_utf8() {
                    crate::collections::vec::nstd_collections_vec_pop(&mut string.bytes);
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
pub unsafe extern "C" fn nstd_str_string_extend(string: &mut NSTDString, chars: &NSTDSlice) {
    let mut ptr = chars.ptr.raw;
    for _ in 0..chars.size {
        let chr = ptr as *const NSTDUnichar;
        nstd_str_string_push(string, *chr);
        ptr = ptr.add(chars.ptr.size);
    }
}

/// Generates string to ctype conversions.
macro_rules! nstd_from_ctype {
    ($name: ident, $type: ty) => {
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(num: $type) -> NSTDString {
            NSTDString::from(num.to_string().into_bytes())
        }
    };
}
nstd_from_ctype!(nstd_str_string_from_float, c_float);
nstd_from_ctype!(nstd_str_string_from_double, c_double);
nstd_from_ctype!(nstd_str_string_from_schar, c_schar);
nstd_from_ctype!(nstd_str_string_from_uchar, c_uchar);
nstd_from_ctype!(nstd_str_string_from_short, c_short);
nstd_from_ctype!(nstd_str_string_from_ushort, c_ushort);
nstd_from_ctype!(nstd_str_string_from_int, c_int);
nstd_from_ctype!(nstd_str_string_from_uint, c_uint);
nstd_from_ctype!(nstd_str_string_from_long, c_long);
nstd_from_ctype!(nstd_str_string_from_ulong, c_ulong);
nstd_from_ctype!(nstd_str_string_from_longlong, c_longlong);
nstd_from_ctype!(nstd_str_string_from_ulonglong, c_ulonglong);
nstd_from_ctype!(nstd_str_string_from_isize, isize);
nstd_from_ctype!(nstd_str_string_from_usize, usize);

/// Frees an `NSTDString` instance.
/// Parameters:
///     `NSTDString *const string` - Pointer to a string.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_str_string_free(string: &mut NSTDString) -> c_int {
    crate::collections::vec::nstd_collections_vec_free(&mut string.bytes)
}
