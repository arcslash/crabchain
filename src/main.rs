mod blockchain;
mod block;
mod transaction;

use transaction::Transaction;
use blockchain::Blockchain;

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

    
    
    let mut valid = crabchain.is_valid();
    // Is the built chain valid?
    println!("Is CrabChain valid {}", valid); // should return up true
    
    
    // let's tamper a bit
    /*
    crabchain.chain[1].data[0].amount = 10;
    valid = crabchain.is_valid();
    println!("Is CrabChain valid {}", valid); // should return up false
    */
    
    println!("{:#?}", crabchain);
}

