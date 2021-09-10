use std::{
    ffi::{CStr, CString},
    os::raw::*,
    ptr,
};

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
        pub unsafe extern "C" fn $name(num: $type) -> *const c_char {
            match CString::new(num.to_string().into_bytes()) {
                Ok(cstr) => cstr.into_raw(),
                _ => ptr::null(),
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
