#ifndef NSTD_COLLECTIONS_BIT_MASK_H_INCLUDED
#define NSTD_COLLECTIONS_BIT_MASK_H_INCLUDED
#include "../core/def.h"
#include "vec.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// A bit mask type with a small memory footprint.
typedef struct
{
    /// Vector of bytes.
    NSTDVec bits;
} NSTDBitMask;

/// Creates a new bit mask with `size` number of bits.
/// Parameters:
///     `const NSTDUInt32 size` - The number of bits in this bit mask.
/// Returns: `NSTDBitMask mask` - The new bit mask.
NSTDAPI NSTDBitMask nstd_collections_bit_mask_new(const NSTDUInt32 size);

/// Sets a bit to either on (1) or off (0) depending on `mode` where `NSTD_BOOL_TRUE` is on.
/// Parameters:
///     `NSTDBitMask *const mask` - The bit mask.
///     `const NSTDUInt32 pos` - The bit index to set.
///     `const NSTDBool mode` - The mode to set the bit.
NSTDAPI void nstd_collections_bit_mask_set(
    NSTDBitMask *const mask,
    const NSTDUInt32 pos,
    const NSTDBool mode);

/// Returns the status of a bit at `pos`.
/// Parameters:
///     `const NSTDBitMask *const mask` - The bit mask.
///     `const NSTDUInt32 pos` - The bit index to check.
/// Returns: `NSTDBool is_on` - `NSTD_BOOL_TRUE` if the bit is on (1).
NSTDAPI NSTDBool nstd_collections_bit_mask_get(const NSTDBitMask *const mask, const NSTDUInt32 pos);

#ifdef __cplusplus
}
#endif
#endif
