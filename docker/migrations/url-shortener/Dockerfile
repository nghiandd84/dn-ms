FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 libpq5 \
    && rm -rf /var/lib/apt/lists/*

COPY target/release/migrations_url_shortener /app/

ENV RUST_LOG=info

CMD ["/app/migrations_url_shortener", "up"]
