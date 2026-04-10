# WSL2 Development Stack Summary

This document summarizes the infrastructure services currently configured as background services via systemd on the local environment.

## 1. Core Infrastructure & Databases

| Service | Port | Management Command | Description |
| :--- | :--- | :--- | :--- |
| **PostgreSQL** | `5432` | `systemctl status postgresql` | Primary relational database for persistence. |
| **Redis** | `6379` | `systemctl status redis-server` | High-performance cache and distributed locking. |
| **Kafka** | `9092` | `systemctl status kafka` | Event streaming and message broker (KRaft mode). |
| **Consul** | `8500` | `systemctl status consul` | Service discovery and KV configuration. |

## 2. Observability & Tracing (OpenTelemetry)

The stack uses **Jaeger v2**, which is OpenTelemetry-native.

| Service | Port | Description |
| :--- | :--- | :--- |
| **Jaeger UI** | `16686` | Web dashboard: [http://localhost:16686](http://localhost:16686) |
| **Jaeger (OTLP)** | `4317` | Direct OTLP/gRPC entry point (Receiver). |
| **OTel Collector** | `4319` | Standalone Contrib Collector (Bridge Port). |
| **OTel Collector** | `4320` | Standalone Contrib Collector (HTTP Port). |

## 3. Quick Management Commands

### Check All Services
```bash
systemctl status postgresql redis-server kafka consul jaeger otelcol-contrib
```

# Restart All Services
```
sudo systemctl restart postgresql redis-server kafka consul jaeger otelcol-contrib
```

# Connection Details
```
DATABASE_URL=postgres://postgres:your_password@localhost:5432/your_db
REDIS_URL=redis://localhost:6379
KAFKA_BROKERS=localhost:9092
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4319
```

