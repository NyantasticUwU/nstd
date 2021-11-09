#ifndef NSTD_CORE_CHAR_TYPES_H_INCLUDED
#define NSTD_CORE_CHAR_TYPES_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a unicode char type.
typedef NSTDCOREUInt32 NSTDCOREUnichar;

/// Checks if an `NSTDCOREUnichar` is alphabetic.
/// Parameters:
///     `const NSTDCOREUnichar chr` - A 32-bit char.
/// Returns: `NSTDCOREInt32 is_alphabetic` - 1 if the character is alphabetic.
NSTDAPI NSTDCOREInt32 nstd_core_char_types_is_alphabetic(const NSTDCOREUnichar chr);

/// Checks if an `NSTDCOREUnichar` is alphanumeric.
/// Parameters:
///     `const NSTDCOREUnichar chr` - A 32-bit char.
/// Returns: `NSTDCOREInt32 is_alphanumeric` - 1 if the character is alphanumeric.
NSTDAPI NSTDCOREInt32 nstd_core_char_types_is_alphanumeric(const NSTDCOREUnichar chr);

/// Checks if an `NSTDCOREUnichar` is numeric.
/// Parameters:
///     `const NSTDCOREUnichar chr` - A 32-bit char.
/// Returns: `NSTDCOREInt32 is_numeric` - 1 if the character is numeric.
NSTDAPI NSTDCOREInt32 nstd_core_char_types_is_numeric(const NSTDCOREUnichar chr);

/// Checks if an `NSTDCOREUnichar` is uppercase.
/// Parameters:
///     `const NSTDCOREUnichar chr` - A 32-bit char.
/// Returns: `NSTDCOREInt32 is_uppercase` - 1 if the character is uppercase.
NSTDAPI NSTDCOREInt32 nstd_core_char_types_is_uppercase(const NSTDCOREUnichar chr);

/// Checks if an `NSTDCOREUnichar` is lowercase.
/// Parameters:
///     `const NSTDCOREUnichar chr` - A 32-bit char.
/// Returns: `NSTDCOREInt32 is_lowercase` - 1 if the character is lowercase.
NSTDAPI NSTDCOREInt32 nstd_core_char_types_is_lowercase(const NSTDCOREUnichar chr);

/// Checks if an `NSTDCOREUnichar` is white space.
/// Parameters:
///     `const NSTDCOREUnichar chr` - A 32-bit char.
/// Returns: `NSTDCOREInt32 is_whitespace` - 1 if the character is white space.
NSTDAPI NSTDCOREInt32 nstd_core_char_types_is_whitespace(const NSTDCOREUnichar chr);

/// Checks if an `NSTDCOREUnichar` is a control character.
/// Parameters:
///     `const NSTDCOREUnichar chr` - A 32-bit char.
/// Returns: `NSTDCOREInt32 is_control` - 1 if the character is a control character.
NSTDAPI NSTDCOREInt32 nstd_core_char_types_is_control(const NSTDCOREUnichar chr);

/// Checks if an `NSTDCOREUnichar` is a hex digit character.
/// Parameters:
///     `const NSTDCOREUnichar chr` - A 32-bit char.
/// Returns: `NSTDCOREInt32 is_hexdigit` - 1 if the character is a hex digit character.
NSTDAPI NSTDCOREInt32 nstd_core_char_types_is_hexdigit(const NSTDCOREUnichar chr);

/// Checks if an `NSTDCOREUnichar` is ascii punctuation.
/// Parameters:
///     `const NSTDCOREUnichar chr` - A 32-bit char.
/// Returns: `NSTDCOREInt32 is_punctuation` - 1 if the character is ascii punctuation.
NSTDAPI NSTDCOREInt32 nstd_core_char_types_is_punctuation(const NSTDCOREUnichar chr);

/// Checks if an `NSTDCOREUnichar` is ascii graphical.
/// Parameters:
///     `const NSTDCOREUnichar chr` - A 32-bit char.
/// Returns: `NSTDCOREInt32 is_graphic` - 1 if the character is ascii graphical.
NSTDAPI NSTDCOREInt32 nstd_core_char_types_is_graphic(const NSTDCOREUnichar chr);

#ifdef __cplusplus
}
#endif
#endif
