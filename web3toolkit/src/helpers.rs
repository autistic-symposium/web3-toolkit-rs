// utils.rs - author: steinkirch
// util methods

use web3::types::{U256};


pub fn wei_to_eth(wei_val: U256) -> f64 {

    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0

}

