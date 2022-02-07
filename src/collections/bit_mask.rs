use crate::{collections::vec::NSTDVec, core::def::NSTDBool};
use std::ptr::addr_of;

/// A bit mask type with a small memory footprint.
#[repr(C)]
#[derive(Debug)]
pub struct NSTDBitMask {
    /// Vector of bytes.
    pub bits: NSTDVec,
}

/// Creates a new bit mask with `size` number of bits.
/// Parameters:
///     `const NSTDUInt32 size` - The number of bits in this bit mask.
/// Returns: `NSTDBitMask mask` - The new bit mask.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_bit_mask_new(size: u32) -> NSTDBitMask {
    let size = (size as f32 / u8::BITS as f32).ceil() as usize;
    let mut bits = crate::collections::vec::nstd_collections_vec_new_with_capacity(1, size);
    let zero = 0u8;
    for _ in 0..size {
        crate::collections::vec::nstd_collections_vec_push(&mut bits, addr_of!(zero).cast());
    }
    NSTDBitMask { bits }
}

/// Sets a bit to either on (1) or off (0) depending on `mode` where `NSTD_BOOL_TRUE` is on.
/// Parameters:
///     `NSTDBitMask *const mask` - The bit mask.
///     `const NSTDUInt32 pos` - The bit index to set.
///     `const NSTDBool mode` - The mode to set the bit.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_bit_mask_set(
    mask: &mut NSTDBitMask,
    pos: u32,
    mode: NSTDBool,
) {
    let byte_pos = (pos as f32 / u8::BITS as f32).floor() as usize;
    let bit_pos = pos % u8::BITS;
    let byte = mask.bits.buffer.ptr.raw.cast::<u8>().add(byte_pos);
    if mode == NSTDBool::NSTD_BOOL_FALSE {
        *byte &= !(1 << bit_pos);
    } else {
        *byte |= 1 << bit_pos;
    }
}

/// Returns the status of a bit at `pos`.
/// Parameters:
///     `const NSTDBitMask *const mask` - The bit mask.
///     `const NSTDUInt32 pos` - The bit index to check.
/// Returns: `NSTDBool is_on` - `NSTD_BOOL_TRUE` if the bit is on (1).
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_bit_mask_get(mask: &NSTDBitMask, pos: u32) -> NSTDBool {
    let byte_pos = (pos as f32 / u8::BITS as f32).floor() as usize;
    let bit_pos = pos % u8::BITS;
    let byte = mask.bits.buffer.ptr.raw.cast::<u8>().add(byte_pos);
    (((*byte >> bit_pos) & 1) != 0).into()
}
