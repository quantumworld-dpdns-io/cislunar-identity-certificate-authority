import httpx
from typing import Optional
from ..config import settings


class PolicyValidationAgent:
    def __init__(self, ollama_host: Optional[str] = None, model: Optional[str] = None):
        self.ollama_host = ollama_host or settings.ollama_host
        self.model = model or settings.ollama_model
        self.client = httpx.AsyncClient(base_url=self.ollama_host, timeout=settings.ollama_timeout)

    async def validate_certificate_policy(
        self, subject: str, key_type: str, validity_days: int, profile: str
    ) -> dict:
        prompt = f"""Validate this certificate request against CA policy:
Subject: {subject}
Key Type: {key_type}
Validity: {validity_days} days
Profile: {profile}

Check:
1. Subject naming convention compliance
2. Key type minimum security requirements
3. Validity period within limits
4. Profile constraints

Return as JSON: {{"valid": bool, "warnings": [...], "blockers": [...]}}"""
        
        response = await self.client.post(
            "/api/generate",
            json={
                "model": self.model,
                "prompt": prompt,
                "stream": False,
                "format": "json",
            },
        )
        result = response.json()
        return result.get("response", {})

    async def analyze_revocation(self, serial: str, reason: str, cert_info: dict) -> dict:
        prompt = f"""Analyze this certificate revocation request:
Serial: {serial}
Reason: {reason}
Certificate: {cert_info}

Check:
1. Revocation reason validity
2. Compromise severity assessment
3. Notification requirements
4. CRL update necessity"""
        
        response = await self.client.post(
            "/api/generate",
            json={"model": self.model, "prompt": prompt, "stream": False},
        )
        return {"analysis": response.json().get("response", "")}

    async def close(self):
        await self.client.aclose()
