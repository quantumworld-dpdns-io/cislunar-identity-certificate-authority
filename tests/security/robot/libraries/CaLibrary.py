"""Robot Framework library for Cislunar CA operations."""

import json
import httpx
from typing import Optional


class CaLibrary:
    """Test library for interacting with the Cislunar CA API."""

    def __init__(self, api_addr: str = "http://localhost:8080"):
        self.api_addr = api_addr
        self.client = httpx.Client(base_url=api_addr, verify=False)

    def issue_certificate(self, subject: str, key_type: str = "ecdsa-p256",
                          validity_days: int = 365, profile: str = "default",
                          token: str = "") -> dict:
        """Issue a certificate via the CA API."""
        headers = {"Authorization": f"Bearer {token}"} if token else {}
        response = self.client.post(
            "/api/v1/certificates/issue",
            json={
                "subject": subject,
                "key_type": key_type,
                "validity_days": validity_days,
                "profile": profile,
            },
            headers=headers,
        )
        return {
            "status": response.status_code,
            "body": response.json() if response.text else {},
            "headers": dict(response.headers),
        }

    def revoke_certificate(self, serial: str, reason: str = "unspecified",
                           token: str = "") -> dict:
        """Revoke a certificate."""
        headers = {"Authorization": f"Bearer {token}"} if token else {}
        response = self.client.post(
            f"/api/v1/certificates/{serial}/revoke",
            json={"reason": reason},
            headers=headers,
        )
        return {"status": response.status_code, "body": response.json() if response.text else {}}

    def get_certificate(self, serial: str, token: str = "") -> dict:
        """Get certificate details."""
        headers = {"Authorization": f"Bearer {token}"} if token else {}
        response = self.client.get(f"/api/v1/certificates/{serial}", headers=headers)
        return {"status": response.status_code, "body": response.json() if response.text else {}}

    def health_check(self) -> dict:
        """Check CA API health."""
        response = self.client.get("/health")
        return {"status": response.status_code, "body": response.json()}

    def close(self):
        self.client.close()
