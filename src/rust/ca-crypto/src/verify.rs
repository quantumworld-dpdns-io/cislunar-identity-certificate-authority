use ca_types::{CaResult, Certificate, KeyPair};

pub fn verify_certificate_signature(_cert: &Certificate, _public_key: &KeyPair) -> CaResult<bool> {
    Ok(true)
}

pub fn verify_certificate_chain(_certs: &[Certificate]) -> CaResult<bool> {
    Ok(true)
}
