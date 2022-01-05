#ifndef NSTD_CORE_ARCH_H_INCLUDED
#define NSTD_CORE_ARCH_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents an endianness of a CPU.
typedef enum
{
    /// An unknown-endian.
    NSTD_ENDIAN_UNKNOWN,
    /// Little-endian.
    NSTD_ENDIAN_LITTLE,
    /// Big-endian.
    NSTD_ENDIAN_BIG,
} NSTDEndian;

/// Returns the size (in bytes) of a pointer.
/// Returns: `NSTDUSize size` - Size of a pointer.
NSTDAPI NSTDUSize nstd_core_arch_ptr_size();

/// Returns the target CPU's endianness.
/// Returns: `NSTDEndian endian` - The target CPU endianness.
NSTDAPI NSTDEndian nstd_core_arch_endian();

#ifdef __cplusplus
}
#endif
#endif
