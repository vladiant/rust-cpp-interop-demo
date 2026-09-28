# CI/CD Pipeline — Rust ↔ C++ Interop Demo

**Stage:** Deployment (Release Engineer)
**Platform:** GitHub Actions — [.github/workflows/ci.yml](../../.github/workflows/ci.yml)
**Local mirror:** [Makefile](../../Makefile) (run `make help`)

This document describes what each pipeline stage does and how to reproduce it
locally. Every command in CI is copied verbatim from
[docs/tech-stack.md](../tech-stack.md) so CI and local development stay in sync.

## Trigger

The workflow runs on every `push` (any branch) and on every `pull_request`.
In-progress runs for the same ref are cancelled when a newer commit arrives
(`concurrency: cancel-in-progress`) to conserve free-tier minutes. Permissions
are read-only (`contents: read`).

## Runner & toolchain

- **Runner:** `ubuntu-24.04` — ships CMake ≥ 3.28, `g++` 13 (C++17), and
  `clang-format` 18, matching the QA verification environment. No extra system
  packages are needed except `valgrind` (installed only in the sanitizers job).
- **Rust:** installed via `dtolnay/rust-toolchain@stable`; the lint job adds the
  `rustfmt` and `clippy` components.
- **Caching:** `Swatinem/rust-cache@v2` caches the Cargo registry and the Cargo
  `target/` dir, keyed on `Cargo.lock`. Corrosion is re-fetched at configure time
  (pinned to tag `v0.5.1` in [CMakeLists.txt](../../CMakeLists.txt)).

## Pinned action versions

| Action | Version |
|--------|---------|
| `actions/checkout` | `v4` |
| `dtolnay/rust-toolchain` | `stable` |
| `Swatinem/rust-cache` | `v2` |

## Jobs

### 1. `build-test` — fast gate

The canonical CMake + CTest entry point (not bare `cargo build`).

| Step | Command |
|------|---------|
| Configure | `cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug` |
| Build | `cmake --build build` |
| Test suite | `ctest --test-dir build --output-on-failure` |
| Rust units | `cargo test -p interop_rs` |

### 2. `lint` — quality gates

| Step | Command |
|------|---------|
| Rust format | `cargo fmt --all -- --check` |
| Rust lint | `cargo clippy --all-targets --all-features -- -D warnings` |
| C++ format | `clang-format --dry-run --Werror $(find cpp -name '*.cpp' -o -name '*.hpp' -o -name '*.h')` |

### 3. `sanitizers` — FFI memory/UB checks

Kept as a **separate, heavier job** (two builds + a `valgrind` install) so it does
not slow the fast gate. It mirrors exactly what QA verified:

- **Valgrind** on both demo binaries from the plain `build/`:
  `valgrind --leak-check=full --error-exitcode=1 ./build/{rust_demo,cpp_demo}`
- **ASan/UBSan** scoped to the **C++-driven targets only** (`cpp_demo`,
  `test_cpp_to_rust`) in a separate `build-asan/` directory.

**Why the split:** a whole-tree `-fsanitize=address,undefined` build cannot link
the rustc/Corrosion-driven `rust_demo` (`undefined symbol: __asan_init`). Scoping
ASan/UBSan to the C++-linked targets and covering `rust_demo` with valgrind keeps
every command copy-paste runnable while still exercising both sides of every
boundary crossing. See the "Memory / UB checkers" nuance in
[docs/tech-stack.md](../tech-stack.md) and QA defect DEF-1 in
[docs/qa/verification-report.md](../qa/verification-report.md).

## Reproduce locally

The [Makefile](../../Makefile) wraps the same commands. Common entry points:

```bash
make            # list all targets (self-documenting help)
make check      # build + test + lint — mirrors the CI fast + lint gates
make run        # run the demo, both directions
make asan       # scoped ASan/UBSan on the C++-driven targets
make valgrind   # valgrind on both demo binaries
```

Or run the raw commands from the tables above directly — they are identical.

## Packaging / release

No registry publishing is in scope for this demo (per tech-stack.md "Out of
Scope §6"). Release plumbing is limited to:

- the reproducible CI pipeline above,
- the `Makefile` as the single obvious local entry point,
- SemVer git tags — **created by the PM's version-publish step at the very end**,
  not by this stage. No `VERSION` file is required for the demo to build or run.
