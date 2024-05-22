//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use subxt::utils::H256;

pub fn main() {
    let hash_input = sp1_zkvm::io::read::<H256>();
    let hash_output: H256 = H256::zero();

    sp1_zkvm::io::commit(&hash_input);
    sp1_zkvm::io::commit(&hash_output);
}
