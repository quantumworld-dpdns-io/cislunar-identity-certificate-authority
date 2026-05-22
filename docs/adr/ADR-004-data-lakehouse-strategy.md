# ADR-004: Data Lakehouse Strategy

## Status
Accepted

## Context
The CA generates structured certificate data, audit logs, identity records, and operational metrics that require analytical querying, reporting, and long-term storage.

## Decision
Adopt a layered data strategy:
- DuckDB for local embedded analytics on certificate data
- Apache Arrow for zero-copy data interchange between language runtimes
- Apache DataFusion for programmatic query federation
- Trino for cross-source analytical queries (certificates + audit + identities)
- Apache Iceberg for long-term audit table storage with time-travel

## Consequences
- Eliminates need for standalone analytical database in early phases
- Natural upgrade path to distributed query engine as data grows
- Standardized data interchange via Arrow across Rust/Go/Python
- Time-travel capability for compliance and forensic analysis
