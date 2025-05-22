mod blockchain;
mod block;
mod transaction;
mod wallet;
mod signed_transaction;

use ed25519_dalek::Verifier;
use transaction::Transaction;
use blockchain::Blockchain;
use wallet::Wallet;

fn main() {
    let mut crabchain = Blockchain::new();
    crabchain.add_block(
        vec![Transaction::new("Alice".into(), "Bob".into(), 100)],
        4
    );
    crabchain.add_block(vec![
        Transaction::new("Bob".into(), "Charlie".into(), 90)
    ], 4);
    crabchain.add_block(vec![
        Transaction::new("Charlie".into(), "Alex".into(), 85)
    ], 4);

    crabchain.add_block(vec![
        Transaction::new("Alex".into(), "Ishara".into(), 80)
    ], 4);



    let valid = crabchain.is_valid();
    // Is the built chain valid?
    println!("Is CrabChain valid {}", valid); // should return up true


    // let's tamper a bit
    /*
    crabchain.chain[1].data[0].amount = 10;
    valid = crabchain.is_valid();
    println!("Is CrabChain valid {}", valid); // should return up false
    */

    println!("{:#?}", crabchain);

    let wallet = Wallet::new();
    println!("Public key: {}", wallet.get_public_key());
    
    let msg = b"hello crabchain";
    let signature = wallet.sign_message(msg);
    println!("Signature: {:?}", signature.to_bytes());
    
    let public_key = wallet.get_verifying_key();
    let is_valid = public_key.verify(msg, &signature).is_ok();
    println!("Signature valid? {}", is_valid);
    
    
    
}

