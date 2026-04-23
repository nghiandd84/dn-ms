#!/bin/bash
# generate_all_schemas.sh
# This script connects to PostgreSQL using $DATABASE_URL and runs migrations for all microservices.

set -e

if [ -z "$DATABASE_URL" ]; then
  echo "Error: DATABASE_URL environment variable is not set."
  exit 1
fi

# Parse connection info from DATABASE_URL
regex='postgresql://([^:]+):([^@]+)@([^:]+):([0-9]+)/(.+)'
if [[ $DATABASE_URL =~ $regex ]]; then
  PGUSER="${BASH_REMATCH[1]}"
  PGPASSWORD="${BASH_REMATCH[2]}"
  PGHOST="${BASH_REMATCH[3]}"
  PGPORT="${BASH_REMATCH[4]}"
  PGDATABASE="${BASH_REMATCH[5]}"
else
  echo "DATABASE_URL format invalid."
  exit 1
fi

export PGUSER
export PGPASSWORD
export PGHOST
export PGPORT
export PGDATABASE

# Test connection
psql "${DATABASE_URL}" -c '\conninfo' || { echo "Failed to connect to PostgreSQL."; exit 1; }

echo "Connected to PostgreSQL successfully."

echo "Running migrations for all microservices..."

SERVICES=(auth booking wallet inventory merchant fee notification profile translation)

for SERVICE in "${SERVICES[@]}"; do
  BIN="migrations_${SERVICE}"
  echo "---"
  echo "Migrating $SERVICE..."
  if cargo run --bin "$BIN" -- -v -u "$DATABASE_URL" -s "$SERVICE"; then
    echo "$SERVICE migration completed."
  else
    echo "Migration failed for $SERVICE."
    exit 1
  fi
  echo "---"
done

echo "All migrations completed successfully."
