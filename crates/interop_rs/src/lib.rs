//! Rust functions exported across the C ABI for the **C++ -> Rust** interop
//! direction.
//!
//! Every symbol here is `#[no_mangle] extern "C"` so that C++ can link
//! `libinterop_rs.a` and call these functions directly. Per the design
//! (`docs/design/design.md` §3.3):
//!
//! * primitives and `#[repr(C)]` PODs cross **by value**;
//! * strings cross as **borrowed, NUL-terminated `*const c_char`** that the
//!   callee only reads for the duration of the call;
//! * boundary functions are **total** (no panics unwind into C++).
//!
//! The public FFI wrappers are intentionally thin. The actual logic lives in
//! private, safe helper functions so it can be unit-tested with `cargo test`
//! without crossing the FFI boundary.

use std::ffi::CStr;
use std::os::raw::c_char;

/// Plain-old-data 3D vector shared with C++.
///
/// The layout is fixed by `#[repr(C)]` and **must** match the C++
/// `struct Vec3 { double x, y, z; }` in `cpp/include/rust_api.h`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Compute the `n`-th Fibonacci number (`fib(0) = 0`, `fib(1) = 1`).
///
/// Iterative and wrapping so it can never panic across the FFI boundary.
fn fibonacci(n: u32) -> u32 {
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    for _ in 0..n {
        let next = a.wrapping_add(b);
        a = b;
        b = next;
    }
    a
}

/// Return a unit-length copy of `v`. A zero-length vector is returned
/// unchanged (no division by zero, no panic).
fn vec3_normalize(v: Vec3) -> Vec3 {
    let len = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if len == 0.0 {
        return v;
    }
    Vec3 {
        x: v.x / len,
        y: v.y / len,
        z: v.z / len,
    }
}

/// Count ASCII vowels (`a e i o u`, case-insensitive) in `bytes`.
///
/// Operates on raw bytes so it is total for any input, including non-UTF-8
/// data: multibyte UTF-8 continuation bytes are `>= 0x80` and never match an
/// ASCII vowel.
fn count_vowels(bytes: &[u8]) -> u32 {
    bytes
        .iter()
        .filter(|b| matches!(b.to_ascii_lowercase(), b'a' | b'e' | b'i' | b'o' | b'u'))
        .count() as u32
}

/// FR-6: primitive in, primitive out.
#[no_mangle]
pub extern "C" fn rust_fibonacci(n: u32) -> u32 {
    fibonacci(n)
}

/// FR-7: `#[repr(C)]` struct in, struct out (by value).
#[no_mangle]
pub extern "C" fn rust_vec3_normalize(v: Vec3) -> Vec3 {
    vec3_normalize(v)
}

/// FR-7: borrowed C string in, primitive out.
///
/// # Safety
///
/// `s` must be a valid, non-null, NUL-terminated C string that stays alive for
/// the duration of the call. The callee only reads it and never stores the
/// pointer. A null pointer is treated as an empty string.
#[no_mangle]
pub unsafe extern "C" fn rust_count_vowels(s: *const c_char) -> u32 {
    if s.is_null() {
        return 0;
    }
    // SAFETY: the caller guarantees `s` is a valid, NUL-terminated C string
    // borrowed for the whole call; we only read it and never retain it.
    let bytes = unsafe { CStr::from_ptr(s) }.to_bytes();
    count_vowels(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn fibonacci_base_and_sequence() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn normalize_unit_length() {
        let n = vec3_normalize(Vec3 {
            x: 3.0,
            y: 0.0,
            z: 4.0,
        });
        assert!((n.x - 0.6).abs() < 1e-12);
        assert_eq!(n.y, 0.0);
        assert!((n.z - 0.8).abs() < 1e-12);
        let len = (n.x * n.x + n.y * n.y + n.z * n.z).sqrt();
        assert!((len - 1.0).abs() < 1e-12);
    }

    #[test]
    fn normalize_zero_vector_is_unchanged() {
        let zero = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(vec3_normalize(zero), zero);
    }

    #[test]
    fn count_vowels_ascii_and_case_insensitive() {
        assert_eq!(count_vowels(b"interop"), 3);
        assert_eq!(count_vowels(b"AEIOU"), 5);
        assert_eq!(count_vowels(b""), 0);
        assert_eq!(count_vowels(b"xyz"), 0);
    }

    #[test]
    fn count_vowels_ignores_utf8_multibyte() {
        // "héllo": only the ASCII 'o' counts; the accented 'é' (bytes 0xC3 0xA9)
        // must not be miscounted as a vowel.
        assert_eq!(count_vowels("héllo".as_bytes()), 1);
    }

    #[test]
    fn ffi_wrappers_agree_with_helpers() {
        assert_eq!(rust_fibonacci(10), 55);
        let s = CString::new("interop").unwrap();
        // SAFETY: `s` is a valid NUL-terminated C string alive for the call.
        let vowels = unsafe { rust_count_vowels(s.as_ptr()) };
        assert_eq!(vowels, 3);
        // SAFETY: null is documented as an empty string.
        assert_eq!(unsafe { rust_count_vowels(std::ptr::null()) }, 0);
    }
}
