// mathlib.h — Rust -> C++ callees.
//
// Declares the `extern "C"` (C ABI) functions implemented in mathlib.cpp and
// called from the Rust `rust_demo` binary. The `Vec3` POD layout must match the
// Rust `#[repr(C)] struct Vec3` in crates/rust_demo/src/main.rs exactly.

#ifndef MATHLIB_H
#define MATHLIB_H

#include <cstddef>
#include <cstdint>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Vec3 {
  double x;
  double y;
  double z;
} Vec3;

// FR-2: primitive in, primitive out.
int32_t cpp_add(int32_t a, int32_t b);

// FR-3: struct in, primitive out.
double cpp_vec3_dot(Vec3 a, Vec3 b);

// FR-3: struct in, struct out (by value).
Vec3 cpp_vec3_scale(Vec3 v, double factor);

// FR-3: borrowed NUL-terminated string in, byte length out (strlen semantics).
size_t cpp_utf8_len(const char* s);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // MATHLIB_H
