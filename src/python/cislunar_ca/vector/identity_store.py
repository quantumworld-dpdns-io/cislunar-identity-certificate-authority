import uuid
from typing import Optional
from chromadb import Client as ChromaClient
from chromadb.config import Settings as ChromaSettings
from ..config import settings


class IdentityVectorStore:
    def __init__(self, collection_name: Optional[str] = None):
        self.collection_name = collection_name or settings.chroma_collection
        self.client = ChromaClient(ChromaSettings(
            chroma_server_host=settings.chroma_host,
            chroma_server_http_port=settings.chroma_port,
        ))
        self.collection = self.client.get_or_create_collection(
            name=self.collection_name,
            metadata={"hnsw:space": "cosine"},
        )

    def add_identity(self, identity_id: str, name: str, organization: str,
                     asset_type: str, attributes: dict, embedding: Optional[list] = None):
        doc = f"{name} {organization} {asset_type} {json.dumps(attributes)}"
        metadata = {
            "name": name,
            "organization": organization,
            "asset_type": asset_type,
            **attributes,
        }
        if embedding:
            self.collection.add(
                ids=[identity_id],
                embeddings=[embedding],
                metadatas=[metadata],
                documents=[doc],
            )
        else:
            self.collection.add(
                ids=[identity_id],
                metadatas=[metadata],
                documents=[doc],
            )

    def query_by_attributes(self, query_text: str, n_results: int = 10) -> list:
        results = self.collection.query(
            query_texts=[query_text],
            n_results=n_results,
        )
        return [
            {
                "id": results["ids"][0][i],
                "metadata": results["metadatas"][0][i],
                "distance": results["distances"][0][i] if results.get("distances") else None,
            }
            for i in range(len(results["ids"][0]))
        ]

    def query_by_vector(self, embedding: list, n_results: int = 10,
                        filter: Optional[dict] = None) -> list:
        results = self.collection.query(
            query_embeddings=[embedding],
            n_results=n_results,
            where=filter,
        )
        return [
            {
                "id": results["ids"][0][i],
                "metadata": results["metadatas"][0][i],
                "distance": results["distances"][0][i],
            }
            for i in range(len(results["ids"][0]))
        ]

    def delete_identity(self, identity_id: str):
        self.collection.delete(ids=[identity_id])

    def count(self) -> int:
        return self.collection.count()
