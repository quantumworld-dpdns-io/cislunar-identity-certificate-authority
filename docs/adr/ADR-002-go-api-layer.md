# ADR-002: Go API Service Layer

## Status
Accepted

## Context
The CA requires REST API, CLI tooling, and web dashboard services that interface with the core Rust engine.

## Decision
Implement the API service layer in Go, leveraging:
- Gin for high-performance REST API routing
- Cobra for CLI tool scaffolding
- Strong standard library for HTTP, TLS, crypto
- goroutines for lightweight concurrency in request handling
- gRPC client for communication with Rust CA engine
- Excellent Kubernetes/cloud-native ecosystem integration

## Consequences
- Fast compilation and deployment cycle
- Rich middleware ecosystem (auth, rate-limiting, observability)
- Excellent horizontal scaling characteristics
- Natural fit for Kubernetes/cloud-native deployment
