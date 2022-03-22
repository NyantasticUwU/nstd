#ifndef NSTD_COLLECTIONS_BIT_MASK_H_INCLUDED
#define NSTD_COLLECTIONS_BIT_MASK_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "../vec.h"
NSTDCPPSTART

/// A bit mask type with a small memory footprint.
typedef struct
{
    /// Vector of bytes.
    NSTDVec bytes;
} NSTDBitMask;

/// Creates a new bit mask with `size` number of bits.
///
/// # Parameters
///
/// - `const NSTDUInt32 size` - The number of bits in this bit mask.
///
/// # Returns
///
/// `NSTDBitMask mask` - The new bit mask.
NSTDAPI NSTDBitMask nstd_collections_bit_mask_new(const NSTDUInt32 size);

/// Sets a bit to either on (1) or off (0) depending on `mode` where `NSTD_BIT_VALUE_ON` is on.
///
/// # Parameters
///
/// - `NSTDBitMask *const mask` - The bit mask.
///
/// - `const NSTDUInt32 pos` - The bit index to set.
///
/// - `const NSTDBitValue mode` - The mode to set the bit.
NSTDAPI void nstd_collections_bit_mask_set(
    NSTDBitMask *const mask,
    const NSTDUInt32 pos,
    const NSTDBitValue mode);

/// Returns the status of a bit at `pos`.
///
/// # Parameters
///
/// - `const NSTDBitMask *const mask` - The bit mask.
///
/// - `const NSTDUInt32 pos` - The bit index to check.
///
/// # Returns
///
/// `NSTDBitValue is_on` - `NSTD_BIT_VALUE_ON` if the bit is on (1).
NSTDAPI NSTDBitValue nstd_collections_bit_mask_get(
    const NSTDBitMask *const mask,
    const NSTDUInt32 pos);

/// Sets all bits to `mode`.
///
/// # Parameters
///
/// - `NSTDBitMask *const mask` - The bit mask.
///
/// - `const NSTDBitValue mode` - The mode to set all bits.
NSTDAPI void nstd_collections_bit_mask_set_all(NSTDBitMask *const mask, const NSTDBitValue mode);

/// Frees an `NSTDBitMask`.
///
/// # Parameters
///
/// - `NSTDBitMask *const mask` - A pointer to the bit mask to free.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_collections_bit_mask_free(NSTDBitMask *const mask);

NSTDCPPEND
#endif
