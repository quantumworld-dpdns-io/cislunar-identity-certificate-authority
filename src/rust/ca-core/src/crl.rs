use ca_types::*;

pub struct CrlManager {
    cert_store: Box<dyn CertificateStore>,
    crl_store: Box<dyn CrlStore>,
}

impl CrlManager {
    pub fn new(cert_store: Box<dyn CertificateStore>, crl_store: Box<dyn CrlStore>) -> Self {
        Self { cert_store, crl_store }
    }

    pub async fn generate_crl(&self, issuer: &str) -> CaResult<CertificateRevocationList> {
        let revoked = self.cert_store.list_certificates(&CertFilter {
            status: Some("Revoked".into()),
            ..Default::default()
        }).await?;

        let entries: Vec<CrlEntry> = revoked.iter()
            .filter_map(|c| {
                let reason = c.revocation_reason.clone().unwrap_or(RevocationReason::Unspecified);
                let date = c.revocation_date.clone().unwrap_or_else(|| chrono::Utc::now().to_rfc3339());
                Some(CrlEntry {
                    serial_number: c.serial_number.clone(),
                    revocation_date: date,
                    reason,
                })
            }).collect();

        let now = chrono::Utc::now();
        let crl = CertificateRevocationList {
            issuer: issuer.into(),
            this_update: now.to_rfc3339(),
            next_update: (now + chrono::Duration::days(7)).to_rfc3339(),
            entries,
            crl_number: chrono::Utc::now().timestamp() as u64,
            signature: vec![],
        };

        self.crl_store.store_crl(&crl).await?;
        Ok(crl)
    }

    pub async fn check_revoked(&self, serial: &str) -> CaResult<bool> {
        self.crl_store.is_revoked(serial).await
    }
}
