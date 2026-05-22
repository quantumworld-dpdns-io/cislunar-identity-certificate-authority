use tonic::{transport::Server, Request, Response, Status};
use ca_core::*;
use ca_types::*;

pub mod ca_service {
    tonic::include_proto!("ca");
}

use ca_service::{
    ca_server::{Ca, CaServer},
    IssueRequest, IssueResponse, RevokeRequest, RevokeResponse,
    ListRequest, ListResponse, GetRequest, GetResponse,
};

#[derive(Debug)]
pub struct CaServiceImpl {
    ca: CertificateAuthority,
}

#[tonic::async_trait]
impl Ca for CaServiceImpl {
    async fn issue_certificate(&self, req: Request<IssueRequest>) -> Result<Response<IssueResponse>, Status> {
        let request = req.into_inner();
        let cert_req = CertificateRequest {
            subject: request.subject,
            subject_alt_names: request.subject_alt_names,
            key_type: KeyType::EcdsaP256,
            validity_days: request.validity_days,
            profile: request.profile,
            extensions: vec![],
        };
        match self.ca.issue_certificate(cert_req).await {
            Ok(cert) => Ok(Response::new(IssueResponse {
                serial_number: cert.serial_number,
                subject: cert.subject,
                fingerprint: cert.fingerprint_sha256,
                issued_at: cert.issued_at,
                expires_at: cert.validity.not_after,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn revoke_certificate(&self, req: Request<RevokeRequest>) -> Result<Response<RevokeResponse>, Status> {
        let request = req.into_inner();
        match self.ca.revoke_certificate(&request.serial_number, RevocationReason::Unspecified).await {
            Ok(_) => Ok(Response::new(RevokeResponse { success: true })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn list_certificates(&self, _req: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        match self.ca.cert_store.list_certificates(&CertFilter::default()).await {
            Ok(certs) => {
                let items = certs.into_iter().map(|c| ca_service::CertificateSummary {
                    serial_number: c.serial_number,
                    subject: c.subject,
                    status: format!("{:?}", c.status),
                    expires_at: c.validity.not_after,
                }).collect();
                Ok(Response::new(ListResponse { certificates: items }))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_certificate(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let serial = req.into_inner().serial_number;
        match self.ca.cert_store.get_certificate(&serial).await {
            Ok(cert) => Ok(Response::new(GetResponse {
                serial_number: cert.serial_number,
                subject: cert.subject,
                issuer: cert.issuer,
                status: format!("{:?}", cert.status),
                pem: cert.pem,
                fingerprint: cert.fingerprint_sha256,
                issued_at: cert.issued_at,
                expires_at: cert.validity.not_after,
                ca_type: format!("{:?}", cert.ca_type),
            })),
            Err(_) => Err(Status::not_found(serial)),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "[::1]:50051".parse()?;
    let config = CaConfig {
        organization: "Cislunar CA".into(),
        country: "US".into(),
        ca_key_type: KeyType::EcdsaP256,
        default_validity_days: 365,
        crl_validity_days: 7,
        enable_pqc: false,
        enable_tee: false,
        max_certificates_per_identity: 100,
    };
    let ca = CertificateAuthority::new(config);
    let service = CaServiceImpl { ca };
    Server::builder()
        .add_service(CaServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
