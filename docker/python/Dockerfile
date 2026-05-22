FROM python:3.12-slim-bookworm AS builder
RUN apt-get update && apt-get install -y gcc
WORKDIR /app
COPY src/python .
RUN pip install --no-cache-dir -e .

FROM python:3.12-slim-bookworm
COPY --from=builder /usr/local/lib/python3.12/site-packages /usr/local/lib/python3.12/site-packages
COPY --from=builder /app /app
WORKDIR /app
EXPOSE 11434 8000
ENTRYPOINT ["python", "-m", "cislunar_ca"]
