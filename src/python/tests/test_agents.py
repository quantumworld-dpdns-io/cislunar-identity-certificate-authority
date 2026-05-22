import pytest
from cislunar_ca.agents import PolicyValidationAgent, CertificateQueryAgent


@pytest.mark.asyncio
async def test_policy_agent_creation():
    agent = PolicyValidationAgent(ollama_host="http://localhost:11434")
    assert agent is not None
    assert agent.model == "llama3.2:3b"
    await agent.close()


@pytest.mark.asyncio
async def test_query_agent_creation():
    agent = CertificateQueryAgent(ollama_host="http://localhost:11434")
    assert agent is not None
    await agent.close()
