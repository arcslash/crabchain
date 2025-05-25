mod blockchain;
mod block;
mod transaction;
mod wallet;
mod signed_transaction;

use ed25519_dalek::Verifier;
use transaction::Transaction;
use blockchain::Blockchain;
use wallet::Wallet;
use crate::signed_transaction::SignedTransaction;

fn main() {
    let mut crabchain = Blockchain::new();
    // crabchain.add_block(
    //     vec![Transaction::new("Alice".into(), "Bob".into(), 100)],
    //     4
    // );
    // crabchain.add_block(vec![
    //     Transaction::new("Bob".into(), "Charlie".into(), 90)
    // ], 4);
    // crabchain.add_block(vec![
    //     Transaction::new("Charlie".into(), "Alex".into(), 85)
    // ], 4);
    //
    // crabchain.add_block(vec![
    //     Transaction::new("Alex".into(), "Ishara".into(), 80)
    // ], 4);

    
    // Transaction will be in two steps now\
    let sender_wallet = Wallet::new();
    // 1. Create a transaction
    let reward_tx = Transaction::new("SYSTEM".into(), "Alice".into(), 1000);
    let tx1 = Transaction::new("Alice".into(), "Bob".into(), 100);
    let message = tx1.to_string();
    println!("{}", message);
    let sig = sender_wallet.sign_message(message.as_bytes());
    let signed_tx = SignedTransaction{
        transaction: tx1,
        public_key: sender_wallet.get_verifying_key(),
        signature: sig,
    };
    crabchain.add_block(vec![signed_tx], 4);

    println!("{:#?}", crabchain);

    // let valid = crabchain.is_valid();
    // // Is the built chain valid?
    // println!("Is CrabChain valid {}", valid); // should return up true


    // let's tamper a bit
    /*
    crabchain.chain[1].data[0].amount = 10;
    valid = crabchain.is_valid();
    println!("Is CrabChain valid {}", valid); // should return up false
    */

    // println!("{:#?}", crabchain);
    // 
    // let wallet = Wallet::new();
    // println!("Public key: {}", wallet.get_public_key());
    // 
    // let msg = b"hello crabchain";
    // let signature = wallet.sign_message(msg);
    // println!("Signature: {:?}", signature.to_bytes());
    // 
    // let public_key = wallet.get_verifying_key();
    // let is_valid = public_key.verify(msg, &signature).is_ok();
    // println!("Signature valid? {}", is_valid);

    let balances = crabchain.get_balances();
    println!("\nFinal balances:");
    for (user, balance) in balances {
        println!("{}: {}", user, balance);
    }


}

