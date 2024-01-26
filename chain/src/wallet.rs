use ripemd::Digest;
use secp256k1::rand::rngs::OsRng;
use secp256k1::Secp256k1;
use secp256k1::{PublicKey, SecretKey};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Wallet {
    public_key: PublicKey,
    private_key: SecretKey,
    address: String,
}

impl Wallet {
    pub fn new(public_key: PublicKey, private_key: SecretKey) -> Self {
        Self {
            public_key,
            private_key,
            address: Self::generate_address(public_key),
        }
    }
    pub fn address(&self) -> String {
        self.address.clone()
    }
    pub fn generate_new() -> Self {
        let secp = Secp256k1::new();
        let (private_key, public_key) = secp.generate_keypair(&mut OsRng);
        let address = Self::generate_address(public_key);

        Self {
            public_key,
            private_key,
            address,
        }
    }

    pub fn generate_address(public_key: PublicKey) -> String {
        //Take the corresponding public key generated with it (33 bytes, 1 byte 0x02 (y-coord is even), and 32 bytes corresponding to X coordinate)
        let address = public_key.serialize_uncompressed();
        //Perform SHA-256 hashing on the public key

        let address_hash = sha2::Sha256::digest(address);

        // Perform ripemd-160 hashing on the result of SHA-256
        let raw_rip_address = ripemd::Ripemd160::digest(&address_hash);
        let rip_address = raw_rip_address.as_slice();

        // Add version byte in front of RIPEMD-160 hash (0x00 for Main Network, 0x6f for Testnet)
        let versioned_rip_address: Vec<u8> = [[0x6f].as_ref(), rip_address].concat();
        // Perform SHA-256 hash on the extended RIPEMD-160 result
        let versioned_rip_address_hash = sha2::Sha256::digest(versioned_rip_address.clone());
        //Perform SHA-256 hash on the result of the previous SHA-256 hash
        let double_hashed_address = sha2::Sha256::digest(versioned_rip_address_hash);
        //Take the first 4 bytes of the second SHA-256 hash. This is the address checksum
        let chksum: [u8; 4] = double_hashed_address[0..4]
            .try_into()
            .expect("Wrong length");

        // Add the 4 checksum bytes from stage 7 at the end of extended RIPEMD-160 hash from stage 4. This is the 25-byte binary Bitcoin Address.
        let chcksum_ripemd = [versioned_rip_address.clone(), chksum.to_vec()].concat();

        bs58::encode(chcksum_ripemd).into_string()
    }
}

#[cfg(test)]
mod tests {

    use reqwest;

    use secp256k1::rand::rngs::OsRng;
    use secp256k1::Secp256k1;
    use serde::Deserialize;
    use std::env;

    use crate::wallet;

    #[derive(Debug, Deserialize)]
    struct Response {
        apiVersion: String,
        requestId: String,
        context: String,
        data: Data,
    }

    #[derive(Debug, Deserialize)]
    struct Data {
        item: Item,
    }

    #[derive(Debug, Deserialize)]
    struct Item {
        address: String,
        isValid: bool,
    }

    #[tokio::test]
    async fn valid_address() {
        let dot_env = dotenv::dotenv().ok().unwrap();

        let api_key = env::var("CRYPTO_API_KEY_TEST").expect("API key not found");

        let secp = Secp256k1::new();
        let client = reqwest::Client::new();

        let (_, public_key) = secp.generate_keypair(&mut OsRng);
        let address = wallet::Wallet::generate_address(public_key);

        let context = String::from("blocksmith address test");
        let req_body = serde_json::json!({
            "context" : context,
            "data": {
                "item" : {
                    "address" : address
                }
            }
        });

        let res = client.post(format!(
            "https://rest.cryptoapis.io/blockchain-tools/bitcoin/testnet/addresses/validate?context={}",context),
        ).header("Content-Type", "application/json")
        .header("X-API-Key", api_key)
        .json(&req_body)
        .send()
        .await
        .expect("Failed to send reqwest");

        let res_text = res.text().await.expect("Failed to read response text");
        let deserialized: Response =
            serde_json::from_str(&res_text).expect("Failed to deserialize");

        println!("isValid: {:#?}", deserialized.data.item.isValid);

        assert!(deserialized.data.item.isValid);
    }
}
