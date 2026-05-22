use serde::{Deserialize, Serialize};
use crate::crypto::KeyType;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidityPeriod {
    pub not_before: String,
    pub not_after: String,
}

impl ValidityPeriod {
    pub fn new(days_valid: u32) -> Self {
        let now = chrono::Utc::now();
        let not_before = now.to_rfc3339();
        let not_after = (now + chrono::Duration::days(days_valid as i64)).to_rfc3339();
        Self { not_before, not_after }
    }

    pub fn is_valid(&self) -> bool {
        let now = chrono::Utc::now();
        let nb = chrono::DateTime::parse_from_rfc3339(&self.not_before)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or(chrono::Utc::now() - chrono::Duration::hours(1));
        let na = chrono::DateTime::parse_from_rfc3339(&self.not_after)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or(chrono::Utc::now() + chrono::Duration::hours(1));
        now >= nb && now <= na
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertStatus {
    Active,
    Revoked,
    Expired,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RevocationReason {
    Unspecified,
    KeyCompromise,
    CaCompromise,
    AffiliationChanged,
    Superseded,
    CessationOfOperation,
    CertificateHold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateRequest {
    pub subject: String,
    pub subject_alt_names: Vec<String>,
    pub key_type: KeyType,
    pub validity_days: u32,
    pub profile: String,
    pub extensions: Vec<CertExtension>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertExtension {
    pub oid: String,
    pub critical: bool,
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub serial_number: String,
    pub issuer: String,
    pub subject: String,
    pub subject_alt_names: Vec<String>,
    pub validity: ValidityPeriod,
    pub key_type: KeyType,
    pub key_usage: Vec<String>,
    pub extended_key_usage: Vec<String>,
    pub status: CertStatus,
    pub revocation_reason: Option<RevocationReason>,
    pub revocation_date: Option<String>,
    pub pem: String,
    pub fingerprint_sha256: String,
    pub ca_type: CaType,
    pub issued_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CaType {
    Root,
    Intermediate,
    EndEntity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrlEntry {
    pub serial_number: String,
    pub revocation_date: String,
    pub reason: RevocationReason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateRevocationList {
    pub issuer: String,
    pub this_update: String,
    pub next_update: String,
    pub entries: Vec<CrlEntry>,
    pub crl_number: u64,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertProfile {
    pub name: String,
    pub key_type: KeyType,
    pub validity_days: u32,
    pub key_usage: Vec<String>,
    pub extended_key_usage: Vec<String>,
    pub max_path_len: Option<u32>,
    pub allow_ca: bool,
}
