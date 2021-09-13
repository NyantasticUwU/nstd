cmake_minimum_required(VERSION 3.0.0)
project(nstd VERSION 0.1.0)
if (WIN32)
set(NSTD_WINDOWS_LIBS Userenv Ws2_32)
else()
set(NSTD_WINDOWS_LIBS)
endif()

# Library.
include_directories(include)
link_directories(lib)
add_library(nstd
    src/core/arch.cpp
    src/core/def.cpp
    src/core/char_types.cpp
    src/core/float_types.cpp
    src/core/int_types.cpp
    src/core/mem.cpp
    src/core/sys.cpp)
target_link_libraries(nstd
    nstd_fs
    nstd_io
    nstd_math
    nstd_os
    nstd_str
    nstd_thread
    nstd_time
    ${NSTD_WINDOWS_LIBS})

# Testing.
link_directories(build)
add_executable(test test/test.c)
target_link_libraries(test nstd)
add_test(Test test)
