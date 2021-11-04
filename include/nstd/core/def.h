#ifndef NSTD_CORE_DEF_H_INCLUDED
#define NSTD_CORE_DEF_H_INCLUDED
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
#define NSTD_CORE_DEF_NULL 0
#else
#define NSTD_CORE_DEF_NULL nullptr
#endif

/// Returns a null pointer.
/// Returns: `void *null` - A null pointer.
NSTDAPI void *nstd_core_def_null();

/// Represents a size of any type, such as a memory block.
typedef unsigned long NSTDCOREUSize;
/// Represents a signed size.
typedef long NSTDCOREISize;

/// Represents a byte.
typedef unsigned char NSTDCOREByte;

#ifdef __cplusplus
}
#endif
#endif
