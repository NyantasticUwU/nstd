use crate::def::NSTDURange;
use core::{ffi::c_void, ptr};

/// Represents a view into a sequence of data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NSTDSlice {
    pub size: usize,
    pub element_size: usize,
    pub data: *mut u8,
}

/// Access methods.
impl NSTDSlice {
    /// Gets the number of used bytes for this slice.
    #[inline]
    pub fn byte_count(&self) -> usize {
        self.size * self.element_size
    }

    /// Returns a pointer to the end of the slice.
    #[inline]
    pub unsafe fn end_unchecked(&self) -> *mut u8 {
        self.data.add(self.byte_count())
    }
}

/// Conversion methods.
impl NSTDSlice {
    /// Returns the NSTDSlice as a byte slice.
    #[inline]
    pub unsafe fn as_byte_slice(&self) -> &[u8] {
        core::slice::from_raw_parts(self.data, self.byte_count())
    }

    /// Returns the NSTDSlice as a mutable byte slice.
    #[inline]
    pub unsafe fn as_byte_slice_mut(&mut self) -> &mut [u8] {
        core::slice::from_raw_parts_mut(self.data, self.byte_count())
    }
}

/// Creates a new slice from raw data.
/// Parameters:
///     `const NSTDUSize size` - Number of elements to slice.
///     `const NSTDUSize element_size` - Size of each element.
///     `void *const data` - Pointer to the raw data.
/// Returns: `NSTDSlice slice` - The new slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_new(
    size: usize,
    element_size: usize,
    data: *mut c_void,
) -> NSTDSlice {
    NSTDSlice {
        size,
        element_size,
        data: data as *mut u8,
    }
}

/// Gets a pointer to an element from a slice.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the slice's memory
/// gets reallocated or deallocated, so it is advised to create a copy of the element after
/// getting it.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDUSize pos` - The position of the element to get.
/// Returns: `void *element` - Pointer to the element.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_get(slice: &NSTDSlice, pos: usize) -> *mut c_void {
    match slice.size > pos {
        true => slice.data.add(pos * slice.element_size) as *mut c_void,
        false => ptr::null_mut(),
    }
}

/// Gets the first element in the slice.
/// NOTE: This function follows the same behaviour rules as `nstd_core_slice_get`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
/// Returns: `void *element` - Pointer to the first element.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_first(slice: &NSTDSlice) -> *mut c_void {
    match slice.size > 0 {
        true => slice.data as *mut c_void,
        false => ptr::null_mut(),
    }
}

/// Gets the last element in the slice.
/// NOTE: This function follows the same behaviour rules as `nstd_core_slice_get`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
/// Returns: `void *element` - Pointer to the last element.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_last(slice: &NSTDSlice) -> *mut c_void {
    match slice.size > 0 {
        true => slice.end_unchecked().sub(slice.element_size) as *mut c_void,
        false => ptr::null_mut(),
    }
}

/// Checks if two slices carry the same data.
/// Parameters:
///     `const NSTDSlice *const s1` - The first slice.
///     `const NSTDSlice *const s2` - The second slice.
/// Returns: `NSTDInt32 is_same` - Nonzero if the two slices carry the same data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_compare(s1: &NSTDSlice, s2: &NSTDSlice) -> i32 {
    if s1.size == s2.size {
        return (s1.as_byte_slice() == s2.as_byte_slice()) as i32;
    }
    0
}

/// Checks if a slice contains `element`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const void *const element` - The element to search for.
/// Returns: `NSTDInt32 is_in` - Nonzero if the slice contains `element`.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_contains(
    slice: &NSTDSlice,
    element: *const c_void,
) -> i32 {
    let mut ptr = slice.data;
    let element = core::slice::from_raw_parts(element as *const u8, slice.element_size);
    for _ in 0..slice.size {
        let data = core::slice::from_raw_parts(ptr, slice.element_size);
        if data == element {
            return 1;
        }
        ptr = ptr.add(slice.element_size);
    }
    0
}

/// Counts the number of `element`s found in `slice`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const void *const element` - The element to count.
/// Returns: `NSTDUSize count` - The number of `element`s in `slice`.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_count(slice: &NSTDSlice, element: *const c_void) -> usize {
    let mut ptr = slice.data;
    let element = core::slice::from_raw_parts(element.cast(), slice.element_size);
    let mut count = 0;
    for _ in 0..slice.size {
        let data = core::slice::from_raw_parts(ptr, slice.element_size);
        if data == element {
            count += 1;
        }
        ptr = ptr.add(slice.element_size);
    }
    count
}

/// Finds the first `element` in `slice` and returns the index of the element.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const void *const element` - The element to search for.
/// Returns: `NSTDUSize index` - The index of the element, -1/usize::MAX if not found.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_find_first(
    slice: &NSTDSlice,
    element: *const c_void,
) -> usize {
    let mut ptr = slice.data;
    let element = core::slice::from_raw_parts(element.cast(), slice.element_size);
    for i in 0..slice.size {
        let data = core::slice::from_raw_parts(ptr, slice.element_size);
        if data == element {
            return i;
        }
        ptr = ptr.add(slice.element_size);
    }
    usize::MAX
}

/// Finds the last `element` in `slice` and returns the index of the element.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const void *const element` - The element to search for.
/// Returns: `NSTDUSize index` - The index of the element, -1/usize::MAX if not found.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_find_last(
    slice: &NSTDSlice,
    element: *const c_void,
) -> usize {
    let mut ptr = slice.end_unchecked().sub(slice.element_size);
    let element = core::slice::from_raw_parts(element.cast(), slice.element_size);
    for i in (0..slice.size).rev() {
        let data = core::slice::from_raw_parts(ptr, slice.element_size);
        if data == element {
            return i;
        }
        ptr = ptr.sub(slice.element_size);
    }
    usize::MAX
}

/// Checks if a slice starts with another slice.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDSlice *const pattern` - The slice pattern.
/// Returns: `NSTDInt32 starts_with` - Nonzero if `slice` starts with `pattern`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_starts_with(
    slice: &NSTDSlice,
    pattern: &NSTDSlice,
) -> i32 {
    let slice = slice.as_byte_slice();
    let pattern = pattern.as_byte_slice();
    slice.starts_with(pattern) as i32
}

/// Checks if a slice ends with another slice.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDSlice *const pattern` - The slice pattern.
/// Returns: `NSTDInt32 ends_with` - Nonzero if `slice` ends with `pattern`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_ends_with(slice: &NSTDSlice, pattern: &NSTDSlice) -> i32 {
    let slice = slice.as_byte_slice();
    let pattern = pattern.as_byte_slice();
    slice.ends_with(pattern) as i32
}

/// Fills a slice with `element`.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const void *const element` - The element.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_fill(slice: &mut NSTDSlice, element: *const c_void) {
    nstd_core_slice_fill_range(
        slice,
        element,
        &NSTDURange {
            start: 0,
            end: slice.size as u64,
        },
    );
}

/// Fills a specific range of a slice with `element`.
/// NOTE: This function does NOT check that `range` is valid for operating on `slice`.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const void *const element` - The element.
///     `const NSTDURange *const range` - The range of the slice to fill.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_fill_range(
    slice: &mut NSTDSlice,
    element: *const c_void,
    range: &NSTDURange,
) {
    let element = core::slice::from_raw_parts(element as *const u8, slice.element_size);
    let mut ptr = slice.data.add(range.start as usize * slice.element_size);
    for _ in range.start..range.end {
        let data = core::slice::from_raw_parts_mut(ptr, slice.element_size);
        data.copy_from_slice(element);
        ptr = ptr.add(slice.element_size);
    }
}

/// Swaps two elements in a slice.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const NSTDUSize i` - The index of the first element.
///     `const NSTDUSize j` - The index of the second element.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_swap(slice: &mut NSTDSlice, i: usize, j: usize) {
    let i = slice.data.add(slice.element_size * i);
    let j = slice.data.add(slice.element_size * j);
    let slicei = core::slice::from_raw_parts_mut(i, slice.element_size);
    let slicej = core::slice::from_raw_parts_mut(j, slice.element_size);
    slicei.swap_with_slice(slicej);
}

/// Reverses a slice's elements.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_reverse(slice: &mut NSTDSlice) {
    let mut ptr = slice.data;
    let mut data = slice.as_byte_slice_mut();
    data.reverse();
    for _ in 0..slice.size {
        data = core::slice::from_raw_parts_mut(ptr, slice.element_size);
        data.reverse();
        ptr = ptr.add(slice.element_size);
    }
}

/// Shifts a slice `x` times to the right.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const NSTDUSize x` - Number of times to shift the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_shift_right(slice: &mut NSTDSlice, x: usize) {
    let rot = x % slice.size * slice.element_size;
    slice.as_byte_slice_mut().rotate_right(rot);
}

/// Shifts a slice `x` times to the left.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const NSTDUSize x` - Number of times to shift the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_shift_left(slice: &mut NSTDSlice, x: usize) {
    let rot = x % slice.size * slice.element_size;
    slice.as_byte_slice_mut().rotate_left(rot);
}

/// Copies elements from `s2` to `s1`. The slices must be the same size in bytes.
/// Parameters:
///     `NSTDSlice *const s1` - The slice to copy to.
///     `const NSTDSlice *const s2` - The slice to copy from.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_copy_from_slice(s1: &mut NSTDSlice, s2: &NSTDSlice) {
    let bc1 = s1.byte_count();
    let bc2 = s2.byte_count();
    if bc1 == bc2 {
        let s1 = s1.as_byte_slice_mut();
        let s2 = s2.as_byte_slice();
        s1.copy_from_slice(s2);
    }
}

/// Swaps the elements of `s1` and `s2`. The slices must be the same size in bytes.
/// Parameters:
///     `NSTDSlice *const s1` - The first slice.
///     `NSTDSlice *const s2` - The second slice.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_swap_with_slice(s1: &mut NSTDSlice, s2: &mut NSTDSlice) {
    let bc1 = s1.byte_count();
    let bc2 = s2.byte_count();
    if bc1 == bc2 {
        let s1 = s1.as_byte_slice_mut();
        let s2 = s2.as_byte_slice_mut();
        s1.swap_with_slice(s2);
    }
}

/// Moves bytes from `s2` to `s1`, sets all `s2` bytes to 0.
/// Parameters:
///     `NSTDSlice *const s1` - The first slice.
///     `NSTDSlice *const s2` - The second slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_move(s1: &mut NSTDSlice, s2: &mut NSTDSlice) {
    const BYTE_SIZE: usize = core::mem::size_of::<u8>();
    nstd_core_slice_copy_from_slice(s1, s2);
    let mut s2 = nstd_core_slice_new(s2.byte_count(), BYTE_SIZE, s2.data as *mut c_void);
    nstd_core_slice_fill(&mut s2, &0u8 as *const u8 as *const c_void);
}
