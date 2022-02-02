use crate::core::{
    def::{NSTDBool, NSTDUnichar},
    slice::NSTDSlice,
};

/// Checks if `chr` is a valid unicode scalar value.
/// Parameters:
///     `const NSTDUnichar chr` - The unicode character.
/// Returns: `NSTDBool is_valid_unicode` - True if `chr` is valid unicode.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_char_types_check(chr: NSTDUnichar) -> NSTDBool {
    match char::from_u32(chr) {
        Some(_) => NSTDBool::NSTD_BOOL_TRUE,
        _ => NSTDBool::NSTD_BOOL_FALSE,
    }
}

/// Converts an `NSTDUInt32` to an `NSTDUnichar`.
/// Parameters:
///     `const NSTDUInt32 num` - The u32.
/// Returns: `NSTDUnichar chr` - `num` interpreted as a numerical character, � on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_char_types_from_u32(num: u32) -> NSTDUnichar {
    match char::from_u32(num) {
        Some(chr) => NSTDUnichar::from(chr),
        _ => NSTDUnichar::from(char::REPLACEMENT_CHARACTER),
    }
}

/// Converts `num` to an `NSTDUnichar` based on `radix`.
/// Parameters:
///     `const NSTDUInt32 num` - The number.
///     `const NSTDUInt32 radix` - The radix.
/// Returns: `NSTDUnichar chr` - `num` interpreted as a numerical character, � on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_char_types_from_digit(num: u32, radix: u32) -> NSTDUnichar {
    match char::from_digit(num, radix) {
        Some(chr) => NSTDUnichar::from(chr),
        _ => NSTDUnichar::from(char::REPLACEMENT_CHARACTER),
    }
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

/// Converts an `NSTDUnichar` to an `NSTDUInt32` based on `radix`.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
///     `const NSTDUInt32 radix` - The radix.
///     `NSTDInt32 *const errc` - Returns as nonzero on error.
/// Returns: `NSTDUInt32 digit` - The digit.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_char_types_to_digit(
    chr: NSTDUnichar,
    radix: u32,
    errc: &mut i32,
) -> u32 {
    match char::from_u32_unchecked(chr).to_digit(radix) {
        Some(digit) => {
            *errc = 0;
            digit
        }
        _ => {
            *errc = 1;
            0
        }
    }
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
        let buf = slice.as_byte_slice_mut();
        char::from_u32_unchecked(chr).encode_utf8(buf);
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