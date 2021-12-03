cmake_minimum_required(VERSION 3.0.0)
project(nstd VERSION 0.1.0)
if (WIN32)
set(NSTD_WINDOWS_LIBS Bcrypt Comctl32 d3dcompiler Dwmapi Imm32 Userenv UxTheme Winmm Ws2_32)
else()
set(NSTD_WINDOWS_LIBS)
endif()

# Testing.
include_directories(include)
link_directories(lib)
add_executable(test test/test.c)
target_link_libraries(test
    nstd_alloc
    nstd_audio
    nstd_collections
    nstd_core
    nstd_env
    nstd_events
    nstd_fs
    nstd_gl
    nstd_gui
    nstd_image
    nstd_input
    nstd_io
    nstd_math
    nstd_net
    nstd_os
    nstd_proc
    nstd_str
    nstd_thread
    nstd_time
    ${NSTD_WINDOWS_LIBS})
add_test(Test test)
