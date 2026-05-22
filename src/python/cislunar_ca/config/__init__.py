from pydantic_settings import BaseSettings
from pydantic import Field
from typing import Optional


class Settings(BaseSettings):
    ca_api_addr: str = Field("http://localhost:8080", description="CA API address")
    ca_api_timeout: int = Field(30, description="CA API timeout seconds")

    ollama_host: str = Field("http://localhost:11434", description="Ollama server host")
    ollama_model: str = Field("llama3.2:3b", description="Default LLM model")
    ollama_embedding_model: str = Field("nomic-embed-text", description="Embedding model")
    ollama_timeout: int = Field(60, description="Ollama timeout seconds")

    chroma_host: str = Field("localhost", description="ChromaDB host")
    chroma_port: int = Field(8000, description="ChromaDB port")
    chroma_collection: str = Field("cislunar-identities", description="Default collection name")

    lancedb_uri: Optional[str] = Field(None, description="LanceDB URI")

    duckdb_path: Optional[str] = Field(None, description="DuckDB database path")

    weave_project: str = Field("cislunar-ca", description="W&B Weave project name")
    weave_enabled: bool = Field(True, description="Enable W&B Weave tracing")

    mcp_port: int = Field(8001, description="MCP server port")
    mcp_server_name: str = Field("cislunar-ca-mcp", description="MCP server name")

    log_level: str = Field("INFO", description="Logging level")

    model_config = {"env_prefix": "CA_", "env_file": ".env", "env_nested_delimiter": "__"}


settings = Settings()
