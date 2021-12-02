#ifndef NSTD_STD_INPUT_INPUT_H_INCLUDED
#define NSTD_STD_INPUT_INPUT_H_INCLUDED
#include "../def.h"
#include "key.h"
#ifdef __cplusplus
extern "C"
{
#endif
/// Left mouse button constant.
#define NSTD_STD_INPUT_LEFT_MOUSE_BUTTON 0
/// Right mouse button constant.
#define NSTD_STD_INPUT_RIGHT_MOUSE_BUTTON 1
/// Middle mouse button constant.
#define NSTD_STD_INPUT_MIDDLE_MOUSE_BUTTON 2

/// A raw input handle.
typedef void *NSTDRawInput;

/// Checks if a key is down.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDKey key` - The key to check.
/// Returns: `int is_down` - 1 if the key is down, 0 otherwise.
NSTDAPI int nstd_std_input_is_key_down(const NSTDRawInput raw_input, const NSTDKey key);

/// Checks if a key is up.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDKey key` - The key to check.
/// Returns: `int is_up` - 1 if the key is up, 0 otherwise.
NSTDAPI int nstd_std_input_is_key_up(const NSTDRawInput raw_input, const NSTDKey key);

/// Checks if a mouse button is down.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDUSize button` - The mouse button to check.
/// Returns: `int is_down` - 1 if the mouse button is up, 0 otherwise.
NSTDAPI int nstd_std_input_is_mouse_down(const NSTDRawInput raw_input, const NSTDUSize button);

/// Checks if a mouse button is up.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDUSize button` - The mouse button to check.
/// Returns: `int is_up` - 1 if the mouse button is up, 0 otherwise.
NSTDAPI int nstd_std_input_is_mouse_up(const NSTDRawInput raw_input, const NSTDUSize button);

#ifdef __cplusplus
}
#endif
#endif
