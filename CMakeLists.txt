cmake_minimum_required(VERSION 3.22)

project(rust_cpp_interop_demo LANGUAGES CXX)

# ---- Global settings -------------------------------------------------------
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)  # clang-tidy compilation database

if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
  set(CMAKE_BUILD_TYPE Debug CACHE STRING "Build type" FORCE)
endif()

# ---- Corrosion: import the Cargo workspace as CMake targets ----------------
include(FetchContent)
FetchContent_Declare(
  Corrosion
  GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
  GIT_TAG v0.5.1  # pinned (tech-stack.md: exact Corrosion tag)
)
FetchContent_MakeAvailable(Corrosion)

# Imports the workspace crates as targets: `interop_rs` (staticlib) and
# `rust_demo` (bin). Corrosion attaches Rust's std/runtime link requirements to
# the `interop_rs` target so C++ can link it cleanly.
corrosion_import_crate(MANIFEST_PATH "${CMAKE_SOURCE_DIR}/Cargo.toml")

# ---- C++ side: Rust -> C++ callees -----------------------------------------
# libmathlib: C++ static library called from the Rust `rust_demo` binary.
add_library(mathlib STATIC cpp/src/mathlib.cpp)
target_include_directories(mathlib PUBLIC "${CMAKE_SOURCE_DIR}/cpp/include")

# ---- Rust -> C++ wiring ----------------------------------------------------
# The Rust `rust_demo` binary links the C++ `libmathlib` static library.
corrosion_link_libraries(rust_demo mathlib)

# ---- C++ -> Rust wiring ----------------------------------------------------
# cpp_demo: C++ binary that calls into the Rust `interop_rs` static library.
add_executable(cpp_demo cpp/demo/cpp_demo.cpp)
target_include_directories(cpp_demo PRIVATE "${CMAKE_SOURCE_DIR}/cpp/include")
target_link_libraries(cpp_demo PRIVATE interop_rs)

# test_cpp_to_rust: dedicated C++ assertion test that links `interop_rs`.
add_executable(test_cpp_to_rust cpp/tests/test_cpp_to_rust.cpp)
target_include_directories(test_cpp_to_rust PRIVATE "${CMAKE_SOURCE_DIR}/cpp/include")
target_link_libraries(test_cpp_to_rust PRIVATE interop_rs)

# ---- Demo runner (FR-10): run both directions in one command ---------------
add_custom_target(run_demo
  COMMAND $<TARGET_FILE:rust_demo>
  COMMAND $<TARGET_FILE:cpp_demo>
  DEPENDS rust_demo cpp_demo
  USES_TERMINAL
  COMMENT "Running both interop directions (Rust -> C++, then C++ -> Rust)")

# ---- Tests (FR-13, FR-14, FR-15) -------------------------------------------
enable_testing()

# FR-13: Rust -> C++ boundary test (rust_demo asserts on C++ return values).
add_test(NAME rust_to_cpp COMMAND $<TARGET_FILE:rust_demo>)

# FR-14: C++ -> Rust boundary test (asserts on Rust return values).
add_test(NAME cpp_to_rust COMMAND $<TARGET_FILE:test_cpp_to_rust>)

# Pure-Rust unit tests of the exported logic (no boundary crossing).
find_program(CARGO_EXECUTABLE cargo REQUIRED)
add_test(
  NAME rust_units
  COMMAND ${CARGO_EXECUTABLE} test -p interop_rs
          --manifest-path "${CMAKE_SOURCE_DIR}/Cargo.toml")
