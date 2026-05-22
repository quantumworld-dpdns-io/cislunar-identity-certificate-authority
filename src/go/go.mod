module github.com/quantumworld-dpdns-io/cislunar-identity-certificate-authority/go

go 1.22

require (
	github.com/gin-gonic/gin v1.9.1
	github.com/spf13/cobra v1.8.0
	github.com/golang-jwt/jwt/v5 v5.2.0
	github.com/prometheus/client_golang v1.19.0
	github.com/grpc-ecosystem/go-grpc-middleware v1.4.0
	github.com/rs/zerolog v1.32.0
	github.com/redis/go-redis/v9 v9.5.1
	go.opentelemetry.io/otel v1.26.0
	go.opentelemetry.io/otel/trace v1.26.0
	go.opentelemetry.io/otel/exporters/otlp/otlptrace v1.26.0
	golang.org/x/crypto v0.22.0
	google.golang.org/grpc v1.63.2
	google.golang.org/protobuf v1.34.0
)
