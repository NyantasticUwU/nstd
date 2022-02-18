#ifndef NSTD_CORE_CHAR_TYPES_H_INCLUDED
#define NSTD_CORE_CHAR_TYPES_H_INCLUDED
#include "../nstd.h"
#include "def.h"
#include "slice.h"
#ifdef NSTDCPP
extern "C"
{
#endif

/// Checks if `chr` is a valid unicode scalar value.
/// Parameters:
///     `const NSTDUnichar chr` - The unicode character.
/// Returns: `NSTDBool is_valid_unicode` - True if `chr` is valid unicode.
NSTDAPI NSTDBool nstd_core_char_types_check(const NSTDUnichar chr);

/// Converts an `NSTDUInt32` to an `NSTDUnichar`.
/// Parameters:
///     `const NSTDUInt32 num` - The u32.
/// Returns: `NSTDUnichar chr` - `num` interpreted as a numerical character, � on error.
NSTDAPI NSTDUnichar nstd_core_char_types_from_u32(const NSTDUInt32 num);

/// Converts `num` to an `NSTDUnichar` based on `radix`.
/// Parameters:
///     `const NSTDUInt32 num` - The number.
///     `const NSTDUInt32 radix` - The radix.
/// Returns: `NSTDUnichar chr` - `num` interpreted as a numerical character, � on error.
NSTDAPI NSTDUnichar nstd_core_char_types_from_digit(const NSTDUInt32 num, const NSTDUInt32 radix);

/// Checks if an `NSTDUnichar` is a digit based on `radix`.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_digit` - True if the character is a digit.
NSTDAPI NSTDBool nstd_core_char_types_is_digit(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is an ASCII character.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_ascii` - True if the character is a ascii character.
NSTDAPI NSTDBool nstd_core_char_types_is_ascii(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is alphabetic.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_alphabetic` - True if the character is alphabetic.
NSTDAPI NSTDBool nstd_core_char_types_is_alphabetic(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is alphanumeric.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_alphanumeric` - True if the character is alphanumeric.
NSTDAPI NSTDBool nstd_core_char_types_is_alphanumeric(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is numeric.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_numeric` - True if the character is numeric.
NSTDAPI NSTDBool nstd_core_char_types_is_numeric(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is uppercase.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_uppercase` - True if the character is uppercase.
NSTDAPI NSTDBool nstd_core_char_types_is_uppercase(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is lowercase.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_lowercase` - True if the character is lowercase.
NSTDAPI NSTDBool nstd_core_char_types_is_lowercase(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is white space.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_whitespace` - True if the character is white space.
NSTDAPI NSTDBool nstd_core_char_types_is_whitespace(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is a control character.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_control` - True if the character is a control character.
NSTDAPI NSTDBool nstd_core_char_types_is_control(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is a hex digit character.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_hexdigit` - True if the character is a hex digit character.
NSTDAPI NSTDBool nstd_core_char_types_is_hexdigit(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is ascii punctuation.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_punctuation` - True if the character is ascii punctuation.
NSTDAPI NSTDBool nstd_core_char_types_is_punctuation(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is ascii graphical.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_graphic` - True if the character is ascii graphical.
NSTDAPI NSTDBool nstd_core_char_types_is_graphic(const NSTDUnichar chr);

/// Converts a character to uppercase.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDUnichar chr` - The uppercase version.
NSTDAPI NSTDUnichar nstd_core_char_types_to_uppercase(const NSTDUnichar chr);

/// Converts a character to lowercase.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDUnichar chr` - The lowercase version.
NSTDAPI NSTDUnichar nstd_core_char_types_to_lowercase(const NSTDUnichar chr);

/// Converts an `NSTDUnichar` to an `NSTDUInt32` based on `radix`.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
///     `const NSTDUInt32 radix` - The radix.
///     `NSTDInt32 *const errc` - Set to nonzero on error.
/// Returns: `NSTDUInt32 digit` - The digit.
NSTDAPI NSTDUInt32 nstd_core_char_types_to_digit(
    const NSTDUnichar chr,
    const NSTDUInt32 radix,
    NSTDInt32 *const errc);

/// Encodes `chr` into `slice`. `slice->size` must be at least 4 and `slice->ptr.size` must be 1.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
///     `NSTDSlice *const slice` - The encoding buffer.
NSTDAPI void nstd_core_char_types_encode(const NSTDUnichar chr, NSTDSlice *const slice);

/// Returns the unicode replacement character (�).
/// Returns: `NSTDUnichar chr` - The unicode replacement character.
NSTDAPI NSTDUnichar nstd_core_char_types_replacement_char();

/// Gets the number of bytes an `NSTDUnichar` requires.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - The unicode character.
/// Returns: `NSTDUSize bytes` - The number of bytes this `NSTDUnichar` requires, 0 on error.
NSTDAPI NSTDUSize nstd_core_char_types_size(const NSTDUnichar chr);

#ifdef NSTDCPP
}
#endif
#endif
