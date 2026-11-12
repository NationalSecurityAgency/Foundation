//! ## Overview
//!
//! This crate provides helper functions for sending and receiving
//! data across Cryptol's Foreign Function Interface (FFI).
//!
//! Arguments and return values are passed using raw pointers. The helpers
//! therefore expose unsafe APIs with explicit caller obligations. Kani
//! harnesses exercise the helpers under those documented preconditions.

#![allow(clippy::ref_as_ptr)]
#![allow(clippy::borrow_as_ptr)]

mod kani_proofs;
mod unit_tests;

/// Returns a new array populated with a copy of the data pointed to
/// by the raw pointer `src`.
///
/// # Safety
///
/// `src` must be properly aligned and valid for reads of `COUNT`
/// consecutive initialized values of type `T`. Alignment is required
/// even when `COUNT` is zero.
pub unsafe fn get_array<T: Copy + Default, const COUNT: usize>(src: *const T) -> [T; COUNT] {
    let mut result = [Default::default(); COUNT];
    // SAFETY: The caller guarantees that src is aligned and readable
    // for COUNT values. result provides writable storage for COUNT values.
    unsafe {
        std::ptr::copy(src, &mut result as *mut T, COUNT);
    }
    result
}

/// Returns a new `std::Vec` populated with a copy of the data pointed
/// to by the raw pointer `src`.
///
/// # Safety
///
/// `src` must be properly aligned and valid for reads of `n`
/// consecutive initialized values of type `T`. Alignment is required
/// even when `n` is zero.
pub unsafe fn get_vec<T: Copy + Default>(n: usize, src: *const T) -> Vec<T> {
    let mut result: Vec<T> = Vec::with_capacity(n);
    // SAFETY: The caller guarantees that src is aligned and readable
    // for n values. The vector has capacity for n values.
    unsafe {
        std::ptr::copy(src, result.as_mut_ptr(), n);
        result.set_len(n);
    }
    result
}

/// Copies the contents of the array `src` into the memory pointed to
/// by `dst`.
///
/// # Safety
///
/// `dst` must be properly aligned and valid for writes of `COUNT`
/// consecutive values of type `T`. Alignment is required even when
/// `COUNT` is zero. The write must not violate Rust's aliasing rules.
pub unsafe fn output_array<T: Copy, const COUNT: usize>(src: &[T; COUNT], dst: *mut T) {
    // SAFETY: The caller guarantees that dst is aligned and writable
    // for COUNT values. src contains COUNT initialized values.
    unsafe {
        std::ptr::copy(src as *const T, dst, COUNT);
    }
}

/// Copies the contents of the vector `src` into the memory pointed to
/// by `dst`.
///
/// # Safety
///
/// `dst` must be properly aligned and valid for writes of `src.len()`
/// consecutive values of type `T`. Alignment is required even when
/// `src` is empty. The write must not violate Rust's aliasing rules.
pub unsafe fn output_vec<T: Copy>(src: &[T], dst: *mut T) {
    // SAFETY: The caller guarantees that dst is aligned and writable
    // for src.len() values.
    unsafe { std::ptr::copy(src.as_ptr(), dst, src.len()) }
}
