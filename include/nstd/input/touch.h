#ifndef NSTD_INPUT_TOUCH_H_INCLUDED
#define NSTD_INPUT_TOUCH_H_INCLUDED

/// Represents a touch screen's state.
typedef enum
{
    NSTD_TOUCH_STATE_MOVED,
    NSTD_TOUCH_STATE_STARTED,
    NSTD_TOUCH_STATE_ENDED,
    NSTD_TOUCH_STATE_CANCELLED
} NSTDTouchState;

#endif
