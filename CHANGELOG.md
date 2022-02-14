# Latest
### `nstd`
- Renamed `nstd_str` to `nstd_string`.
- Require the `nstd_os` feature when using any `nstd.os.*` modules.
### `nstd.alloc`
- Added `nstd_alloc_heap_from_raw`.
- Added `nstd_alloc_heap_from_existing`.
- Low level functions now make calls to `nstd.os.linux.alloc`.
- Added `NSTDAllocator`.
- `nstd.alloc` now makes use of `NSTDErrorCode`.
### `nstd.audio`
- `nstd_audio_sink_append_from_file` now takes a ptr to `NSTDFile`.
### `nstd.collections`
- Added `nstd_collections_vec_from_existing`.
- Added `NSTDRC` reference counter type.
- Added `NSTDBitMask` type and associated functions.
### `nstd.core`
- Added `NSTD_CORE_NULL` constant.
- Added `nstd_core_cstr_as_slice`.
- Moved the `cstr` module out of `str`.
### `nstd.fs`
- Make use of nstd's primitives.
- `NSTDFile` now "inherits" `NSTDIOStream`.
- Added `file` module.
### `nstd.io`
- Removed functional IO.
- Added basic IO streams.
### `nstd.os`
- Added `windows.alloc.heap` module.
- Added `windows.def` module.
- Added `windows.io` module.
- Added `linux.alloc` module.
### `nstd.string`
- Added `nstd_string_from_existing`.
- Make use of nstd's primitives.
- `nstd_string_from_*` functions now use strong primitives.
### `nstd.sys`
- Now using `NSTDString`.
### `nstd.time`
- Make use of nstd's primitives.
# 0.8.0
### `nstd`
- Updated `image` crate to version 0.24.0.
- Moved `nstd.os` to `nstd.sys`.
### `nstd.alloc`
- Avoid platform specific code, instead make calls to `nstd.os.*.alloc`.
### `nstd.collections`
- The `vec` module now makes use of `NSTDErrorCode`.
- The `vec` module now makes use of `NSTDAny[Const]`.
- `NSTDVec` now uses `NSTDSlice`.
### `nstd.core`
- Added `nstd.core.str.cstr` module.
- The `str` module now makes use of `NSTDErrorCode`.
- Added `NSTDErrorCode`.
### `nstd.events`
- `NSTDEventData::mouse_delta` now uses `NSTDFloat64`.
### `nstd.gl`
- `NSTDGLDeviceInfo` now uses `NSTDString`.
### `nstd.os`
- Basic Windows heap allocation.
### `nstd.str`
- Removed all C string functions.
### `nstd.thread`
- `nstd_thread_join` now uses `NSTDErrorCode`.
- `nstd_thread_sleep` now uses `NSTDFloat64`.
- Added `NSTDThreadReturn`.
# 0.7.0
### `nstd`
- Fix Cargo.toml.
### `nstd.core`
- Removed false comment on `nstd_core_abort`.
# 0.6.0
- Conjoined all crates into `nstd`.
# 0.5.1
- Updated `nstd_gl` to version 0.5.1.
# 0.5.0
- Updated `nstd_core` to version 0.5.0.
- Updated `nstd_alloc` to version 0.5.0.
- Updated `nstd_audio` to version 0.5.0.
- Updated `nstd_collections` to version 0.5.0.
- Updated `nstd_env` to version 0.5.0.
- Updated `nstd_events` to version 0.5.0.
- Updated `nstd_fs` to version 0.5.0.
- Updated `nstd_gl` to version 0.5.0.
- Updated `nstd_gui` to version 0.5.0.
- Updated `nstd_image` to version 0.5.0.
- Updated `nstd_input` to version 0.5.0.
- Updated `nstd_io` to version 0.5.0.
- Updated `nstd_math` to version 0.5.0.
- Updated `nstd_net` to version 0.5.0.
- Updated `nstd_os` to version 0.5.0.
- Updated `nstd_proc` to version 0.5.0.
- Updated `nstd_str` to version 0.5.0.
- Updated `nstd_thread` to version 0.5.0.
- Updated `nstd_time` to version 0.5.0.
# 0.4.0
- Updated `nstd_core` to version 0.4.0.
- Updated `nstd_alloc` to version 0.4.0.
- Updated `nstd_audio` to version 0.4.0.
- Updated `nstd_collections` to version 0.4.0.
- Updated `nstd_env` to version 0.4.0.
- Updated `nstd_events` to version 0.4.0.
- Updated `nstd_fs` to version 0.4.0.
- Updated `nstd_gl` to version 0.4.0.
- Updated `nstd_gui` to version 0.4.0.
- Updated `nstd_image` to version 0.4.0.
- Updated `nstd_input` to version 0.4.0.
- Updated `nstd_io` to version 0.4.0.
- Updated `nstd_math` to version 0.4.0.
- Updated `nstd_net` to version 0.4.0.
- Updated `nstd_os` to version 0.4.0.
- Updated `nstd_proc` to version 0.4.0.
- Updated `nstd_str` to version 0.4.0.
- Updated `nstd_thread` to version 0.4.0.
- Updated `nstd_time` to version 0.4.0.
# 0.3.5
- Updated `nstd_str` to version 0.3.1.
# 0.3.4
- Updated `nstd_core` to version 0.3.2.
- Updated `nstd_alloc` to version 0.3.3.
- Updated `nstd_audio` to version 0.3.3.
- Updated `nstd_collections` to version 0.3.3.
- Updated `nstd_env` to version 0.3.3.
- Updated `nstd_events` to version 0.3.2.
- Updated `nstd_fs` to version 0.3.3.
- Updated `nstd_gl` to version 0.3.4.
- Updated `nstd_gui` to version 0.3.4.
- Updated `nstd_image` to version 0.3.3.
- Updated `nstd_input` to version 0.3.2.
- Updated `nstd_io` to version 0.3.3.
# 0.3.3
- Updated `nstd_core` to version 0.3.1.
- Updated `nstd_alloc` to version 0.3.2.
- Updated `nstd_audio` to version 0.3.2.
- Updated `nstd_collections` to version 0.3.2.
- Updated `nstd_env` to version 0.3.2.
- Updated `nstd_fs` to version 0.3.2.
- Updated `nstd_gl` to version 0.3.3.
- Updated `nstd_gui` to version 0.3.3.
- Updated `nstd_image` to version 0.3.2.
- Updated `nstd_io` to version 0.3.2.
# 0.3.2
- Updated `nstd_gl` to version 0.3.2.
- Updated `nstd_gui` to version 0.3.2.
- Updated `nstd_events` to version 0.3.1.
- Updated `nstd_input` to version 0.3.1.
# 0.3.1
- Added `deps` feature.
- Added `clib` feature.
# 0.3.0
- Updated all crates to 0.3.0 or 0.3.1 if available.
# 0.2.0
- Updated all crates to 0.2.0.
# 0.1.0
- First public release.
