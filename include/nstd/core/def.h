#ifndef NSTD_CORE_DEF_H_INCLUDED
#define NSTD_CORE_DEF_H_INCLUDED
#include <stddef.h>
#include <stdint.h>
#if defined(__WINDOWS__)\
    || defined(_WIN32)\
    || defined(_WIN64)\
    || defined(__WIN32__)\
    || defined(__TOS_WIN__)
#define NSTDAPI __declspec(dllexport)
#else
#define NSTDAPI
#endif
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a pointer to any type.
typedef void *NSTDAny;

/// Represents a size of any type, such as a memory block.
typedef size_t NSTDUSize;
/// Represents a signed size.
typedef ptrdiff_t NSTDISize;

/// 8-bit unsigned int.
typedef uint8_t NSTDUInt8;
/// 8-bit signed int.
typedef int8_t NSTDInt8;
/// 16-bit unsigned int.
typedef uint16_t NSTDUInt16;
/// 16-bit signed int.
typedef int16_t NSTDInt16;
/// 32-bit unsigned int.
typedef uint32_t NSTDUInt32;
/// 32-bit signed int.
typedef int32_t NSTDInt32;
/// 64-bit unsigned int.
typedef uint64_t NSTDUInt64;
/// 64-bit signed int.
typedef int64_t NSTDInt64;

/// 32-bit float.
typedef float NSTDFloat32;
/// 64-bit float.
typedef double NSTDFloat64;

/// Represents a byte.
typedef NSTDUInt8 NSTDByte;

/// A boolean type.
typedef enum
{
    /// Boolean false or 0.
    NSTD_BOOL_FALSE,
    /// Boolean true or 1.
    NSTD_BOOL_TRUE
} NSTDBool;

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
