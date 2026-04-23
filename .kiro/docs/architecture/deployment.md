# Deployment & Service Communication

This document outlines how services are deployed and communicate in the dn-ms Rust monorepo, with a focus on Docker Compose, service discovery, and infrastructure orchestration.

---

## 1. Deployment Overview
- All services, infrastructure, and supporting tools are containerized using Docker.
- The main deployment is orchestrated via `docker-compose.yml` in the `docker/` folder.
- Each microservice, migration job, and app has its own Dockerfile for reproducible builds.

## 2. Infrastructure Services
- **PostgreSQL**: Main relational database, exposed on port 6500.
- **Redis**: Caching and distributed locking, exposed on port 6379.
- **Kafka & Zookeeper**: Event streaming and message broker, exposed on ports 9092/2181.
- **Consul**: Service discovery and configuration, exposed on port 8500.
- **OpenTelemetry Collector**: Receives traces/logs/metrics from all services.
- **PgAdmin, Redis Commander**: Web UIs for DB and cache management.

## 3. Database Migrations
- Each microservice has a migration container (e.g., `migrations-auth`) that runs before the API containers start.
- Migrations are run against PostgreSQL using the correct schema for each service.

## 4. Microservice Deployment
- Each API is deployed as one or more containers (e.g., `api-auth-1`, `api-auth-2` for scaling/HA).
- Each service connects to PostgreSQL, Redis, Kafka, Consul, and OpenTelemetry via internal Docker network (`dn-ms-network`).
- Environment variables configure DB URLs, Redis, Kafka topics, Consul address, and tracing endpoints.
- Services register with Consul for discovery and health checks.

## 5. Application Layer
- Gateway and notification apps are deployed as containers (e.g., `app-gateway`, `app-notification-1`).
- The gateway proxies requests to the appropriate API service.

## 6. Service Communication
- **HTTP**: Gateway and apps communicate with APIs via HTTP (internal Docker network ports).
- **Kafka**: Services publish/subscribe to events for async workflows and integration.
- **Consul**: Used for service discovery, health checks, and dynamic configuration.
- **Redis**: Used for caching, distributed locks, and idempotency.
- **OpenTelemetry**: All services export traces and logs for observability.

## 7. Scaling & High Availability
- Most APIs are deployed with two instances for HA (e.g., `api-auth-1`, `api-auth-2`).
- Load balancing is handled by the gateway or external orchestrator.
- All containers share the same Docker bridge network for secure, isolated communication.

## 8. Example: Service Startup Sequence
1. Infrastructure containers (Postgres, Redis, Kafka, Consul, OTel Collector) start first.
2. Migration containers run and initialize DB schemas.
3. API and app containers start, register with Consul, and begin serving requests.
4. Gateway and apps proxy traffic to APIs.

---

## 9. References
- See `docker/docker-compose.yml` for full service definitions.
- See `setup-guide.md` for local development setup.
- See `overview-diagram.md` for a visual system overview.
