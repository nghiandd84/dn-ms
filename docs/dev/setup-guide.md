# Project Setup Guide

This guide provides step-by-step instructions to set up the development environment for this Rust monorepo, including all required infrastructure services.

---

## 1. Prerequisites
- WSL2 (for Windows users)
- Ubuntu 22.04+ recommended
- Rust (1.85.0+): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Docker (optional, for running infra via Compose)

---

## 2. Core Infrastructure Setup

### PostgreSQL
- Install: `sudo apt install postgresql postgresql-contrib`
- Start: `sudo systemctl start postgresql`
- Enable: `sudo systemctl enable postgresql`
- Set password:
  ```
  sudo -u postgres psql
  ALTER USER postgres PASSWORD 'password123';
  ```

### Redis
- Add GPG key and repo:
  ```
  curl -fsSL https://packages.redis.io/gpg | sudo gpg --dearmor -o /usr/share/keyrings/redis-archive-keyring.gpg
  echo "deb [signed-by=/usr/share/keyrings/redis-archive-keyring.gpg] https://packages.redis.io/deb $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/redis.list
  sudo apt-get update
  sudo apt-get install redis-server
  ```
- Configure for systemd: Edit `/etc/redis/redis.conf`, set `supervised systemd`
- (Optional) Bind to all interfaces: `bind 0.0.0.0`
- Start: `sudo systemctl start redis-server`
- Enable: `sudo systemctl enable redis-server`

### Kafka (KRaft mode)
- Install Java: `sudo apt install default-jdk -y`
- Download Kafka:
  ```
  wget https://archive.apache.org/dist/kafka/3.7.0/kafka_2.13-3.7.0.tgz
  tar -xzf kafka_2.13-3.7.0.tgz
  sudo mv kafka_2.13-3.7.0 /opt/kafka
  ```
- Generate Cluster ID:
  ```
  KAFKA_CLUSTER_ID=$(/opt/kafka/bin/kafka-storage.sh random-uuid)
  /opt/kafka/bin/kafka-storage.sh format -t $KAFKA_CLUSTER_ID -c /opt/kafka/config/kraft/server.properties
  ```
- Create systemd service: `/etc/systemd/system/kafka.service`

### Consul
- Install:
  ```
  sudo apt-get update && sudo apt-get install -y gnupg software-properties-common curl
  curl -fsSL https://apt.releases.hashicorp.com/gpg | sudo gpg --dearmor -o /usr/share/keyrings/hashicorp-archive-keyring.gpg
  echo "deb [signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] https://apt.releases.hashicorp.com $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/hashicorp.list
  sudo apt-get update && sudo apt-get install consul
  ```
- Start in dev mode: `consul agent -dev`
- For production, see `/etc/consul.d/server.hcl` example in `docs/dev/installs/consult.md`

---

## 3. Observability & Tracing

### Jaeger v2
- Download and install:
  ```
  sudo mkdir -p /opt/jaeger
  wget https://github.com/jaegertracing/jaeger/releases/download/v2.17.0/jaeger-2.17.0-linux-amd64.tar.gz
  tar -xzf jaeger-2.17.0-linux-amd64.tar.gz
  sudo mv jaeger-2.17.0-linux-amd64/jaeger /usr/bin/jaeger
  ```
- Create systemd service: `/etc/systemd/system/jaeger.service`

### OpenTelemetry Collector
- Download and install:
  ```
  OTEL_VERSION="0.96.0"
  wget https://github.com/open-telemetry/opentelemetry-collector-releases/releases/download/v${OTEL_VERSION}/otelcol-contrib_${OTEL_VERSION}_linux_amd64.deb
  sudo dpkg -i otelcol-contrib_${OTEL_VERSION}_linux_amd64.deb
  ```
- Configure: `/etc/otelcol-contrib/config.yaml` (see `docs/dev/installs/opentelemetry-collector.md`)

### OpenObserve (optional)
- Download and install:
  ```
  sudo mkdir -p /usr/local/bin/openobserve
  curl -L -o openobserve.tar.gz https://downloads.openobserve.ai/releases/o2-enterprise/latest/openobserve-ee-linux-amd64.tar.gz
  tar -xzf openobserve.tar.gz
  sudo mv openobserve /usr/local/bin/openobserve-bin
  chmod +x /usr/local/bin/openobserve-bin
  ```
- Configure: `/etc/openobserve/openobserve.env`
- Create systemd service: `/etc/systemd/system/openobserve.service`

---

## 4. Service Management
- Check all services:
  ```
  systemctl status postgresql redis-server kafka consul jaeger otelcol-contrib
  ```
- Restart all services:
  ```
  sudo systemctl restart postgresql redis-server kafka consul jaeger otelcol-contrib
  ```

---


## 5. Project Build & Run
- Build all: `cargo build`
- Run all microservices: `bash start.sh`
- Start infrastructure: `bash start-service.sh`
- Run all database migrations for every microservice:
  ```bash
  export DATABASE_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
  bash docs/dev/generate_all_schemas.sh
  ```
- For manual migration steps, see migrations.md

---

## 6. Connection Details
- PostgreSQL: `postgres://postgres:password123@localhost:5432/your_db`
- Redis: `redis://localhost:6379`
- Jaeger UI: [http://localhost:16686](http://localhost:16686)

---

- See `docs/dev/installs/` for detailed service setup
- See `AGENTS.md` for project conventions
- See `docs/dev/README.md` for more developer guides
