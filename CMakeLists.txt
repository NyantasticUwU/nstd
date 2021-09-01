cmake_minimum_required(VERSION 3.0.0)
project(nstd VERSION 0.1.0)

# Library.
include_directories(include)
add_library(nstd
    src/core/arch.cpp
    src/core/char_types.cpp
    src/core/int_types.cpp
    src/core/mem.cpp
    src/core/sys.cpp)

# Testing.
link_directories(build)
add_executable(test test/test.c)
target_link_libraries(test nstd)
add_test(Test test)
