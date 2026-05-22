.PHONY: all rust go python build test lint clean docker proto

all: build

## Rust targets
rust-build:
	cd src/rust && cargo build --workspace

rust-test:
	cd src/rust && cargo test --workspace

rust-lint:
	cd src/rust && cargo clippy --workspace -- -D warnings

rust-fmt:
	cd src/rust && cargo fmt

rust-clean:
	cd src/rust && cargo clean

## Go targets
go-build:
	cd src/go && go build ./...

go-test:
	cd src/go && go test -v -race -count=1 ./...

go-lint:
	cd src/go && golangci-lint run

go-vet:
	cd src/go && go vet ./...

go-clean:
	cd src/go && go clean

## Python targets
python-install:
	cd src/python && pip install -e ".[dev]"

python-test:
	cd src/python && pytest -v --cov=cislunar_ca

python-lint:
	cd src/python && ruff check .

python-typecheck:
	cd src/python && mypy cislunar_ca

python-clean:
	cd src/python && rm -rf build dist *.egg-info

## Combined targets
build: rust-build go-build

test: rust-test go-test python-test

lint: rust-lint go-lint python-lint

clean: rust-clean go-clean python-clean

## Proto
proto:
	cd proto && protoc --rust_out ../src/rust/ca-grpc/src --tonic_out ../src/rust/ca-grpc/src ca.proto

## Docker
docker-build:
	docker compose build

docker-up:
	docker compose up -d

docker-down:
	docker compose down

## Robot Framework
robot-test:
	robot --variable CA_API:http://localhost:8080 tests/security/robot/tests/

security-test: robot-test

## Pre-commit
pre-commit:
	pre-commit run --all-files
