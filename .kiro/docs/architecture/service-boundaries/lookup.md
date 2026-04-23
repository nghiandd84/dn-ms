# Lookup Service

Offers lookup tables and reference data (e.g., country codes, status enums) for other services.
- CRUD for lookup tables and values
- Centralizes enums and reference data for consistency
- Used by all services for validation and display
- Typical endpoints: `/lookup/{type}`, `/lookup/{type}/{id}`
