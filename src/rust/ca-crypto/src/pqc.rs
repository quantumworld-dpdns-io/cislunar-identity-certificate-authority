use ca_types::{CaResult, KeyType, KeyPair};

pub fn generate_pqc_keypair(key_type: KeyType) -> CaResult<KeyPair> {
    match key_type {
        KeyType::PqcKyber768 => kem_keygen("Kyber768"),
        KeyType::PqcKyber1024 => kem_keygen("Kyber1024"),
        KeyType::PqcDilithium3 => signature_keygen("Dilithium3"),
        KeyType::PqcDilithium5 => signature_keygen("Dilithium5"),
        KeyType::HybridEcdsaPqc => hybrid_keygen(),
        _ => Err(ca_types::CaError::InvalidKeyType(format!("{:?}", key_type))),
    }
}

fn kem_keygen(_alg: &str) -> CaResult<KeyPair> {
    Err(ca_types::CaError::CryptoError("PQC KEM keygen not yet implemented".into()))
}

fn signature_keygen(_alg: &str) -> CaResult<KeyPair> {
    Err(ca_types::CaError::CryptoError("PQC signature keygen not yet implemented".into()))
}

fn hybrid_keygen() -> CaResult<KeyPair> {
    Err(ca_types::CaError::CryptoError("Hybrid keygen not yet implemented".into()))
}
