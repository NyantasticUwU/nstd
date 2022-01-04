#ifndef NSTD_CORE_DEF_H_INCLUDED
#define NSTD_CORE_DEF_H_INCLUDED
#include <stddef.h>
#include <stdint.h>
#ifdef _WIN32
#define NSTDAPI __declspec(dllexport)
#else
#define NSTDAPI
#endif
#ifdef __cplusplus
extern "C"
{
#endif

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

/// Represents a byte.
typedef NSTDUInt8 NSTDByte;

/// A boolean type.
typedef enum
{
    NSTD_BOOL_FALSE,
    NSTD_BOOL_TRUE
} NSTDBool;

/// Represents a signed range.
typedef struct
{
    NSTDInt64 start;
    NSTDInt64 end;
} NSTDIRange;
/// Represents an unsigned range.
typedef struct
{
    NSTDUInt64 start;
    NSTDUInt64 end;
} NSTDURange;

#ifdef __cplusplus
}
#endif
#endif
