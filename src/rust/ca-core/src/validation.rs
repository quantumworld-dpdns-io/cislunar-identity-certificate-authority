use ca_types::*;

pub struct CertValidator;

impl CertValidator {
    pub fn validate_csr(_csr: &CertificateRequest) -> CaResult<()> {
        if _csr.subject.is_empty() {
            return Err(CaError::ValidationError("Subject cannot be empty".into()));
        }
        if _csr.validity_days == 0 || _csr.validity_days > 8250 {
            return Err(CaError::ValidationError(
                format!("Invalid validity period: {} days", _csr.validity_days)));
        }
        Ok(())
    }

    pub fn validate_certificate_chain(chain: &[Certificate]) -> CaResult<bool> {
        if chain.is_empty() {
            return Err(CaError::ValidationError("Empty certificate chain".into()));
        }
        let root = &chain[0];
        if root.ca_type != CaType::Root {
            return Err(CaError::ValidationError("First cert in chain must be root CA".into()));
        }
        for cert in chain {
            if !cert.validity.is_valid() {
                return Err(CaError::ValidationError(
                    format!("Certificate {} is expired", cert.serial_number)));
            }
        }
        Ok(true)
    }

    pub fn validate_key_usage(cert: &Certificate, required_usage: &str) -> CaResult<bool> {
        Ok(cert.key_usage.contains(&required_usage.to_string()))
    }

    pub fn validate_extended_key_usage(cert: &Certificate, required_eku: &str) -> CaResult<bool> {
        Ok(cert.extended_key_usage.contains(&required_eku.to_string()))
    }
}
