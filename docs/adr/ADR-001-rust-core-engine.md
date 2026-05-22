# ADR-001: Rust Core CA Engine

## Status
Accepted

## Context
The Certificate Authority needs a high-performance, memory-safe core engine for cryptographic operations, certificate management, and storage.

## Decision
Implement the core CA engine in Rust, leveraging:
- Memory safety without garbage collection
- Strong crypto ecosystem (RustCrypto, x509-parser, ed25519-dalek)
- Zero-cost abstractions for high-throughput CA operations
- Native PQC support via liboqs Rust bindings
- Async runtime (Tokio) for concurrent certificate operations
- Tonic for gRPC inter-service communication

## Consequences
- Higher initial development investment vs Python/Go
- Best-in-class crypto safety guarantees
- Easy cross-compilation for edge/space-hardened deployments
- Direct integration with Apache Arrow ecosystem via official Rust bindings
