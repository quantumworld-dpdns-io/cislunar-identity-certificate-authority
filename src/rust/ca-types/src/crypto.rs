use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyType {
    Rsa2048,
    Rsa4096,
    EcdsaP256,
    EcdsaP384,
    Ed25519,
    PqcKyber768,
    PqcKyber1024,
    PqcDilithium3,
    PqcDilithium5,
    HybridEcdsaPqc,
}

impl KeyType {
    pub fn algorithm_oid(&self) -> &str {
        match self {
            KeyType::Rsa2048 | KeyType::Rsa4096 => "1.2.840.113549.1.1.1",
            KeyType::EcdsaP256 => "1.2.840.10045.2.1",
            KeyType::EcdsaP384 => "1.2.840.10045.2.1",
            KeyType::Ed25519 => "1.3.101.112",
            KeyType::PqcKyber768 | KeyType::PqcKyber1024 => "2.16.840.1.114027.80.4.1",
            KeyType::PqcDilithium3 | KeyType::PqcDilithium5 => "2.16.840.1.114027.80.3.1",
            KeyType::HybridEcdsaPqc => "2.16.840.1.114027.80.5.1",
        }
    }

    pub fn security_bits(&self) -> u32 {
        match self {
            KeyType::Rsa2048 => 112,
            KeyType::Rsa4096 => 128,
            KeyType::EcdsaP256 => 128,
            KeyType::EcdsaP384 => 192,
            KeyType::Ed25519 => 128,
            KeyType::PqcKyber768 => 192,
            KeyType::PqcKyber1024 => 256,
            KeyType::PqcDilithium3 => 128,
            KeyType::PqcDilithium5 => 192,
            KeyType::HybridEcdsaPqc => 256,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub key_type: KeyType,
    #[serde(with = "hex")]
    public_key: Vec<u8>,
    #[serde(with = "hex")]
    #[zeroize(skip)]
    private_key: Vec<u8>,
    pub created_at: String,
}

impl KeyPair {
    pub fn new(key_type: KeyType, public_key: Vec<u8>, private_key: Vec<u8>) -> Self {
        Self {
            key_type,
            public_key,
            private_key,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn public_key_bytes(&self) -> &[u8] {
        &self.public_key
    }

    pub fn private_key_bytes(&self) -> &[u8] {
        &self.private_key
    }
}
