//! This is a Rust implementation of Keccak [FIPS-PUB-202] where `w
//! = 64`.

#![allow(non_snake_case)]

/// The `State` type is a 5-by-5-by-w array of bits that represents
/// the state for a `Keccak-p` permutation.  The indices for `x`, `y`,
/// and `z` coordinates range from 0 to 4, 0 to 4, and 0 to `w-1`,
/// respectively [FIPS-PUB-202 Section 2.1].
type State = [[u64; 5]; 5];

/// # `theta` [FIPS-PUB-202 Algorithm 1]
///
/// 1. For all pairs `(x,z)` such that `0≤x<5` and `0≤z<w`, let
///    `C[x,z]=A[x,0,z] ⨁ A[x,1,z] ⨁ A[x,2,z] ⨁ A[x,3,z] ⨁ A[x,4,z]`.
///
/// 2. For all pairs `(x,z)` such that `0≤x<5` and `0≤z<w`, let
///    `D[x,z] = C[(x-1) mod 5, z] ⨁ C[(x+1) mod 5, (z-1) mod w]`.
///
/// 3. For all triples (x,y,z) such that `0≤x<5`, `0≤y<5`, and
///    `0≤z<w`, let `A'[x,y,z] = A[x,y,z] ⨁ D[x,z]`.
#[allow(clippy::needless_range_loop)]
fn theta(A: &mut State) {
    let mut C: [u64; 5] = [0; 5];
    let mut D: [u64; 5] = [0; 5];

    for x in 0..5usize {
        C[x] = A[x][0] ^ A[x][1] ^ A[x][2] ^ A[x][3] ^ A[x][4];
    }

    for x in 0..5usize {
        D[x] = C[(x + 5 - 1) % 5] ^ C[(x + 1) % 5].rotate_right(1);
    }

    for x in 0..5usize {
        for y in 0..5usize {
            A[x][y] ^= D[x];
        }
    }
}

/// # `rho` [FIPS-PUB-202 Algorithm 2]
///
/// 1. For all `z` such that `0≤z<w`, let `A'[0,0,z] = A[0,0,z]`.
/// 2. Let `(x,y) = (1,0)`.
/// 3. For `t` from 0 to 23:
///    a. for all `z` such that `0≤z<w`, let `A'[x,y,z]=A[x,y,(z-(t+1)(t+2)/2 mod w]`
///    b. let `(x,y) = (y,(2x+3y) mod 5)`.
///
/// Here we specify the offset table variant.
fn rho(A: &mut State) {
    let offsets = [
        [0, 36, 3, 105, 210],
        [1, 300, 10, 45, 66],
        [190, 6, 171, 15, 253],
        [28, 55, 153, 21, 120],
        [91, 276, 231, 136, 78],
    ];

    for x in 0..5usize {
        for y in 0..5usize {
            A[x][y] = A[x][y].rotate_right(offsets[x][y]);
        }
    }
}

/// # `pi` [FIPS-PUB-202 Algorithm 3]
///
/// For all triples `(x,y,z)` such that `0≤x<5`, `0≤y<5`, `0≤z<w`, let
/// `A'[x,y,z] = A[(x+3y) mod 5, x, z]`
fn pi(A: &mut State) {
    let A_p = *A;
    for x in 0..5usize {
        for y in 0..5usize {
            A[x][y] = A_p[(x + 3 * y) % 5][x];
        }
    }
}

/// # `chi` [FIPS-PUB-202 Algorithm 4]
///
/// For all triples `(x,y,z)` such that `0≤x<5`, `0≤y<5`, `0≤z<w`, let
/// `A'[x,y,z] = A[x,y,z] ⨁ ((A[(x+1) mod 5, y, z] ⨁ 1) ⦁ A[(x+2) mod
/// 5, y, z])`.
fn chi(A: &mut State) {
    let A_p = *A;
    for x in 0..5usize {
        for y in 0..5usize {
            A[x][y] = A_p[x][y] ^ !A_p[(x + 1) % 5][y] & A_p[(x + 2) % 5][y];
        }
    }
}

/// # `rc` [FIPS-PUB-202 Algorithm 5]
///
/// The specification of `iota`'s round constant `rc`, specialized and
/// tabled given `w = 64`. The round constant bits for `ir` are
/// excited by RC[2^^j - 1] = rc(j+7*ir)
const RC: [u64; 24] = [
    0x8000_0000_0000_0000,
    0x4101_0000_0000_0000,
    0x5101_0000_0000_0001,
    0x0001_0001_0000_0001,
    0xd101_0000_0000_0000,
    0x8000_0001_0000_0000,
    0x8101_0001_0000_0001,
    0x9001_0000_0000_0001,
    0x5100_0000_0000_0000,
    0x1100_0000_0000_0000,
    0x9001_0001_0000_0000,
    0x5000_0001_0000_0000,
    0xd101_0001_0000_0000,
    0xd100_0000_0000_0001,
    0x9101_0000_0000_0001,
    0xc001_0000_0000_0001,
    0x4001_0000_0000_0001,
    0x0100_0000_0000_0001,
    0x5001_0000_0000_0000,
    0x5000_0001_0000_0001,
    0x8101_0001_0000_0001,
    0x0101_0000_0000_0001,
    0x8000_0001_0000_0000,
    0x1001_0001_0000_0001,
];

/// # `iota` [FIPS-PUB-202 Algorithm 6]
///
/// For all triples `(x,y,z)` such that `0≤x<5`, `0≤y<5`, `0≤z<w`, let
/// `A'[0,0,z]=A'[0,0,z] ⨁ RC[z]`. Otherwise, `A'[x,y,z] = A[x,y,z]`.
fn iota(A: &mut State, ir: usize) {
    A[0][0] ^= RC[ir];
}

/// The Keccak Round Function, `Rnd`, is the transformation that
/// results from applying the step functions in order [FIPS-PUB-202
/// part of Algorithm 7].
fn Rnd(A: &mut State, ir: usize) {
    theta(A);
    rho(A);
    pi(A);
    chi(A);
    iota(A, ir);
}

/// # `Keccak-p` [FIPS-PUB-202 Algorithm 7]
///
/// This function is specialized to `w = 64` and operates directly on
/// `State`s.
/// 1. For `ir` from `12+2l-nr` to `12+2l-1`, let `A = Rnd(A, ir)`.
/// 2. Return `A`.
fn Keccak_p(A: &mut State) {
    let l: usize = 6;
    let nr: usize = 24;
    for ir in (12 + 2 * l - nr)..(12 + 2 * l) {
        Rnd(A, ir);
    }
}

/// FFI entrypoint for the `Keccak-p` function, specialized to `w = 64`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[allow(clippy::needless_range_loop)]
#[export_name = "Keccak_p_FFI"]
pub extern "C" fn Keccak_p_FFI(A_in: *const State, A_out: *mut State) {
    let A = unsafe { &mut *A_out };
    for x in 0..5usize {
        for y in 0..5usize {
            A[x][y] = unsafe { *A_in }[x][y];
        }
    }
    Keccak_p(A);
}

/// # `pad10s1` [CSF-0.1 Section 2.1.2].
///
/// Appends a single 1 bit followed by the minimum number of 0 bits
/// followed by a single 1 bit such that the length of the result is a
/// multiple of the block length.
fn pad10s1(rate_bytes: usize, mut message: Vec<u8>) -> Vec<u8> {
    // Apply `pad10*1` padding to the message
    let padding_length = rate_bytes - (message.len() % rate_bytes);
    if padding_length == 1 {
        message.push(0x81);
    } else {
        message.push(0x80); // Domain separator
        message.resize(message.len() + padding_length - 2, 0x00);
        message.push(0x01); // Final bit
    }
    message
}

/// # `absorb` [CFS-0.1 Section 2.4.1].
///
/// `r`-bit input message blocks are `XORed` into the first `r` bits
/// of the state, interleaved with applications of the function
/// `Keccak-p`.
fn absorb(rate_bytes: usize, padded_message: &[u8]) -> State {
    // Initialize the state
    let mut state: State = [[0u64; 5]; 5];

    for block in padded_message.chunks(rate_bytes) {
        // Expand block to state
        for (i, chunk) in block.chunks(8).enumerate() {
            let mut value = 0u64;
            for (j, &byte) in chunk.iter().enumerate() {
                value |= u64::from(byte) << (8 * (7 - j));
            }
            state[i % 5][i / 5] ^= value;
        }
        Keccak_p(&mut state);
    }
    state
}

/// # `squeeze` [CSF-0.1 Section 2.4.2]
///
/// Concatenation of the left-most `r` bits of compositions of
/// `Keccak-p`.
#[allow(clippy::needless_range_loop)]
fn squeeze(rate_bytes: usize, digest_size: usize, mut state: State) -> Vec<u8> {
    assert!(
        rate_bytes.is_multiple_of(8),
        "Rate must be a multiple of 64."
    );
    let mut hash = Vec::new();
    while hash.len() < digest_size {
        for x in 0..5 {
            #[allow(clippy::needless_range_loop)]
            for y in 0..5 {
                if hash.len() < digest_size && (x * 5 * 8 + y * 8) < rate_bytes {
                    hash.extend_from_slice(&state[y][x].to_be_bytes());
                }
            }
        }
        if hash.len() < digest_size {
            Keccak_p(&mut state);
        }
    }
    hash.truncate(digest_size);
    hash
}

/// # `Keccak`
///
/// Implements the Keccak family of functions. This function uses the sponge
/// construction with the `Keccak_p` permutation, `pad10*1` padding, and a
/// specified rate `r` (1600 - capacity).
///
/// - `capacity`: The capacity of the sponge (in bits).
/// - `digest_size`: The desired hash length in bytes.
/// - `message`: The input message as a byte slice.
///
/// Returns the hash as a `Vec<u8>`.
///
/// # Panics
///
/// This function will panic is the capacity is out of range.
#[must_use]
pub fn Keccak(capacity: usize, digest_size: usize, message: &[u8]) -> Vec<u8> {
    const B: usize = 1600; // State size in bits
    let r = B - capacity; // Rate in bits
    let rate_bytes = r / 8;

    assert!(capacity < B, "Capacity must be less than 1600 bits.");
    assert!(r >= 1, "Rate must be at least 1 bit.");
    assert!(r.is_multiple_of(8), "Rate must be a multiple of 8.");

    let padded_message = pad10s1(rate_bytes, Vec::from(message));
    let state = absorb(rate_bytes, &padded_message);
    squeeze(rate_bytes, digest_size, state)
}
