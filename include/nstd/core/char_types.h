#ifndef NSTD_CORE_CHAR_TYPES_H_INCLUDED
#define NSTD_CORE_CHAR_TYPES_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a unicode char type.
typedef NSTDUInt32 NSTDUnichar;

/// Checks if an `NSTDUnichar` is alphabetic.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_alphabetic` - True if the character is alphabetic.
NSTDAPI NSTDBool nstd_core_char_types_is_alphabetic(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is alphanumeric.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_alphanumeric` - True if the character is alphanumeric.
NSTDAPI NSTDBool nstd_core_char_types_is_alphanumeric(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is numeric.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_numeric` - True if the character is numeric.
NSTDAPI NSTDBool nstd_core_char_types_is_numeric(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is uppercase.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_uppercase` - True if the character is uppercase.
NSTDAPI NSTDBool nstd_core_char_types_is_uppercase(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is lowercase.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_lowercase` - True if the character is lowercase.
NSTDAPI NSTDBool nstd_core_char_types_is_lowercase(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is white space.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_whitespace` - True if the character is white space.
NSTDAPI NSTDBool nstd_core_char_types_is_whitespace(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is a control character.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_control` - True if the character is a control character.
NSTDAPI NSTDBool nstd_core_char_types_is_control(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is a hex digit character.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_hexdigit` - True if the character is a hex digit character.
NSTDAPI NSTDBool nstd_core_char_types_is_hexdigit(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is ascii punctuation.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_punctuation` - True if the character is ascii punctuation.
NSTDAPI NSTDBool nstd_core_char_types_is_punctuation(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is ascii graphical.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDBool is_graphic` - True if the character is ascii graphical.
NSTDAPI NSTDBool nstd_core_char_types_is_graphic(const NSTDUnichar chr);

/// Converts a character to uppercase.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDUnichar chr` - The uppercase version.
NSTDAPI NSTDUnichar nstd_core_char_types_to_uppercase(const NSTDUnichar chr);

/// Converts a character to lowercase.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDUnichar chr` - The lowercase version.
NSTDAPI NSTDUnichar nstd_core_char_types_to_lowercase(const NSTDUnichar chr);

/// Returns the unicode replacement character (�).
/// Returns: `NSTDUnichar chr` - The unicode replacement character.
NSTDAPI NSTDUnichar nstd_core_char_types_replacement_char();

/// Gets the number of bytes an `NSTDUnichar` requires.
/// Parameters:
///     `const NSTDUnichar chr` - The unicode character.
/// Returns: `NSTDUSize bytes` - The number of bytes this `NSTDUnichar` requires, 0 on error.
NSTDAPI NSTDUSize nstd_core_char_types_size(const NSTDUnichar chr);

#ifdef __cplusplus
}
#endif
#endif
