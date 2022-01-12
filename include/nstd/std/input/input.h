#ifndef NSTD_STD_INPUT_INPUT_H_INCLUDED
#define NSTD_STD_INPUT_INPUT_H_INCLUDED
#include "../../core/def.h"
#include "key.h"
#include "mouse.h"
#ifdef __cplusplus
extern "C"
{
#endif

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
///     `const NSTDMouseButton button` - The mouse button to check.
/// Returns: `int is_down` - 1 if the mouse button is up, 0 otherwise.
NSTDAPI int nstd_std_input_is_mouse_down(
    const NSTDRawInput raw_input,
    const NSTDMouseButton button);

/// Checks if a mouse button is up.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDMouseButton button` - The mouse button to check.
/// Returns: `int is_up` - 1 if the mouse button is up, 0 otherwise.
NSTDAPI int nstd_std_input_is_mouse_up(const NSTDRawInput raw_input, const NSTDMouseButton button);

#ifdef __cplusplus
}
#endif
#endif
