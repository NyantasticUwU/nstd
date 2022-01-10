# nstd_alloc
## 0.3.3
- Updated `nstd_core` to version 0.3.2.
## 0.3.2
- Updated `nstd_core` to version 0.3.1.
- Updated `windows` crate to version 0.29.0.
- Added syscalls for Linux and MacOS.
- Now making direct syscalls on Windows, resulting in huge performance gain.
## 0.3.1
- Using `deps` feature for nstd dependencies.
## 0.3.0
- Added `deps` feature for exposing dependencies.
- Added `NSTDHeap` type for single object heap allocation.
## 0.2.0
- `clib` feature.
## 0.1.0
- `nstd_std_alloc_{allocate, allocate_zeroed, deallocate, reallocate}` functions.
