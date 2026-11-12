//! Cryptol FFI boundary for AES encryption and decryption.

use super::{cipher, inv_cipher, Block};
use ffi_utils::{get_array, get_vec, output_array};

/// FFI entrypoint for the `Cipher` function.
///
/// Invalid AES key sizes and null pointers are rejected before raw-pointer
/// conversion.
///
/// # Safety
///
/// When `k` is 128, 192, or 256 and all pointers are non-null:
///
/// - `expanded_key_raw` must be valid for reads of `k / 32 + 7`
///   initialized `Block` values.
/// - `pt_raw` must be valid for reads of 16 initialized bytes.
/// - `out_raw` must be valid for writes of 16 bytes.
/// - The referenced storage must remain valid for the duration of the call,
///   and the output write must not violate Rust's aliasing rules.
#[export_name = "Cipher"]
pub unsafe extern "C" fn cipher_ffi(
    k: usize,
    expanded_key_raw: *const Block,
    pt_raw: *const u8,
    out_raw: *mut u8,
) {
    if !matches!(k, 128 | 192 | 256) {
        return;
    }
    if expanded_key_raw.is_null() || pt_raw.is_null() || out_raw.is_null() {
        return;
    }

    let nr = k / 32 + 6;

    // SAFETY: The caller contract covers validity and lifetime for the
    // accepted non-null pointer. The key size bounds nr + 1.
    let expanded_key = unsafe { get_vec::<Block>(nr + 1, expanded_key_raw) };

    // SAFETY: The caller contract covers a readable 16-byte input buffer.
    let pt = unsafe { get_array::<u8, 16>(pt_raw) };

    let ct = cipher(&expanded_key, pt);

    // SAFETY: The caller contract covers a writable 16-byte output buffer
    // and the required aliasing constraints.
    unsafe { output_array::<u8, 16>(&ct, out_raw) };
}

/// FFI entrypoint for the `InvCipher` function.
///
/// Invalid AES key sizes and null pointers are rejected before raw-pointer
/// conversion.
///
/// # Safety
///
/// When `k` is 128, 192, or 256 and all pointers are non-null:
///
/// - `expanded_key_raw` must be valid for reads of `k / 32 + 7`
///   initialized `Block` values.
/// - `ct_raw` must be valid for reads of 16 initialized bytes.
/// - `out_raw` must be valid for writes of 16 bytes.
/// - The referenced storage must remain valid for the duration of the call,
///   and the output write must not violate Rust's aliasing rules.
#[export_name = "InvCipher"]
pub unsafe extern "C" fn inv_cipher_ffi(
    k: usize,
    expanded_key_raw: *const Block,
    ct_raw: *const u8,
    out_raw: *mut u8,
) {
    if !matches!(k, 128 | 192 | 256) {
        return;
    }
    if expanded_key_raw.is_null() || ct_raw.is_null() || out_raw.is_null() {
        return;
    }

    let nr = k / 32 + 6;

    // SAFETY: The caller contract covers validity and lifetime for the
    // accepted non-null pointer. The key size bounds nr + 1.
    let expanded_key = unsafe { get_vec::<Block>(nr + 1, expanded_key_raw) };

    // SAFETY: The caller contract covers a readable 16-byte input buffer.
    let ct = unsafe { get_array::<u8, 16>(ct_raw) };

    let pt = inv_cipher(&expanded_key, ct);

    // SAFETY: The caller contract covers a writable 16-byte output buffer
    // and the required aliasing constraints.
    unsafe { output_array::<u8, 16>(&pt, out_raw) };
}

#[cfg(test)]
mod tests {
    use super::{cipher_ffi, inv_cipher_ffi};
    use std::ptr::{null, null_mut};

    #[test]
    fn cipher_rejects_invalid_key_size_before_pointer_access() {
        // SAFETY: An invalid key size returns before any pointer is accessed.
        unsafe { cipher_ffi(0, null(), null(), null_mut()) };
    }

    #[test]
    fn cipher_rejects_null_pointers_before_pointer_access() {
        // SAFETY: Null pointers are rejected before raw-pointer conversion.
        unsafe { cipher_ffi(128, null(), null(), null_mut()) };
    }

    #[test]
    fn inv_cipher_rejects_invalid_key_size_before_pointer_access() {
        // SAFETY: An invalid key size returns before any pointer is accessed.
        unsafe { inv_cipher_ffi(0, null(), null(), null_mut()) };
    }

    #[test]
    fn inv_cipher_rejects_null_pointers_before_pointer_access() {
        // SAFETY: Null pointers are rejected before raw-pointer conversion.
        unsafe { inv_cipher_ffi(128, null(), null(), null_mut()) };
    }
}
