use ed25519_dalek::ed25519::signature::Keypair;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use rand_core::OsRng;

pub struct Wallet {
    signing_key: SigningKey,
}

impl Wallet {
    pub fn new() -> Self {
        // Generate a full Keypair first
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);

        Wallet { signing_key }
    }

    pub fn get_public_key(&self) -> String {
        let pubkey_bytes = self.signing_key.verifying_key().to_bytes();
        hex::encode(pubkey_bytes)
    }
    
    pub fn get_verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }
    
    pub fn sign_message(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }
}
