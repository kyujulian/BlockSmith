use secp256k1::rand::rngs::OsRng;
use secp256k1::Secp256k1;
use secp256k1::{PublicKey, SecretKey};
pub struct Wallet {
    public_key: PublicKey,
    private_key: SecretKey,
    balance: i64,
}

impl Wallet {
    pub fn new(public_key: PublicKey, private_key: SecretKey, balance: i64) -> Self {
        Self {
            public_key,
            private_key,
            balance,
        }
    }
    pub fn generate_new() -> Self {
        let secp = Secp256k1::new();
        let (private_key, public_key) = secp.generate_keypair(&mut OsRng);

        Self {
            public_key,
            private_key,
            balance: 0,
        }
    }
}
