/// Represents a unicode char type.
pub type NSTDCOREUnichar = u32;

macro_rules! check_char {
    ($name: ident, $method: ident) => {
        #[inline]
        #[no_mangle]
        pub unsafe extern "C" fn $name(chr: NSTDCOREUnichar) -> i32 {
            match char::from_u32(chr) {
                Some(chr) => chr.$method() as i32,
                None => 1,
            }
        }
    };
}
check_char!(nstd_core_char_types_is_alphabetic, is_alphabetic);
check_char!(nstd_core_char_types_is_alphanumeric, is_alphanumeric);
check_char!(nstd_core_char_types_is_numeric, is_numeric);
check_char!(nstd_core_char_types_is_uppercase, is_uppercase);
check_char!(nstd_core_char_types_is_lowercase, is_lowercase);
check_char!(nstd_core_char_types_is_whitespace, is_whitespace);
check_char!(nstd_core_char_types_is_control, is_control);
check_char!(nstd_core_char_types_is_hexdigit, is_ascii_hexdigit);
check_char!(nstd_core_char_types_is_punctuation, is_ascii_punctuation);
check_char!(nstd_core_char_types_is_graphic, is_ascii_graphic);
