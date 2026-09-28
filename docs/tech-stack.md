# Tech Stack Reference

This file is the **single source of truth** for the language- and toolchain-specific
choices in this repository. Every SDLC agent (and every human contributor) reads
this file before doing language-specific work. It was authored by the
**System Architect** stage against `docs/requirements/srs.md`.

Scope reminder: this is a **portfolio demo** of bidirectional Rust ↔ C++ FFI on
**Linux** (constraint C-1). Choices favor clarity, reproducibility, and minimal
dependencies (NFR-1, NFR-2, NFR-7) over breadth.

## Language & Standard
- **Primary languages:**
  - **Rust**, stable channel, **edition 2021**. Minimum Supported Rust Version
    (MSRV): **1.74** (any recent stable toolchain works; pin via `rust-toolchain.toml`
    is optional and left to the Developer).
  - **C++17**, compiled with **GCC (`g++` ≥ 11)** or **Clang (`clang++` ≥ 14)**.
- **Secondary/glue languages:** none. The FFI boundary is expressed in the C ABI
  (`extern "C"` on both sides); no IDL, no generated glue language.

**Why these:** Rust stable + C++17 are the lowest-friction, most widely available
combination on common Linux distros (NFR-1). C++17 gives `std::string_view` and
structured bindings — enough for readable marshalling code — without requiring a
C++20 toolchain. See design doc Design Decisions for the full rationale.

## Build System
- **Tool:** **CMake (≥ 3.22)** as the single top-level orchestrator, integrating
  **Cargo** via **Corrosion** (fetched at configure time with CMake `FetchContent`,
  pinned to a tagged release).
- **Entry point(s) / commands:**
  - Configure: `cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug`
  - Build everything: `cmake --build build`
  - Run the full demo (both directions): `cmake --build build --target run_demo`
  - Run all tests: `ctest --test-dir build --output-on-failure`
- **Build output layout:** all artifacts under `build/` (git-ignored). Cargo
  artifacts live in Cargo's own `target/` (managed by Corrosion); CMake links
  the resulting static libraries. No artifact is checked into the repo.

**Why CMake + Corrosion:** the demo needs *two* genuine executables — a Rust
binary that links a C++ library, and a C++ binary that links a Rust library —
plus one command to build, run, and test both. CMake is the industry-standard
C++ orchestrator, and Corrosion is the de-facto bridge for importing Cargo
targets into CMake. This gives a single coherent build graph and a portfolio-
grade demonstration of cross-language build orchestration. The trade-off (one
extra build-time dependency, Corrosion) is analyzed in the design doc.

## Dependency / Package Management
- **Rust:** **Cargo**. Manifest(s): a workspace `Cargo.toml` at the repo root plus
  one `Cargo.toml` per crate (`crates/interop_rs`, `crates/rust_demo`). Versions
  pinned via the checked-in `Cargo.lock`.
- **C++:** no package manager required — the only C++ dependency is the standard
  library. Any future C++ dependency would be introduced through CMake
  `FetchContent` or `find_package`, never vendored by hand.
- **Build-system dependency:** **Corrosion**, declared in the top-level
  `CMakeLists.txt` via `FetchContent_Declare(Corrosion GIT_TAG v0.5.x)` (pin an
  exact tag). This is the only third-party build dependency.
- **How to add a new dependency:**
  - Rust runtime/dev dep: `cargo add <crate>` in the relevant crate; commit the
    updated `Cargo.toml` + `Cargo.lock`.
  - C++/build dep: add a `FetchContent_Declare(...)` (pinned tag) or `find_package`
    to `CMakeLists.txt`; document it here.
  - The default posture is **no new dependencies** (NFR-7).

## Test Framework
- **Rust unit tests:** built-in `#[test]` / `cargo test`. Used for pure-Rust
  sanity checks of the exported Rust functions (no boundary crossing).
- **Boundary/integration tests:** driven by **CTest** so a single command covers
  both directions:
  - **Rust → C++** (FR-13): the `rust_demo` binary (Corrosion-linked to the C++
    `mathlib`) asserts on the values returned from C++ and exits non-zero on
    mismatch; registered as a CTest test.
  - **C++ → Rust** (FR-14): a dedicated C++ assertion test executable
    (`test_cpp_to_rust`) links the Rust `interop_rs` static library, calls the
    exported Rust functions, and asserts; registered as a CTest test.
- **Command to run the full suite:** `ctest --test-dir build --output-on-failure`
- **Command to run a single test:** `ctest --test-dir build -R rust_to_cpp`
  (or `-R cpp_to_rust`); pure-Rust units: `cargo test -p interop_rs`.
- **Coverage tool:** none required for this iteration (NFR-7). Optional:
  `cargo llvm-cov` for the Rust side if ever needed.

**Why:** keeps external test dependencies at zero — Rust's built-in harness plus
CTest (already present with CMake). No GoogleTest/Catch2 needed; the C++ test uses
plain `<cassert>` assertions, which is sufficient for a handful of boundary checks
and keeps the demo minimal and readable.

## Static Analysis / Linting / Formatting
Exact commands the **QA / Release** stages should run (all from repo root):

- **Rust format check:** `cargo fmt --all -- --check`
- **Rust lint:** `cargo clippy --all-targets --all-features -- -D warnings`
- **C++ format check:** `clang-format --dry-run --Werror $(find cpp -name '*.cpp' -o -name '*.hpp' -o -name '*.h')`
  (requires a checked-in `.clang-format`)
- **C++ lint (optional):** `clang-tidy` over `cpp/` using CMake's
  `CMAKE_EXPORT_COMPILE_COMMANDS=ON` compilation database.
- **Memory / UB checkers (important for FFI):**
  - Address + Undefined Behavior sanitizers on the C++ targets (stable path):
    configure a sanitized build
    `cmake -S . -B build-asan -DCMAKE_BUILD_TYPE=Debug -DCMAKE_CXX_FLAGS="-fsanitize=address,undefined -fno-omit-frame-pointer"`
    then `cmake --build build-asan && ctest --test-dir build-asan --output-on-failure`.
  - Valgrind on the demo binaries as an alternative/extra check:
    `valgrind --leak-check=full --error-exitcode=1 ./build/rust_demo`
    and `valgrind --leak-check=full --error-exitcode=1 ./build/cpp_demo`.
  - Note: Rust's `-Zsanitizer=address` needs nightly; it is **not** required.
    Running the linked binaries under the C++ ASan/UBSan build already exercises
    both sides of each boundary crossing.

## Style & Idioms
- **Ownership / resource management:**
  - Rust: the borrow checker + RAII; FFI structs are `#[repr(C)]` POD passed/
    returned **by value** (copied, no ownership transfer). Strings cross as
    **borrowed** NUL-terminated `*const c_char` — the callee only reads within the
    call and never stores the pointer. No heap allocation crosses the boundary,
    so no cross-language `free` pairing is needed.
  - C++: RAII; `std::string_view` / `std::strlen` for borrowed C strings; POD
    structs by value.
- **Error handling:**
  - Across the boundary: `extern "C"` functions are **total and panic/exception
    free** — the demonstrated operations have no error paths. Rust boundary
    functions must not unwind across FFI (avoid panics; a `catch_unwind` guard is
    unnecessary for these total functions but the rule stands). C++ boundary
    functions are effectively `noexcept`.
  - Inside each language: Rust uses `Result`/`assert!` (tests); C++ uses
    `<cassert>` (tests). No error handling is designed beyond what the demo needs.
- **`unsafe` policy (NFR-5):** `unsafe` appears only in the thin FFI declaration/
  call sites (the `extern "C"` blocks and `CStr::from_ptr`), each with a short
  `// SAFETY:` comment. It is localized to the boundary modules and nowhere else.
- **Naming / style guide:** Rust follows `rustfmt` defaults + Rust API guidelines;
  C++ follows the checked-in `.clang-format`. FFI symbols are prefixed by their
  implementing language: `cpp_*` for C++-implemented functions, `rust_*` for
  Rust-implemented functions.
- **Module/package layout:** see design doc "Build / Target Structure". C++ lives
  under `cpp/`, Rust crates under `crates/`, each interop direction in clearly
  named files (FR-12).

## CI/CD
- **Platform:** none required this iteration (Out of Scope §6). If added later:
  GitHub Actions running the build, `ctest`, and the lint/format/sanitizer
  commands above.
- **Caching strategy:** N/A (no CI yet). Locally, Cargo and CMake caches persist
  under `target/` and `build/`.
- **Packaging / release mechanism:** none (Out of Scope §6). No publishing to any
  registry.
- **Versioning scheme:** SemVer (a `VERSION` file / git tags may be introduced by
  the Release stage; not required for the demo to function).

## Documentation Conventions
- **API docs:** `rustdoc` (`cargo doc`) for the Rust crates; Doxygen-style comments
  are optional for the C++ headers. Not required to satisfy the SRS.
- **README / CHANGELOG:** a top-level `README.md` (FR-11) written by the Developer/
  Technical Writer stage, in standard GitHub Markdown. No CHANGELOG required yet.

---

**Usage note for agents:** treat every value above as decided. If a downstream
stage needs a change (e.g. a new dependency or a C++ standard bump), update this
file in the same change and note the rationale, rather than diverging silently.
