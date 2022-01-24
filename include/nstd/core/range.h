#ifndef NSTD_CORE_RANGE_H_INCLUDED
#define NSTD_CORE_RANGE_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

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

#ifdef __cplusplus
}
#endif
#endif
