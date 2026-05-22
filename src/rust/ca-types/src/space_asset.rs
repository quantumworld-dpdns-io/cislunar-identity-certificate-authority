use serde::{Deserialize, Serialize};
use crate::identity::SpaceAssetType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceAsset {
    pub id: String,
    pub name: String,
    pub asset_type: SpaceAssetType,
    pub organization: String,
    pub mission: Option<String>,
    pub coordinates: Option<SpaceCoordinates>,
    pub operational_status: OperationalStatus,
    pub certificates: Vec<String>,
    pub metadata: Vec<AssetMetadata>,
    pub registered_at: String,
    pub last_seen: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceCoordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude_km: f64,
    pub reference_frame: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationalStatus {
    Active,
    Inactive,
    Decommissioned,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub key: String,
    pub value: String,
    pub namespace: String,
}

pub trait AssetValidator {
    fn validate_identity(&self, asset: &SpaceAsset) -> Result<bool, String>;
    fn validate_certificate_binding(&self, asset: &SpaceAsset, cert_serial: &str) -> Result<bool, String>;
}
