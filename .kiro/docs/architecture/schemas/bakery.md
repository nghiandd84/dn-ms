
# Bakery Microservice Database Schema

## Entity Relationship Diagram (Mermaid)

```mermaid
erDiagram
	bakery {
		integer id PK
		varchar name
		float profit_margin
		timestamp created_at
		timestamp updated_at
	}
	baker {
		integer id PK
		varchar name
		json contact_details
		integer bakery_id FK
		timestamp created_at
		timestamp updated_at
	}
	cake {
		integer id PK
		varchar name
		float price
		boolean gluten_free
		uuid serial
		integer bakery_id FK
		timestamp created_at
		timestamp updated_at
	}
	cakes_bakers {
		integer cake_id PK, FK
		integer baker_id PK, FK
		timestamp created_at
		timestamp updated_at
	}
	customer {
		integer id PK
		varchar name
		varchar notes
		timestamp created_at
		timestamp updated_at
	}
	"order" {
		integer id PK
		float total
		integer bakery_id FK
		integer customer_id FK
		timestamp placed_at
		timestamp created_at
		timestamp updated_at
	}
	lineitem {
		integer id PK
		float price
		integer quantity
		integer order_id FK
		integer cake_id FK
		timestamp created_at
		timestamp updated_at
	}
	seaql_migrations {
		varchar version PK
		bigint applied_at
	}
    
	baker ||--o{ cake : "bakes"
	cake ||--o{ cakes_bakers : "is baked by"
	baker ||--o{ cakes_bakers : "bakes"
	bakery ||--o{ baker : "employs"
	bakery ||--o{ cake : "offers"
	bakery ||--o{ "order" : "receives"
	customer ||--o{ "order" : "places"
	"order" ||--o{ lineitem : "contains"
	cake ||--o{ lineitem : "is in"
```

## Database Schema (bakery)

### Tables

#### baker
| Column          | Type                | Default             | Constraints         |
|-----------------|---------------------|---------------------|---------------------|
| id              | integer             |                     | PK, NOT NULL        |
| name            | varchar             |                     | NOT NULL            |
| contact_details | json                |                     | NOT NULL            |
| bakery_id       | integer             |                     |                     |
| created_at      | timestamp           | CURRENT_TIMESTAMP   | NOT NULL            |
| updated_at      | timestamp           | CURRENT_TIMESTAMP   | NOT NULL            |

---

#### order
| Column      | Type      | Default           | Constraints         |
|-------------|-----------|-------------------|---------------------|
| id          | integer   |                   | PK, NOT NULL        |
| total       | float     |                   | NOT NULL            |
| bakery_id   | integer   |                   | NOT NULL, FK        |
| customer_id | integer   |                   | NOT NULL, FK        |
| placed_at   | timestamp |                   | NOT NULL            |
| created_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### cake
| Column      | Type      | Default           | Constraints         |
|-------------|-----------|-------------------|---------------------|
| id          | integer   |                   | PK, NOT NULL        |
| name        | varchar   |                   | NOT NULL            |
| price       | float     |                   | NOT NULL            |
| gluten_free | boolean   |                   | NOT NULL            |
| serial      | uuid      |                   | UNIQUE, NOT NULL    |
| bakery_id   | integer   |                   | NOT NULL, FK        |
| created_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### customer
| Column      | Type      | Default           | Constraints         |
|-------------|-----------|-------------------|---------------------|
| id          | integer   |                   | PK, NOT NULL        |
| name        | varchar   |                   | NOT NULL            |
| notes       | varchar   |                   |                     |
| created_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

*Add more tables below following this format.*

#### bakery
| Column        | Type      | Default           | Constraints         |
|--------------|-----------|-------------------|---------------------|
| id           | integer   |                   | PK, NOT NULL        |
| name         | varchar   |                   | NOT NULL            |
| profit_margin| float     |                   | NOT NULL            |
| created_at   | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at   | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### cakes_bakers
| Column      | Type      | Default           | Constraints         |
|-------------|-----------|-------------------|---------------------|
| cake_id     | integer   |                   | PK, NOT NULL, FK    |
| baker_id    | integer   |                   | PK, NOT NULL, FK    |
| created_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### lineitem
| Column      | Type      | Default           | Constraints         |
|-------------|-----------|-------------------|---------------------|
| id          | integer   |                   | PK, NOT NULL        |
| price       | float     |                   | NOT NULL            |
| quantity    | integer   |                   | NOT NULL            |
| order_id    | integer   |                   | NOT NULL, FK        |
| cake_id     | integer   |                   | NOT NULL, FK        |
| created_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### seaql_migrations
| Column      | Type      | Default | Constraints         |
|-------------|-----------|---------|---------------------|
| version     | varchar   |         | PK                  |
| applied_at  | bigint    |         | NOT NULL            |

---
