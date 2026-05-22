package ca

import (
	"context"
	"fmt"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

type Certificate struct {
	SerialNumber string `json:"serial_number"`
	Subject      string `json:"subject"`
	Issuer       string `json:"issuer"`
	Status       string `json:"status"`
	PEM          string `json:"pem"`
	Fingerprint  string `json:"fingerprint"`
	IssuedAt     string `json:"issued_at"`
	ExpiresAt    string `json:"expires_at"`
	CAType       string `json:"ca_type"`
}

type IssueRequest struct {
	Subject     string   `json:"subject"`
	AltNames    []string `json:"subject_alt_names"`
	KeyType     string   `json:"key_type"`
	ValidityDay int      `json:"validity_days"`
	Profile     string   `json:"profile"`
}

type IssueResponse struct {
	SerialNumber string `json:"serial_number"`
	Subject      string `json:"subject"`
	Fingerprint  string `json:"fingerprint"`
	IssuedAt     string `json:"issued_at"`
	ExpiresAt    string `json:"expires_at"`
}

type Client struct {
	conn   *grpc.ClientConn
	config ClientConfig
}

type ClientConfig struct {
	Addr    string
	Timeout time.Duration
}

func NewClient(config ClientConfig) (*Client, error) {
	ctx, cancel := context.WithTimeout(context.Background(), config.Timeout)
	defer cancel()

	conn, err := grpc.DialContext(ctx, config.Addr,
		grpc.WithTransportCredentials(insecure.NewCredentials()),
		grpc.WithBlock(),
	)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to CA engine: %w", err)
	}

	return &Client{
		conn:   conn,
		config: config,
	}, nil
}

func (c *Client) Close() error {
	return c.conn.Close()
}

func (c *Client) IssueCertificate(ctx context.Context, req *IssueRequest) (*IssueResponse, error) {
	// gRPC call to Rust CA engine
	return &IssueResponse{
		SerialNumber: "placeholder-serial",
		Subject:      req.Subject,
		Fingerprint:  "placeholder-fingerprint",
		IssuedAt:     time.Now().UTC().Format(time.RFC3339),
		ExpiresAt:    time.Now().UTC().AddDate(0, 0, req.ValidityDay).Format(time.RFC3339),
	}, nil
}

func (c *Client) GetCertificate(ctx context.Context, serial string) (*Certificate, error) {
	return &Certificate{
		SerialNumber: serial,
		Status:       "Active",
	}, nil
}

func (c *Client) ListCertificates(ctx context.Context, status string, limit, offset int) ([]Certificate, error) {
	return []Certificate{}, nil
}

func (c *Client) RevokeCertificate(ctx context.Context, serial, reason string) error {
	return nil
}
