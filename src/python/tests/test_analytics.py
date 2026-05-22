import pytest
from cislunar_ca.analytics import CertificateAnalytics


@pytest.fixture
def analytics():
    db = CertificateAnalytics(db_path=":memory:")
    db.initialize_schema()
    yield db
    db.close()


def test_authority_summary_empty(analytics):
    summary = analytics.get_authority_summary()
    assert summary["total"] == 0


def test_record_and_summary(analytics):
    analytics.record_certificate(
        serial="abc123",
        subject="test.cislunar.local",
        issuer="Cislunar CA",
        key_type="ecdsa-p256",
        ca_type="EndEntity",
        status="Active",
        issued_at="2026-01-01T00:00:00Z",
        expires_at="2027-01-01T00:00:00Z",
    )
    summary = analytics.get_authority_summary()
    assert summary["total"] == 1
    assert summary["active"] == 1


def test_expiry_forecast(analytics):
    analytics.record_certificate(
        serial="expiring1",
        subject="old.cislunar.local",
        issuer="Cislunar CA",
        key_type="ecdsa-p256",
        ca_type="EndEntity",
        status="Active",
        issued_at="2025-01-01T00:00:00Z",
        expires_at="2026-06-01T00:00:00Z",
    )
    forecast = analytics.get_expiry_forecast(365)
    assert len(forecast) > 0


def test_issuance_summary(analytics):
    analytics.record_certificate(
        serial="s1",
        subject="a.cislunar.local",
        issuer="Cislunar CA",
        key_type="ecdsa-p256",
        ca_type="EndEntity",
        status="Active",
        issued_at="2026-05-01T00:00:00Z",
        expires_at="2027-05-01T00:00:00Z",
    )
    summary = analytics.get_issuance_summary(90)
    assert len(summary) > 0
