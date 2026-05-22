FROM golang:1.22-bookworm AS builder
WORKDIR /app
COPY src/go .
RUN go build -o /go/bin/ca-api ./cmd/api

FROM gcr.io/distroless/base-debian12:latest
COPY --from=builder /go/bin/ca-api /usr/local/bin/ca-api
EXPOSE 8080 9090
ENTRYPOINT ["/usr/local/bin/ca-api"]
