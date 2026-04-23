# Database Migrations Guide

This guide explains how to generate and run database schema migrations for each microservice in the monorepo.

---

## General Migration Workflow

1. **Ensure Infrastructure is Running**
   - Start PostgreSQL and other dependencies: `bash start-service.sh`
2. **Navigate to the Service Migration Binary**
   - Each microservice has a migration binary in its `features/<service>/migrations` or `apis/<service>/migrations` folder.
3. **Run Migration Command**
    - Use the following command pattern:
       ```bash
       export DATABASE_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
       cargo run --bin migrations_<service> -- -v -u $DATABASE_URL -s <service>
       ```
    - Example for `auth`:
       ```bash
       export DATABASE_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
       cargo run --bin migrations_auth -- -v -u $DATABASE_URL -s auth
       ```
4. **Rollback Migration (if needed)**
    - To rollback:
       ```bash
       cargo run --bin migrations_<service> -- -v -u $DATABASE_URL -s <service> down
       ```
5. **Check Migration Status**
    - To check status:
       ```bash
       cargo run --bin migrations_<service> -- -v -u $DATABASE_URL -s <service> status
       ```

---

## Service Migration Commands

Below are the migration commands for each microservice. Replace `<db>` and credentials as needed.


### Auth Service
```bash
export DATABASE_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
cargo run --bin migrations_auth -- -v -u $DATABASE_URL -s auth
```

### Booking Service
```bash
export DATABASE_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
cargo run --bin migrations_booking -- -v -u $DATABASE_URL -s booking
```

### Wallet Service
```bash
export DATABASE_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
cargo run --bin migrations_wallet -- -v -u $DATABASE_URL -s wallet
```

### Inventory Service
```bash
export DATABASE_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
cargo run --bin migrations_inventory -- -v -u $DATABASE_URL -s inventory
```

### Merchant Service
```bash
export DATABASE_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
cargo run --bin migrations_merchant -- -v -u $DATABASE_URL -s merchant
```

### Fee Service
```bash
export DATABASE_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
cargo run --bin migrations_fee -- -v -u $DATABASE_URL -s fee
```

### Notification Service
```bash
export DATABASE_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
cargo run --bin migrations_notification -- -v -u $DATABASE_URL -s notification
```

### Profile Service
```bash
export DATABASE_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
cargo run --bin migrations_profile -- -v -u $DATABASE_URL -s profile
```

### Translation Service
```bash
export DATABASE_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
cargo run --bin migrations_translation -- -v -u $DATABASE_URL -s translation
```

---

## Notes
- Each migration binary is typically found in `features/<service>/migrations`.
- Use the `down` argument to rollback, and `status` to check migration status.
- Update the database URL and service name as appropriate for your environment.
