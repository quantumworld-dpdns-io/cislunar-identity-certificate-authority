# Cislunar Identity Certificate Authority - Architecture

## System Context

```
┌──────────────────────────────────────────────────────────────┐
│                    Cislunar Identity CA                        │
├──────────────────┬──────────────────┬────────────────────────┤
│   Rust Core      │   Go Services    │   Python AI/ML         │
│   (CA Engine)    │   (API/CLI)      │   (Integration)        │
├──────────────────┼──────────────────┼────────────────────────┤
│ • Cert issuance  │ • REST API       │ • Ollama policy agent  │
│ • Key mgmt       │ • gRPC API       │ • Chroma vector store  │
│ • CRL/OCSP       │ • CLI tool       │ • DuckDB analytics     │
│ • PQC crypto     │ • Web UI         │ • W&B Weave tracing    │
│ • TEE integration│ • RBAC/Auth      │ • MCP server           │
│ • HSM support    │ • Audit logging  │ • Agent Skills         │
└──────────────────┴──────────────────┴────────────────────────┘
         │                  │                      │
         └──────────────────┼──────────────────────┘
                            │
              ┌─────────────┴─────────────┐
              │    Data & Storage Layer    │
              │  DuckDB | Arrow | Iceberg  │
              │  Chroma | Trino | Parquet  │
              └───────────────────────────┘
                            │
              ┌─────────────┴─────────────┐
              │   Infrastructure Layer     │
              │ Docker | Helm | Terraform  │
              │ Prometheus | Grafana       │
              └───────────────────────────┘
```

## Language Responsibilities

| Layer | Language | Role |
|-------|----------|------|
| Core Engine | Rust | Certificate operations, crypto, PQC, TEE |
| Service Layer | Go | REST API, CLI, Web UI, auth, audit |
| AI Integration | Python | LLM policy, vector search, analytics, MCP |
| Testing | Python/Robot | Unit, integration, OWASP Top 10 |

## Communication Patterns

- Rust ↔ Go: gRPC (protobuf)
- Rust ↔ Python: Arrow Flight RPC
- Go ↔ Python: REST/HTTP
- All ↔ Storage: Language-native clients

## Security Zones

1. **Red Zone** (Rust): Key material, signing operations, PQC secrets
2. **Yellow Zone** (Go): API tokens, session data, audit logs
3. **Green Zone** (Python): AI models, vector embeddings, analytics
