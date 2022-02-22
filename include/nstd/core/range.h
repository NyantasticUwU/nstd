#ifndef NSTD_CORE_RANGE_H_INCLUDED
#define NSTD_CORE_RANGE_H_INCLUDED
#include "../nstd.h"
#include "def.h"
NSTDCPPSTART

/// Represents an 8-bit signed range.
typedef struct
{
    /// Start of the range (included).
    NSTDInt8 start;
    /// End of the range (exluded).
    NSTDInt8 end;
} NSTDIRange8;
/// Represents an 8-bit unsigned range.
typedef struct
{
    /// Start of the range (included).
    NSTDUInt8 start;
    /// End of the range (exluded).
    NSTDUInt8 end;
} NSTDURange8;

/// Represents a 16-bit signed range.
typedef struct
{
    /// Start of the range (included).
    NSTDInt16 start;
    /// End of the range (exluded).
    NSTDInt16 end;
} NSTDIRange16;
/// Represents a 16-bit unsigned range.
typedef struct
{
    /// Start of the range (included).
    NSTDUInt16 start;
    /// End of the range (exluded).
    NSTDUInt16 end;
} NSTDURange16;

/// Represents a 32-bit signed range.
typedef struct
{
    /// Start of the range (included).
    NSTDInt32 start;
    /// End of the range (exluded).
    NSTDInt32 end;
} NSTDIRange32;
/// Represents a 32-bit unsigned range.
typedef struct
{
    /// Start of the range (included).
    NSTDUInt32 start;
    /// End of the range (exluded).
    NSTDUInt32 end;
} NSTDURange32;

/// Represents a 64-bit signed range.
typedef struct
{
    /// Start of the range (included).
    NSTDInt64 start;
    /// End of the range (exluded).
    NSTDInt64 end;
} NSTDIRange64;
/// Represents a 64-bit unsigned range.
typedef struct
{
    /// Start of the range (included).
    NSTDUInt64 start;
    /// End of the range (exluded).
    NSTDUInt64 end;
} NSTDURange64;

/// Represents a signed range.
typedef struct
{
    /// Start of the range (included).
    NSTDISize start;
    /// End of the range (exluded).
    NSTDISize end;
} NSTDIRange;
/// Represents an unsigned range.
typedef struct
{
    /// Start of the range (included).
    NSTDUSize start;
    /// End of the range (exluded).
    NSTDUSize end;
} NSTDURange;

NSTDCPPEND
#endif
