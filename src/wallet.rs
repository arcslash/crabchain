use std::fs;
use ed25519_dalek::ed25519::signature::Keypair;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

pub struct Wallet {
    signing_key: SigningKey,
}

#[derive(Serialize, Deserialize)]
pub struct WalletFile{
    pub private_key: Vec<u8>
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
    
    pub fn save_to_file(&self, path: &str){
        let wallet_file = WalletFile{
            private_key: self.signing_key.to_bytes().to_vec()
        };
        let json = serde_json::to_string_pretty(&wallet_file).unwrap();
        fs::write(path, json).unwrap();
        println!("💾 Wallet saved to {}", path);
    }
    
    pub fn from_file(path: &str) -> Self{
        let data = fs::read_to_string(path).expect("Error reading wallet file");
        let wallet_file: WalletFile = serde_json::from_str(&data).unwrap();
        let signing_key = SigningKey::from_bytes(&wallet_file.private_key.try_into().unwrap());
        Wallet { signing_key }
    }
}
