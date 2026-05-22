package auth

import (
	"errors"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

var (
	ErrInvalidToken = errors.New("invalid token")
	ErrExpiredToken = errors.New("expired token")
	ErrForbidden    = errors.New("forbidden")
)

type Role string

const (
	RoleAdmin    Role = "admin"
	RoleOperator Role = "operator"
	RoleAuditor  Role = "auditor"
	RoleViewer   Role = "viewer"
)

type Permission string

const (
	PermIssueCertificate  Permission = "certificate:issue"
	PermRevokeCertificate Permission = "certificate:revoke"
	PermListCertificates  Permission = "certificate:list"
	PermGetCertificate    Permission = "certificate:get"
	PermRegisterIdentity  Permission = "identity:register"
	PermListIdentities    Permission = "identity:list"
	PermViewAuditLog      Permission = "audit:view"
	PermManageProfiles    Permission = "profile:manage"
	PermManageConfig      Permission = "config:manage"
)

var rolePermissions = map[Role][]Permission{
	RoleAdmin: {
		PermIssueCertificate, PermRevokeCertificate,
		PermListCertificates, PermGetCertificate,
		PermRegisterIdentity, PermListIdentities,
		PermViewAuditLog, PermManageProfiles, PermManageConfig,
	},
	RoleOperator: {
		PermIssueCertificate, PermRevokeCertificate,
		PermListCertificates, PermGetCertificate,
		PermRegisterIdentity, PermListIdentities,
	},
	RoleAuditor: {
		PermListCertificates, PermGetCertificate,
		PermListIdentities, PermViewAuditLog,
	},
	RoleViewer: {
		PermListCertificates, PermGetCertificate, PermListIdentities,
	},
}

type Claims struct {
	Subject string `json:"sub"`
	Role    Role   `json:"role"`
	Org     string `json:"org"`
	jwt.RegisteredClaims
}

type Authenticator struct {
	secret []byte
}

func NewAuthenticator(secret string) *Authenticator {
	return &Authenticator{secret: []byte(secret)}
}

func (a *Authenticator) GenerateToken(subject string, role Role, org string, expiry time.Duration) (string, error) {
	claims := Claims{
		Subject: subject,
		Role:    role,
		Org:     org,
		RegisteredClaims: jwt.RegisteredClaims{
			ExpiresAt: jwt.NewNumericDate(time.Now().Add(expiry)),
			IssuedAt:  jwt.NewNumericDate(time.Now()),
			Issuer:    "cislunar-ca",
		},
	}
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	return token.SignedString(a.secret)
}

func (a *Authenticator) ValidateToken(tokenStr string) (*Claims, error) {
	token, err := jwt.ParseWithClaims(tokenStr, &Claims{}, func(t *jwt.Token) (interface{}, error) {
		if _, ok := t.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, ErrInvalidToken
		}
		return a.secret, nil
	})
	if err != nil {
		return nil, ErrInvalidToken
	}
	claims, ok := token.Claims.(*Claims)
	if !ok || !token.Valid {
		return nil, ErrInvalidToken
	}
	return claims, nil
}

func HasPermission(role Role, perm Permission) bool {
	perms, ok := rolePermissions[role]
	if !ok {
		return false
	}
	for _, p := range perms {
		if p == perm {
			return true
		}
	}
	return false
}
