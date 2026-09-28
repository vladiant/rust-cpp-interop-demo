# QA Verification Report — Rust ↔ C++ Interop Demo

**Stage:** Testing (QA Engineer)
**Date:** 2026-09-28
**Inputs verified against:** [docs/requirements/srs.md](../requirements/srs.md),
[docs/design/design.md](../design/design.md), [docs/tech-stack.md](../tech-stack.md)

**Overall verdict: PASS** (with one non-blocking process/documentation defect, DEF-1).

---

## 1. Environment

| Tool         | Version                         |
|--------------|---------------------------------|
| CMake        | 3.28.3                          |
| g++          | 13.3.0 (Ubuntu 24.04)           |
| rustc / cargo| 1.98.1 (stable, installed via rustup) |
| clang-format | 18.1.3                          |
| valgrind     | 3.22.0                          |
| OS           | Linux (Ubuntu 24.04)            |

All required and optional QA tools were available; nothing was skipped for
tool unavailability.

---

## 2. Commands run and outcomes

Clean build: the stale `build/` directory was removed first for a true clean build.

| # | Command | Result |
|---|---------|--------|
| 1 | `rm -rf build && cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug` | **PASS** (configure, exit 0; Corrosion fetched, Rust toolchain detected) |
| 2 | `cmake --build build` | **PASS** (exit 0; `libinterop_rs.a`, `libmathlib.a`, `rust_demo`, `cpp_demo`, `test_cpp_to_rust` all built) |
| 3 | `cmake --build build --target run_demo` | **PASS** (exit 0; both directions printed correct values — see §3) |
| 4 | `ctest --test-dir build --output-on-failure` | **PASS** (3/3: `rust_to_cpp`, `cpp_to_rust`, `rust_units`) |
| 5 | `cargo test -p interop_rs` | **PASS** (6/6 unit tests) |
| 6 | `cargo fmt --all -- --check` | **PASS** (exit 0, no diff) |
| 7 | `cargo clippy --all-targets --all-features -- -D warnings` | **PASS** (exit 0, no warnings) |
| 8 | `clang-format --dry-run --Werror $(find cpp ...)` | **PASS** (exit 0) |
| 9 | ASan/UBSan build of full tree (`build-asan`, `CMAKE_CXX_FLAGS=-fsanitize=address,undefined`) | **FAIL to link `rust_demo`** — see DEF-1 |
| 10 | ASan/UBSan run of C++-driven targets `cpp_demo` + `test_cpp_to_rust` | **PASS** (exit 0; no ASan/UBSan reports, `detect_leaks=1`) |
| 11 | `valgrind --leak-check=full --error-exitcode=1 ./build/rust_demo` | **PASS** (0 errors, 0 bytes lost) |
| 12 | `valgrind --leak-check=full --error-exitcode=1 ./build/cpp_demo` | **PASS** (0 errors, 0 bytes lost) |

---

## 3. Demo output (both directions, verified values)

```
=== Rust -> C++ (rust_demo links libmathlib) ===
cpp_add(2, 3) = 5
cpp_vec3_dot(Vec3 { x: 1.0, y: 2.0, z: 3.0 }, Vec3 { x: 4.0, y: -5.0, z: 6.0 }) = 12
cpp_vec3_scale(Vec3 { x: 1.0, y: 2.0, z: 3.0 }, 2.0) = Vec3 { x: 2.0, y: 4.0, z: 6.0 }
cpp_utf8_len("héllo") = 6 (bytes)
Rust -> C++: all assertions passed.
=== C++ -> Rust (cpp_demo links libinterop_rs) ===
rust_fibonacci(10) = 55
rust_vec3_normalize({3.0, 0.0, 4.0}) = {0.6000, 0.0000, 0.8000}
rust_count_vowels("interop") = 3
C++ -> Rust: demo complete.
```

Independent hand-verification of marshalled values:

- `cpp_add(2,3)` = 5 ✔
- `cpp_vec3_dot((1,2,3),(4,-5,6))` = 4 − 10 + 18 = 12 ✔ (struct-by-value in, primitive out)
- `cpp_vec3_scale((1,2,3), 2.0)` = (2,4,6) ✔ (struct in, struct out)
- `cpp_utf8_len("héllo")` = 6 bytes ✔ (`h` + `é`(2 bytes) + `l` + `l` + `o`; borrowed `const char*`)
- `rust_fibonacci(10)` = 55 ✔
- `rust_vec3_normalize((3,0,4))` = (0.6, 0, 0.8), ‖·‖ = 1 ✔
- `rust_count_vowels("interop")` = 3 (i, e, o) ✔

Both directions cross primitives, a `#[repr(C)]`/POD `Vec3` struct by value, and a
borrowed NUL-terminated string, with values interpreted identically on both sides
(NFR-6 confirmed).

---

## 4. Test results (per test)

| Test | Kind | Result |
|------|------|--------|
| `rust_to_cpp` (CTest) | `rust_demo` asserts C++ returns across boundary | **PASS** |
| `cpp_to_rust` (CTest) | `test_cpp_to_rust` asserts Rust returns across boundary | **PASS** |
| `rust_units` (CTest → `cargo test -p interop_rs`) | pure-Rust logic units | **PASS** |
| `cargo test -p interop_rs` (direct) | 6 unit tests | **PASS** (6/6) |

No flaky or skipped tests observed across repeated runs.

---

## 5. Requirements traceability (SRS §5 acceptance criteria)

| Acceptance criterion (SRS §5) | FRs/NFRs | Evidence | Status |
|-------------------------------|----------|----------|--------|
| Clean checkout builds using only documented steps | FR-9, NFR-2 | Cmds 1–2 (fresh `build/`), exit 0 | **PASS** |
| Documented run command exercises both directions with human-readable output | FR-10 | Cmd 3 output (§3) | **PASS** |
| Rust→C++ passes a primitive, receives a primitive | FR-1, FR-2, FR-4 | `cpp_add(2,3)=5` printed & asserted in `rust_demo` | **PASS** |
| Rust→C++ marshals a string and/or struct | FR-3 | `cpp_vec3_dot`/`cpp_vec3_scale` (struct), `cpp_utf8_len` (string) | **PASS** |
| C++→Rust passes a primitive, receives a primitive | FR-5, FR-6, FR-8 | `rust_fibonacci(10)=55` printed (`cpp_demo`) & asserted (`test_cpp_to_rust`) | **PASS** |
| C++→Rust marshals a string and/or struct | FR-7 | `rust_vec3_normalize` (struct), `rust_count_vowels` (string) | **PASS** |
| Automated Rust→C++ test exists and passes | FR-13 | CTest `rust_to_cpp` PASS | **PASS** |
| Automated C++→Rust test exists and passes | FR-14 | CTest `cpp_to_rust` PASS | **PASS** |
| All tests run via one documented command, clear pass/fail | FR-15 | `ctest --test-dir build` → 3/3 PASS | **PASS** |
| README explains purpose/build/run/test reproducibly | FR-11, NFR-4 | [README.md](../../README.md) covers all four | **PASS** |
| Source layout makes each direction's code identifiable | FR-12 | `cpp/` vs `crates/`; direction-named files | **PASS** |
| Build and run succeed on a common Linux toolchain | NFR-1 | g++ 13 + Rust stable, Ubuntu 24.04 | **PASS** |
| FFI/unsafe localized to boundary, no broader unsafe | NFR-5 | `unsafe` only at `extern "C"` decl/call sites + `CStr::from_ptr`, each with `// SAFETY:`; clippy clean | **PASS** |
| Values cross without corruption or ownership errors | NFR-6 | Value checks (§3) + valgrind (0 leaks/errors) + ASan/UBSan clean on C++→Rust | **PASS** |

Additional NFR checks: NFR-3 (both directions test-covered) — PASS; NFR-7
(minimal, zero runtime deps) — PASS (only build-time dep is pinned Corrosion
`v0.5.1`).

**Every SRS acceptance-criteria item is PASS.** No criterion is failed or
not-verifiable.

---

## 6. Coverage gaps found and closed

- **Gap:** `rust_count_vowels` documents null-pointer → 0 behavior and has a
  Rust-side unit test, but that null case was never exercised **across the actual
  C++→Rust FFI boundary**.
- **Action:** Added a single assertion to
  [cpp/tests/test_cpp_to_rust.cpp](../../cpp/tests/test_cpp_to_rust.cpp)
  (`assert(rust_count_vowels(nullptr) == 0);`). Test-only change in the declared
  `<cassert>`/CTest framework; no feature code touched. Re-ran build + full
  `ctest`: still 3/3 PASS; clang-format clean.

Remaining minor gaps (not defects, noted for the record): the C++ `cpp_*`
functions' own null guard (`cpp_utf8_len(nullptr)`) is not tested directly, since
the Rust demo never passes null — acceptable for a demo per SRS A-3.

---

## 7. Defects

### DEF-1 — Documented ASan/UBSan recipe does not link the Rust-driven `rust_demo` (Severity: Low — process/docs, not functional)

- **Where:** [docs/tech-stack.md](../tech-stack.md) → "Memory / UB checkers"
  sanitized-build recipe.
- **Steps to reproduce:**
  ```
  cmake -S . -B build-asan -DCMAKE_BUILD_TYPE=Debug \
        -DCMAKE_CXX_FLAGS="-fsanitize=address,undefined -fno-omit-frame-pointer"
  cmake --build build-asan
  ```
- **Expected:** whole tree builds and `ctest --test-dir build-asan` runs under
  sanitizers.
- **Actual:** build fails while linking `rust_demo`:
  `rust-lld: error: undefined symbol: __asan_init` (and `__ubsan_handle_*`,
  `__asan_*`). Root cause: `CMAKE_CXX_FLAGS` instruments the C++ `libmathlib.a`,
  but `rust_demo` is linked by **rustc/Corrosion**, which does not pull in the
  ASan/UBSan runtime, so the sanitizer symbols in `libmathlib.a` are unresolved.
- **Impact:** the *documented* "configure a sanitized build then `ctest`" path
  cannot run as written for this project layout. It is **not** a defect in the
  demo's functional interop code — the sanitizer confirmed clean results on the
  C++→Rust boundary (Cmd 10), and valgrind confirmed both binaries leak/error-free
  (Cmds 11–12).
- **Workaround used by QA (verification still achieved):** built and ran only the
  C++-driven sanitized targets (`cpp_demo`, `test_cpp_to_rust`) under ASan/UBSan
  (clean), and used valgrind on both demo binaries (clean) to cover the
  `rust_demo` crossing that ASan could not link.
- **Routing:** this traces to the tech-stack documentation / build recipe, not to
  a requirement or the demo code. Recommend the PM route it to the Developer /
  System Architect to either (a) scope the sanitizer recipe to the C++-driven
  targets + valgrind for `rust_demo` (matching what QA did), or (b) add the ASan
  runtime to the Rust link flags (`RUSTFLAGS`/Corrosion) so the full tree links.
  Documentation-only or small-config fix; does **not** block release of the demo.

No functional defects were found.

---

## 8. Verdict & Handoff

- **Build (clean):** PASS
- **Demo (both directions, correct values):** PASS
- **Tests:** `ctest` 3/3 PASS; `cargo test -p interop_rs` 6/6 PASS
- **Quality checks:** `cargo fmt --check` PASS, `cargo clippy -D warnings` PASS,
  `clang-format --Werror` PASS
- **FFI memory/UB checks:** ASan/UBSan on C++→Rust boundary PASS; valgrind on both
  binaries PASS (0 leaks/errors). Full-tree ASan build blocked by DEF-1
  (low-severity docs/process issue, functionally worked around).
- **SRS acceptance criteria:** all items PASS.
- **Defects:** DEF-1 (Low, documentation/process — non-blocking); no functional
  defects.

**QA PASSES.** All SRS acceptance criteria are independently verified and the
interop is functionally correct and memory-clean. The work is **ready for the
Release Engineer**. DEF-1 should be routed to the System Architect/Developer to
correct the sanitizer recipe in `docs/tech-stack.md`, but it does not block release
of the demo.
