use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use crate::transaction::Transaction;

pub struct SignedTransaction {
    pub transaction: Transaction,
    pub public_key: VerifyingKey,
    pub signature: Signature
}

impl SignedTransaction {
    pub fn is_valid(&self) -> bool{
        // Serialize the transaction to bytes
        let message = self.transaction.to_string();
        self.public_key.verify(message.as_bytes(), &self.signature).is_ok()
    }
}