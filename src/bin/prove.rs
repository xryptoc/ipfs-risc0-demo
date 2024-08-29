use ipfstest_methods::IPFS_CONTENT_ZK_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};

use tokio::main;
use std::env;

fn load_env() {
    dotenv::dotenv().ok();
}

// This is a Hello World demo for the RISC Zero zkVM.
// By running the demo, Alice can produce a receipt that proves that she knows
// some numbers a and b, such that a*b == 391.
// The factors a and b are kept secret.

// Compute the product a*b inside the zkVM
#[main]
async fn main() {
    load_env();

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: prove <ipfs_cid> <start> <end>");
        std::process::exit(1);
    }

    println!("All arguments: {:?}", args);

    let start_time = std::time::Instant::now();

    let input = ipfs_host::v0_proof::select_from_ipfs_generate_guest_input(
        args[1].as_str(), 
        args[2].parse::<u64>().unwrap(), 
        args[3].parse::<u64>().unwrap(),
    ).await;

    let env = ExecutorEnv::builder()
        // Send a & b to the guest
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, IPFS_CONTENT_ZK_ELF).unwrap().receipt;

    println!("Processing time: {} ms", start_time.elapsed().as_millis());

    // 序列化回执
    let serialized = bincode::serialize(&receipt).unwrap();

    // 回执写入文件
    match std::fs::write("./receipt.bin", &serialized) {
        Ok(_) => println!("Receipt written to receipt.bin"),
        Err(e) => println!("Error writing receipt.bin: {}", e),
    }
}