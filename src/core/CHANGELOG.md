# nstd_core
## Latest
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
