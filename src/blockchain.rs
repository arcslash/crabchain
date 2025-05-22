use crate::block::Block;
use crate::signed_transaction::SignedTransaction;
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, String::new(), vec![]);
        Blockchain{
            chain: vec![genesis_block]
        }
    }
    pub fn add_block(&mut self, data: Vec<SignedTransaction>, diffuilty: usize) {
        let last_block = self.chain.last().unwrap();
        let new_index = last_block.index + 1;
        let prev_hash = last_block.hash.clone();

        for tx in &data {
            if !tx.is_valid() {
                panic!("Invalid transaction detected!");
            }
        }

        let mut new_block = Block::new(new_index, prev_hash, data);
        
        new_block.mine(diffuilty);
        self.chain.push(new_block)
    }

    pub fn is_valid(&self) -> bool{
        for i in 1..self.chain.len(){
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // check the hash linkage to the previous block
            if current.previous_hash != previous.hash{
                return false;
            }
            // Check hash integrity
            if current.calculate_hash() != current.hash{
                return false;
            }
        }
        true
    }
}