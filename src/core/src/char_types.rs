/// Represents a unicode char type.
pub type NSTDUnichar = u32;

macro_rules! check_char {
    ($name: ident, $method: ident) => {
        #[inline]
        #[no_mangle]
        pub unsafe extern "C" fn $name(chr: NSTDUnichar) -> i32 {
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

/// Converts a character to uppercase.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDUnichar chr` - The uppercase version.
#[no_mangle]
pub unsafe extern "C" fn nstd_core_char_types_to_uppercase(chr: NSTDUnichar) -> NSTDUnichar {
    match char::from_u32(chr) {
        Some(chr) => NSTDUnichar::from(chr.to_ascii_uppercase()),
        _ => chr,
    }
}

/// Converts a character to lowercase.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDUnichar chr` - The lowercase version.
#[no_mangle]
pub unsafe extern "C" fn nstd_core_char_types_to_lowercase(chr: NSTDUnichar) -> NSTDUnichar {
    match char::from_u32(chr) {
        Some(chr) => NSTDUnichar::from(chr.to_ascii_lowercase()),
        _ => chr,
    }
}

/// Returns the unicode replacement character (�).
/// Returns: `NSTDUnichar chr` - The unicode replacement character.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_char_types_replacement_char() -> NSTDUnichar {
    NSTDUnichar::from(char::REPLACEMENT_CHARACTER)
}
