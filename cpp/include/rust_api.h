// rust_api.h — C++ -> Rust callees.
//
// Declares the `extern "C"` symbols exported by the Rust `interop_rs` static
// library (crates/interop_rs/src/lib.rs) so C++ can call them. The `Vec3` POD
// layout must match the Rust `#[repr(C)] struct Vec3` exactly.

#ifndef RUST_API_H
#define RUST_API_H

#include <cstdint>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Vec3 {
  double x;
  double y;
  double z;
} Vec3;

// FR-6: primitive in, primitive out.
uint32_t rust_fibonacci(uint32_t n);

// FR-7: struct in, struct out (by value).
Vec3 rust_vec3_normalize(Vec3 v);

// FR-7: borrowed NUL-terminated string in, vowel count out.
uint32_t rust_count_vowels(const char* s);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // RUST_API_H
