#ifndef NSTD_STD_DEF_H_INCLUDED
#define NSTD_STD_DEF_H_INCLUDED
#include <stddef.h>
#include <stdint.h>

/// Represents a size of any type, such as a memory block.
typedef size_t NSTDUSize;
/// Represents a signed size.
typedef ptrdiff_t NSTDISize;

/// Represents a byte.
typedef unsigned char NSTDByte;

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

#endif
