use clap::{Parser, Subcommand};
use ca_core::*;
use ca_types::*;

#[derive(Parser)]
#[command(name = "ca-cli", about = "Cislunar Identity Certificate Authority CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(long, default_value = "ecdsa-p256")]
        key_type: String,
    },
    Issue {
        #[arg(long)]
        subject: String,
        #[arg(long, default_value = "default")]
        profile: String,
        #[arg(long, default_value_t = 365)]
        validity_days: u32,
    },
    Revoke {
        #[arg(long)]
        serial: String,
        #[arg(long, default_value = "unspecified")]
        reason: String,
    },
    List,
    Get {
        #[arg(long)]
        serial: String,
    },
    Crl,
    Ocsp {
        #[arg(long)]
        serial: String,
    },
    Verify {
        #[arg(long)]
        serial: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
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

    match &cli.command {
        Commands::Init { key_type } => {
            let kt = match key_type.as_str() {
                "rsa-2048" => KeyType::Rsa2048,
                "rsa-4096" => KeyType::Rsa4096,
                "ecdsa-p256" => KeyType::EcdsaP256,
                "ecdsa-p384" => KeyType::EcdsaP384,
                "ed25519" => KeyType::Ed25519,
                "pqc-dilithium3" => KeyType::PqcDilithium3,
                _ => KeyType::EcdsaP256,
            };
            let config = CaConfig {
                ca_key_type: kt,
                ..config
            };
            let ca = CertificateAuthority::new(config);
            match ca.initialize_root_ca().await {
                Ok(cert) => println!("Root CA initialized: {}", cert.serial_number),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Issue { subject, profile, validity_days } => {
            let req = CertificateRequest {
                subject: subject.clone(),
                subject_alt_names: vec![],
                key_type: KeyType::EcdsaP256,
                validity_days: *validity_days,
                profile: profile.clone(),
                extensions: vec![],
            };
            match ca.issue_certificate(req).await {
                Ok(cert) => println!("Certificate issued: {} (serial: {})", cert.subject, cert.serial_number),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Revoke { serial, reason } => {
            let r = match reason.as_str() {
                "key-compromise" => RevocationReason::KeyCompromise,
                "ca-compromise" => RevocationReason::CaCompromise,
                "superseded" => RevocationReason::Superseded,
                "cessation" => RevocationReason::CessationOfOperation,
                _ => RevocationReason::Unspecified,
            };
            match ca.revoke_certificate(serial, r).await {
                Ok(_) => println!("Certificate {} revoked", serial),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::List => {
            match ca.cert_store.list_certificates(&CertFilter::default()).await {
                Ok(certs) => {
                    for cert in &certs {
                        println!("{} | {} | {:?} | {}", 
                            cert.serial_number, cert.subject, cert.status, cert.validity.not_after);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Get { serial } => {
            match ca.cert_store.get_certificate(serial).await {
                Ok(cert) => println!("{:?}", cert),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Crl => {
            let crl_mgr = CrlManager::new(
                Box::new(store::InMemoryCertStore::new()),
                Box::new(store::InMemoryCrlStore::new()),
            );
            match crl_mgr.generate_crl("Cislunar CA").await {
                Ok(crl) => println!("CRL generated: {} entries, {}", crl.entries.len(), crl.next_update),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Ocsp { serial } => {
            let ocsp = OcspResponder::new(Box::new(store::InMemoryCertStore::new()));
            let req = OcspRequest {
                serial_number: serial.clone(),
                issuer_hash: String::new(),
            };
            match ocsp.respond(&req).await {
                Ok(resp) => println!("OCSP: {:?}", resp.status),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Verify { serial } => {
            match ca.cert_store.get_certificate(serial).await {
                Ok(cert) => {
                    let valid = CertValidator::validate_certificate_chain(&[cert]);
                    match valid {
                        Ok(true) => println!("Certificate valid"),
                        Ok(false) => println!("Certificate invalid"),
                        Err(e) => eprintln!("Validation error: {}", e),
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
    Ok(())
}
