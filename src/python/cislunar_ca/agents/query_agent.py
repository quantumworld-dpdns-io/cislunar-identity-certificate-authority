import httpx
from typing import Optional
from ..config import settings


class CertificateQueryAgent:
    def __init__(self, ollama_host: Optional[str] = None, model: Optional[str] = None):
        self.ollama_host = ollama_host or settings.ollama_host
        self.model = model or settings.ollama_model
        self.client = httpx.AsyncClient(base_url=self.ollama_host, timeout=settings.ollama_timeout)

    async def natural_language_query(self, question: str, context: dict) -> str:
        prompt = f"""Answer this question about the Cislunar Certificate Authority:
Question: {question}
Context: {context}

Provide a clear, concise answer based on the CA data."""
        
        response = await self.client.post(
            "/api/generate",
            json={"model": self.model, "prompt": prompt, "stream": False},
        )
        return response.json().get("response", "")

    async def search_certificates_by_description(self, description: str, certs: list) -> list:
        prompt = f"""Given this description: "{description}"
Find matching certificates from: {certs}
Return serial numbers of matches as a JSON list."""
        
        response = await self.client.post(
            "/api/generate",
            json={
                "model": self.model,
                "prompt": prompt,
                "stream": False,
                "format": "json",
            },
        )
        return response.json().get("response", [])

    async def close(self):
        await self.client.aclose()
