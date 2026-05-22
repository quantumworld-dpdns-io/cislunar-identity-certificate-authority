package main

import (
	"log"

	"github.com/gin-gonic/gin"
	"github.com/rs/zerolog"
	"os"

	"github.com/quantumworld-dpdns-io/cislunar-identity-certificate-authority/go/internal/api"
	"github.com/quantumworld-dpdns-io/cislunar-identity-certificate-authority/go/internal/config"
)

func main() {
	logger := zerolog.New(os.Stderr).With().Timestamp().Logger()
	cfg := config.Load()

	gin.SetMode(gin.ReleaseMode)
	router := gin.New()
	router.Use(gin.Recovery())
	router.Use(api.SecurityHeadersMiddleware())
	router.Use(api.CORSMiddleware())

	handler := api.NewHandler()
	handler.RegisterRoutes(router)

	addr := cfg.Server.Addr
	logger.Info().Str("addr", addr).Msg("Starting Cislunar CA API server")
	if err := router.Run(addr); err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}
