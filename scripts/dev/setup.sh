#!/bin/bash
set -euo pipefail

echo "=== Cislunar CA Development Setup ==="

# Check prerequisites
command -v rustc >/dev/null 2>&1 || { echo "Rust not installed. Install from https://rustup.rs"; exit 1; }
command -v go >/dev/null 2>&1 || { echo "Go not installed. Install from https://go.dev"; exit 1; }
command -v python3 >/dev/null 2>&1 || { echo "Python 3 not installed."; exit 1; }
command -v docker >/dev/null 2>&1 || { echo "Docker not installed."; exit 1; }

echo "[1/5] Setting up Rust workspace..."
cd src/rust && cargo check && cd ../..

echo "[2/5] Setting up Go module..."
cd src/go && go mod tidy && cd ../..

echo "[3/5] Setting up Python environment..."
python3 -m venv .venv
source .venv/bin/activate
pip install -e src/python

echo "[4/5] Setting up pre-commit hooks..."
pip install pre-commit
pre-commit install

echo "[5/5] Creating .env from template..."
cp -n .env.example .env 2>/dev/null || echo ".env already exists"

echo "=== Setup complete! ==="
echo "Run 'make rust-build' to build Rust, 'make go-build' for Go"
echo "Run 'docker compose up' to start all services"
