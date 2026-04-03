FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 libpq5 \
    && rm -rf /var/lib/apt/lists/*

COPY target/release/migrations_fee /app/

CMD ["/app/migrations_fee"]
