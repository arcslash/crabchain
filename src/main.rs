use chrono::prelude::*;
struct Transaction{}
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
            index: index,
            timestamp: Utc::now().to_string(),
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0
        };
        new_block
        
        
    }
}
fn main() {
    println!("Hello, world!");
}
