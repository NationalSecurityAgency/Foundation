//! This is a Rust implementation of the Secure Hash Algorithm 2
//! (SHA-2) main functions [FIPS-PUB-180-4 Sections 6.2 and 6.4].

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::functions::{Ch, Maj};

/// Inner loop of SHA-2 that processes a single message block `M`
/// [FIPS-PUB-180-4 Sections 6.2.2 and 6.4.2]. `KLEN` is the number of
/// elements of `K` (and `W`) [FIPS-PUB-180-4 Sections 4.2.2 and
/// 4.2.3]. Should be `64` when `w = 32` and `80` when `w = 64`.
pub fn inner<const KLEN: usize, w: num::PrimInt + num::traits::WrappingAdd>(
    sigma_0: &impl Fn(w) -> w,
    sigma_1: &impl Fn(w) -> w,
    SIGMA_0: &impl Fn(w) -> w,
    SIGMA_1: &impl Fn(w) -> w,
    K: &[w; KLEN],
    H: [w; 8],
    M: [w; 16],
) -> [w; 8] {
    // Step 1 [FIPS-PUB-180-4 Sections 6.2.2 and 6.4.2].
    let mut W: [w; KLEN] = [w::zero(); KLEN];
    W[..16].copy_from_slice(&M);
    for t in 16..KLEN {
        W[t] = sigma_1(W[t - 2])
            .wrapping_add(&W[t - 7])
            .wrapping_add(&sigma_0(W[t - 15]))
            .wrapping_add(&W[t - 16]);
    }

    // Steps 2 and 3 [FIPS-PUB-180-4 Sections 6.2.2 and 6.4.2].
    let KtWt = std::iter::zip(K.iter().copied(), W).take(KLEN);
    let Hp = KtWt.fold(H, |Hacc, (Kt, Wt)| step3(SIGMA_0, SIGMA_1, Hacc, Kt, Wt));

    // Step 4 [FIPS-PUB-180-4 Sections 6.2.2 and 6.4.2].
    [
        H[0].wrapping_add(&Hp[0]),
        H[1].wrapping_add(&Hp[1]),
        H[2].wrapping_add(&Hp[2]),
        H[3].wrapping_add(&Hp[3]),
        H[4].wrapping_add(&Hp[4]),
        H[5].wrapping_add(&Hp[5]),
        H[6].wrapping_add(&Hp[6]),
        H[7].wrapping_add(&Hp[7]),
    ]
}

// Step 3 of the inner loop [FIPS-PUB-180-4 Section 6.4.2].
#[allow(clippy::many_single_char_names)]
fn step3<w: num::PrimInt + num::traits::WrappingAdd>(
    SIGMA_0: &impl Fn(w) -> w,
    SIGMA_1: &impl Fn(w) -> w,
    [a, b, c, d, e, f, g, h]: [w; 8],
    Kt: w,
    Wt: w,
) -> [w; 8] {
    let T1 = h
        .wrapping_add(&SIGMA_1(e))
        .wrapping_add(&Ch(e, f, g))
        .wrapping_add(&Kt)
        .wrapping_add(&Wt);
    let T2 = SIGMA_0(a).wrapping_add(&Maj(a, b, c));
    [T1.wrapping_add(&T2), a, b, c, d.wrapping_add(&T1), e, f, g]
}
