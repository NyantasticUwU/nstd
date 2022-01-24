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
    NSTDInt64 start;
    /// End of the range (exluded).
    NSTDInt64 end;
} NSTDIRange;
/// Represents an unsigned range.
typedef struct
{
    /// Start of the range (included).
    NSTDUInt64 start;
    /// End of the range (exluded).
    NSTDUInt64 end;
} NSTDURange;

#ifdef __cplusplus
}
#endif
#endif
