// test_cpp_to_rust.cpp — C++ -> Rust boundary test (FR-14).
//
// Links the Rust static library libinterop_rs and asserts on the values
// returned by the `rust_*` functions across the FFI boundary. Registered with
// CTest as `cpp_to_rust`; a failed assertion aborts with a non-zero exit code.

#include <cassert>
#include <cmath>
#include <cstdio>

#include "rust_api.h"

int main() {
  // FR-6: primitive in, primitive out.
  assert(rust_fibonacci(0) == 0);
  assert(rust_fibonacci(1) == 1);
  assert(rust_fibonacci(10) == 55);

  // FR-7: struct in, struct out — (3, 0, 4) normalizes to (0.6, 0, 0.8).
  Vec3 n = rust_vec3_normalize(Vec3{3.0, 0.0, 4.0});
  assert(std::fabs(n.x - 0.6) < 1e-12);
  assert(std::fabs(n.y - 0.0) < 1e-12);
  assert(std::fabs(n.z - 0.8) < 1e-12);
  const double len = std::sqrt(n.x * n.x + n.y * n.y + n.z * n.z);
  assert(std::fabs(len - 1.0) < 1e-12);

  // A zero-length vector is returned unchanged (no division by zero).
  Vec3 z = rust_vec3_normalize(Vec3{0.0, 0.0, 0.0});
  assert(z.x == 0.0 && z.y == 0.0 && z.z == 0.0);

  // FR-7: borrowed string in, primitive out.
  assert(rust_count_vowels("interop") == 3);
  assert(rust_count_vowels("AEIOU") == 5);
  assert(rust_count_vowels("xyz") == 0);
  assert(rust_count_vowels("") == 0);

  // A null pointer is documented to cross safely as an empty string (0 vowels).
  assert(rust_count_vowels(nullptr) == 0);

  std::printf("C++ -> Rust: all assertions passed.\n");
  return 0;
}
