# Latest
### `nstd.gl`
- Added `nstd_gl_device_submit`.
- Removed `nstd_gl_state_render`.
- Added ability to present `NSTDGLSurfaceTexture`s.
- Added ability to create `NSTDGLRenderPass`es.
- `nstd_gl_instance_new` now takes an `NSTDGLBackend`.
- Split `pipeline` into `render.[pass|pipeline]` and `shader.module`.
- Added `NSTDGLTextureView`.
- Added `NSTDGLCommand[Buffer|Encoder]`.
- Added `device.info` module.
- Added `device.handle` module.
### `nstd.gui`
- Renamed `nstd_gui_window_create[_child]` to `nstd_gui_window_new[_child]`.
# 0.9.8
### `nstd.gl`
- Added `NSTDGLSurfaceTexture`.
- Added ability to create `NSTDSurfaceConfig`s.
- Added `surface.config` module.
- Renamed `NSTDGLSurfaceConfiguration` to `NSTDGLSurfaceConfig`.
- Removed `NSTDGLStateDescriptor`.
- Added `NSTDGLTextureFormat`.
- Added ability to create `NSTDGLDevice`s.
- Removed `NSTDGLQueue`.
- Added ability to create `NSTDGLDeviceHandle`s.
- Added ability to create `NSTDGLSurface`s.
- Added `NSTDGLInstance`.
- Put all types and helper functions in their own modules.
### `nstd.os`
- Reimplemented `nstd_os_windows_io_print[_line]`.
- Added stdin support to `windows.io`.
# 0.9.7
### `nstd.events`
- `NSTDEventCallbacks::on_window_[cursor_moved|line_scroll]` now use primitives instead of
  `NSTDSlice`.
- Make use of `nstd.gui.def`.
### `nstd.gui`
- Added `def` module.
# 0.9.6
### `nstd`
- Fix Cargo.toml.
# 0.9.5
### `nstd`
- Updated `windows-sys` to version 0.33.0.
### `nstd.alloc`
- Using `Layout::from_size_align_unchecked` with align of 1 on non-linux/windows platforms.
### `nstd.gl`
- Removed `NSTDGLState::size`.
### `nstd.input`
- Removed the `touch` module, this is of course temporary.
- Made `key` easier to maintain; no more implicit key type casting.
# 0.9.4
### `nstd.alloc`
- `NSTDAllocator` now owns an `NSTDErrorCode`.
### `nstd.core`
- Added `nstd_core_cstr_len_with_null`.
- Fixed `nstd_core_cstr_len` returning the size including the null terminator.
### `nstd.fs`
- `nstd_fs_file_open` now takes an `NSTDStr` for the file name.
# 0.9.3
### `nstd.collections`
- Make use of `NSTDBitValue`.
- Added `nstd_collections_bit_mask_set_all`.
### `nstd.core`
- Added `NSTDBitValue`.
- Cleaned `char_types`.
### `nstd.events`
- Added `NSTDEventData` struct.
### `nstd.os`
- Added `windows.io.handle` module.
# 0.9.2
### `nstd.events`
- Reimplemented.
### `nstd.input`
- Renamed `NSTDMouseButtonState`'s members.
# 0.9.1
### `nstd`
- Cleaned.
### `nstd.collections`
- `nstd_collections_bit_mask_new` now uses `nstd_core_math_div_ceil_u32`.
- Added `nstd_collections_bit_mask_free`.
- Optimized `nstd_collections_bit_mask_new`.
### `nstd.core`
- Added `nstd_core_math_div_floor_*`.
- Added `nstd_core_math_div_ceil_*`.
- Optimized `nstd_core_slice_move`.
- Made `nstd_core_slice_reverse` ~10x faster.
- Use `clamp` impl on value for `nstd_core_math_clamp_*`.
### `nstd.fs`
- Fixed constant names in `file`.
# 0.9.0
### `nstd`
- Use `windows-sys`.
- `impl<T> From<&[T]> for NSTD[String|Vec]`.
- Renamed `nstd_str` to `nstd_string`.
### `nstd.alloc`
- `NSTDAudioSink` is now `NSTDAny`.
- Added `nstd_alloc_heap_from_raw`.
- Added `nstd_alloc_heap_from_existing`.
- Low level functions now make calls to `nstd.os.linux.alloc`.
- Added `NSTDAllocator`.
- `nstd.alloc` now makes use of `NSTDErrorCode`.
### `nstd.audio`
- Make use of nstd's primitives.
### `nstd.collections`
- Added `NSTDStack`.
- Added `nstd_collections_vec_from_existing`.
- Added `NSTDRC` reference counter type.
- Added `NSTDBitMask` type and associated functions.
### `nstd.core`
- Added `NSTD_CORE_NULL` constant.
- Added `nstd_core_cstr_as_slice`.
- Moved the `cstr` module out of `str`.
### `nstd.env`
- Make use of nstd's primitives.
### `nstd.events`
- `nstd_events_event_loop_run` now takes `NSTDBool.
### `nstd.fs`
- Make use of nstd's primitives.
- `NSTDFile` now "inherits" `NSTDIOStream`.
- Added `file` module.
### `nstd.gui`
- `NSTD[Display|Window]` are now `NSTDAny`.
- `nstd_gui_[display|window]_get_scale_factor` now returns `NSTDFloat64`.
- `nstd_gui_window_set_title` now takes `NSTDStr`.
### `nstd.image`
- `nstd_image_open` now uses `NSTDChar`.
### `nstd.input`
- `nstd_input_*` functions now return `NSTDBool`.
### `nstd.io`
- Removed functional IO.
- Added basic IO streams.
### `nstd.math`
- Make use of nstd's primitives.
### `nstd.net`
- Make use of nstd's primitives.
- Split `tcp` and `udp` into different modules.
### `nstd.os`
- Added `windows.alloc.heap` module.
- Added `windows.def` module.
- Added `windows.io` module.
- Added `linux.alloc` module.
### `nstd.proc`
- Make use of nstd's primitives.
- Added `NSTDExitCode` typedef.
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
