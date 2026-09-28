# Project Status — Rust ↔ C++ Interop Demo

**Last updated:** 2026-09-28
**Current stage:** Documentation complete — **ready for versioning (PM to cut the tag).**

## Summary

A bidirectional Rust ↔ C++ FFI interop demo. Both directions ship end-to-end as
two genuine executables built by a single CMake + Corrosion build graph, with a
CTest suite and GitHub Actions CI. The feature set is complete and independently
QA-verified against every SRS acceptance criterion.

## Stage status

| SDLC stage        | Owner              | Status      | Artifact / evidence |
|-------------------|--------------------|-------------|---------------------|
| Requirements      | Business Analyst   | ✅ Complete | [srs.md](requirements/srs.md) |
| Architecture      | System Architect   | ✅ Complete | [tech-stack.md](tech-stack.md) |
| Design            | System Architect   | ✅ Complete | [design.md](design/design.md) |
| Implementation    | Developer          | ✅ Complete | `crates/`, `cpp/`, `CMakeLists.txt` |
| Testing / QA      | QA Engineer        | ✅ PASS     | [verification-report.md](qa/verification-report.md) |
| CI / Deployment   | Release Engineer   | ✅ Complete | [pipeline](ci-cd/rust-cpp-interop-demo-pipeline.md), `.github/workflows/ci.yml` |
| Documentation     | Technical Writer   | ✅ Complete | `README.md`, `CHANGELOG.md`, this file |
| Versioning / tag  | Project Manager    | ⏳ Pending  | `[Unreleased]` in `CHANGELOG.md` |

## Feature completeness

Both interop directions ship with primitives, a `#[repr(C)]` `Vec3` struct by
value, and a borrowed `const char*` string crossing the boundary:

- **Rust → C++:** `cpp_add`, `cpp_vec3_dot`, `cpp_vec3_scale`, `cpp_utf8_len`.
- **C++ → Rust:** `rust_fibonacci`, `rust_vec3_normalize`, `rust_count_vowels`.

## QA verdict

**PASS.** All SRS acceptance criteria independently verified; interop is
functionally correct and memory-clean (valgrind: 0 leaks/errors on both
binaries; ASan/UBSan clean on the C++-driven targets).

- CTest: 3/3 PASS (`rust_to_cpp`, `cpp_to_rust`, `rust_units`).
- `cargo test -p interop_rs`: 6/6 PASS.
- `cargo fmt --check`, `cargo clippy -D warnings`, `clang-format --Werror`: PASS.

**DEF-1 (resolved):** The original whole-tree ASan/UBSan recipe in
`docs/tech-stack.md` could not link the rustc-driven `rust_demo`
(`undefined symbol: __asan_init`). The System Architect corrected the recipe to
the scoped ASan-on-C++-targets + valgrind-on-both approach that QA and CI use;
`docs/tech-stack.md` and `.github/workflows/ci.yml` now match. Low-severity
documentation issue, closed before release. See
[verification-report.md §7](qa/verification-report.md).

## CI status

GitHub Actions ([.github/workflows/ci.yml](../.github/workflows/ci.yml)) runs on
every push and pull request:

- `build-test` — CMake configure/build + CTest + `cargo test`.
- `lint` — rustfmt, clippy (`-D warnings`), clang-format.
- `sanitizers` — scoped ASan/UBSan on C++-driven targets + valgrind on both
  binaries.

## Next action

Project Manager to assign the SemVer version, replace the `[Unreleased]` heading
in `CHANGELOG.md` with the version and date, and cut the annotated git tag.
