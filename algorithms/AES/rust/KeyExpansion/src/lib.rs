//! This is a Rust implementation of the `KeyExpansion` method in
//! the Advanced Encryption Standard (AES) [FIPS-PUB-197].

#![allow(non_snake_case)]

mod ffi;

use Cipher::{Block, Word, SBOX};

/// This table holds the word array for the round constant
/// [FIPS-PUB-197 Sections 2.1 and 5.2]. Constants are used here rather
/// than computing the values in place.
static RCON: [Word; 11] = [
    [0x00, 0x00, 0x00, 0x00], // undefined
    [0x01, 0x00, 0x00, 0x00],
    [0x02, 0x00, 0x00, 0x00],
    [0x04, 0x00, 0x00, 0x00],
    [0x08, 0x00, 0x00, 0x00],
    [0x10, 0x00, 0x00, 0x00],
    [0x20, 0x00, 0x00, 0x00],
    [0x40, 0x00, 0x00, 0x00],
    [0x80, 0x00, 0x00, 0x00],
    [0x1b, 0x00, 0x00, 0x00],
    [0x36, 0x00, 0x00, 0x00],
];

/// This function takes a word [a0,a1,a2,a3] as input and performs a
/// cyclic permutation, returning the word [a1,a2,a3,a0] [FIPS-PUB-197
/// Section 5.2].
fn rotword([a0, a1, a2, a3]: Word) -> Word {
    [a1, a2, a3, a0]
}

/// This function takes a four-byte input word and applies the S-box to
/// each of the four bytes to produce an output word [FIPS-PUB-197
/// Section 5.2].
fn subword(w: Word) -> Word {
    let mut ret = [0, 0, 0, 0];
    for i in 0..=3 {
        ret[i] = SBOX[w[i] as usize];
    }
    ret
}

/// Takes two `Word`s and returns the first `Word` XOR'd with the
/// second.
fn xorw(mut a: Word, b: Word) -> Word {
    for i in 0..=3 {
        a[i] ^= b[i];
    }
    a
}

/// This is the main AES key expansion routine [FIPS-PUB-197 Section
/// 5.2].
#[must_use]
pub fn key_expansion(k: usize, key: &[Word]) -> [Block; 15] {
    let nk = k / 32;
    let nr = nk + 6;
    let mut w: [Word; 64] = [[0; 4]; 64];
    for i in 0..(4 * (nr + 1)) {
        if i < nk {
            w[i] = key[i];
        } else if i % nk == 0 {
            w[i] = xorw(w[i - nk], xorw(subword(rotword(w[i - 1])), RCON[i / nk]));
        } else if (i % nk == 4) && (nk > 6) {
            w[i] = xorw(w[i - nk], subword(w[i - 1]));
        } else {
            w[i] = xorw(w[i - nk], w[i - 1]);
        }
    }
    let mut expanded_key: [Block; 15] = [[0u8; 16]; 15];
    for i in 0..=nr {
        for j in 0..=15 {
            expanded_key[i][j] = w[i * 4 + j / 4][j % 4];
        }
    }
    expanded_key
}
