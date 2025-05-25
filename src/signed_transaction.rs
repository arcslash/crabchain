use std::fmt;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub public_key: VerifyingKey,
    pub signature: Signature
}

impl SignedTransaction {
    pub fn is_valid(&self) -> bool {
        // Serialize the transaction to bytes
        let message = self.transaction.to_string();
        self.public_key.verify(message.as_bytes(), &self.signature).is_ok()
    }
}

impl fmt::Display for SignedTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.transaction.fmt(f)
    }
}