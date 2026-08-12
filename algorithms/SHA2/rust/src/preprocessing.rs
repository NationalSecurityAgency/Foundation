//! This module specifies Secure Hash Algorithm (SHA) preprocessing
//! functions such as padding and parsing [FIPS-PUB-180-4 Section 5].

/// The purpose of this padding is to ensure that the padded message
/// is a multiple of 512, depending on the algorithm
/// [FIPS-PUB-180-4 Section 5.1].
pub fn pad32(message: &[u8]) -> Vec<u8> {
    let mut M = message[0..].to_vec();
    let l: u64 = M.len() as u64 * 8;
    let mut zeroes = 64 - ((M.len() + 1 + 8) % 64);
    if zeroes == 64 {
        zeroes = 0;
    }
    M.push(0x80);
    M.resize(M.len() + zeroes, 0);
    M.extend_from_slice(&l.to_be_bytes());

    M
}

/// The purpose of this padding is to ensure that the padded message
/// is a multiple of 1024, depending on the algorithm
/// [FIPS-PUB-180-4 Section 5.1].
pub fn pad64(message: &[u8]) -> Vec<u8> {
    let mut M = message[0..].to_vec();
    let l: u128 = M.len() as u128 * 8;
    let mut zeroes = 128 - ((M.len() + 1 + 16) % 128);
    if zeroes == 128 {
        zeroes = 0;
    }
    M.push(0x80);
    M.resize(M.len() + zeroes, 0);
    M.extend_from_slice(&l.to_be_bytes());

    M
}

/// This function parses part of a message/padding into 16 32-bit
/// blocks [FIPS-PUB-180-4 Section 5.2].
///
/// # Panics
///
/// This function will not panic.
pub fn parse32(block: &[u8; 64]) -> [u32; 16] {
    let chunks = block.chunks_exact(4);
    let words: Vec<u32> = chunks
        .map(|word: &[u8]| u32::from_be_bytes(word.try_into().unwrap()))
        .collect();
    words.try_into().unwrap()
}

/// This function parses part of a message/padding into 16 64-bit
/// blocks [FIPS-PUB-180-4 Section 5.2].
///
/// # Panics
///
/// This function will not panic.
pub fn parse64(block: &[u8; 128]) -> [u64; 16] {
    let chunks = block.chunks_exact(8);
    let words: Vec<u64> = chunks
        .map(|word: &[u8]| u64::from_be_bytes(word.try_into().unwrap()))
        .collect();
    words.try_into().unwrap()
}
