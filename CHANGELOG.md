# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-09-28

Initial release. Feature set is complete and QA-verified (all SRS acceptance
criteria PASS).

### Added

- **Bidirectional Rust ↔ C++ FFI interop demo** over a hand-written C ABI
  (`extern "C"`) on both sides, with no binding generator.
- **Rust → C++ direction** (`rust_demo` links the C++ `libmathlib` static
  library and calls its `cpp_*` symbols):
  - `cpp_add(i32, i32) -> i32` — primitive in, primitive out.
  - `cpp_vec3_dot(Vec3, Vec3) -> f64` — `#[repr(C)]`/POD struct by value in,
    primitive out.
  - `cpp_vec3_scale(Vec3, f64) -> Vec3` — struct by value in and out.
  - `cpp_utf8_len(const char*) -> size_t` — borrowed NUL-terminated string in,
    byte length out.
- **C++ → Rust direction** (`cpp_demo` and `test_cpp_to_rust` link the Rust
  `libinterop_rs` static library and call its `#[no_mangle] extern "C"` symbols):
  - `rust_fibonacci(u32) -> u32` — primitive in, primitive out.
  - `rust_vec3_normalize(Vec3) -> Vec3` — `#[repr(C)]` struct by value in and out.
  - `rust_count_vowels(const char*) -> u32` — borrowed NUL-terminated string in,
    count out (null pointer treated as empty).
- **Shared `Vec3` POD** (`#[repr(C)]` in Rust, matching C++ `struct`) marshalled
  by value across both directions.
- **CMake + Corrosion build** as the single top-level orchestrator, fetching a
  pinned Corrosion release at configure time and linking Cargo static libraries
  into the C++ build graph.
- **`run_demo` target** that runs both demo binaries and prints labeled,
  human-readable results for each boundary crossing.
- **Test suite via CTest** (`rust_to_cpp`, `cpp_to_rust`, `rust_units`) plus the
  pure-Rust unit tests (`cargo test -p interop_rs`), all runnable with one
  command.
- **GitHub Actions CI** (`.github/workflows/ci.yml`) with three jobs: build+test,
  lint/format gates (rustfmt, clippy, clang-format), and scoped ASan/UBSan +
  valgrind FFI memory/UB checks.
- **Self-documenting root `Makefile`** wrapping the canonical CMake/CTest
  workflow (`make help`, `build`, `run`, `test`, `lint`, `asan`, `valgrind`,
  `check`).
- **Project documentation:** requirements (SRS), design specification, tech-stack
  reference, CI/CD pipeline description, QA verification report, and a
  portfolio-facing `README.md`.

### Security

- `unsafe` is confined to the FFI declaration/call sites (the `extern "C"` blocks
  and the single `CStr::from_ptr` call), each annotated with a `// SAFETY:`
  comment. Boundary functions are total and never unwind across FFI. No heap
  allocation crosses the boundary, so no cross-language `free` pairing is needed.

[0.1.0]: https://github.com/vladiant/rust-cpp-interop-demo/releases/tag/v0.1.0
