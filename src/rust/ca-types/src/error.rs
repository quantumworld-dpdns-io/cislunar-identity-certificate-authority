use thiserror::Error;

#[derive(Error, Debug)]
pub enum CaError {
    #[error("Invalid key type: {0}")]
    InvalidKeyType(String),

    #[error("Certificate error: {0}")]
    CertificateError(String),

    #[error("Revocation error: {0}")]
    RevocationError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Crypto error: {0}")]
    CryptoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type CaResult<T> = Result<T, CaError>;
