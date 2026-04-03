# ── Build stage ─────────────────────────────────────────────
FROM rust:1.89-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev libpq-dev cmake \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

RUN set -eux; \
    for lib in \
        libs/shared/shared/data/app \
        libs/shared/shared/data/auth \
        libs/shared/shared/data/cache \
        libs/shared/shared/data/core \
        libs/shared/shared/data/error \
        libs/shared/shared/app \
        libs/shared/shared/observability \
        libs/shared/shared/auth \
        libs/shared/shared/config \
        libs/shared/shared/extractor \
        libs/shared/shared/macro \
        libs/shared/shared/macro-rule \
        libs/shared/shared/middleware \
        libs/tools/dn-consul; do \
        mkdir -p "$lib/src" && echo '' > "$lib/src/lib.rs"; \
    done; \
    for feat in \
        features/auth features/booking features/event features/inventory \
        features/merchant features/fee features/profiles features/translation \
        features/notification features/bakery features/payments/core \
        features/payments/stripe features/email-template features/wallet; do \
        for layer in entities migrations model repo service remote stream; do \
            mkdir -p "$feat/$layer/src" && echo '' > "$feat/$layer/src/lib.rs"; \
        done; \
    done; \
    for api in \
        apis/auth apis/profile apis/translation apis/bakery \
        apis/email-template apis/event apis/inventory apis/booking \
        apis/merchant apis/fee apis/wallet apis/notification \
        apis/payment-core "apis/paypments/stripe"; do \
        mkdir -p "$api/src" && echo 'fn main() {}' > "$api/src/main.rs"; \
    done; \
    for app in apps/gateway apps/notification apps/auth-notification; do \
        mkdir -p "$app/src" && echo 'fn main() {}' > "$app/src/main.rs"; \
    done

RUN cargo build --release 2>&1 || true

RUN rm -rf /app/apis /app/apps /app/features /app/libs
COPY . .

RUN cargo build --release --bin migrations_event

# ── Runtime stage ───────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 libpq5 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/migrations_event /app/

CMD ["/app/migrations_event"]
