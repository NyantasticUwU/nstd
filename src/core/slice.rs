//! A dynamically-sized view into a contiguous sequence of values.
use crate::core::{
    def::{NSTDAny, NSTDAnyConst, NSTDBool},
    pointer::NSTDPointer,
    range::NSTDURange,
    NSTD_CORE_NULL,
};

/// Represents a view into a sequence of data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NSTDSlice {
    /// The number of elements the slice contains.
    pub size: usize,
    /// A pointer to the data.
    pub ptr: NSTDPointer,
}

/// Access methods.
impl NSTDSlice {
    /// Gets the number of used bytes for this slice.
    #[inline]
    pub fn byte_count(&self) -> usize {
        self.size * self.ptr.size
    }

    /// Returns a pointer to the end of the slice.
    #[inline]
    pub unsafe fn end_unchecked(&self) -> *mut u8 {
        self.ptr.raw.add(self.byte_count()).cast()
    }
}

/// Conversion methods.
impl NSTDSlice {
    /// Returns the NSTDSlice as a byte slice.
    #[inline]
    pub unsafe fn as_byte_slice(&self) -> &[u8] {
        core::slice::from_raw_parts(self.ptr.raw.cast(), self.byte_count())
    }

    /// Returns the NSTDSlice as a mutable byte slice.
    #[inline]
    pub unsafe fn as_byte_slice_mut(&mut self) -> &mut [u8] {
        core::slice::from_raw_parts_mut(self.ptr.raw.cast(), self.byte_count())
    }
}

/// Creates a new slice from raw data.
/// Parameters:
///     `const NSTDUSize size` - Number of elements to slice.
///     `const NSTDUSize element_size` - Size of each element.
///     `const NSTDAny data` - Pointer to the raw data.
/// Returns: `NSTDSlice slice` - The new slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_new(
    size: usize,
    element_size: usize,
    data: NSTDAny,
) -> NSTDSlice {
    NSTDSlice {
        size,
        ptr: crate::core::pointer::nstd_core_pointer_new(data, element_size),
    }
}

/// Gets a pointer to an element from a slice.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the slice's memory
/// gets reallocated or deallocated, so it is advised to create a copy of the element after
/// getting it.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDUSize pos` - The position of the element to get.
/// Returns: `NSTDAny element` - Pointer to the element.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_get(slice: &NSTDSlice, pos: usize) -> NSTDAny {
    match slice.size > pos {
        true => slice.ptr.raw.add(pos * slice.ptr.size),
        false => NSTD_CORE_NULL,
    }
}

/// Gets the first element in the slice.
/// NOTE: This function follows the same behaviour rules as `nstd_core_slice_get`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
/// Returns: `NSTDAny element` - Pointer to the first element.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_first(slice: &NSTDSlice) -> NSTDAny {
    match slice.size > 0 {
        true => slice.ptr.raw,
        false => NSTD_CORE_NULL,
    }
}

/// Gets the last element in the slice.
/// NOTE: This function follows the same behaviour rules as `nstd_core_slice_get`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
/// Returns: `NSTDAny element` - Pointer to the last element.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_last(slice: &NSTDSlice) -> NSTDAny {
    match slice.size > 0 {
        true => slice.end_unchecked().sub(slice.ptr.size).cast(),
        false => NSTD_CORE_NULL,
    }
}

/// Checks if two slices carry the same data.
/// Parameters:
///     `const NSTDSlice *const s1` - The first slice.
///     `const NSTDSlice *const s2` - The second slice.
/// Returns: `NSTDBool is_same` - True if the two slices carry the same data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_compare(s1: &NSTDSlice, s2: &NSTDSlice) -> NSTDBool {
    match s1.size == s2.size {
        true => NSTDBool::from(s1.as_byte_slice() == s2.as_byte_slice()),
        false => NSTDBool::NSTD_BOOL_FALSE,
    }
}

/// Checks if a slice contains `element`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDAnyConst element` - The element to search for.
/// Returns: `NSTDBool is_in` - True if the slice contains `element`.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_contains(
    slice: &NSTDSlice,
    element: NSTDAnyConst,
) -> NSTDBool {
    let mut ptr = slice.ptr.raw as *const u8;
    let element = core::slice::from_raw_parts(element.cast(), slice.ptr.size);
    for _ in 0..slice.size {
        let data = core::slice::from_raw_parts(ptr, slice.ptr.size);
        if data == element {
            return NSTDBool::NSTD_BOOL_TRUE;
        }
        ptr = ptr.add(slice.ptr.size);
    }
    NSTDBool::NSTD_BOOL_FALSE
}

/// Counts the number of `element`s found in `slice`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDAnyConst element` - The element to count.
/// Returns: `NSTDUSize count` - The number of `element`s in `slice`.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_count(slice: &NSTDSlice, element: NSTDAnyConst) -> usize {
    let mut ptr = slice.ptr.raw as *const u8;
    let element = core::slice::from_raw_parts(element.cast(), slice.ptr.size);
    let mut count = 0;
    for _ in 0..slice.size {
        let data = core::slice::from_raw_parts(ptr, slice.ptr.size);
        if data == element {
            count += 1;
        }
        ptr = ptr.add(slice.ptr.size);
    }
    count
}

/// Finds the first `element` in `slice` and returns the index of the element.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDAnyConst element` - The element to search for.
/// Returns: `NSTDUSize index` - The index of the element, -1/usize::MAX if not found.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_find_first(
    slice: &NSTDSlice,
    element: NSTDAnyConst,
) -> usize {
    let mut ptr = slice.ptr.raw as *const u8;
    let element = core::slice::from_raw_parts(element.cast(), slice.ptr.size);
    for i in 0..slice.size {
        let data = core::slice::from_raw_parts(ptr, slice.ptr.size);
        if data == element {
            return i;
        }
        ptr = ptr.add(slice.ptr.size);
    }
    usize::MAX
}

/// Finds the last `element` in `slice` and returns the index of the element.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDAnyConst element` - The element to search for.
/// Returns: `NSTDUSize index` - The index of the element, -1/usize::MAX if not found.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_find_last(
    slice: &NSTDSlice,
    element: NSTDAnyConst,
) -> usize {
    let mut ptr = slice.end_unchecked().sub(slice.ptr.size);
    let element = core::slice::from_raw_parts(element.cast(), slice.ptr.size);
    for i in (0..slice.size).rev() {
        let data = core::slice::from_raw_parts(ptr, slice.ptr.size);
        if data == element {
            return i;
        }
        ptr = ptr.sub(slice.ptr.size);
    }
    usize::MAX
}

/// Checks if a slice starts with another slice.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDSlice *const pattern` - The slice pattern.
/// Returns: `NSTDBool starts_with` - True if `slice` starts with `pattern`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_starts_with(
    slice: &NSTDSlice,
    pattern: &NSTDSlice,
) -> NSTDBool {
    let slice = slice.as_byte_slice();
    let pattern = pattern.as_byte_slice();
    NSTDBool::from(slice.starts_with(pattern))
}

/// Checks if a slice ends with another slice.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDSlice *const pattern` - The slice pattern.
/// Returns: `NSTDBool ends_with` - True if `slice` ends with `pattern`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_ends_with(
    slice: &NSTDSlice,
    pattern: &NSTDSlice,
) -> NSTDBool {
    let slice = slice.as_byte_slice();
    let pattern = pattern.as_byte_slice();
    NSTDBool::from(slice.ends_with(pattern))
}

/// Fills a slice with `element`.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const NSTDAnyConst element` - The element.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_fill(slice: &mut NSTDSlice, element: NSTDAnyConst) {
    nstd_core_slice_fill_range(
        slice,
        element,
        &NSTDURange {
            start: 0,
            end: slice.size,
        },
    );
}

/// Fills a specific range of a slice with `element`.
/// NOTE: This function does NOT check that `range` is valid for operating on `slice`.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const NSTDAnyConst element` - The element.
///     `const NSTDURange *const range` - The range of the slice to fill.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_fill_range(
    slice: &mut NSTDSlice,
    element: NSTDAnyConst,
    range: &NSTDURange,
) {
    let element = core::slice::from_raw_parts(element as *const u8, slice.ptr.size);
    let mut ptr = slice.ptr.raw.add(range.start * slice.ptr.size).cast();
    for _ in range.start..range.end {
        let data = core::slice::from_raw_parts_mut(ptr, slice.ptr.size);
        data.copy_from_slice(element);
        ptr = ptr.add(slice.ptr.size);
    }
}

/// Swaps two elements in a slice.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const NSTDUSize i` - The index of the first element.
///     `const NSTDUSize j` - The index of the second element.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_swap(slice: &mut NSTDSlice, i: usize, j: usize) {
    let i = slice.ptr.raw.add(slice.ptr.size * i);
    let j = slice.ptr.raw.add(slice.ptr.size * j);
    let slicei = core::slice::from_raw_parts_mut(i, slice.ptr.size);
    let slicej = core::slice::from_raw_parts_mut(j, slice.ptr.size);
    slicei.swap_with_slice(slicej);
}

/// Reverses a slice's elements.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_reverse(slice: &mut NSTDSlice) {
    let mut left_ptr = slice.ptr.raw.cast();
    let mut right_ptr = slice.end_unchecked().sub(slice.ptr.size);
    while left_ptr < right_ptr {
        let left_element = core::slice::from_raw_parts_mut(left_ptr, slice.ptr.size);
        let right_element = core::slice::from_raw_parts_mut(right_ptr, slice.ptr.size);
        left_element.swap_with_slice(right_element);
        left_ptr = left_ptr.add(slice.ptr.size);
        right_ptr = right_ptr.sub(slice.ptr.size);
    }
}

/// Shifts a slice `x` times to the right.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const NSTDUSize x` - Number of times to shift the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_shift_right(slice: &mut NSTDSlice, x: usize) {
    let rot = x % slice.size * slice.ptr.size;
    slice.as_byte_slice_mut().rotate_right(rot);
}

/// Shifts a slice `x` times to the left.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const NSTDUSize x` - Number of times to shift the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_shift_left(slice: &mut NSTDSlice, x: usize) {
    let rot = x % slice.size * slice.ptr.size;
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
    nstd_core_slice_copy_from_slice(s1, s2);
    s2.as_byte_slice_mut().fill(0);
}
