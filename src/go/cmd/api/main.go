package main

import (
	"log"
	"net/http"
	"os"

	"github.com/gin-gonic/gin"
	"github.com/rs/zerolog"
)

func main() {
	logger := zerolog.New(os.Stderr).With().Timestamp().Logger()
	router := gin.Default()

	router.GET("/health", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{"status": "ok", "service": "cislunar-ca-api"})
	})

	api := router.Group("/api/v1")
	{
		api.POST("/certificates/issue", handleIssueCertificate)
		api.GET("/certificates/:serial", handleGetCertificate)
		api.GET("/certificates", handleListCertificates)
		api.POST("/certificates/:serial/revoke", handleRevokeCertificate)
		api.GET("/crl", handleGetCrl)
		api.GET("/ocsp/:serial", handleOcspRequest)
		api.POST("/identities/register", handleRegisterIdentity)
		api.GET("/identities/:id", handleGetIdentity)
	}

	addr := ":8080"
	logger.Info().Str("addr", addr).Msg("Starting CA API server")
	if err := router.Run(addr); err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}

func handleIssueCertificate(c *gin.Context) {
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented"})
}

func handleGetCertificate(c *gin.Context) {
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented"})
}

func handleListCertificates(c *gin.Context) {
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented"})
}

func handleRevokeCertificate(c *gin.Context) {
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented"})
}

func handleGetCrl(c *gin.Context) {
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented"})
}

func handleOcspRequest(c *gin.Context) {
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented"})
}

func handleRegisterIdentity(c *gin.Context) {
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented"})
}

func handleGetIdentity(c *gin.Context) {
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented"})
}
