#!/bin/bash
set -euo pipefail

echo "=== Running all CI checks ==="

echo "--- Rust ---"
cd src/rust
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo audit
cd ../..

echo "--- Go ---"
cd src/go
go vet ./...
go test -v -race -count=1 ./...
gosec ./...
cd ../..

echo "--- Python ---"
cd src/python
ruff check .
mypy cislunar_ca
pytest -v --cov=cislunar_ca
bandit -r cislunar_ca
cd ../..

echo "--- Robot Framework ---"
robot --variable CA_API:http://localhost:8080 tests/security/robot/tests/

echo "--- Docker ---"
docker compose build

echo "=== All checks passed ==="
