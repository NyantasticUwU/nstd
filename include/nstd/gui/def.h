#ifndef NSTD_GUI_DEF_H_INCLUDED
#define NSTD_GUI_DEF_H_INCLUDED
#include "../core/def.h"

/// Represents a display handle.
typedef NSTDAny NSTDDisplay;

/// Represents a window's position.
typedef struct
{
    /// The position on the x-axis.
    NSTDInt32 x;
    /// The position on the y-axis.
    NSTDInt32 y;
} NSTDWindowPosition;

/// Represents a window's size.
typedef struct
{
    /// The width.
    NSTDUInt32 width;
    /// The height.
    NSTDUInt32 height;
} NSTDWindowSize;

#endif
