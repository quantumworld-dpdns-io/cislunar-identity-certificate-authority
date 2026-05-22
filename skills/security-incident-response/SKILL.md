---
name: security-incident-response
description: Respond to security incidents involving the Cislunar Identity CA, including key compromise and unauthorized access
---

# Security Incident Response Skill

## Overview
Procedures for detecting, analyzing, and responding to security incidents affecting the Cislunar Identity Certificate Authority.

## Incident Types

### Key Compromise
1. **Detection**: Identify anomalous certificate issuance patterns or unauthorized access
2. **Containment**: Immediately revoke affected certificates
3. **Analysis**: Determine scope of compromise
4. **Recovery**: Re-issue certificates with new keys
5. **Post-mortem**: Update security controls

### Unauthorized Access
1. **Detection**: Review audit logs for suspicious API calls
2. **Containment**: Revoke access tokens, rotate API keys
3. **Analysis**: Identify breached accounts/resources
4. **Recovery**: Restore from secure backup
5. **Hardening**: Update RBAC policies

## Runbooks
- `runbooks/key-compromise.md`
- `runbooks/unauthorized-access.md`
- `runbooks/data-breach.md`

## References
- NIST SP 800-61 Rev 2: Incident Handling Guide
- CIS Controls: Incident Response
