FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 libpq5 \
    && rm -rf /var/lib/apt/lists/*

COPY target/release/app-gateway /app/

RUN mkdir -p /app/logs

EXPOSE 8080

ENV RUST_LOG=info

CMD ["/app/app-gateway"]
