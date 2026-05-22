# cislunar-identity-certificate-authority

> Cislunar identity and certificate authority – issues hardware-backed Verifiable Credentials for lunar relays, landers, and rovers

## Overview

Multi-language Certificate Authority for the Earth-Moon cisunar space domain. Issues X.509 certificates and W3C Verifiable Credentials with hardware-backed attestation, Post-Quantum Cryptography (PQC), and Trusted Execution Environment (TEE) integration.

**Languages:** Rust (core engine) · Go (API/CLI) · Python (AI/ML integration)  
**Testing:** Robot Framework (OWASP Top 10) · Language-native unit tests  
**Infrastructure:** Docker · Helm · Terraform · GitHub Actions

## Architecture

```
┌──────────────────┬──────────────────┬──────────────────────┐
│   Rust Core      │   Go Services    │   Python AI/ML       │
│   (CA Engine)    │   (API/CLI)      │   (Integration)      │
├──────────────────┼──────────────────┼──────────────────────┤
│ • Cert issuance  │ • REST API       │ • Ollama policy      │
│ • Key mgmt       │ • gRPC API       │ • Chroma vectors     │
│ • CRL/OCSP       │ • CLI tool       │ • DuckDB analytics   │
│ • PQC crypto     │ • Web UI         │ • W&B Weave tracing  │
│ • TEE integration│ • RBAC/Auth      │ • MCP server         │
│ • HSM support    │ • Audit logging  │ • Agent Skills       │
└──────────────────┴──────────────────┴──────────────────────┘
         │                  │                      │
         └──────────────────┼──────────────────────┘
                            │
              ┌─────────────┴─────────────┐
              │   Data & Storage Layer    │
              │ DuckDB | Arrow | Iceberg  │
              │ Chroma | Trino | Parquet  │
              └───────────────────────────┘
                            │
              ┌─────────────┴─────────────┐
              │   Infrastructure Layer     │
              │ Docker | Helm | Terraform  │
              │ Prometheus | Grafana       │
              └───────────────────────────┘
```

## Project Structure

```
├── src/
│   ├── rust/           # Core CA engine (ca-types, ca-crypto, ca-core, ca-cli, ca-grpc)
│   ├── go/             # API services (cmd/api, cmd/cli, internal/)
│   └── python/         # AI/ML integration (agents, analytics, vector, mcp, observability)
├── tests/
│   ├── unit/           # Language-native unit tests
│   ├── integration/    # Cross-service integration tests
│   ├── e2e/            # End-to-end tests
│   └── security/       # Robot Framework OWASP Top 10 tests
├── docker/             # Dockerfiles per language
├── helm/               # Kubernetes Helm charts
├── terraform/          # Infrastructure as Code (EKS, RDS, HSM)
├── .github/workflows/  # CI/CD pipelines (10 workflows)
├── skills/             # Agent Skills for AI-assisted CA management
├── docs/adr/           # Architecture Decision Records
├── proto/              # Protocol Buffer definitions
└── configs/            # Service configurations
```

## Quick Start

```bash
# Setup development environment
./scripts/dev/setup.sh

# Build all services
make build

# Run tests
make test

# Start full stack
docker compose up

# Run OWASP security tests
make security-test
```

## Integrated Technologies

| Category | Tools | Status |
|----------|-------|--------|
| **Local AI** | Ollama, llama.cpp, vLLM, SGLang, LM Studio | Policy validation, NL queries |
| **Vector DB** | Chroma, LanceDB, Milvus, Qdrant, Weaviate | Identity vector store |
| **Data Lakehouse** | DuckDB, Arrow, DataFusion, Trino, Iceberg | Analytics, auditing |
| **Agent Protocols** | MCP, Agent Skills, DXT/MCPB | AI agent integration |
| **Observability** | W&B Weave, OpenTelemetry | LLM tracing, metrics |
| **Cloud Security** | PQC (liboqs), Teaclave (TEE), WASM/WASI | Post-quantum crypto |

## CI/CD Pipelines

| Workflow | Trigger | Scope |
|----------|---------|-------|
| Rust CI | push/PR to `src/rust/` | Build, lint, test, audit |
| Go CI | push/PR to `src/go/` | Build, lint, test, vet |
| Python CI | push/PR to `src/python/` | Lint, typecheck, test, security |
| Security Scan | scheduled/PR | SAST, DAST, SCA, secrets |
| Robot Framework | scheduled/PR | OWASP Top 10 tests |
| Docker Build | push main/tags | Multi-arch container images |
| Release | tags | Changelog, release artifacts |
| CodeQL | push/PR/scheduled | Cross-language analysis |
| Integration | push/PR | Cross-service integration |
| Dependency Review | PR | Dependency vulnerability check |

## OWASP Top 10 Coverage

| Category | Tests | File |
|----------|-------|------|
| A01 Broken Access Control | 4 tests | `A01-broken-access-control.robot` |
| A02 Cryptographic Failures | 5 tests | `A02-cryptographic-failures.robot` |
| A03 Injection | 4 tests | `A03-injection.robot` |
| A04 Insecure Design | 4 tests | `A04-insecure-design.robot` |
| A05 Security Misconfiguration | 5 tests | `A05-security-misconfiguration.robot` |
| A06 Vulnerable Components | 4 tests | `A06-vulnerable-components.robot` |
| A07 Authentication Failures | 5 tests | `A07-auth-failures.robot` |
| A08 Integrity Failures | 4 tests | `A08-integrity-failures.robot` |
| A09 Logging Failures | 4 tests | `A09-logging-failures.robot` |
| A10 SSRF | 3 tests | `A10-ssrf.robot` |

## Contributing

Please read [CONTRIBUTING.md](docs/CONTRIBUTING.md) before opening a pull request.

## License

[MIT](LICENSE)
