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
#ifndef __cplusplus
#define NSTD_CORE_DEF_NULL NULL
#else
#define NSTD_CORE_DEF_NULL nullptr
#endif

/// Returns a null pointer.
/// Returns: `void *null` - A null pointer.
NSTDAPI void *nstd_core_def_null();

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

#ifdef __cplusplus
}
#endif
#endif
