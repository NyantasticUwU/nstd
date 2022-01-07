use crate::def::NSTDBool;

/// Represents a UTF-8 char.
pub type NSTDChar8 = u8;

/// Represents a UTF-16 char.
pub type NSTDChar16 = u16;

/// Represents a UTF-32 char.
pub type NSTDChar32 = u32;

/// Represents a unicode char type.
pub type NSTDUnichar = NSTDChar32;

/// Generates the `nstd_core_char_types_is_*` functions.
macro_rules! check_char {
    ($name: ident, $method: ident) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(chr: NSTDUnichar) -> NSTDBool {
            NSTDBool::from(char::from_u32_unchecked(chr).$method())
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
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDUnichar chr` - The uppercase version.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_char_types_to_uppercase(chr: NSTDUnichar) -> NSTDUnichar {
    NSTDUnichar::from(char::from_u32_unchecked(chr).to_ascii_uppercase())
}

/// Converts a character to lowercase.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDUnichar chr` - The lowercase version.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_char_types_to_lowercase(chr: NSTDUnichar) -> NSTDUnichar {
    NSTDUnichar::from(char::from_u32_unchecked(chr).to_ascii_lowercase())
}

/// Returns the unicode replacement character (�).
/// Returns: `NSTDUnichar chr` - The unicode replacement character.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_char_types_replacement_char() -> NSTDUnichar {
    NSTDUnichar::from(char::REPLACEMENT_CHARACTER)
}

/// Gets the number of bytes an `NSTDUnichar` requires.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - The unicode character.
/// Returns: `NSTDUSize bytes` - The number of bytes this `NSTDUnichar` requires, 0 on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_char_types_size(chr: NSTDUnichar) -> usize {
    char::from_u32_unchecked(chr).len_utf8()
}
