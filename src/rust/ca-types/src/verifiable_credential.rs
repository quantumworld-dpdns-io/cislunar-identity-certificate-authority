use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    pub context: Vec<String>,
    pub id: String,
    pub credential_type: Vec<String>,
    pub issuer: String,
    pub issuance_date: String,
    pub expiration_date: Option<String>,
    pub credential_subject: CredentialSubject,
    pub proof: Option<CredentialProof>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialSubject {
    pub id: String,
    pub asset_type: String,
    pub organization: String,
    pub public_key: String,
    pub attributes: Vec<CredentialAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialAttribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialProof {
    pub proof_type: String,
    pub created: String,
    pub verification_method: String,
    pub proof_value: String,
}
