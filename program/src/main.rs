//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

// use anyhow::Result;
// use avail_subxt::AvailClient;
use subxt::utils::H256;

// async fn get_genesis_hash() -> Result<H256> {
//     let client = AvailClient::new("wss://turing-rpc.avail.so/ws")
//         .await
//         .unwrap();
//     let genesis_hash = client.genesis_hash();
//     Ok(genesis_hash)
// }

pub fn main() {
    let hash_input = sp1_zkvm::io::read::<H256>();
    let hash_output: H256 = H256::zero();

    // let rt = tokio::runtime::Runtime::new().unwrap();
    // let hash_output = rt.block_on(async {
    //     get_genesis_hash().await.unwrap()
    // });


    sp1_zkvm::io::commit(&hash_input);
    sp1_zkvm::io::commit(&hash_output);
    sp1_zkvm::io::commit(&1);
}
