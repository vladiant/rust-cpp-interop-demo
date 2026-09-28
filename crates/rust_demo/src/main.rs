//! Rust binary demonstrating the **Rust -> C++** interop direction (FR-13).
//!
//! It links the C++ static library `libmathlib` (wired by Corrosion in the
//! top-level `CMakeLists.txt`), calls its `extern "C"` `cpp_*` functions,
//! prints human-readable results (FR-10), and `assert!`s the expected values
//! so the process exits non-zero on any mismatch. CTest runs this binary as
//! the `rust_to_cpp` test.

use std::ffi::CString;
use std::os::raw::c_char;

/// Plain-old-data 3D vector shared with C++.
///
/// The layout is fixed by `#[repr(C)]` and **must** match the C++
/// `struct Vec3 { double x, y, z; }` in `cpp/include/mathlib.h`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// SAFETY: these declarations mirror the C++ `extern "C"` definitions in
// `cpp/src/mathlib.cpp` exactly (C ABI, matching `#[repr(C)]` layouts). The
// functions are total and never throw, so no unwinding crosses the boundary.
extern "C" {
    fn cpp_add(a: i32, b: i32) -> i32;
    fn cpp_vec3_dot(a: Vec3, b: Vec3) -> f64;
    fn cpp_vec3_scale(v: Vec3, factor: f64) -> Vec3;
    fn cpp_utf8_len(s: *const c_char) -> usize;
}

fn main() {
    println!("=== Rust -> C++ (rust_demo links libmathlib) ===");

    // FR-2: primitive in, primitive out.
    // SAFETY: `cpp_add` is a total C ABI function taking two i32 by value.
    let sum = unsafe { cpp_add(2, 3) };
    println!("cpp_add(2, 3) = {sum}");
    assert_eq!(sum, 5, "cpp_add returned an unexpected value");

    // FR-3: struct in, primitive out.
    let a = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let b = Vec3 {
        x: 4.0,
        y: -5.0,
        z: 6.0,
    };
    // SAFETY: `Vec3` is `#[repr(C)]` and passed by value, matching the C++ ABI.
    let dot = unsafe { cpp_vec3_dot(a, b) };
    println!("cpp_vec3_dot({a:?}, {b:?}) = {dot}");
    assert!((dot - 12.0).abs() < 1e-12, "cpp_vec3_dot returned {dot}");

    // FR-3: struct in, struct out.
    // SAFETY: `Vec3` crosses by value in and out; no ownership transfers.
    let scaled = unsafe { cpp_vec3_scale(a, 2.0) };
    println!("cpp_vec3_scale({a:?}, 2.0) = {scaled:?}");
    assert_eq!(
        scaled,
        Vec3 {
            x: 2.0,
            y: 4.0,
            z: 6.0
        },
        "cpp_vec3_scale returned an unexpected value"
    );

    // FR-3: borrowed string in, primitive out (byte length via strlen).
    let text = "héllo"; // 6 UTF-8 bytes: h + é(2) + l + l + o.
    let c_text = CString::new(text).expect("demo string has no interior NUL");
    // SAFETY: `c_text` owns a valid NUL-terminated buffer that stays alive for
    // the whole call; C++ only reads it via strlen and never retains it.
    let byte_len = unsafe { cpp_utf8_len(c_text.as_ptr()) };
    println!("cpp_utf8_len({text:?}) = {byte_len} (bytes)");
    assert_eq!(byte_len, 6, "cpp_utf8_len returned an unexpected value");

    println!("Rust -> C++: all assertions passed.");
}
