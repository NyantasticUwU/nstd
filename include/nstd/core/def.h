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
typedef size_t NSTDCOREUSize;
/// Represents a signed size.
typedef ptrdiff_t NSTDCOREISize;

/// 8-bit unsigned int.
typedef uint8_t NSTDCOREUInt8;
/// 8-bit signed int.
typedef int8_t NSTDCOREInt8;
/// 16-bit unsigned int.
typedef uint16_t NSTDCOREUInt16;
/// 16-bit signed int.
typedef int16_t NSTDCOREInt16;
/// 32-bit unsigned int.
typedef uint32_t NSTDCOREUInt32;
/// 32-bit signed int.
typedef int32_t NSTDCOREInt32;
/// 64-bit unsigned int.
typedef uint64_t NSTDCOREUInt64;
/// 64-bit signed int.
typedef int64_t NSTDCOREInt64;

/// Represents a byte.
typedef NSTDCOREUInt8 NSTDCOREByte;

#ifdef __cplusplus
}
#endif
#endif
