//! A bit mask type with a small memory footprint.
use crate::{
    core::def::{NSTDBitValue, NSTDErrorCode},
    vec::NSTDVec,
};

/// A bit mask type with a small memory footprint.
#[repr(C)]
#[derive(Debug)]
pub struct NSTDBitMask {
    /// Vector of bytes.
    pub bytes: NSTDVec,
}

/// Creates a new bit mask with `size` number of bits.
///
/// # Parameters
///
/// - `const NSTDUInt32 size` - The number of bits in this bit mask.
///
/// # Returns
///
/// `NSTDBitMask mask` - The new bit mask.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_bit_mask_new(size: u32) -> NSTDBitMask {
    let size = crate::core::math::nstd_core_math_div_ceil_u32(size, u8::BITS) as usize;
    let mut bytes = crate::vec::nstd_vec_new_with_capacity(1, size);
    crate::vec::nstd_vec_resize(&mut bytes, size);
    NSTDBitMask { bytes }
}

/// Sets a bit to either on (1) or off (0) depending on `mode` where `NSTD_BIT_VALUE_ON` is on.
///
/// # Parameters
///
/// - `NSTDBitMask *const mask` - The bit mask.
///
/// - `const NSTDUInt32 pos` - The bit index to set.
///
/// - `const NSTDBitValue mode` - The mode to set the bit.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_bit_mask_set(
    mask: &mut NSTDBitMask,
    pos: u32,
    mode: NSTDBitValue,
) {
    let byte_pos = crate::core::math::nstd_core_math_div_floor_u32(pos, u8::BITS) as usize;
    let bit_pos = pos % u8::BITS;
    let byte = mask.bytes.buffer.ptr.raw.cast::<u8>().add(byte_pos);
    match mode {
        NSTDBitValue::NSTD_BIT_VALUE_ON => *byte |= 1 << bit_pos,
        NSTDBitValue::NSTD_BIT_VALUE_OFF => *byte &= !(1 << bit_pos),
    }
}

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
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_bit_mask_get(
    mask: &NSTDBitMask,
    pos: u32,
) -> NSTDBitValue {
    let byte_pos = crate::core::math::nstd_core_math_div_floor_u32(pos, u8::BITS) as usize;
    let bit_pos = pos % u8::BITS;
    let byte = mask.bytes.buffer.ptr.raw.cast::<u8>().add(byte_pos);
    (((*byte >> bit_pos) & 1) != 0).into()
}

/// Sets all bits to `mode`.
///
/// # Parameters
///
/// - `NSTDBitMask *const mask` - The bit mask.
///
/// - `const NSTDBitValue mode` - The mode to set all bits.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_bit_mask_set_all(
    mask: &mut NSTDBitMask,
    mode: NSTDBitValue,
) {
    let mode = match mode {
        NSTDBitValue::NSTD_BIT_VALUE_ON => !0,
        NSTDBitValue::NSTD_BIT_VALUE_OFF => 0,
    };
    for byte in mask.bytes.buffer.as_byte_slice_mut() {
        *byte = mode;
    }
}

/// Frees an `NSTDBitMask`.
///
/// # Parameters
///
/// - `NSTDBitMask *const mask` - A pointer to the bit mask to free.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_bit_mask_free(mask: &mut NSTDBitMask) -> NSTDErrorCode {
    crate::vec::nstd_vec_free(&mut mask.bytes)
}
