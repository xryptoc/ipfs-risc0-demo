use ipfstest_methods::IPFS_CONTENT_ZK_ID;
use risc0_zkvm::Receipt;

fn main(){
    // 导入由 prove 生成的 receipt
    let receipt_path ="./receipt.bin".to_string();
    let receipt_file = std::fs::read(receipt_path).unwrap();
   
    // 反序列化
    let receipt = bincode::deserialize::<Receipt>(&receipt_file).unwrap();
    
    // 验证生成的 receipt 是否未被篡改
    let _verification = match receipt.verify(IPFS_CONTENT_ZK_ID){
    Ok(()) => println!("Proof is Valid"),
    Err(_) => println!("Something went wrong !!"),
    };
}