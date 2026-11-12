//! Cryptol FFI boundary for AES key expansion.

use super::key_expansion;
use ffi_utils::{get_vec, output_vec};
use Cipher::{Block, Word};

/// FFI entrypoint for the `KeyExpansion` function.
///
/// Invalid AES key sizes and null pointers are rejected before raw-pointer
/// conversion.
///
/// # Safety
///
/// When `k` is 128, 192, or 256 and both pointers are non-null:
///
/// - `key_raw` must be valid for reads of `k / 32` initialized `Word`
///   values.
/// - `out_raw` must be valid for writes of `k / 32 + 7` `Block` values.
/// - The referenced storage must remain valid for the duration of the call,
///   and the output write must not violate Rust's aliasing rules.
#[export_name = "KeyExpansion"]
pub unsafe extern "C" fn key_expansion_ffi(k: usize, key_raw: *const Word, out_raw: *mut Block) {
    if !matches!(k, 128 | 192 | 256) {
        return;
    }
    if key_raw.is_null() || out_raw.is_null() {
        return;
    }

    // SAFETY: The caller contract covers validity and lifetime for the
    // accepted non-null key pointer. The validated key size bounds k / 32.
    let key = unsafe { get_vec::<Word>(k / 32, key_raw) };

    let expanded_key = key_expansion(k, &key);
    let mut expanded_key_vec: Vec<Block> = expanded_key.to_vec();
    expanded_key_vec.resize(k / 32 + 7, [0; 16]);

    // SAFETY: The caller contract covers sufficient writable output storage
    // and the required aliasing constraints.
    unsafe { output_vec::<Block>(&expanded_key_vec, out_raw) };
}

#[cfg(test)]
mod tests {
    use super::key_expansion_ffi;
    use std::ptr::{null, null_mut};

    #[test]
    fn key_expansion_rejects_invalid_key_size_before_pointer_access() {
        // SAFETY: An invalid key size returns before any pointer is accessed.
        unsafe { key_expansion_ffi(0, null(), null_mut()) };
    }

    #[test]
    fn key_expansion_rejects_null_pointers_before_pointer_access() {
        // SAFETY: Null pointers are rejected before raw-pointer conversion.
        unsafe { key_expansion_ffi(128, null(), null_mut()) };
    }
}
