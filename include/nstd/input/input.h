#ifndef NSTD_INPUT_INPUT_H_INCLUDED
#define NSTD_INPUT_INPUT_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "key.h"
#include "mouse.h"
NSTDCPPSTART

/// A raw input handle.
typedef NSTDAny NSTDRawInput;

/// Checks if a key is down.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDKey key` - The key to check.
/// Returns: `NSTDBool is_down` - `NSTD_BOOL_TRUE` if the key is down.
NSTDAPI NSTDBool nstd_input_is_key_down(const NSTDRawInput raw_input, const NSTDKey key);

/// Checks if a key is up.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDKey key` - The key to check.
/// Returns: `NSTDBool is_up` - `NSTD_BOOL_TRUE` if the key is up.
NSTDAPI NSTDBool nstd_input_is_key_up(const NSTDRawInput raw_input, const NSTDKey key);

/// Checks if a mouse button is down.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDMouseButton button` - The mouse button to check.
/// Returns: `NSTDBool is_down` - `NSTD_BOOL_TRUE` if the mouse button is up.
NSTDAPI NSTDBool nstd_input_is_mouse_down(
    const NSTDRawInput raw_input,
    const NSTDMouseButton button);

/// Checks if a mouse button is up.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDMouseButton button` - The mouse button to check.
/// Returns: `NSTDBool is_up` - `NSTD_BOOL_TRUE` if the mouse button is up.
NSTDAPI NSTDBool nstd_input_is_mouse_up(const NSTDRawInput raw_input, const NSTDMouseButton button);

NSTDCPPEND
#endif
