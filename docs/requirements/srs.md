# Software Requirements Specification — Rust ↔ C++ Interop Demo

## 1. Introduction

### 1.1 Purpose
This document specifies the requirements for a small, high-quality portfolio
project that demonstrates **bidirectional Foreign Function Interface (FFI)
interop between Rust and C++**. It shows, end-to-end, both:

- **Rust → C++**: Rust code calling into a C++ function/library.
- **C++ → Rust**: C++ code calling into a Rust function/library.

The document defines what the software must do (functional requirements), the
qualities it must exhibit (non-functional requirements), the boundaries of the
effort, and the acceptance criteria a QA Engineer can later verify.

### 1.2 Scope
The project is a **demonstration / portfolio piece**, not a production library.
Its value is clarity: a reader browsing the repository should quickly understand
that both interop directions work and how data crosses the language boundary.

In scope:
- At least one working example per direction.
- Real data marshalling across the boundary (primitives plus at least one string
  and/or struct/aggregate), not empty no-argument calls.
- A reproducible local build and a way to run the demo.
- Automated tests proving both directions work.

Out of scope items are listed in Section 6.

### 1.3 Definitions
- **FFI** — Foreign Function Interface; mechanism by which code written in one
  language calls code compiled from another.
- **Boundary** — The point at which control and data pass between the Rust and
  C++ sides.
- **Marshalling** — Converting a value into a representation that is valid and
  correctly interpreted on the other side of the boundary.
- **Demo runner** — The single command or documented entry point that builds
  and/or executes the demonstration.

### 1.4 Intended audience
- **System Architect** — designs against these requirements and records the
  chosen toolchain and build system in `docs/tech-stack.md`.
- **Developers** — implement the examples.
- **QA Engineer** — verifies the acceptance criteria.
- **Portfolio reviewers** — browse the repository to assess the author's skills.

---

## 2. Functional Requirements

Each requirement is independently testable. Requirements are written to be
independent of any specific toolchain or build system.

### 2.1 Rust → C++ direction
- **FR-1**: The project shall provide a C++ function that is callable from Rust
  across the FFI boundary.
- **FR-2**: The Rust → C++ example shall pass at least one primitive value
  (e.g. an integer or floating-point number) into the C++ function and receive a
  primitive result back.
- **FR-3**: The Rust → C++ example shall demonstrate marshalling of at least one
  non-primitive value — a string and/or a struct/aggregate — across the
  boundary in at least one direction of the call.
- **FR-4**: The result returned from C++ to Rust shall be observable by the Rust
  side (e.g. printed, returned, or asserted) so the interaction is verifiable.

### 2.2 C++ → Rust direction
- **FR-5**: The project shall provide a Rust function that is callable from C++
  across the FFI boundary.
- **FR-6**: The C++ → Rust example shall pass at least one primitive value into
  the Rust function and receive a primitive result back.
- **FR-7**: The C++ → Rust example shall demonstrate marshalling of at least one
  non-primitive value — a string and/or a struct/aggregate — across the
  boundary in at least one direction of the call.
- **FR-8**: The result returned from Rust to C++ shall be observable by the C++
  side (e.g. printed, returned, or asserted) so the interaction is verifiable.

### 2.3 Demo runner and documentation
- **FR-9**: The project shall provide a single documented way to build the demo
  from a clean checkout.
- **FR-10**: The project shall provide a single documented way to run the demo,
  which exercises both interop directions and produces human-readable output
  showing each direction working.
- **FR-11**: The repository shall include a README (or equivalent top-level
  document) that explains what the project demonstrates, how to build it, how to
  run it, and how to run the tests.
- **FR-12**: Source files on each side of the boundary shall be organized and
  named such that a reader can identify which code belongs to the Rust → C++
  example and which belongs to the C++ → Rust example.

### 2.4 Tests
- **FR-13**: The project shall include an automated test that proves the
  Rust → C++ direction works by asserting on the value(s) returned across the
  boundary.
- **FR-14**: The project shall include an automated test that proves the
  C++ → Rust direction works by asserting on the value(s) returned across the
  boundary.
- **FR-15**: The automated tests shall be runnable via a single documented
  command and shall report a clear pass/fail result.

---

## 3. Non-Functional Requirements

- **NFR-1 (Portability)**: The project shall build and run on common Linux
  developer toolchains without requiring exotic or proprietary dependencies.
- **NFR-2 (Reproducibility)**: A clean checkout shall build and run using only
  the documented steps, with no undocumented manual setup, so results are
  reproducible on another compatible machine.
- **NFR-3 (Test coverage of intent)**: Both interop directions shall be covered
  by automated tests; a green test run shall be sufficient evidence that both
  directions function.
- **NFR-4 (Clarity / documentation)**: Documentation shall be sufficient for a
  reader unfamiliar with the project to understand its purpose and reproduce the
  build, run, and test steps without external help.
- **NFR-5 (Contained unsafety)**: Any `unsafe`/FFI-boundary code shall be limited
  to what the interop inherently requires and shall be localized and identifiable
  rather than spread throughout the codebase. No unsafe behavior beyond the
  minimum required to cross the language boundary shall be introduced.
- **NFR-6 (Correctness of marshalling)**: Data passed across the boundary shall
  be interpreted with the same meaning on both sides (no truncation, encoding
  corruption, or memory-ownership errors in the demonstrated examples).
- **NFR-7 (Minimalism)**: The project shall remain small and focused; it shall
  favor a minimal, readable demonstration over breadth of features.

---

## 4. Constraints & Assumptions

- **C-1**: Target operating system is **Linux**. Other platforms are not a
  requirement of this iteration.
- **C-2**: The exact language toolchain versions, build system, and interop
  mechanism/binding approach are **deliberately not specified here**. Selecting
  these is the **System Architect's** responsibility and shall be recorded in
  `docs/tech-stack.md`. As of this SRS, that file does not yet exist and the
  System Architect will need to create it.
- **C-3**: The project uses both the Rust and C++ languages by definition; this
  is an inherent constraint of the demo's purpose, not a design choice to be
  revisited.
- **A-1**: The audience is technical (developers / portfolio reviewers) and can
  run standard Linux command-line build/test tooling.
- **A-2**: A single-machine, local build and run is sufficient; no networked,
  distributed, or deployed environment is assumed.
- **A-3**: "Non-trivial-but-simple" examples are acceptable — the intent is to
  show real marshalling of primitives plus at least one string or struct, not to
  cover every possible type or edge case.
- **A-4**: Licensing follows the repository's existing `LICENSE` file.

---

## 5. Acceptance Criteria

A QA Engineer can consider the requirements met when all of the following hold:

- [ ] From a clean checkout, the demo builds using only the documented steps
      (satisfies FR-9, NFR-2).
- [ ] Running the documented demo command exercises **both** directions and
      prints human-readable output showing each direction working
      (satisfies FR-10).
- [ ] The Rust → C++ example demonstrably passes a primitive and receives a
      primitive result (satisfies FR-1, FR-2, FR-4).
- [ ] The Rust → C++ example demonstrably marshals at least one string and/or
      struct/aggregate across the boundary (satisfies FR-3).
- [ ] The C++ → Rust example demonstrably passes a primitive and receives a
      primitive result (satisfies FR-5, FR-6, FR-8).
- [ ] The C++ → Rust example demonstrably marshals at least one string and/or
      struct/aggregate across the boundary (satisfies FR-7).
- [ ] An automated test for the Rust → C++ direction exists and passes
      (satisfies FR-13).
- [ ] An automated test for the C++ → Rust direction exists and passes
      (satisfies FR-14).
- [ ] All automated tests run via a single documented command and report a clear
      pass/fail result (satisfies FR-15).
- [ ] A README explains purpose, build, run, and test steps clearly enough to
      reproduce without external help (satisfies FR-11, NFR-4).
- [ ] Source layout makes it clear which code belongs to which interop direction
      (satisfies FR-12).
- [ ] The build and run succeed on a common Linux toolchain (satisfies NFR-1).
- [ ] FFI/unsafe code is localized to the boundary and no broader unsafe behavior
      is introduced (satisfies NFR-5).
- [ ] Values cross the boundary without corruption or ownership errors in the
      demonstrated examples (satisfies NFR-6).

---

## 6. Out of Scope

The following are explicitly **not** part of this iteration:

- Supporting operating systems other than Linux (e.g. Windows, macOS).
- Production-grade error handling, logging, configuration, or observability.
- Exhaustive coverage of FFI type marshalling (callbacks, complex generics,
  exceptions/panics across the boundary, async, threading models). One string
  and/or struct example per direction is sufficient.
- Performance benchmarking or optimization of the boundary crossing.
- Packaging, publishing, or distributing artifacts (crates, packages, containers,
  binaries) to any registry.
- Continuous Integration pipeline setup (may be added later; not required to
  satisfy these requirements).
- Choosing the specific toolchain, build system, or binding/codegen approach —
  deferred to the System Architect (see C-2).

---

**Requirements are ready for the System Architect agent to design against.**
