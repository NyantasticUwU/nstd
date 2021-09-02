#include <nstd/core/char_types.h>

extern "C"
{
    /// Checks if `chr` is a control character.
    /// Parameters:
    ///     `const char chr` - The character to check.
    /// Returns: `int bool` - 1 if `chr` is a control char, 0 otherwise.
    NSTDAPI int nstd_core_char_types_is_control(const char chr)
    {
        return (chr >= 0x00 && chr <= 0x1F) || chr == 0x7F;
    }

    /// Checks if `chr` is a blank character.
    /// Parameters:
    ///     `const char chr` - The character to check.
    /// Returns: `int bool` - 1 if `chr` is a blank char, 0 otherwise.
    NSTDAPI int nstd_core_char_types_is_blank(const char chr)
    {
        return chr == 0x09 || chr == 0x20;
    }

    /// Checks if `chr` is a space character.
    /// Parameters:
    ///     `const char chr` - The character to check.
    /// Returns: `int bool` - 1 if `chr` is a space char, 0 otherwise.
    NSTDAPI int nstd_core_char_types_is_space(const char chr)
    {
        return (chr >= 0x09 && chr <= 0x0D) || chr == 0x20;
    }

    /// Checks if `chr` is an uppercase character.
    /// Parameters:
    ///     `const char chr` - The character to check.
    /// Returns: `int bool` - 1 if `chr` is an uppercase char, 0 otherwise.
    NSTDAPI int nstd_core_char_types_is_uppercase(const char chr)
    {
        return chr >= 'A' && chr <= 'Z';
    }

    /// Checks if `chr` is a lowercase character.
    /// Parameters:
    ///     `const char chr` - The character to check.
    /// Returns: `int bool` - 1 if `chr` is a lowercase char, 0 otherwise.
    NSTDAPI int nstd_core_char_types_is_lowercase(const char chr)
    {
        return chr >= 'a' && chr <= 'z';
    }

    /// Checks if `chr` is an alpha character.
    /// Parameters:
    ///     `const char chr` - The character to check.
    /// Returns: `int bool` - 1 if `chr` is an alpha char, 0 otherwise.
    NSTDAPI int nstd_core_char_types_is_alpha(const char chr)
    {
        return nstd_core_char_types_is_uppercase(chr) || nstd_core_char_types_is_lowercase(chr);
    }

    /// Checks if `chr` is a digit character.
    /// Parameters:
    ///     `const char chr` - The character to check.
    /// Returns: `int bool` - 1 if `chr` is a digit char, 0 otherwise.
    NSTDAPI int nstd_core_char_types_is_digit(const char chr)
    {
        return chr >= '0' && chr <= '9';
    }

    /// Checks if `chr` is a hex digit character.
    /// Parameters:
    ///     `const char chr` - The character to check.
    /// Returns: `int bool` - 1 if `chr` is a hex digit char, 0 otherwise.
    NSTDAPI int nstd_core_char_types_is_hex_digit(const char chr)
    {
        return nstd_core_char_types_is_digit(chr)
            || (chr >= 'A' && chr <= 'F')
            || (chr >= 'a' && chr <= 'f');
    }

    /// Checks if `chr` is an alpha numeric character.
    /// Parameters:
    ///     `const char chr` - The character to check.
    /// Returns: `int bool` - 1 if `chr` is an alpha numeric char, 0 otherwise.
    NSTDAPI int nstd_core_char_types_is_alpha_numeric(const char chr)
    {
        return nstd_core_char_types_is_digit(chr)
            || nstd_core_char_types_is_uppercase(chr)
            || nstd_core_char_types_is_lowercase(chr);
    }

    /// Checks if `chr` is a punctuation character.
    /// Parameters:
    ///     `const char chr` - The character to check.
    /// Returns: `int bool` - 1 if `chr` is a punctuation char, 0 otherwise.
    NSTDAPI int nstd_core_char_types_is_punctuation(const char chr)
    {
        return (chr >= 0x21 && chr <= 0x2F)
            || (chr >= 0x3A && chr <= 0x40)
            || (chr >= 0x5B && chr <= 0x60)
            || (chr >= 0x7B && chr <= 0x7E);
    }

    /// Checks if `chr` is a graphical character.
    /// Parameters:
    ///     `const char chr` - The character to check.
    /// Returns: `int bool` - 1 if `chr` is a graphical char, 0 otherwise.
    NSTDAPI int nstd_core_char_types_is_graphical(const char chr)
    {
        return chr >= 0x21 && chr <= 0x7E;
    }

    /// Checks if `chr` is a printable character.
    /// Parameters:
    ///     `const char chr` - The character to check.
    /// Returns: `int bool` - 1 if `chr` is a printable char, 0 otherwise.
    NSTDAPI int nstd_core_char_types_is_printable(const char chr)
    {
        return chr >= 0x20 && chr <= 0x7E;
    }

    /// Converts `chr` to uppercase if possible.
    /// Parameters:
    ///     `const char chr` - The character to convert.
    /// Returns: `char upper` - The uppercase version of chr.
    NSTDAPI char nstd_core_char_types_to_uppercase(const char chr)
    {
        return nstd_core_char_types_is_lowercase(chr) ? chr - 32 : chr;
    }

    /// Converts `chr` to lowercase if possible.
    /// Parameters:
    ///     `const char chr` - The character to convert.
    /// Returns: `char lower` - The lowercase version of chr.
    NSTDAPI char nstd_core_char_types_to_lowercase(const char chr)
    {
        return nstd_core_char_types_is_uppercase(chr) ? chr + 32 : chr;
    }
}
