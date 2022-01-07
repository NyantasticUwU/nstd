#ifndef NSTD_CORE_CHAR_TYPES_H_INCLUDED
#define NSTD_CORE_CHAR_TYPES_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a UTF-8 char.
typedef NSTDUInt8 NSTDChar8;

/// Represents a UTF-16 char.
typedef NSTDUInt16 NSTDChar16;

/// Represents a UTF-32 char.
typedef NSTDUInt32 NSTDChar32;

/// Represents a unicode char type.
typedef NSTDChar32 NSTDUnichar;

/// Checks if `chr` is a valid unicode scalar value.
/// Parameters:
///     `const NSTDUnichar chr` - The unicode character.
/// Returns: `NSTDBool is_valid_unicode` - True if `chr` is valid unicode.
NSTDAPI NSTDBool nstd_core_char_types_check(const NSTDUnichar chr);

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

/// Returns the unicode replacement character (�).
/// Returns: `NSTDUnichar chr` - The unicode replacement character.
NSTDAPI NSTDUnichar nstd_core_char_types_replacement_char();

/// Gets the number of bytes an `NSTDUnichar` requires.
/// NOTE: This function does not check the validity of `chr`.
/// Parameters:
///     `const NSTDUnichar chr` - The unicode character.
/// Returns: `NSTDUSize bytes` - The number of bytes this `NSTDUnichar` requires, 0 on error.
NSTDAPI NSTDUSize nstd_core_char_types_size(const NSTDUnichar chr);

#ifdef __cplusplus
}
#endif
#endif
