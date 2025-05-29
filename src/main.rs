mod blockchain;
mod block;
mod transaction;
mod wallet;
mod signed_transaction;
mod cli;

use clap::Parser;
use ed25519_dalek::Verifier;
use transaction::Transaction;
use blockchain::Blockchain;
use wallet::Wallet;
use crate::cli::{Cli, Commands};
use crate::signed_transaction::SignedTransaction;
use crate::wallet::WalletFile;
// fn main() {
//     let mut crabchain = Blockchain::new();
//     
//     // Transaction will be in two steps now
//     let sender_wallet = Wallet::new();
//     
//     let reward_tx = Transaction::new("SYSTEM".into(), "Alice".into(), 1000);
//     let reward_tx_clone = reward_tx.clone();
//     let reward_signed = SignedTransaction{
//         transaction: reward_tx,
//         public_key: sender_wallet.get_verifying_key(),
//         signature: sender_wallet.sign_message(reward_tx_clone.to_string().as_bytes()),
//     };
//     crabchain.add_block(vec![reward_signed], 4, "Miner1".into());
//     let tx1 = Transaction::new("Alice".into(), "Bob".into(), 100);
//     let message = tx1.to_string();
//     println!("{}", message);
//     let sig = sender_wallet.sign_message(message.as_bytes());
//     let signed_tx = SignedTransaction{
//         transaction: tx1,
//         public_key: sender_wallet.get_verifying_key(),
//         signature: sig,
//     };
//     crabchain.add_block(vec![signed_tx], 4, "Miner1".into());
// 
//     println!("{:#?}", crabchain);
// 
//     let balances = crabchain.get_balances();
//     println!("\nFinal balances:");
//     for (user, balance) in balances {
//         println!("{}: {}", user, balance);
//     }
// 
// 
// }

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
        
        Commands::Send { from, to, amount } => {
            println!("Sending {} from {} to {}", amount, from, to);
        }
        
        
        Commands::Mine {miner} => {
            println!("Mining block for {}", miner);
        }
        
        Commands::Balances =>{
            println!("Getting balances");
        }
        
    }
    
    
}
