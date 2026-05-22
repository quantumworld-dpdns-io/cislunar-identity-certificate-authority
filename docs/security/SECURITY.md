# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

Report security vulnerabilities to security@cislunar-ca.space. Do not file public issues.

## OWASP Top 10 Compliance

This project is tested against OWASP Top 10 using Robot Framework. See `tests/security/robot/tests/`.

## Security Controls

1. **Cryptographic**: RSA/ECDSA/Ed25519/PQC key support, hardware attestation, TEE sealing
2. **Access Control**: RBAC, JWT, API keys, mTLS, OAuth2/OIDC
3. **Audit**: All CA operations logged, tamper-evident audit trail
4. **Infrastructure**: Secret scanning, SAST/DAST/SCA in CI, container scanning
5. **Supply Chain**: Dependency review, signed commits, SBOM generation
