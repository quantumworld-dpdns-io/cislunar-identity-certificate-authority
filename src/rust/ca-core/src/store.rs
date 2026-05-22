use std::collections::HashMap;
use async_trait::async_trait;
use ca_types::*;

pub struct InMemoryCertStore {
    certs: tokio::sync::RwLock<HashMap<String, Certificate>>,
}

impl InMemoryCertStore {
    pub fn new() -> Self {
        Self { certs: tokio::sync::RwLock::new(HashMap::new()) }
    }
}

#[async_trait]
impl CertificateStore for InMemoryCertStore {
    async fn store_certificate(&self, cert: &Certificate) -> CaResult<()> {
        let mut store = self.certs.write().await;
        store.insert(cert.serial_number.clone(), cert.clone());
        Ok(())
    }

    async fn get_certificate(&self, serial: &str) -> CaResult<Certificate> {
        let store = self.certs.read().await;
        store.get(serial).cloned().ok_or_else(|| CaError::NotFound(serial.into()))
    }

    async fn list_certificates(&self, _filter: &CertFilter) -> CaResult<Vec<Certificate>> {
        let store = self.certs.read().await;
        Ok(store.values().cloned().collect())
    }

    async fn revoke_certificate(&self, serial: &str, _reason: &str) -> CaResult<()> {
        let mut store = self.certs.write().await;
        if let Some(cert) = store.get_mut(serial) {
            cert.status = CertStatus::Revoked;
            Ok(())
        } else {
            Err(CaError::NotFound(serial.into()))
        }
    }

    async fn get_certificate_count(&self) -> CaResult<u64> {
        let store = self.certs.read().await;
        Ok(store.len() as u64)
    }

    async fn get_active_certificates(&self) -> CaResult<Vec<Certificate>> {
        let store = self.certs.read().await;
        Ok(store.values().filter(|c| c.status == CertStatus::Active).cloned().collect())
    }

    async fn get_expired_certificates(&self) -> CaResult<Vec<Certificate>> {
        let store = self.certs.read().await;
        let now = chrono::Utc::now();
        Ok(store.values()
            .filter(|c| {
                chrono::DateTime::parse_from_rfc3339(&c.validity.not_after)
                    .map(|d| d.with_timezone(&chrono::Utc) < now)
                    .unwrap_or(false)
            })
            .cloned().collect())
    }

    async fn search_subject(&self, subject: &str) -> CaResult<Vec<Certificate>> {
        let store = self.certs.read().await;
        Ok(store.values()
            .filter(|c| c.subject.contains(subject))
            .cloned().collect())
    }
}

pub struct InMemoryCrlStore {
    crls: tokio::sync::RwLock<Vec<CertificateRevocationList>>,
}

impl InMemoryCrlStore {
    pub fn new() -> Self {
        Self { crls: tokio::sync::RwLock::new(Vec::new()) }
    }
}

#[async_trait]
impl CrlStore for InMemoryCrlStore {
    async fn store_crl(&self, crl: &CertificateRevocationList) -> CaResult<()> {
        let mut store = self.crls.write().await;
        store.push(crl.clone());
        Ok(())
    }

    async fn get_latest_crl(&self) -> CaResult<CertificateRevocationList> {
        let store = self.crls.read().await;
        store.last().cloned().ok_or_else(|| CaError::NotFound("No CRL available".into()))
    }

    async fn is_revoked(&self, serial: &str) -> CaResult<bool> {
        let store = self.crls.read().await;
        Ok(store.iter().any(|crl| crl.entries.iter().any(|e| e.serial_number == serial)))
    }

    async fn get_crl_history(&self, _limit: u32) -> CaResult<Vec<CertificateRevocationList>> {
        let store = self.crls.read().await;
        Ok(store.clone())
    }
}

pub struct InMemoryKeyStore {
    keys: tokio::sync::RwLock<HashMap<String, Vec<u8>>>,
}

impl InMemoryKeyStore {
    pub fn new() -> Self {
        Self { keys: tokio::sync::RwLock::new(HashMap::new()) }
    }
}

#[async_trait]
impl KeyStore for InMemoryKeyStore {
    async fn store_key(&self, serial: &str, key_data: &[u8]) -> CaResult<()> {
        let mut store = self.keys.write().await;
        store.insert(serial.into(), key_data.to_vec());
        Ok(())
    }

    async fn get_key(&self, serial: &str) -> CaResult<Vec<u8>> {
        let store = self.keys.read().await;
        store.get(serial).cloned().ok_or_else(|| CaError::NotFound(serial.into()))
    }

    async fn delete_key(&self, serial: &str) -> CaResult<()> {
        let mut store = self.keys.write().await;
        store.remove(serial);
        Ok(())
    }

    async fn key_exists(&self, serial: &str) -> CaResult<bool> {
        let store = self.keys.read().await;
        Ok(store.contains_key(serial))
    }
}

pub struct InMemoryIdentityStore {
    identities: tokio::sync::RwLock<HashMap<String, Identity>>,
}

impl InMemoryIdentityStore {
    pub fn new() -> Self {
        Self { identities: tokio::sync::RwLock::new(HashMap::new()) }
    }
}

#[async_trait]
impl IdentityStore for InMemoryIdentityStore {
    async fn store_identity(&self, identity: &Identity) -> CaResult<()> {
        let mut store = self.identities.write().await;
        store.insert(identity.id.clone(), identity.clone());
        Ok(())
    }

    async fn get_identity(&self, id: &str) -> CaResult<Identity> {
        let store = self.identities.read().await;
        store.get(id).cloned().ok_or_else(|| CaError::NotFound(id.into()))
    }

    async fn list_identities(&self, _filter: &IdentityFilter) -> CaResult<Vec<Identity>> {
        let store = self.identities.read().await;
        Ok(store.values().cloned().collect())
    }

    async fn search_by_attribute(&self, _key: &str, _value: &str) -> CaResult<Vec<Identity>> {
        let store = self.identities.read().await;
        Ok(store.values().cloned().collect())
    }
}

pub struct InMemoryProfileStore {
    profiles: tokio::sync::RwLock<HashMap<String, CertProfile>>,
}

impl InMemoryProfileStore {
    pub fn new() -> Self {
        let mut profiles = HashMap::new();
        profiles.insert("default".into(), CertProfile {
            name: "default".into(),
            key_type: KeyType::EcdsaP256,
            validity_days: 365,
            key_usage: vec!["digitalSignature".into()],
            extended_key_usage: vec!["clientAuth".into()],
            max_path_len: None,
            allow_ca: false,
        });
        profiles.insert("server".into(), CertProfile {
            name: "server".into(),
            key_type: KeyType::EcdsaP256,
            validity_days: 365,
            key_usage: vec!["digitalSignature".into(), "keyEncipherment".into()],
            extended_key_usage: vec!["serverAuth".into()],
            max_path_len: None,
            allow_ca: false,
        });
        profiles.insert("ca".into(), CertProfile {
            name: "ca".into(),
            key_type: KeyType::EcdsaP384,
            validity_days: 1825,
            key_usage: vec!["keyCertSign".into(), "cRLSign".into()],
            extended_key_usage: vec![],
            max_path_len: Some(2),
            allow_ca: true,
        });
        profiles.insert("pqc".into(), CertProfile {
            name: "pqc".into(),
            key_type: KeyType::PqcDilithium3,
            validity_days: 365,
            key_usage: vec!["digitalSignature".into()],
            extended_key_usage: vec!["clientAuth".into()],
            max_path_len: None,
            allow_ca: false,
        });
        Self { profiles: tokio::sync::RwLock::new(profiles) }
    }
}

#[async_trait]
impl ProfileStore for InMemoryProfileStore {
    async fn store_profile(&self, profile: &CertProfile) -> CaResult<()> {
        let mut store = self.profiles.write().await;
        store.insert(profile.name.clone(), profile.clone());
        Ok(())
    }

    async fn get_profile(&self, name: &str) -> CaResult<CertProfile> {
        let store = self.profiles.read().await;
        store.get(name).cloned().ok_or_else(|| CaError::NotFound(name.into()))
    }

    async fn list_profiles(&self) -> CaResult<Vec<CertProfile>> {
        let store = self.profiles.read().await;
        Ok(store.values().cloned().collect())
    }
}
