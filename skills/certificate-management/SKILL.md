---
name: certificate-management
description: Manage the full lifecycle of X.509 certificates in the Cislunar Identity CA
---

# Certificate Management Skill

## Overview
This skill provides workflows for managing X.509 certificates in the Cislunar Identity Certificate Authority across issuance, renewal, revocation, and verification.

## Workflows

### Issue a Certificate
1. Verify identity exists and is active
2. Select appropriate profile (default, server, ca, pqc)
3. Submit certificate request with subject and validity period
4. Sign and store certificate
5. Return certificate PEM and serial number

### Revoke a Certificate
1. Verify serial number exists
2. Select revocation reason (key-compromise, affiliation-changed, superseded, cessation)
3. Update certificate status
4. Trigger CRL regeneration
5. Notify OCSP responder

### Verify Certificate Chain
1. Retrieve certificate by serial
2. Validate signature chain to root
3. Check CRL for revocation status
4. Verify key usage and extended key usage
5. Return validation result

## Scripts
- `scripts/issue-cert.sh`: Issue certificate from CSR
- `scripts/revoke-cert.sh`: Revoke by serial number
- `scripts/verify-chain.sh`: Validate certificate chain

## References
- RFC 5280: Internet X.509 PKI
- RFC 6960: OCSP
- RFC 5759: Certificate Extensions
