cmake_minimum_required(VERSION 3.20)
project(OpenSteamToolTools LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CONFIGURATION_TYPES "Debug;Release" CACHE STRING "" FORCE)

add_executable(ipc_codegen
    ipc_codegen/ipc_codegen.cpp
)
target_compile_features(ipc_codegen PRIVATE cxx_std_20)


add_executable(extract_tickets
    extract_tickets/extract_tickets.cpp
)
target_compile_features(extract_tickets PRIVATE cxx_std_20)

