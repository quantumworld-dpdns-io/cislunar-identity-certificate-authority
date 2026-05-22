use serde::{Deserialize, Serialize};
use ca_types::CaResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub event_id: String,
    pub timestamp: String,
    pub event_type: AuditEventType,
    pub actor: String,
    pub action: String,
    pub resource: String,
    pub result: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditEventType {
    CertificateIssued,
    CertificateRevoked,
    CertificateExpired,
    IdentityRegistered,
    IdentitySuspended,
    IdentityRevoked,
    KeyGenerated,
    KeyExported,
    PolicyChanged,
    ConfigurationChanged,
    AccessDenied,
    AuthenticationSuccess,
    AuthenticationFailure,
}

pub trait AuditLogger: Send + Sync {
    fn log_event(&self, event: AuditEvent) -> CaResult<()>;
    fn query_events(&self, filter: &AuditFilter) -> CaResult<Vec<AuditEvent>>;
}

pub struct ConsoleAuditLogger;

impl AuditLogger for ConsoleAuditLogger {
    fn log_event(&self, event: AuditEvent) -> CaResult<()> {
        println!("[AUDIT] {:?} | {} | {} | {} | {}",
            event.event_type, event.actor, event.action, event.resource, event.result);
        Ok(())
    }

    fn query_events(&self, _filter: &AuditFilter) -> CaResult<Vec<AuditEvent>> {
        Ok(vec![])
    }
}

#[derive(Debug, Default)]
pub struct AuditFilter {
    pub event_type: Option<AuditEventType>,
    pub actor: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub limit: Option<u32>,
}
