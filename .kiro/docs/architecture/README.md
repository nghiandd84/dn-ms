
# Architecture Documentation

This folder contains all high-level system design, diagrams, and architectural records for the project.

## Key Documents

- [overview-diagram.md](overview-diagram.md): Visual system architecture diagram (Mermaid)
- [layered-architecture.md](layered-architecture.md): Layered architecture explanation
- [technology-stack.md](technology-stack.md): Technology stack and tools used
- [data-flow.md](data-flow.md): How data moves between services and databases
- [deployment.md](deployment.md): Deployment, orchestration, and service communication
- [service-boundaries/](service-boundaries/): Purpose and boundaries of each microservice
	- [summary.md](service-boundaries/summary.md): Microservice summary
	- One file per service (e.g., [auth.md](service-boundaries/auth.md), [wallet.md](service-boundaries/wallet.md), ...)

## How to Use
- Start with the [overview-diagram.md](overview-diagram.md) for a visual map of the system.
- Read [layered-architecture.md](layered-architecture.md) to understand the code structure.
- See [technology-stack.md](technology-stack.md) for all core technologies and libraries.
- Follow [data-flow.md](data-flow.md) to trace how requests and events move through the system.
- Review [deployment.md](deployment.md) for deployment and orchestration details.
- Explore [service-boundaries/](service-boundaries/) for deep dives into each microservice.

## ADRs & Diagrams
- Add new ADRs (Architectural Decision Records) as needed for major decisions.
- Store additional diagrams as .md, .png, or .svg files in this folder.
