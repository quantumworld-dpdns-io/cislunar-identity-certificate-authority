FROM rust:1.79-slim-bookworm AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev protobuf-compiler cmake
WORKDIR /app
COPY src/rust .
RUN cargo build --release --bin ca-grpc

FROM gcr.io/distroless/cc-debian12:latest
COPY --from=builder /app/target/release/ca-grpc /usr/local/bin/ca-grpc
COPY --from=builder /app/configs/rust /etc/cislunar-ca/
EXPOSE 50051
ENTRYPOINT ["/usr/local/bin/ca-grpc"]
