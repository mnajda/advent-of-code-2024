cmake_minimum_required(VERSION 3.12)
project(day8)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(SOURCES main.cpp)

add_executable(day8 ${SOURCES})

if(MSVC)
    set(RELEASE_OPTIONS "/O2")
    target_compile_options(day8 PRIVATE "/W4" "/permissive-" "$<$<CONFIG:RELEASE>:${RELEASE_OPTIONS}>")
else()
    target_compile_options(day8 PRIVATE "-O3" "-Wall" "-Wextra" )
endif()
