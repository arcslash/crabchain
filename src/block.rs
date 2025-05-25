use chrono::Utc;
use sha2::{Digest, Sha256};
use crate::signed_transaction::SignedTransaction;

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub data: Vec<SignedTransaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64
}
impl Block {
    pub fn new(index: u64,
           previous_hash: String,
           data: Vec<SignedTransaction>) -> Self{

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

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Mined block with hash: {}", self.hash);

    }


    pub fn calculate_hash(&self) -> String {
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