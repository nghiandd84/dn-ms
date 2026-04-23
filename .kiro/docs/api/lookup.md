# Lookup Service API

Offers lookup tables and reference data (e.g., country codes, status enums) for other services.

## Endpoints
- `GET /lookup/{type}` — List lookup values for a type
- `POST /lookup/{type}` — Add a lookup value
- `GET /lookup/{type}/{id}` — Get lookup value details
- `PUT /lookup/{type}/{id}` — Update lookup value
- `DELETE /lookup/{type}/{id}` — Delete lookup value

## Integrations
- All services
