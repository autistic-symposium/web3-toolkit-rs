// src/utils/maths.rs
// author: steinkirch


use web3::types::{U256};
use tiny_keccak::{Hasher, Keccak};


// Simple function to convert wei to eth
pub fn wei_to_eth(wei_val: U256) -> f64 {

    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0

}

/// Compute the Keccak-256 hash of input bytes
#[allow(dead_code)]
pub fn keccak256(bytes: &[u8]) -> [u8; 32] {

    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();

    hasher.update(bytes);
    hasher.finalize(&mut output);

    output
}