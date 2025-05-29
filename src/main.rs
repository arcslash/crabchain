mod blockchain;
mod block;
mod transaction;
mod wallet;
mod signed_transaction;
mod cli;

use clap::Parser;
use ed25519_dalek::Verifier;
use wallet::Wallet;
use crate::cli::{Cli, Commands};
use crate::signed_transaction::SignedTransaction;
use crate::transaction::Transaction;

// Moving into CLI control
fn main() {
    let cli = Cli::parse();
    
    match cli.command{
        Commands::WalletNew => {
            println!("Generating a new Wallet...");
            let wallet = Wallet::new(); 
            wallet.save_to_file("wallet.json");
            println!("🔑 Public address: {}", wallet.get_public_key());
            println!("Wallet saved to wallet.json");
        }
        
        Commands::WalletAddress { keyfile } => {
            println!("Getting the address of the wallet {}", keyfile);
            let wallet = Wallet::from_file("wallet.json");
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
        
        
        Commands::Mine {miner} => {
            println!("Mining block for {}", miner);
        }
        
        Commands::Balances =>{
            println!("Getting balances");
        }
        
    }
    
    
}
