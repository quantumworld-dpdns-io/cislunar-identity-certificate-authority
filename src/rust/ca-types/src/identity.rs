use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub id: String,
    pub name: String,
    pub organization: String,
    pub asset_type: SpaceAssetType,
    pub public_key_fingerprint: String,
    pub hardware_attestation: Option<HardwareAttestation>,
    pub attributes: Vec<IdentityAttribute>,
    pub status: IdentityStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpaceAssetType {
    LunarRelay,
    Lander,
    Rover,
    Orbiter,
    GroundStation,
    Gateway,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareAttestation {
    pub attestation_type: AttestationType,
    pub hardware_id: String,
    pub manufacturer: String,
    pub attestation_data: Vec<u8>,
    pub verified_at: Option<String>,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AttestationType {
    Tpm2_0,
    SecureElement,
    AppleSecureEnclave,
    AndroidStrongBox,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAttribute {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IdentityStatus {
    Active,
    Suspended,
    Revoked,
}
