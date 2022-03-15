use crate::{
    core::{
        def::{NSTDChar, NSTDErrorCode, NSTDUnichar},
        slice::NSTDSlice,
        str::NSTDStr,
    },
    vec::NSTDVec,
};
use std::{ffi::CStr, ptr::addr_of};

/// Represents a dynamic-sized array of UTF-8 chars.
#[repr(C)]
#[derive(Clone, Debug, Default, Hash)]
pub struct NSTDString {
    /// The internal UTF-8 encoded buffer.
    pub bytes: NSTDVec,
}
impl<T: Copy> From<&[T]> for NSTDString {
    #[inline]
    fn from(vec: &[T]) -> Self {
        Self {
            bytes: NSTDVec::from(vec),
        }
    }
}

/// Creates a new `NSTDString` instance.
/// Returns: `NSTDString string` - The new string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_string_new() -> NSTDString {
    const BYTE_SIZE: usize = std::mem::size_of::<u8>();
    let bytes = crate::vec::nstd_vec_new(BYTE_SIZE);
    NSTDString { bytes }
}

/// Creates an `NSTDString` from existing data.
/// Parameters:
///     `const NSTDVec *const bytes` - The existing raw data.
/// Returns: `NSTDString string` - The new `NSTDString` object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_string_from_existing(bytes: &NSTDVec) -> NSTDString {
    let bytes = crate::vec::nstd_vec_from_existing(0, &bytes.buffer);
    NSTDString { bytes }
}

/// Creates a new `NSTDString` from a raw C string.
/// Parameters:
///     `const NSTDChar *const cstr` - The C string.
/// Returns: `NSTDString string` - The new NSTD string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_string_from_cstring(cstr: *const NSTDChar) -> NSTDString {
    NSTDString::from(CStr::from_ptr(cstr).to_bytes())
}

/// Creates a string view from an `NSTDString`.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDStr str` - The new string view.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_string_as_str(string: &NSTDString) -> NSTDStr {
    crate::core::str::nstd_core_str_from_bytes(&nstd_string_as_slice(string))
}

/// Creates an `NSTDSlice` from an `NSTDString`.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDSlice slice` - The new slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_string_as_slice(string: &NSTDString) -> NSTDSlice {
    crate::vec::nstd_vec_as_slice(&string.bytes)
}

/// Gets the length of a string.
/// Parameters:
///     `const NSTDString *const string` - The string.
/// Returns: `NSTDUSize len` - The length of the UTF-8 encoded string, -1 on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_string_len(string: &NSTDString) -> usize {
    let bytes = nstd_string_as_slice(string);
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
pub unsafe extern "C" fn nstd_string_byte_len(string: &NSTDString) -> usize {
    string.bytes.size
}

/// Pushes an `NSTDUnichar` to an `NSTDString`.
/// Parameters:
///     `NSTDString *const string` - The string.
///     `const NSTDUnichar chr` - The unicode character to push to the string.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_string_push(
    string: &mut NSTDString,
    chr: NSTDUnichar,
) -> NSTDErrorCode {
    if let Some(chr) = char::from_u32(chr) {
        let mut bytes = [0u8; 4];
        chr.encode_utf8(&mut bytes);
        for i in 0..chr.len_utf8() {
            let byteptr = addr_of!(bytes[i]).cast();
            crate::vec::nstd_vec_push(&mut string.bytes, byteptr);
        }
        return 0;
    }
    1
}

/// Removes an `NSTDUnichar` from the end of an `NSTDString`.
/// Parameters:
///     `NSTDString *const string` - The string.
/// Returns: `NSTDUnichar chr` - The unichar that was popped off the string, fill char on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_string_pop(string: &mut NSTDString) -> NSTDUnichar {
    let bytes = crate::vec::nstd_vec_as_slice(&string.bytes);
    if let Ok(str) = std::str::from_utf8(bytes.as_byte_slice()) {
        if let Some(chr) = str.chars().rev().next() {
            for _ in 0..chr.len_utf8() {
                crate::vec::nstd_vec_pop(&mut string.bytes);
            }
            return chr as NSTDUnichar;
        }
    }
    char::REPLACEMENT_CHARACTER as NSTDUnichar
}

/// Extends an `NSTDString` by an `NSTDSlice` of `NSTDUnichar`s.
/// Parameters:
///     `NSTDString *const string` - The string.
///     `const NSTDSlice chars` - `NSTDSlice` of `NSTDUnichar`s.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_string_extend(string: &mut NSTDString, chars: &NSTDSlice) {
    let mut ptr = chars.ptr.raw;
    for _ in 0..chars.size {
        let chr = ptr as *const NSTDUnichar;
        nstd_string_push(string, *chr);
        ptr = ptr.add(chars.ptr.size);
    }
}

/// Generates string to ctype conversions.
macro_rules! nstd_from_ctype {
    ($name: ident, $type: ty) => {
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(num: $type) -> NSTDString {
            NSTDString::from(num.to_string().as_bytes())
        }
    };
}
nstd_from_ctype!(nstd_string_from_f32, f32);
nstd_from_ctype!(nstd_string_from_f64, f64);
nstd_from_ctype!(nstd_string_from_i8, i8);
nstd_from_ctype!(nstd_string_from_u8, u8);
nstd_from_ctype!(nstd_string_from_i16, i16);
nstd_from_ctype!(nstd_string_from_u16, u16);
nstd_from_ctype!(nstd_string_from_i32, i32);
nstd_from_ctype!(nstd_string_from_u32, u32);
nstd_from_ctype!(nstd_string_from_i64, i64);
nstd_from_ctype!(nstd_string_from_u64, u64);
nstd_from_ctype!(nstd_string_from_isize, isize);
nstd_from_ctype!(nstd_string_from_usize, usize);

/// Frees an `NSTDString` instance.
/// Parameters:
///     `NSTDString *const string` - Pointer to a string.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_string_free(string: &mut NSTDString) -> NSTDErrorCode {
    crate::vec::nstd_vec_free(&mut string.bytes)
}
