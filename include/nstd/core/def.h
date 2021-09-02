#ifndef NSTD_CORE_DEF_H_INCLUDED
#define NSTD_CORE_DEF_H_INCLUDED
#ifndef __cplusplus
#define NSTDC_NULL 0
#else
#define NSTDC_NULL nullptr
#endif

/// Returns a null pointer.
/// Returns: `void *null` - A null pointer.
void *nstd_core_def_null();

/// Represents a size of any type, such as a memory block.
typedef unsigned long NSTDCSize;
/// Represents a signed size.
typedef long NSTDCISize;

/// Represents a byte.
typedef unsigned char NSTDCByte;

#endif
