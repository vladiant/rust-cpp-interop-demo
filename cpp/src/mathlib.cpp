// mathlib.cpp — implementation of the Rust -> C++ callees.
//
// These `extern "C"` functions are compiled by CMake into libmathlib.a and
// linked into the Rust `rust_demo` binary by Corrosion. They are total and
// effectively noexcept: no exception unwinds across the FFI boundary.

#include "mathlib.h"

#include <cstring>

extern "C" {

int32_t cpp_add(int32_t a, int32_t b) { return a + b; }

double cpp_vec3_dot(Vec3 a, Vec3 b) {
  return a.x * b.x + a.y * b.y + a.z * b.z;
}

Vec3 cpp_vec3_scale(Vec3 v, double factor) {
  return Vec3{v.x * factor, v.y * factor, v.z * factor};
}

size_t cpp_utf8_len(const char* s) {
  // Borrowed, read-only string; the caller owns the buffer. Null is treated as
  // an empty string. Returns the byte length (std::strlen semantics).
  if (s == nullptr) {
    return 0;
  }
  return std::strlen(s);
}

}  // extern "C"
