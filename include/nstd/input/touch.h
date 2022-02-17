#ifndef NSTD_INPUT_TOUCH_H_INCLUDED
#define NSTD_INPUT_TOUCH_H_INCLUDED
#include "../nstd.h"

/// Represents a touch screen's state.
typedef enum
{
    /// The press was moved.
    NSTD_TOUCH_STATE_MOVED,
    /// A press has started.
    NSTD_TOUCH_STATE_STARTED,
    /// A press has ended
    NSTD_TOUCH_STATE_ENDED,
    /// A press was canceled.
    NSTD_TOUCH_STATE_CANCELLED
} NSTDTouchState;

#endif
