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
/// Represents a const pointer to any type.
typedef const void *NSTDAnyConst;

/// Represents a size of any type, such as a memory block.
typedef size_t NSTDUSize;
/// Represents a signed size.
typedef ptrdiff_t NSTDISize;

/// 8-bit unsigned int.
typedef uint_least8_t NSTDUInt8;
/// 8-bit signed int.
typedef int_least8_t NSTDInt8;
/// 16-bit unsigned int.
typedef uint_least16_t NSTDUInt16;
/// 16-bit signed int.
typedef int_least16_t NSTDInt16;
/// 32-bit unsigned int.
typedef uint_least32_t NSTDUInt32;
/// 32-bit signed int.
typedef int_least32_t NSTDInt32;
/// 64-bit unsigned int.
typedef uint_least64_t NSTDUInt64;
/// 64-bit signed int.
typedef int_least64_t NSTDInt64;

/// 32-bit float.
typedef float NSTDFloat32;
/// 64-bit float.
typedef double NSTDFloat64;

/// Alias for C's char primitive.
typedef char NSTDChar;
/// Represents a 8-bit char.
typedef NSTDUInt8 NSTDChar8;
/// Represents a 16-bit char.
typedef NSTDUInt16 NSTDChar16;
/// Represents a 32-bit char.
typedef NSTDUInt32 NSTDChar32;
/// Represents a unicode char type.
typedef NSTDChar32 NSTDUnichar;

/// Represents a byte.
typedef NSTDUInt8 NSTDByte;

/// Represents an error code. An error code of 0 usually means success, while anything else
/// indicates a failure.
typedef NSTDInt32 NSTDErrorCode;

/// A boolean type.
typedef enum
{
    /// Boolean false or 0.
    NSTD_BOOL_FALSE,
    /// Boolean true or 1.
    NSTD_BOOL_TRUE
} NSTDBool;

#ifdef __cplusplus
}
#endif
#endif
