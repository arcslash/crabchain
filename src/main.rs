use chrono::prelude::*;
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Transaction{
    sender: String,
    recipient: String,
    amount: u64
}
#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: String,
    data: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    nonce: u64
}



impl Block {
    fn new(index: u64,
           previous_hash: String,
           data: Vec<Transaction>) -> Self{
        
        let new_block = Block {
            index,
            timestamp: Utc::now().to_string(),
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0
        };
        new_block
    }
    
    
    fn calculate_hash(&self) -> String{
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_string());
        let result = hasher.finalize();
        let hash_hex = format!("{:x}", result);
        hash_hex
    }
}



impl Transaction {
    fn new(sender: String, recipient: String, amount: u64) -> Self{
        Transaction{sender, recipient, amount}
    }
}
fn main() {
    let block = Block::new(
        0,
        String::new(),
        vec![]
    );
    println!("{:?}", block);
}
