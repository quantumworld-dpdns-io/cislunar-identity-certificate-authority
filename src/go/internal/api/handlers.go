package api

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

type Handler struct {
	// caClient *ca.Client
	// auth     *auth.Authenticator
	// audit    *audit.Logger
}

func NewHandler() *Handler {
	return &Handler{}
}

func (h *Handler) RegisterRoutes(r *gin.Engine) {
	r.GET("/health", h.Health)

	v1 := r.Group("/api/v1")
	{
		v1.POST("/certificates/issue", h.IssueCertificate)
		v1.GET("/certificates/:serial", h.GetCertificate)
		v1.GET("/certificates", h.ListCertificates)
		v1.POST("/certificates/:serial/revoke", h.RevokeCertificate)
		v1.GET("/crl", h.GetCRL)
		v1.GET("/ocsp/:serial", h.CheckOCSP)
		v1.POST("/identities/register", h.RegisterIdentity)
		v1.GET("/identities/:id", h.GetIdentity)
		v1.GET("/identities", h.ListIdentities)
		v1.GET("/metrics", h.Metrics)
	}
}

func (h *Handler) Health(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"status":  "ok",
		"service": "cislunar-ca-api",
		"version": "0.1.0",
	})
}

func (h *Handler) IssueCertificate(c *gin.Context) {
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented"})
}

func (h *Handler) GetCertificate(c *gin.Context) {
	serial := c.Param("serial")
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented", "serial": serial})
}

func (h *Handler) ListCertificates(c *gin.Context) {
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented"})
}

func (h *Handler) RevokeCertificate(c *gin.Context) {
	serial := c.Param("serial")
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented", "serial": serial})
}

func (h *Handler) GetCRL(c *gin.Context) {
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented"})
}

func (h *Handler) CheckOCSP(c *gin.Context) {
	serial := c.Param("serial")
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented", "serial": serial})
}

func (h *Handler) RegisterIdentity(c *gin.Context) {
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented"})
}

func (h *Handler) GetIdentity(c *gin.Context) {
	id := c.Param("id")
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented", "id": id})
}

func (h *Handler) ListIdentities(c *gin.Context) {
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not implemented"})
}

func (h *Handler) Metrics(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"certificate_count": 0,
		"identity_count":    0,
		"active_certs":      0,
		"revoked_certs":     0,
		"uptime_seconds":    0,
	})
}
