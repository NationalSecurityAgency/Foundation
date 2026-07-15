//! ## Overview
//!
//! This crate provides helper functions for sending and receiving
//! data across Cryptol's Foreign Function Interface (FFI).
//!
//! Arguments and return values are passed using raw pointers, so unsafe
//! code is required. All functions in this crate have been checked for
//! memory safety using Kani.

#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::ref_as_ptr)]
#![allow(clippy::borrow_as_ptr)]

mod kani_proofs;
mod unit_tests;

/// Returns a new array populated with a copy of the data pointed to
/// by the raw pointer `src`.
pub fn get_array<T: Copy + Default, const COUNT: usize>(src: *const T) -> [T; COUNT] {
    let mut result = [Default::default(); COUNT];
    unsafe {
        std::ptr::copy(src, &mut result as *mut T, COUNT);
    }
    result
}

/// Returns a new `std::Vec` populated with a copy of the data pointed
/// to by the raw pointer `src`.
pub fn get_vec<T: Copy + Default>(n: usize, src: *const T) -> Vec<T> {
    let mut result: Vec<T> = Vec::with_capacity(n);
    unsafe {
        std::ptr::copy(src, result.as_mut_ptr(), n);
        result.set_len(n);
    }
    result
}

/// Copies the contents of the array `src` into the memory pointed to
/// by `dst`. Assumes that `dst` points to a valid memory range large
/// enough to hold the contents of `src`.
pub fn output_array<T: Copy, const COUNT: usize>(src: &[T; COUNT], dst: *mut T) {
    unsafe {
        std::ptr::copy(src as *const T, dst, COUNT);
    }
}

/// Copies the contents of the vector `src` into the memory pointed to
/// by `dst`. Assumes that `dst` points to a valid memory range large
/// enough to hold the contents of `src`.
pub fn output_vec<T: Copy>(src: &[T], dst: *mut T) {
    unsafe { std::ptr::copy(src.as_ptr(), dst, src.len()) }
}
