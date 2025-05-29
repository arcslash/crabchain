use std::fmt;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;
use crate::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    
    #[serde(serialize_with = "serialize_verifying_key", deserialize_with = "deserialize_verifying_key")]
    pub public_key: VerifyingKey,
    #[serde(serialize_with = "serialize_signature", deserialize_with = "deserialize_signature")]
    pub signature: Signature
}

// Serialization and Deserialization functions

/// Serialize a Signature as hex string
/// 
pub fn serialize_verifying_key<S>(key: &VerifyingKey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&hex::encode(key.to_bytes()))
}

pub fn serialize_signature<S>(sig: &Signature, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&hex::encode(sig.to_bytes()))
}

/// Deserialize a Signature from hex string
pub fn deserialize_signature<'de, D>(deserializer: D) -> Result<Signature, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let bytes = hex::decode(&s).map_err(D::Error::custom)?;

    let fixed: [u8; 64] = bytes
        .try_into()
        .map_err(|_| D::Error::custom("Expected 64-byte signature"))?;

    // ✅ Signature::from_bytes returns Signature directly
    Ok(Signature::from_bytes(&fixed))
}

pub fn deserialize_verifying_key<'de, D>(deserializer: D) -> Result<VerifyingKey, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let bytes = hex::decode(&s).map_err(D::Error::custom)?;

    let fixed: [u8; 32] = bytes
        .try_into()
        .map_err(|_| D::Error::custom("Expected 32-byte public key"))?;

    VerifyingKey::from_bytes(&fixed).map_err(D::Error::custom)
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