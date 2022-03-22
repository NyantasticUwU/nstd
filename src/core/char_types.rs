//! Provides functions for operating on character types.
use crate::core::{
    def::{NSTDBool, NSTDErrorCode, NSTDUnichar},
    slice::NSTDSlice,
};

/// Checks if `chr` is a valid unicode scalar value.
/// Parameters:
///     `const NSTDUnichar chr` - The unicode character.
/// Returns: `NSTDBool is_valid_unicode` - True if `chr` is valid unicode.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_char_types_check(chr: NSTDUnichar) -> NSTDBool {
    if char::from_u32(chr).is_some() {
        return NSTDBool::NSTD_BOOL_TRUE;
    }
    NSTDBool::NSTD_BOOL_FALSE
}

/// Converts `num` to an `NSTDUnichar` based on `radix`.
/// Parameters:
///     `const NSTDUInt32 num` - The number.
///     `const NSTDUInt32 radix` - The radix.
/// Returns: `NSTDUnichar chr` - `num` interpreted as a numerical character, � on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_char_types_from_digit(num: u32, radix: u32) -> NSTDUnichar {
    if let Some(chr) = char::from_digit(num, radix) {
        return NSTDUnichar::from(chr);
    }
    NSTDUnichar::from(char::REPLACEMENT_CHARACTER)
}

/// Converts an `NSTDUnichar` to an `NSTDUInt32` based on `radix`.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
///     `const NSTDUInt32 radix` - The radix.
///     `NSTDUInt32 *const digit` - Returns as the digit on success.
/// Returns: `NSTDErrorCode errc` - Set to nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_char_types_to_digit(
    chr: NSTDUnichar,
    radix: u32,
    digit: &mut u32,
) -> NSTDErrorCode {
    if let Some(d) = char::from_u32_unchecked(chr).to_digit(radix) {
        *digit = d;
        return 0;
    }
    1
}

/// Converts an `NSTDUInt32` to an `NSTDUnichar`.
/// NOTE: This function does not check the validity of `num`.
/// Parameters:
///     `const NSTDUInt32 num` - The u32.
/// Returns: `NSTDUnichar chr` - `num` interpreted as a numerical character, � on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_char_types_from_u32(num: u32) -> NSTDUnichar {
    NSTDUnichar::from(char::from_u32_unchecked(num))
}

/// Checks if an `NSTDUnichar` is a digit based on `radix`.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_digit` - True if the character is a digit.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_char_types_is_digit(chr: NSTDUnichar, radix: u32) -> NSTDBool {
    NSTDBool::from(char::from_u32_unchecked(chr).is_digit(radix))
}

/// Generates the `nstd_core_char_types_is_*` functions.
macro_rules! check_char {
    ($name: ident, $method: ident) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(chr: NSTDUnichar) -> NSTDBool {
            NSTDBool::from(char::from_u32_unchecked(chr).$method())
        }
    };
}
check_char!(nstd_core_char_types_is_ascii, is_ascii);
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

/// Encodes `chr` into `slice`. `slice->size` must be at least 4 and `slice->ptr.size` must be 1.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
///     `NSTDSlice *const slice` - The encoding buffer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_char_types_encode(chr: NSTDUnichar, slice: &mut NSTDSlice) {
    const BYTE_SIZE: usize = core::mem::size_of::<u8>();
    const CHAR_SIZE: usize = core::mem::size_of::<char>();
    if slice.size >= CHAR_SIZE && slice.ptr.size == BYTE_SIZE {
        char::from_u32_unchecked(chr).encode_utf8(slice.as_byte_slice_mut());
    }
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
