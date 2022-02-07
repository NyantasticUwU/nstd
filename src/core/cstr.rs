use crate::core::{
    def::{NSTDAny, NSTDBool, NSTDChar},
    slice::NSTDSlice,
};

/// Creates a slice over a C string.
/// Parameters:
///     `const NSTDChar *const cstr` - The C string.
/// Returns: `NSTDSlice slice` - A slice representing the C string's data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_as_slice(cstr: *const NSTDChar) -> NSTDSlice {
    const C_CHAR_SIZE: usize = core::mem::size_of::<NSTDChar>();
    let len = nstd_core_cstr_len(cstr);
    crate::core::slice::nstd_core_slice_new(len, C_CHAR_SIZE, cstr as NSTDAny)
}

/// Returns the length (in bytes) of a null terminated C string.
/// Parameters:
///     `const NSTDChar *const cstr` - The C string.
/// Returns: `NSTDUSize len` - The length of the C string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_len(cstr: *const NSTDChar) -> usize {
    let mut len = 0;
    while {
        len += 1;
        *cstr.add(len) != 0
    } {}
    len
}

/// Compares two C strings and returns `NSTD_BOOL_TRUE` if they contain the same data.
/// Parameters:
///     `const NSTDChar *cstr1` - The first C string.
///     `const NSTDChar *cstr2` - The second C string.
/// Returns: `NSTDBool is_eq` - `NSTD_BOOL_TRUE` if the two strings are lexicographically equal.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_cstr_compare(
    mut cstr1: *const NSTDChar,
    mut cstr2: *const NSTDChar,
) -> NSTDBool {
    loop {
        if *cstr1 == 0 && *cstr2 == 0 {
            break NSTDBool::NSTD_BOOL_TRUE;
        } else if *cstr1 != *cstr2 {
            break NSTDBool::NSTD_BOOL_FALSE;
        }
        cstr1 = cstr1.add(1);
        cstr2 = cstr2.add(1);
    }
}
