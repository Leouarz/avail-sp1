//! A simple script to generate and verify the proof of a given program.

use anyhow::Result;
use avail_subxt::AvailClient;
use sp1_sdk::{ProverClient, SP1Stdin};
use subxt::utils::H256;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

async fn get_genesis_hash() -> Result<H256> {
    let client = AvailClient::new("wss://turing-rpc.avail.so/ws")
        .await
        .unwrap();
    let genesis_hash = client.genesis_hash();
    Ok(genesis_hash)
}

#[tokio::main]
async fn main() {
    // Generate proof.
    let hash_input = get_genesis_hash().await.unwrap();

    let mut stdin = SP1Stdin::new();
    stdin.write(&hash_input);

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove_compressed(&pk, stdin).expect("proving failed");

    // Read output.
    let hash_input = proof.public_values.read::<H256>();
    let hash_output = proof.public_values.read::<H256>();
    println!("hash input: {}", hash_input);
    println!("hash output: {}", hash_output);

    // Verify proof.
    client
        .verify_compressed(&proof, &vk)
        .expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
