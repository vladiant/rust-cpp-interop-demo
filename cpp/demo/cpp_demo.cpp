// cpp_demo.cpp — demonstrates the C++ -> Rust interop direction (FR-8, FR-10).
//
// Links the Rust static library libinterop_rs and calls its `extern "C"`
// `rust_*` functions, printing human-readable results. The dedicated assertion
// test lives in tests/test_cpp_to_rust.cpp (CTest `cpp_to_rust`).

#include <cstdio>

#include "rust_api.h"

int main() {
  std::printf("=== C++ -> Rust (cpp_demo links libinterop_rs) ===\n");

  // FR-6: primitive in, primitive out.
  uint32_t fib = rust_fibonacci(10);
  std::printf("rust_fibonacci(10) = %u\n", fib);

  // FR-7: struct in, struct out.
  Vec3 v{3.0, 0.0, 4.0};
  Vec3 n = rust_vec3_normalize(v);
  std::printf("rust_vec3_normalize({%.1f, %.1f, %.1f}) = {%.4f, %.4f, %.4f}\n",
              v.x, v.y, v.z, n.x, n.y, n.z);

  // FR-7: borrowed string in, primitive out.
  const char* word = "interop";
  uint32_t vowels = rust_count_vowels(word);
  std::printf("rust_count_vowels(\"%s\") = %u\n", word, vowels);

  std::printf("C++ -> Rust: demo complete.\n");
  return 0;
}
