# Notification Microservice Database Schema

## Entity Relationship Diagram (Mermaid)

```mermaid
erDiagram
	seaql_migrations {
		varchar version PK
		bigint applied_at
	}
```

## Database Schema (notification)

### Tables

#### seaql_migrations
| Column      | Type      | Default | Constraints         |
|-------------|-----------|---------|---------------------|
| version     | varchar   |         | PK                  |
| applied_at  | bigint    |         | NOT NULL            |

