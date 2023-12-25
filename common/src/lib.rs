use hex;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use std::str::FromStr;

pub struct EthKey {
    private_key: SecretKey,
    address: String,
}

impl EthKey {
    pub fn get_lowercase_address_with_0x_prefix(&self) -> String {
        return self.address.clone();
    }

    pub fn get_secret_key_string(&self) -> String {
        hex::encode(self.private_key.secret_bytes())
    }
}

pub fn generate_from_private(private_key: SecretKey) -> EthKey {
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, &private_key);
    let hashed_public_key = Keccak256::digest(&public_key.serialize_uncompressed()[1..]);
    let address = hex::encode(&hashed_public_key[12..]);
    let format_address = format!("0x{}", address);

    EthKey {
        private_key: private_key,
        address: format_address,
    }
}

pub fn generate_random_private_key() -> SecretKey {
    SecretKey::new(&mut rand::thread_rng())
}

pub fn genetrate_random_eth_key() -> EthKey {
    let private_key = generate_random_private_key();
    generate_from_private(private_key)
}

pub fn generate_private_key_from_known_str(key_str: &str) -> SecretKey {
    SecretKey::from_str(key_str).unwrap()
}

pub fn generate_private_key_from_knwon_wallet_str(wallet_str: &str) -> SecretKey {
    let bytes = hex::decode(wallet_str).expect("Failed to decode hex string");
    let secret_key = SecretKey::from_slice(&bytes).unwrap();

    secret_key
}

#[cfg(test)]
mod tests {
    use crate::generate_from_private;
    use crate::generate_private_key_from_known_str;
    use crate::generate_private_key_from_knwon_wallet_str;
    use crate::genetrate_random_eth_key;

    #[test]
    fn test_generate_from_known_key_address() {
        let test_known_key = "ea6c44ac03bff858b476bba40716402b03e41b8e97e276d1baec7c37d42484a0";
        let test_known_address = "0x2546BcD3c84621e976D8185a91A922aE77ECEc30".to_lowercase();
        let private_key = generate_private_key_from_knwon_wallet_str(test_known_key);
        let eth_key = generate_from_private(private_key);
        assert_eq!(eth_key.address, test_known_address);
    }
}
