pub mod cstr;
use crate::core::{
    def::{NSTDAny, NSTDBool, NSTDChar, NSTDErrorCode},
    range::NSTDURange,
    slice::NSTDSlice,
};

/// Represents a view into an array of UTF-8 chars.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NSTDStr {
    /// UTF-8 encoded bytes.
    pub bytes: NSTDSlice,
}

/// Creates a new `NSTDStr` from a cstring.
/// Parameters:
///     `const NSTDChar *const cstr` - The cstring.
/// Returns: `NSTDStr str` - The new string slice, excluding the null terminator.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_from_cstring(cstring: *const NSTDChar) -> NSTDStr {
    const C_CHAR_SIZE: usize = core::mem::size_of::<NSTDChar>();
    let mut size = 0;
    while *cstring.add(size) != 0 {
        size += 1;
    }
    NSTDStr {
        bytes: crate::core::slice::nstd_core_slice_new(size, C_CHAR_SIZE, cstring as NSTDAny),
    }
}

/// Creates a new `NSTDStr` from a cstring.
/// Parameters:
///     `const NSTDChar *const cstr` - The cstring.
/// Returns: `NSTDStr str` - The new string slice, including the null terminator.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_from_cstring_with_null(cstring: *const NSTDChar) -> NSTDStr {
    let mut str = nstd_core_str_from_cstring(cstring);
    str.bytes.size += 1;
    str
}

/// Creates a new `NSTDStr` from an `NSTDSlice`. `slice->element_size` must be the size of one byte.
/// Parameters:
///     `const NSTDSlice *const slice` - The UTF-8 encoded byte slice.
/// Returns: `NSTDStr str` - The new string slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_from_bytes(slice: &NSTDSlice) -> NSTDStr {
    NSTDStr {
        bytes: if slice.ptr.size == 1 {
            *slice
        } else {
            crate::core::slice::nstd_core_slice_new(0, 0, core::ptr::null_mut())
        },
    }
}

/// Gets the length of a string slice.
/// Parameters:
///     `const NSTDStr *const str` - The string slice.
/// Returns: `NSTDUSize len` - The length of the UTF-8 encoded string slice, -1 on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_len(str: &NSTDStr) -> usize {
    match core::str::from_utf8(str.bytes.as_byte_slice()) {
        Ok(str) => str.chars().count(),
        _ => usize::MAX,
    }
}

/// Returns the number of bytes used by this string slice.
/// Parameters:
///     `const NSTDStr *const str` - The string slice.
/// Returns: `NSTDUSize len` - The number of bytes in the string slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_byte_len(str: &NSTDStr) -> usize {
    str.bytes.byte_count()
}

/// Returns a subslice of `str` based on `range`.
/// Parameters:
///     `const NSTDStr *const str` - The string slice.
///     `const NSTDURange *const range` - The range of bytes to make a subslice out of.
/// Returns: `NSTDStr subslice` - The string subslice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_get(str: &NSTDStr, range: &NSTDURange) -> NSTDStr {
    let len = range.end - range.start;
    let start = str.bytes.ptr.raw.add(range.start);
    let slice = crate::core::slice::nstd_core_slice_new(len, str.bytes.ptr.size, start);
    nstd_core_str_from_bytes(&slice)
}

/// Checks if a string slice is entirely ASCII.
/// Parameters:
///     `const NSTDStr *const str` - The string slice.
/// Returns: `NSTDBool is_ascii` - True if the string slice is entirely ASCII.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_is_ascii(str: &NSTDStr) -> NSTDBool {
    match core::str::from_utf8(str.bytes.as_byte_slice()) {
        Ok(str) => NSTDBool::from(str.is_ascii()),
        _ => NSTDBool::NSTD_BOOL_FALSE,
    }
}

/// Compares two string slices.
/// Parameters:
///     `const NSTDStr *const str1` - The first string slice.
///     `const NSTDStr *const str2` - The second string slice.
/// Returns: `NSTDBool is_eq` - True if the two slices are equal.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_compare(str1: &NSTDStr, str2: &NSTDStr) -> NSTDBool {
    NSTDBool::from(str1.bytes.as_byte_slice() == str2.bytes.as_byte_slice())
}

/// Generates pattern checking functions.
macro_rules! nstd_str_pat_check {
    ($fn_name: ident, $method: ident) => {
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $fn_name(str: &NSTDStr, pattern: &NSTDStr) -> NSTDBool {
            let str = str.bytes.as_byte_slice();
            let pattern = pattern.bytes.as_byte_slice();
            match (core::str::from_utf8(str), core::str::from_utf8(pattern)) {
                (Ok(str), Ok(pattern)) => NSTDBool::from(str.$method(pattern)),
                _ => NSTDBool::NSTD_BOOL_FALSE,
            }
        }
    };
}
nstd_str_pat_check!(nstd_core_str_contains, contains);
nstd_str_pat_check!(nstd_core_str_starts_with, starts_with);
nstd_str_pat_check!(nstd_core_str_ends_with, ends_with);

/// Generates pattern finding functions.
macro_rules! nstd_str_find {
    ($fn_name: ident, $method: ident) => {
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $fn_name(str: &NSTDStr, pattern: &NSTDStr) -> usize {
            let str = str.bytes.as_byte_slice();
            let pattern = pattern.bytes.as_byte_slice();
            match (core::str::from_utf8(str), core::str::from_utf8(pattern)) {
                (Ok(str), Ok(pattern)) => str.$method(pattern).unwrap_or(usize::MAX),
                _ => usize::MAX,
            }
        }
    };
}
nstd_str_find!(nstd_core_str_find, find);
nstd_str_find!(nstd_core_str_find_last, rfind);

/// Generates `nstd_core_str_to_*case` functions.
macro_rules! nstd_core_str_to_case {
    ($name: ident, $method: ident) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(str: &mut NSTDStr) -> NSTDErrorCode {
            match core::str::from_utf8_mut(str.bytes.as_byte_slice_mut()) {
                Ok(str) => {
                    str.$method();
                    0
                }
                _ => 1,
            }
        }
    };
}
nstd_core_str_to_case!(nstd_core_str_to_uppercase, make_ascii_uppercase);
nstd_core_str_to_case!(nstd_core_str_to_lowercase, make_ascii_lowercase);

/// Generates str => number conversion functions.
macro_rules! nstd_str_to_num {
    ($name: ident, $type: ty) => {
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(str: &NSTDStr, is_err: &mut NSTDErrorCode) -> $type {
            if let Ok(str) = core::str::from_utf8(str.bytes.as_byte_slice()) {
                if let Ok(v) = str.parse::<$type>() {
                    *is_err = 0;
                    return v;
                }
            }
            *is_err = 1;
            <$type>::default()
        }
    };
}
nstd_str_to_num!(nstd_core_str_to_f32, f32);
nstd_str_to_num!(nstd_core_str_to_f64, f64);
nstd_str_to_num!(nstd_core_str_to_i8, i8);
nstd_str_to_num!(nstd_core_str_to_u8, u8);
nstd_str_to_num!(nstd_core_str_to_i16, i16);
nstd_str_to_num!(nstd_core_str_to_u16, u16);
nstd_str_to_num!(nstd_core_str_to_i32, i32);
nstd_str_to_num!(nstd_core_str_to_u32, u32);
nstd_str_to_num!(nstd_core_str_to_i64, i64);
nstd_str_to_num!(nstd_core_str_to_u64, u64);
nstd_str_to_num!(nstd_core_str_to_isize, isize);
nstd_str_to_num!(nstd_core_str_to_usize, usize);
