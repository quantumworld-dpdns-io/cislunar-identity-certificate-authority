package config

import (
	"os"
	"strconv"
	"time"
)

type Config struct {
	Server   ServerConfig
	CA       CAConfig
	Auth     AuthConfig
	Rate     RateConfig
	Logging  LogConfig
}

type ServerConfig struct {
	Addr            string
	ReadTimeout     time.Duration
	WriteTimeout    time.Duration
	MaxRequestSize  int64
}

type CAConfig struct {
	GRPCAddr    string
	GRPCTimeout time.Duration
}

type AuthConfig struct {
	JWTSecret     string
	TokenExpiry   time.Duration
	APIKeyHeader  string
}

type RateConfig struct {
	Enabled         bool
	RequestsPerSec  int
	Burst           int
}

type LogConfig struct {
	Level  string
	Format string
}

func Load() *Config {
	return &Config{
		Server: ServerConfig{
			Addr:           getEnv("SERVER_ADDR", ":8080"),
			ReadTimeout:    getDuration("SERVER_READ_TIMEOUT", 30*time.Second),
			WriteTimeout:   getDuration("SERVER_WRITE_TIMEOUT", 30*time.Second),
			MaxRequestSize: getInt64("SERVER_MAX_REQUEST_SIZE", 10<<20),
		},
		CA: CAConfig{
			GRPCAddr:    getEnv("CA_GRPC_ADDR", "localhost:50051"),
			GRPCTimeout: getDuration("CA_GRPC_TIMEOUT", 10*time.Second),
		},
		Auth: AuthConfig{
			JWTSecret:    getEnv("JWT_SECRET", "change-me"),
			TokenExpiry:  getDuration("AUTH_TOKEN_EXPIRY", 24*time.Hour),
			APIKeyHeader: getEnv("AUTH_API_KEY_HEADER", "X-API-Key"),
		},
		Rate: RateConfig{
			Enabled:        getBool("RATE_LIMIT_ENABLED", true),
			RequestsPerSec: getInt("RATE_LIMIT_RPS", 100),
			Burst:          getInt("RATE_LIMIT_BURST", 200),
		},
		Logging: LogConfig{
			Level:  getEnv("LOG_LEVEL", "info"),
			Format: getEnv("LOG_FORMAT", "json"),
		},
	}
}

func getEnv(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}

func getInt(key string, fallback int) int {
	if v := os.Getenv(key); v != "" {
		if i, err := strconv.Atoi(v); err == nil {
			return i
		}
	}
	return fallback
}

func getInt64(key string, fallback int64) int64 {
	if v := os.Getenv(key); v != "" {
		if i, err := strconv.ParseInt(v, 10, 64); err == nil {
			return i
		}
	}
	return fallback
}

func getDuration(key string, fallback time.Duration) time.Duration {
	if v := os.Getenv(key); v != "" {
		if d, err := time.ParseDuration(v); err == nil {
			return d
		}
	}
	return fallback
}

func getBool(key string, fallback bool) bool {
	if v := os.Getenv(key); v != "" {
		if b, err := strconv.ParseBool(v); err == nil {
			return b
		}
	}
	return fallback
}
