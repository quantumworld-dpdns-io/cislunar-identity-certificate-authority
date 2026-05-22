use ca_types::*;
use ca_crypto::*;

pub struct CertificateAuthority {
    pub config: CaConfig,
    pub cert_store: Box<dyn CertificateStore>,
    pub crl_store: Box<dyn CrlStore>,
    pub key_store: Box<dyn KeyStore>,
    pub identity_store: Box<dyn IdentityStore>,
    pub profile_store: Box<dyn ProfileStore>,
}

pub struct CaConfig {
    pub organization: String,
    pub country: String,
    pub ca_key_type: KeyType,
    pub default_validity_days: u32,
    pub crl_validity_days: u32,
    pub enable_pqc: bool,
    pub enable_tee: bool,
    pub max_certificates_per_identity: u32,
}

impl CertificateAuthority {
    pub fn new(config: CaConfig) -> Self {
        Self {
            config,
            cert_store: Box::new(store::InMemoryCertStore::new()),
            crl_store: Box::new(store::InMemoryCrlStore::new()),
            key_store: Box::new(store::InMemoryKeyStore::new()),
            identity_store: Box::new(store::InMemoryIdentityStore::new()),
            profile_store: Box::new(store::InMemoryProfileStore::new()),
        }
    }

    pub fn with_stores(
        config: CaConfig,
        cert_store: Box<dyn CertificateStore>,
        crl_store: Box<dyn CrlStore>,
        key_store: Box<dyn KeyStore>,
        identity_store: Box<dyn IdentityStore>,
        profile_store: Box<dyn ProfileStore>,
    ) -> Self {
        Self { config, cert_store, crl_store, key_store, identity_store, profile_store }
    }

    pub async fn initialize_root_ca(&self) -> CaResult<Certificate> {
        let key_pair = generate_key_pair(self.config.ca_key_type)?;
        let serial = uuid::Uuid::new_v4().to_string();
        let mut cert = Certificate {
            serial_number: serial.clone(),
            issuer: self.config.organization.clone(),
            subject: self.config.organization.clone(),
            subject_alt_names: vec![],
            validity: ValidityPeriod::new(self.config.default_validity_days * 10),
            key_type: self.config.ca_key_type,
            key_usage: vec!["keyCertSign".into(), "cRLSign".into()],
            extended_key_usage: vec![],
            status: CertStatus::Active,
            revocation_reason: None,
            revocation_date: None,
            pem: String::new(),
            fingerprint_sha256: String::new(),
            ca_type: CaType::Root,
            issued_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };
        sign_certificate(&mut cert, &key_pair)?;
        self.cert_store.store_certificate(&cert).await?;
        self.key_store.store_key(&serial, &[]).await?;
        Ok(cert)
    }

    pub async fn issue_certificate(&self, request: CertificateRequest) -> CaResult<Certificate> {
        let profile = self.profile_store.get_profile(&request.profile).await
            .unwrap_or(CertProfile {
                name: "default".into(),
                key_type: request.key_type,
                validity_days: request.validity_days,
                key_usage: vec!["digitalSignature".into()],
                extended_key_usage: vec![],
                max_path_len: None,
                allow_ca: false,
            });

        let key_pair = generate_key_pair(request.key_type)?;
        let serial = uuid::Uuid::new_v4().to_string();
        let mut cert = Certificate {
            serial_number: serial.clone(),
            issuer: self.config.organization.clone(),
            subject: request.subject,
            subject_alt_names: request.subject_alt_names,
            validity: ValidityPeriod::new(request.validity_days.min(profile.validity_days)),
            key_type: request.key_type,
            key_usage: profile.key_usage.clone(),
            extended_key_usage: profile.extended_key_usage.clone(),
            status: CertStatus::Active,
            revocation_reason: None,
            revocation_date: None,
            pem: String::new(),
            fingerprint_sha256: String::new(),
            ca_type: CaType::EndEntity,
            issued_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };
        sign_certificate(&mut cert, &key_pair)?;
        self.cert_store.store_certificate(&cert).await?;
        self.key_store.store_key(&serial, &[]).await?;
        Ok(cert)
    }

    pub async fn revoke_certificate(&self, serial: &str, reason: RevocationReason) -> CaResult<()> {
        let mut cert = self.cert_store.get_certificate(serial).await?;
        cert.status = CertStatus::Revoked;
        cert.revocation_reason = Some(reason);
        cert.revocation_date = Some(chrono::Utc::now().to_rfc3339());
        cert.updated_at = chrono::Utc::now().to_rfc3339();
        self.cert_store.revoke_certificate(serial, &format!("{:?}", reason)).await?;
        Ok(())
    }

    pub async fn check_revocation_status(&self, serial: &str) -> CaResult<CertStatus> {
        let cert = self.cert_store.get_certificate(serial).await?;
        Ok(cert.status)
    }
}
