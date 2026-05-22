import json
import csv
import io
from typing import Optional
from datetime import datetime
from .certificate_analytics import CertificateAnalytics


class ReportGenerator:
    def __init__(self, analytics: CertificateAnalytics):
        self.analytics = analytics

    def generate_summary_report(self, output_format: str = "json") -> str:
        summary = self.analytics.get_authority_summary()
        expiry = self.analytics.get_expiry_forecast(30)
        issuance = self.analytics.get_issuance_summary()
        
        report = {
            "generated_at": datetime.utcnow().isoformat(),
            "authority_summary": summary,
            "expiring_soon": [
                {"subject": r[0], "serial": r[1], "expires": str(r[2]), "days_remaining": r[3]}
                for r in expiry
            ],
            "recent_issuance": [
                {"key_type": r[0], "status": r[1], "count": r[2]}
                for r in issuance
            ],
            "expiring_count": len(expiry),
        }

        if output_format == "csv":
            buffer = io.StringIO()
            writer = csv.writer(buffer)
            writer.writerow(["Metric", "Value"])
            writer.writerow(["Total Certificates", summary["total"]])
            writer.writerow(["Active", summary["active"]])
            writer.writerow(["Revoked", summary["revoked"]])
            writer.writerow(["Expired", summary["expired"]])
            writer.writerow(["Expiring in 30 days", len(expiry)])
            for r in expiry:
                writer.writerow(["Expiring", f"{r[0]} ({r[1]}) - {r[3]} days"])
            return buffer.getvalue()

        return json.dumps(report, indent=2)
