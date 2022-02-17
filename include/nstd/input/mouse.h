#ifndef NSTD_INPUT_MOUSE_H_INCLUDED
#define NSTD_INPUT_MOUSE_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"

/// Represents a mouse button state.
typedef enum
{
    /// A mouse button is/was pressed.
    NSTD_MOUSE_BUTTON_PRESSED,
    /// A mouse button is/was released.
    NSTD_MOUSE_BUTTON_RELEASED
} NSTDMouseButtonState;

/// Represents a mouse button.
typedef enum
{
    /// The left mouse button.
    NSTD_MOUSE_BUTTON_LEFT,
    /// The right mouse button.
    NSTD_MOUSE_BUTTON_RIGHT,
    /// The middle mouse button.
    NSTD_MOUSE_BUTTON_MIDDLE,
    /// An extra mouse button.
    NSTD_MOUSE_BUTTON_OTHER
} NSTDMouseButton;

/// Represents a mouse button event.
typedef struct
{
    /// The mouse button.
    NSTDMouseButton button;
    /// The index of an extra button.
    NSTDUInt16 extra_button;
    /// The state of the mouse button.
    NSTDMouseButtonState state;
} NSTDMouseButtonEvent;

#endif
