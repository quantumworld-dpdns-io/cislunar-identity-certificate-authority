import duckdb
from typing import Optional
from datetime import datetime, timedelta
from ..config import settings


class CertificateAnalytics:
    def __init__(self, db_path: Optional[str] = None):
        self.db_path = db_path or settings.duckdb_path
        self.conn = duckdb.connect(self.db_path) if self.db_path else duckdb.connect(":memory:")

    def initialize_schema(self):
        self.conn.execute("""
            CREATE TABLE IF NOT EXISTS certificate_analytics (
                serial VARCHAR,
                subject VARCHAR,
                issuer VARCHAR,
                key_type VARCHAR,
                ca_type VARCHAR,
                status VARCHAR,
                issued_at TIMESTAMP,
                expires_at TIMESTAMP,
                revoked_at TIMESTAMP,
                days_valid INTEGER
            )
        """)
        self.conn.execute("""
            CREATE TABLE IF NOT EXISTS issuance_daily (
                date DATE,
                count INTEGER,
                key_type VARCHAR
            )
        """)

    def record_certificate(self, serial: str, subject: str, issuer: str,
                           key_type: str, ca_type: str, status: str,
                           issued_at: str, expires_at: str):
        self.conn.execute("""
            INSERT INTO certificate_analytics
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, NULL, 
                    DATEDIFF('day', ?::TIMESTAMP, ?::TIMESTAMP))
        """, [serial, subject, issuer, key_type, ca_type, status,
              issued_at, expires_at, issued_at, expires_at])

    def get_expiry_forecast(self, days_ahead: int = 30) -> list:
        result = self.conn.execute("""
            SELECT subject, serial, expires_at, 
                   DATEDIFF('day', CURRENT_TIMESTAMP, expires_at) as days_remaining
            FROM certificate_analytics
            WHERE status = 'Active'
              AND expires_at <= CURRENT_TIMESTAMP + INTERVAL ? DAY
            ORDER BY expires_at
        """, [days_ahead])
        return result.fetchall()

    def get_issuance_summary(self, days: int = 30) -> list:
        result = self.conn.execute("""
            SELECT key_type, status, COUNT(*) as count
            FROM certificate_analytics
            WHERE issued_at >= CURRENT_TIMESTAMP - INTERVAL ? DAY
            GROUP BY key_type, status
            ORDER BY count DESC
        """, [days])
        return result.fetchall()

    def get_revocation_analysis(self) -> list:
        result = self.conn.execute("""
            SELECT key_type, COUNT(*) as revoked_count
            FROM certificate_analytics
            WHERE status = 'Revoked'
            GROUP BY key_type
            ORDER BY revoked_count DESC
        """)
        return result.fetchall()

    def get_authority_summary(self) -> dict:
        total = self.conn.execute("SELECT COUNT(*) FROM certificate_analytics").fetchone()[0]
        active = self.conn.execute(
            "SELECT COUNT(*) FROM certificate_analytics WHERE status = 'Active'"
        ).fetchone()[0]
        revoked = self.conn.execute(
            "SELECT COUNT(*) FROM certificate_analytics WHERE status = 'Revoked'"
        ).fetchone()[0]
        expired = self.conn.execute(
            "SELECT COUNT(*) FROM certificate_analytics WHERE status = 'Expired'"
        ).fetchone()[0]
        return {
            "total": total,
            "active": active,
            "revoked": revoked,
            "expired": expired,
        }

    def close(self):
        self.conn.close()
