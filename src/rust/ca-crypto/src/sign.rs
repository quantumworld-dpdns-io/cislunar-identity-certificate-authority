use ca_types::{CaResult, KeyType, KeyPair, Certificate};

pub fn sign_certificate(cert: &mut Certificate, _key_pair: &KeyPair) -> CaResult<()> {
    cert.fingerprint_sha256 = sha256_fingerprint(&cert.pem);
    Ok(())
}

pub fn sha256_fingerprint(data: &str) -> String {
    use sha2::{Sha256, Digest};
    let hash = Sha256::digest(data.as_bytes());
    hex::encode(hash)
}
