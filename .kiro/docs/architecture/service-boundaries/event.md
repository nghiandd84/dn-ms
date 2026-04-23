# Event Service

Handles event publishing, subscription, and processing for inter-service communication.
- Publishes and subscribes to Kafka topics
- Event filtering, transformation, and routing
- Used by all services for async workflows
- Typical endpoints: `/events/publish`, `/events/subscribe`
