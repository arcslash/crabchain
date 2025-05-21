use std::fmt;
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

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Block {
    fn new(index: u64,
           previous_hash: String,
           data: Vec<Transaction>) -> Self{
        
        let mut new_block = Block {
            index,
            timestamp: Utc::now().to_string(),
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0
        };
        new_block.hash = new_block.calculate_hash();
        new_block
    }
    
    fn mine(&mut self, difficulty: usize){
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Mined block with hash: {}", self.hash);
        
    }


    fn calculate_hash(&self) -> String{
        let tx_strings: Vec<String> = self.data.iter()
            .map(|tx| tx.to_string())
            .collect();
        let joined_txs = tx_strings.join(",");

        // Combine all into one string
        let serialized = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            joined_txs,
            self.previous_hash,
            self.nonce
        );


        let mut hasher = Sha256::new();
        hasher.update(serialized); // change to serialized block
        let result = hasher.finalize();
        let hash_hex = format!("{:x}", result);
        hash_hex
    }
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, String::new(), vec![]);
        Blockchain{
            chain: vec![genesis_block]
        }
    }
    fn add_block(&mut self, data: Vec<Transaction>, diffuilty: usize) {
        let last_block = self.chain.last().unwrap();
        let new_index = last_block.index + 1;
        let prev_hash = last_block.hash.clone();
        let mut new_block = Block::new(new_index, prev_hash, data);
        new_block.mine(diffuilty);
        self.chain.push(new_block)
    }
}

impl Transaction {
    fn new(sender: String, recipient: String, amount: u64) -> Self{
        Transaction{sender, recipient, amount}
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}->{}:{}", self.sender, self.recipient, self.amount)
    }
}

fn main() {
    let mut crabchain = Blockchain::new();
    crabchain.add_block(
        vec![Transaction::new("Alice".into(), "Bob".into(), 5)],
        4
    );
    crabchain.add_block(vec![
        Transaction::new("Bob".into(), "Charlie".into(), 5)
    ], 4);

    println!("{:#?}", crabchain);
}

