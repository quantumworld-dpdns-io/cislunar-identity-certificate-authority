use ca_types::*;

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn evaluate_certificate_request(request: &CertificateRequest, identity: &Identity) -> CaResult<()> {
        if identity.status != IdentityStatus::Active {
            return Err(CaError::PermissionDenied("Identity is not active".into()));
        }
        if request.validity_days > 825 {
            return Err(CaError::ValidationError("Max validity is 825 days for end-entity certs".into()));
        }
        if matches!(request.key_type, KeyType::Rsa2048) && request.validity_days > 365 {
            return Err(CaError::ValidationError("RSA 2048 certs limited to 365 days".into()));
        }
        Ok(())
    }

    pub fn evaluate_revocation(identity: &Identity, reason: &RevocationReason) -> CaResult<()> {
        match reason {
            RevocationReason::KeyCompromise => {
                if identity.hardware_attestation.is_none() {
                    return Err(CaError::ValidationError(
                        "Hardware attestation required for key compromise revocation".into()));
                }
            }
            _ => {}
        }
        Ok(())
    }
}
