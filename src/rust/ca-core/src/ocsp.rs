use ca_types::*;

#[derive(Debug, Clone)]
pub enum OcspStatus {
    Good,
    Revoked(RevocationReason),
    Unknown,
}

#[derive(Debug, Clone)]
pub struct OcspRequest {
    pub serial_number: String,
    pub issuer_hash: String,
}

#[derive(Debug, Clone)]
pub struct OcspResponse {
    pub serial_number: String,
    pub status: OcspStatus,
    pub produced_at: String,
    pub this_update: String,
    pub next_update: String,
}

pub struct OcspResponder {
    cert_store: Box<dyn CertificateStore>,
}

impl OcspResponder {
    pub fn new(cert_store: Box<dyn CertificateStore>) -> Self {
        Self { cert_store }
    }

    pub async fn respond(&self, request: &OcspRequest) -> CaResult<OcspResponse> {
        let cert = self.cert_store.get_certificate(&request.serial_number).await;
        let status = match cert {
            Ok(c) => match c.status {
                CertStatus::Active | CertStatus::Pending => OcspStatus::Good,
                CertStatus::Revoked => OcspStatus::Revoked(
                    c.revocation_reason.unwrap_or(RevocationReason::Unspecified)),
                CertStatus::Expired => OcspStatus::Revoked(RevocationReason::CessationOfOperation),
            },
            Err(_) => OcspStatus::Unknown,
        };
        let now = chrono::Utc::now();
        Ok(OcspResponse {
            serial_number: request.serial_number.clone(),
            status,
            produced_at: now.to_rfc3339(),
            this_update: now.to_rfc3339(),
            next_update: (now + chrono::Duration::hours(4)).to_rfc3339(),
        })
    }
}
