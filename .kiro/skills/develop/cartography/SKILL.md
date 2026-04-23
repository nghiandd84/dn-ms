---
name: cartography
description: Use when exploring unfamiliar codebase, understanding architecture across services/libs, onboarding to the project, or mapping dependencies between crates
---

# Cartography Skill

Generate hierarchical codemaps for the dn-ms Rust microservices workspace to help agents build mental model of the codebase.

## What it does

1. **Scans directory structure** - Maps apis, apps, features, libs, and key configuration files
2. **Generates codemap.md** - Creates/updates codemap per folder with:
    - Responsibility (what this crate owns)
    - Design patterns used
    - Data/control flow
    - Integration points
3. **Tracks changes** - Stores file hashes for change detection

## When to use

- When exploring unfamiliar codebase
- When understanding architecture across services/libs
- When onboarding to the project
- When mapping dependencies between crates

## Codebase structure

```
dn-ms/
├── apis/                    # API server crates (axum HTTP servers)
│   ├── auth
│   ├── bakery
│   ├── booking
│   ├── email-template
│   ├── event
│   ├── fee
│   ├── inventory
│   ├── lookup
│   ├── merchant
│   ├── notification
│   ├── payment-core
│   ├── paypments
│   ├── profile
│   ├── translation
│   └── wallet
├── apps/                    # Standalone applications
│   ├── auth-notification
│   ├── auth-web
│   ├── gateway
│   ├── gateway-bk
│   └── notification
├── features/                # Domain logic crates (business rules, entities, migrations)
│   ├── auth
│   ├── bakery
│   ├── booking
│   ├── email-template
│   ├── event
│   ├── fee
│   ├── inventory
│   ├── lookup
│   ├── merchant
│   ├── notification
│   ├── payments
│   ├── profiles
│   ├── translation
│   └── wallet
├── libs/                    # Shared library crates
│   ├── shared              # Common types, utilities, middleware
│   └── tools               # Build/dev tooling
├── docker/                  # Docker configs, migrations, compose
├── keys/                    # JWT keys
├── Cargo.toml               # Workspace root
└── .kiro/                   # AI agent configs, docs, skills
```

## Output format

### codemap.md

```markdown
# Crate Name

## Responsibility

Brief description of what this crate owns.

## Design Patterns

- Pattern 1: Description
- Pattern 2: Description

## Data Flow

Input → Processing → Output

## Integration Points

| Dependency | Location | Purpose |
|------------|----------|---------|
| crate-name | features/name | Domain logic |

## Key Files

| File | Purpose |
|------|---------|
| src/lib.rs | Crate entry point |
```

### .kiro/cartography.json (tracking)

```json
{
    "version": "1.0.0",
    "lastUpdated": "2026-04-23",
    "hashes": {
        "apis/auth": "abc123"
    }
}
```

## Usage

When invoked, generate codemap.md for the requested folder or root if unspecified.
