# ADR-003: Python AI/ML Integration Layer

## Status
Accepted

## Context
The CA needs AI-powered policy validation, natural language interfaces, vector search for identities, and analytics capabilities.

## Decision
Implement the AI/ML integration layer in Python, leveraging:
- Ollama SDK for local LLM policy validation
- ChromaDB for identity vector storage and semantic search
- DuckDB for local analytics and reporting
- W&B Weave for LLM observability and evaluation
- MCP SDK for agent protocol integrations
- Rich ecosystem for data science and ML workflows

## Consequences
- Fastest path to AI/ML feature integration
- Access to best-in-class data science tooling
- Natural fit for Robot Framework test infrastructure
- Performance-critical paths delegated to Rust core
