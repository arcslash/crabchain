use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::block::Block;
use crate::signed_transaction::SignedTransaction;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

// TODO: implement serialization
//#[derive(Debug, Serialize, Deserialize)]
#[derive(Debug)]
pub struct Blockchain {
    //#[serde(serialize_with = "serialize_chain", deserialize_with = "deserialize_chain")]
    chain: Vec<Block>,
}

const BLOCK_REWARD: u64 = 100;
const SYSTEM_ACCOUNT: &str = "SYSTEM";

// pub fn serialize_chain(){
// 
// }
// 
// pub fn deserialize_chain(){
// 
// }

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, String::new(), vec![]);
        Blockchain{
            chain: vec![genesis_block]
        }
    }
    pub fn add_block(&mut self, mut data: Vec<SignedTransaction>, difficulty: usize, miner_address: String) {
        
        // Adding Miner Rewards here
        let system_wallet = Wallet::new();
        
        let reward_tx = Transaction::new(
            SYSTEM_ACCOUNT.into(),
            miner_address,
            BLOCK_REWARD, // TODO: Make this a calculated value later
        );
        
        let reward_signature = system_wallet.sign_message(reward_tx.to_string().as_bytes());
        let reward_signed_tx = SignedTransaction{
            transaction: reward_tx,
            public_key: system_wallet.get_verifying_key(),
            signature: reward_signature
        };
        data.insert(0, reward_signed_tx);
        
        
        
        // lets check balances
        let mut balances = self.get_balances();

        for signed_tx in &data {
            let sender = &signed_tx.transaction.sender;
            let amount = signed_tx.transaction.amount;
            
            if sender != "SYSTEM" {
                let balance = balances.get(sender).cloned().unwrap_or(0);
                if balance < amount {
                    panic!("Insufficient funds for {}", sender);
                }

                // Simulate the transfer so multiple txs in one block are validated
                *balances.entry(sender.clone()).or_insert(0) -= amount;
            }
            
            
            
            
            *balances.entry(signed_tx.transaction.recipient.clone()).or_insert(0) += amount;
        }

        let last_block = self.chain.last().unwrap();
        let new_index = last_block.index + 1;
        let prev_hash = last_block.hash.clone();
        for tx in &data {
            if !tx.is_valid() {
                panic!("Invalid transaction detected!");
            }
        }
        let mut new_block = Block::new(new_index, prev_hash, data);
        
        new_block.mine(difficulty);
        self.chain.push(new_block)
    }

    pub fn is_valid(&self) -> bool {
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


    pub fn get_balances(&self) -> HashMap<String, u64> {
        let mut balances: HashMap<String, u64> = HashMap::new();

        for block in &self.chain {
            for signed_tx in &block.data {
                let sender = signed_tx.transaction.sender.clone();
                let recipient = signed_tx.transaction.recipient.clone();
                let amount = signed_tx.transaction.amount;

                if sender != "SYSTEM" {
                    // Subtract from sender
                    *balances.entry(sender).or_insert(0) -= amount;
                }
                // Add to recipient
                *balances.entry(recipient).or_insert(0) += amount;
            }
        }
        balances
    }
}