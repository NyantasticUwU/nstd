use crate::core::def::{NSTDAny, NSTDBool, NSTDChar};

/// Returns the length (in bytes) of a null terminated C string.
/// Parameters:
///     `const NSTDChar *const cstr` - The C string.
/// Returns: `NSTDUSize len` - The length of the C string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_cstr_len(cstr: *const NSTDChar) -> usize {
    let mut len = 0;
    while {
        len += 1;
        *cstr.add(len) != 0
    } {}
    len
}

/// Compares two C strings and returns `NSTD_BOOL_TRUE` if they contain the same data.
/// Parameters:
///     `const NSTDChar *const cstr1` - The first C string.
///     `const NSTDChar *const cstr2` - The second C string.
/// Returns: `NSTDBool is_eq` - `NSTD_BOOL_TRUE` if the two strings are lexicographically equal.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_str_cstr_compare(
    cstr1: *const NSTDChar,
    cstr2: *const NSTDChar,
) -> NSTDBool {
    const C_CHAR_SIZE: usize = core::mem::size_of::<NSTDChar>();
    let cstr1_len = nstd_core_str_cstr_len(cstr1);
    let cstr1 = crate::core::slice::nstd_core_slice_new(cstr1_len, C_CHAR_SIZE, cstr1 as NSTDAny);
    let cstr2_len = nstd_core_str_cstr_len(cstr2);
    let cstr2 = crate::core::slice::nstd_core_slice_new(cstr2_len, C_CHAR_SIZE, cstr2 as NSTDAny);
    crate::core::slice::nstd_core_slice_compare(&cstr1, &cstr2)
}
