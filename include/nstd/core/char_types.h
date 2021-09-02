#ifndef NSTD_CORE_CHAR_TYPES_H_INCLUDED
#define NSTD_CORE_CHAR_TYPES_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Checks if `chr` is a control character.
/// Parameters:
///     `const char chr` - The character to check.
/// Returns: `int bool` - 1 if `chr` is a control char, 0 otherwise.
NSTDAPI int nstd_core_char_types_is_control(const char chr);

/// Checks if `chr` is a blank character.
/// Parameters:
///     `const char chr` - The character to check.
/// Returns: `int bool` - 1 if `chr` is a blank char, 0 otherwise.
NSTDAPI int nstd_core_char_types_is_blank(const char chr);

/// Checks if `chr` is a space character.
/// Parameters:
///     `const char chr` - The character to check.
/// Returns: `int bool` - 1 if `chr` is a space char, 0 otherwise.
NSTDAPI int nstd_core_char_types_is_space(const char chr);

/// Checks if `chr` is an uppercase character.
/// Parameters:
///     `const char chr` - The character to check.
/// Returns: `int bool` - 1 if `chr` is an uppercase char, 0 otherwise.
NSTDAPI int nstd_core_char_types_is_uppercase(const char chr);

/// Checks if `chr` is a lowercase character.
/// Parameters:
///     `const char chr` - The character to check.
/// Returns: `int bool` - 1 if `chr` is a lowercase char, 0 otherwise.
NSTDAPI int nstd_core_char_types_is_lowercase(const char chr);

/// Checks if `chr` is an alpha character.
/// Parameters:
///     `const char chr` - The character to check.
/// Returns: `int bool` - 1 if `chr` is an alpha char, 0 otherwise.
NSTDAPI int nstd_core_char_types_is_alpha(const char chr);

/// Checks if `chr` is a digit character.
/// Parameters:
///     `const char chr` - The character to check.
/// Returns: `int bool` - 1 if `chr` is a digit char, 0 otherwise.
NSTDAPI int nstd_core_char_types_is_digit(const char chr);

/// Checks if `chr` is a hex digit character.
/// Parameters:
///     `const char chr` - The character to check.
/// Returns: `int bool` - 1 if `chr` is a hex digit char, 0 otherwise.
NSTDAPI int nstd_core_char_types_is_hex_digit(const char chr);

/// Checks if `chr` is an alpha numeric character.
/// Parameters:
///     `const char chr` - The character to check.
/// Returns: `int bool` - 1 if `chr` is an alpha numeric char, 0 otherwise.
NSTDAPI int nstd_core_char_types_is_alpha_numeric(const char chr);

/// Checks if `chr` is a punctuation character.
/// Parameters:
///     `const char chr` - The character to check.
/// Returns: `int bool` - 1 if `chr` is a punctuation char, 0 otherwise.
NSTDAPI int nstd_core_char_types_is_punctuation(const char chr);

/// Checks if `chr` is a graphical character.
/// Parameters:
///     `const char chr` - The character to check.
/// Returns: `int bool` - 1 if `chr` is a graphical char, 0 otherwise.
NSTDAPI int nstd_core_char_types_is_graphical(const char chr);

/// Checks if `chr` is a printable character.
/// Parameters:
///     `const char chr` - The character to check.
/// Returns: `int bool` - 1 if `chr` is a printable char, 0 otherwise.
NSTDAPI int nstd_core_char_types_is_printable(const char chr);

/// Converts `chr` to uppercase if possible.
/// Parameters:
///     `const char chr` - The character to convert.
/// Returns: `char upper` - The uppercase version of chr.
NSTDAPI char nstd_core_char_types_to_uppercase(const char chr);

/// Converts `chr` to lowercase if possible.
/// Parameters:
///     `const char chr` - The character to convert.
/// Returns: `char lower` - The lowercase version of chr.
NSTDAPI char nstd_core_char_types_to_lowercase(const char chr);

#ifdef __cplusplus
}
#endif
#endif
