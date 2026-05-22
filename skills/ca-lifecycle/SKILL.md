---
name: ca-lifecycle
description: Manage the Certificate Authority lifecycle including initialization, renewal, and decommissioning
---

# CA Lifecycle Management Skill

## Overview
Procedures for managing the Certificate Authority itself - initialization, root CA rotation, intermediate CA management, and decommissioning.

## Workflows

### Initialize Root CA
1. Generate root key pair (minimum ECDSA P-384 or PQC Dilithium3)
2. Create self-signed root certificate
3. Configure CRL distribution point
4. Set up OCSP responder
5. Secure root key in HSM/TEE

### Rotate Root CA
1. Generate new root key pair
2. Issue new root certificate with cross-signing
3. Issue intermediate certificates from new root
4. Migrate end-entity certificates
5. Revoke old root certificate

### Decommission CA
1. Revoke all outstanding certificates
2. Generate final CRL
3. Archive audit logs and database
4. Secure wipe key material
5. Remove CA from trust stores

## References
- CA/B Forum Baseline Requirements
- NIST SP 800-32: Key Management
- ISO 27040: Storage Security
