mod blockchain;
mod block;
mod transaction;
mod wallet;
mod signed_transaction;
mod cli;

use std::fs;
use clap::Parser;
use ed25519_dalek::Verifier;
use wallet::Wallet;
use crate::blockchain::Blockchain;
use crate::cli::{Cli, Commands};
use crate::signed_transaction::SignedTransaction;
use crate::transaction::Transaction;
use crate::wallet::{register_wallet, WalletInfo};

// Moving into CLI control
fn main() {
    let cli = Cli::parse();
    
    match cli.command{
        Commands::WalletNew => {
            println!("Generating a new Wallet...");
            let wallet = Wallet::new();

            // Prompt for filename
            let wallet_key_filename = inquire::Text::new("Save wallet key as (without .json):")
                .prompt()
                .unwrap();

            // Ensure wallets directory exists
            fs::create_dir_all("wallets").expect("Failed to create wallets directory");

            // Construct full path
            let wallet_file = format!("wallets/{}.json", wallet_key_filename);

            // Save wallet key
            wallet.save_to_file(&wallet_file);

            let public_key = wallet.get_public_key();
            println!("🔑 Public address: {}", public_key);

            // Prompt for user-friendly name
            let name = inquire::Text::new("Enter a name for this wallet:")
                .prompt()
                .unwrap();

            // Register wallet metadata
            register_wallet(&name, &public_key, &wallet_file);

            println!("💾 Wallet saved to {} and registered as '{}'", wallet_file, name);
        }


        Commands::WalletAddress { keyfile } => {
            println!("Getting the address of the wallet {}", keyfile);
            let wallet = Wallet::from_file("wallets/system_wallet.json");
            println!("Wallet address: {}", wallet.get_public_key());
        }
        
        Commands::Send { keyfile, from, to, amount } => {
            println!("Sending {} from {} to {}", amount, from, to);
            let wallet = Wallet::from_file(&keyfile);
            let tx = Transaction{
                sender: from,
                recipient: to,
                amount,
            };
            let message = tx.to_string();
            let signature = wallet.sign_message(message.as_bytes());
            let signed_tx = SignedTransaction{
                transaction: tx,
                public_key: wallet.get_verifying_key(),
                signature,
            };

            let mempool_file = "mempool.json";

            // Read existing mempool (if any)
            let mut mempool: Vec<SignedTransaction> = if std::path::Path::new(mempool_file).exists() {
                let contents = std::fs::read_to_string(mempool_file).unwrap();
                serde_json::from_str(&contents).unwrap()
            } else {
                vec![]
            };

            // Add the new transaction
            mempool.push(signed_tx);

            // Write back to file
            let json = serde_json::to_string_pretty(&mempool).unwrap();
            std::fs::write(mempool_file, json).unwrap();

            println!("✅ Transaction saved to mempool.json");
        }


        Commands::Mine { miner } => {
            println!("⛏️ Mining block for {}", miner);
            let chain_file = "chain.json";
            let mempool_file = "mempool.json";

            // Load blockchain
            let mut crabchain = if std::path::Path::new(chain_file).exists() {
                let chain_data = std::fs::read_to_string(chain_file).unwrap();
                serde_json::from_str(&chain_data).unwrap()
            } else {
                Blockchain::new()
            };

            // Load mempool
            if std::path::Path::new(mempool_file).exists() {
                let contents = std::fs::read_to_string(mempool_file).unwrap();
                let mut mempool: Vec<SignedTransaction> = serde_json::from_str(&contents).unwrap();

                // Add mining reward
                let reward_tx = Transaction::new("SYSTEM".into(), miner.clone(), 50);
                let reward = SignedTransaction::reward(reward_tx);
                mempool.push(reward);

                // Mine the block and add it
                crabchain.validate_mempool(mempool.as_slice());
                crabchain.add_block(mempool, 4, miner);

                // Save the updated chain
                let updated = serde_json::to_string_pretty(&crabchain).unwrap();
                std::fs::write(chain_file, updated).unwrap();
                println!("✅ Block mined and added to chain!");

                // Clear the mempool
                std::fs::remove_file(mempool_file).unwrap();
            } else {
                println!("⚠️ No transactions in mempool");
            }
        }


        Commands::Balances =>{
            println!("📊 Balances:");
            let crabchain = Blockchain::new(); 
            let balances = crabchain.get_balances();

            for (address, balance) in balances {
                println!("{}: {}", address, balance);
            }
        }

        Commands::WalletList => {
            let data = std::fs::read_to_string("wallets.json").unwrap();
            let wallets: Vec<WalletInfo> = serde_json::from_str(&data).unwrap();
            for w in wallets {
                println!("{} => {}", w.name, w.public_key);
            }
        }


    }
    
    
}
