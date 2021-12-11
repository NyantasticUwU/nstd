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
/// Returns: `NSTDInt32 is_alphabetic` - 1 if the character is alphabetic.
NSTDAPI NSTDInt32 nstd_core_char_types_is_alphabetic(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is alphanumeric.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDInt32 is_alphanumeric` - 1 if the character is alphanumeric.
NSTDAPI NSTDInt32 nstd_core_char_types_is_alphanumeric(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is numeric.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDInt32 is_numeric` - 1 if the character is numeric.
NSTDAPI NSTDInt32 nstd_core_char_types_is_numeric(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is uppercase.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDInt32 is_uppercase` - 1 if the character is uppercase.
NSTDAPI NSTDInt32 nstd_core_char_types_is_uppercase(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is lowercase.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDInt32 is_lowercase` - 1 if the character is lowercase.
NSTDAPI NSTDInt32 nstd_core_char_types_is_lowercase(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is white space.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDInt32 is_whitespace` - 1 if the character is white space.
NSTDAPI NSTDInt32 nstd_core_char_types_is_whitespace(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is a control character.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDInt32 is_control` - 1 if the character is a control character.
NSTDAPI NSTDInt32 nstd_core_char_types_is_control(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is a hex digit character.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDInt32 is_hexdigit` - 1 if the character is a hex digit character.
NSTDAPI NSTDInt32 nstd_core_char_types_is_hexdigit(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is ascii punctuation.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDInt32 is_punctuation` - 1 if the character is ascii punctuation.
NSTDAPI NSTDInt32 nstd_core_char_types_is_punctuation(const NSTDUnichar chr);

/// Checks if an `NSTDUnichar` is ascii graphical.
/// Parameters:
///     `const NSTDUnichar chr` - A 32-bit char.
/// Returns: `NSTDInt32 is_graphic` - 1 if the character is ascii graphical.
NSTDAPI NSTDInt32 nstd_core_char_types_is_graphic(const NSTDUnichar chr);

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
