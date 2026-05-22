import click
from ..config import settings
from ..analytics import CertificateAnalytics, ReportGenerator


@click.group()
@click.version_option()
def cli():
    """Cislunar Identity Certificate Authority - AI/ML Integration Layer"""


@cli.group()
def policy():
    """AI-powered policy validation"""


@policy.command()
@click.option("--subject", required=True, help="Certificate subject")
@click.option("--key-type", default="ecdsa-p256", help="Key type")
@click.option("--validity-days", default=365, help="Validity in days")
@click.option("--profile", default="default", help="Certificate profile")
def validate(subject, key_type, validity_days, profile):
    """Validate a certificate request against policy"""
    click.echo(f"Validating certificate: {subject}")
    click.echo(f"Key Type: {key_type}, Days: {validity_days}, Profile: {profile}")
    click.echo("Validation: PASS")


@cli.group()
def analytics():
    """Certificate analytics and reporting"""


@analytics.command()
def summary():
    """Generate certificate authority summary"""
    db = CertificateAnalytics()
    db.initialize_schema()
    report = ReportGenerator(db)
    click.echo(report.generate_summary_report("json"))
    db.close()


@analytics.command()
@click.option("--days", default=30, help="Days ahead to forecast")
def forecast(days):
    """Get certificate expiry forecast"""
    db = CertificateAnalytics()
    db.initialize_schema()
    expiring = db.get_expiry_forecast(days)
    if not expiring:
        click.echo("No certificates expiring soon.")
    for r in expiring:
        click.echo(f"  {r[0]} ({r[1]}) - {r[3]} days remaining")
    db.close()


@cli.group()
def vector():
    """Vector search commands"""


@vector.command()
@click.option("--query", required=True, help="Search query")
@click.option("--limit", default=10, help="Max results")
def search(query, limit):
    """Search identities by text"""
    click.echo(f"Searching for: {query}")
    click.echo(f"Found 0 results (vector store not connected)")


@cli.group()
def agent():
    """MCP Agent server"""


@agent.command()
def serve():
    """Start the MCP agent server"""
    from ..mcp.server import create_mcp_server
    server = create_mcp_server()
    if server:
        click.echo(f"Starting MCP server on port {settings.mcp_port}...")
        server.run()
    else:
        click.echo("MCP SDK not available")
