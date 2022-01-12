#ifndef NSTD_STD_INPUT_MOUSE_H_INCLUDED
#define NSTD_STD_INPUT_MOUSE_H_INCLUDED
#include "../../core/def.h"

/// Represents a mouse button state.
typedef enum
{
    NSTD_MOUSE_BUTTON_PRESSED,
    NSTD_MOUSE_BUTTON_RELEASED
} NSTDMouseButtonState;

/// Represents a mouse button.
typedef enum
{
    NSTD_MOUSE_BUTTON_LEFT,
    NSTD_MOUSE_BUTTON_RIGHT,
    NSTD_MOUSE_BUTTON_MIDDLE,
    NSTD_MOUSE_BUTTON_OTHER
} NSTDMouseButton;

/// Represents a mouse button event.
typedef struct
{
    NSTDMouseButton button;
    NSTDUInt16 extra_button;
    NSTDMouseButtonState state;
} NSTDMouseButtonEvent;

#endif
