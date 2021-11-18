use core::ffi::c_void;

/// Copies bytes from `other` to `copycat`.
/// Parameters:
///     `NSTDByte *const copycat` - Pointer to memory to be copied to.
///     `const NSTDByte *const other` - Pointer to memory to be copied from.
///     `const NSTDUSize size` - Number of bytes to copy.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_mem_copy(copycat: *mut u8, other: *const u8, size: usize) {
    core::ptr::copy(other, copycat, size);
}

/// Moves bytes from `from` to `to`. Zeroes out `from`'s memory.
/// Parameters:
///     `NSTDByte *const from` - Memory to be moved from.
///     `NSTDByte *const to` - Memory to be moved to.
///     `const NSTDUSize size` - Number of bytes to move.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_mem_move(from: *mut u8, to: *mut u8, size: usize) {
    nstd_core_mem_copy(to, from, size);
    for i in 0..size {
        (*from.add(i)) = 0;
    }
}

/// Moves memory from `*ptr1` to `*ptr2` and vice versa.
/// Parameters:
///     `void *const ptr1` - First pointer to memory to swap.
///     `void *const ptr2` - Second pointer to memory to swap.
///     `const NSTDUSize size` - Number of bytes to swap.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_mem_switch(ptr1: *mut c_void, ptr2: *mut c_void, size: usize) {
    let x = core::slice::from_raw_parts_mut(ptr1 as *mut u8, size);
    let y = core::slice::from_raw_parts_mut(ptr2 as *mut u8, size);
    x.swap_with_slice(y);
}

/// Fills a block of memory with `byte`.
/// Parameters:
///     `NSTDByte *const ptr` - Pointer to block of memory.
///     `const NSTDUSize size` - Size of block.
///     `const NSTDByte byte` - Byte to fill with.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_mem_fill(ptr: *mut u8, size: usize, byte: u8) {
    for i in 0..size {
        (*ptr.add(i)) = byte;
    }
}

/// Zeros a memory range pointed to by `ptr`.
/// Parameters:
///     `NSTDByte *const ptr` - Pointer to memory to be zeroed.
///     `NSTDUSize start` - Starting index of memory to be zeroed.
///     `const NSTDUSize end` - Ending index of memory to be zeroed. (Excluded).
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_mem_zero(ptr: *mut u8, mut start: usize, end: usize) {
    while start < end {
        (*ptr.add(start)) = 0;
        start += 1;
    }
}
