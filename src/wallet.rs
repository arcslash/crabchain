use ed25519_dalek::ed25519::signature::Keypair;
use rand::rand_core::OsRng;

pub struct Wallet{
    keypair: dyn Keypair<VerifyingKey=()>
}

impl Wallet{
    // pub fn new() -> Self{
    //     let keypair: Keypair = Keypair::generate(&mut OsRng);
    //     Wallet{keypair}
    // }
    //
    // pub fn get_public_key(&self) -> String {
    //     // public key as hex
    // }
}