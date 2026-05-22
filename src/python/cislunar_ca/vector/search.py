from typing import Optional
from .identity_store import IdentityVectorStore


class IdentitySearch:
    def __init__(self, vector_store: IdentityVectorStore):
        self.store = vector_store

    def find_similar_identities(self, identity_id: str, n_results: int = 5) -> list:
        # Use document search as proxy when no vector provided
        return self.store.query_by_attributes(identity_id, n_results)

    def search_by_organization(self, organization: str, n_results: int = 20) -> list:
        return self.store.query_by_attributes(
            f"organization: {organization}", n_results
        )

    def search_by_asset_type(self, asset_type: str, n_results: int = 20) -> list:
        return self.store.query_by_attributes(
            f"asset_type: {asset_type}", n_results
        )

    def find_anomalies(self, reference_identity_id: str, threshold: float = 0.3) -> list:
        similar = self.find_similar_identities(reference_identity_id, 100)
        anomalies = [
            s for s in similar
            if (s.get("distance") or 1.0) > threshold
        ]
        return anomalies
