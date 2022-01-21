# nstd_core
## Latest
- The `slice` module now uses `NSTDAny` instead of `void *`.
- More to `char_types`.
- Renamed `NSTDEndian` to `NSTDCPUEndian`.
- Moved `NSTDEndian` into `platform`.
- `NSTDSlice` now uses `NSTDPointer`.
- Added `NSTDPlatform` and `nstd_core_platform_target`.
- Renamed `NSTDPointer`'s `data` to `raw`.
- The `nstd_core_str_to_*` functions now use `NSTDBool` for error checking.
## 0.4.0
- `nstd_core_abort` only panics if the `panics` feature is enabled.
- Added `nstd_core_str_from_cstring_with_null`.
- The result of `nstd_core_str_from_cstring` no longer includes the null terminator.
## 0.3.2
- `core.pointer` now uses `NSTDAny`.
- Added `nstd_core_math_[degrees|radians]_f[32|64]`.
- Added `nstd_core_char_types_from_digit`.
- Added `nstd_core_char_types_check`.
- Added `NSTDAny` type.
- Added `sys` module.
- Inlined `nstd_core_char_types_to_[uppercase | lowercase]`.
- `core.float_types`, `core.math` and `core.str` now use the `NSTDFloat*` typedefs.
- Added `NSTDFloat*` typedefs.
- Added `NSTDChar*` typedefs.
- Big changes to the `math` module.
- `core.[[char|float]_types|slice|str]` functions now return `NSTDBool` where possible.
- Added primitive typedefs.
- Added `NSTDBool` type.
- Added `str` module.
- Added `nstd_core_slice_compare`.
## 0.3.1
- Removed `mem`, use the `slice` module instead.
- Removed `nstd_core_def_null`.
- Added `math` module.
- Added `nstd_core_arch_endian`.
- Added `NSTDEndian`.
- Added `nstd_core_char_types_size`.
- Added `nstd_core_slice_fill_range`.
- Added `nstd_core_slice_move`.
- Added `nstd_core_slice_count`.
- Added `nstd_core_slice_find_{first | last}`.
## 0.3.0
- Added `deps` feature for exposing dependencies.
- Added `nstd_core_pointer_write`.
## 0.2.3
- Removed `AsRef<[T]>` and `AsMut<[T]>` implementations for `NSTDSlice`.
- Added `as_byte_slice[_mut]` methods to `NSTDPointer`.
## 0.2.2
- Added `NSTDPointer` type.
- Added `as_byte_slice[_mut]` methods to `NSTDSlice`.
## 0.2.1
- `nstd_core_slice_new` now takes a `void *` instead of an `NSTDByte *`.
- Lower-level modules are now `pub mod {mod};` instead of `pub use {mod}::*;`.
- `nstd_core_slice_shift_{left | right}` no longer panic when `x` is greater than `slice.size`.
## 0.2.0
- `clib` feature.
## 0.1.3
- `NSTDView` renamed to `NSTDSlice`.
## 0.1.2
- `NSTD*Range` was added.
- `NSTDView` was added.
- `platform` module.
## 0.1.1
- `NSTDUnichar` type added.
- `{char, float, int}_types` modules.
## 0.1.0
- `arch`, `def`, and `mem` modules.
