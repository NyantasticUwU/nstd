#ifndef NSTD_EVENTS_DEF_H_INCLUDED
#define NSTD_EVENTS_DEF_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"

/// Represents a button ID.
typedef NSTDUInt32 NSTDButtonID;

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
