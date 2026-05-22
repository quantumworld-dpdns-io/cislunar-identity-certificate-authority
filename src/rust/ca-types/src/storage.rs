use async_trait::async_trait;
use crate::certificate::{Certificate, CertificateRevocationList, CertProfile};
use crate::identity::Identity;
use crate::error::CaResult;

#[async_trait]
pub trait CertificateStore: Send + Sync {
    async fn store_certificate(&self, cert: &Certificate) -> CaResult<()>;
    async fn get_certificate(&self, serial: &str) -> CaResult<Certificate>;
    async fn list_certificates(&self, filter: &CertFilter) -> CaResult<Vec<Certificate>>;
    async fn revoke_certificate(&self, serial: &str, reason: &str) -> CaResult<()>;
    async fn get_certificate_count(&self) -> CaResult<u64>;
    async fn get_active_certificates(&self) -> CaResult<Vec<Certificate>>;
    async fn get_expired_certificates(&self) -> CaResult<Vec<Certificate>>;
    async fn search_subject(&self, subject: &str) -> CaResult<Vec<Certificate>>;
}

#[async_trait]
pub trait CrlStore: Send + Sync {
    async fn store_crl(&self, crl: &CertificateRevocationList) -> CaResult<()>;
    async fn get_latest_crl(&self) -> CaResult<CertificateRevocationList>;
    async fn is_revoked(&self, serial: &str) -> CaResult<bool>;
    async fn get_crl_history(&self, limit: u32) -> CaResult<Vec<CertificateRevocationList>>;
}

#[async_trait]
pub trait KeyStore: Send + Sync {
    async fn store_key(&self, serial: &str, key_data: &[u8]) -> CaResult<()>;
    async fn get_key(&self, serial: &str) -> CaResult<Vec<u8>>;
    async fn delete_key(&self, serial: &str) -> CaResult<()>;
    async fn key_exists(&self, serial: &str) -> CaResult<bool>;
}

#[async_trait]
pub trait IdentityStore: Send + Sync {
    async fn store_identity(&self, identity: &Identity) -> CaResult<()>;
    async fn get_identity(&self, id: &str) -> CaResult<Identity>;
    async fn list_identities(&self, filter: &IdentityFilter) -> CaResult<Vec<Identity>>;
    async fn search_by_attribute(&self, key: &str, value: &str) -> CaResult<Vec<Identity>>;
}

#[async_trait]
pub trait ProfileStore: Send + Sync {
    async fn store_profile(&self, profile: &CertProfile) -> CaResult<()>;
    async fn get_profile(&self, name: &str) -> CaResult<CertProfile>;
    async fn list_profiles(&self) -> CaResult<Vec<CertProfile>>;
}

#[derive(Debug, Default, Clone)]
pub struct CertFilter {
    pub status: Option<String>,
    pub ca_type: Option<String>,
    pub key_type: Option<String>,
    pub subject: Option<String>,
    pub issuer: Option<String>,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Default, Clone)]
pub struct IdentityFilter {
    pub status: Option<String>,
    pub asset_type: Option<String>,
    pub organization: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
