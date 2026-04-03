#!/bin/bash
# ═══════════════════════════════════════════════════════════
# dn-ms Docker Build Script
# Reads version from Cargo.toml, increments it, builds binaries,
# then builds Docker images with version tags.
#
# Usage:
#   ./build.sh              # Bump version, build all binaries, build all images
#   ./build.sh api-auth     # Bump version, build single binary, build single image
#   ./build.sh --no-bump    # Skip version bump (use current version)
#   ./build.sh --no-cargo   # Skip cargo build (use existing target/release/)
# ═══════════════════════════════════════════════════════════

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
CARGO_TOML="$PROJECT_DIR/Cargo.toml"

cd "$PROJECT_DIR"

# ── Parse arguments ─────────────────────────────────────────
SKIP_BUMP=false
SKIP_CARGO=false
SERVICE=""

for arg in "$@"; do
    case "$arg" in
        --no-bump)  SKIP_BUMP=true ;;
        --no-cargo) SKIP_CARGO=true ;;
        *) SERVICE="$arg" ;;
    esac
done

# ── Step 1: Read & bump version ─────────────────────────────
CURRENT_VERSION=$(grep -m1 '^version = ' "$CARGO_TOML" | sed 's/version = "\(.*\)"/\1/')

if [ "$SKIP_BUMP" = true ]; then
    VERSION="$CURRENT_VERSION"
    echo "Using current version: $VERSION"
else
    # Increment patch version: 0.0.1 → 0.0.2
    MAJOR=$(echo "$CURRENT_VERSION" | cut -d. -f1)
    MINOR=$(echo "$CURRENT_VERSION" | cut -d. -f2)
    PATCH=$(echo "$CURRENT_VERSION" | cut -d. -f3)
    PATCH=$((PATCH + 1))
    VERSION="${MAJOR}.${MINOR}.${PATCH}"

    # Update Cargo.toml (workspace.package version)
    sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$VERSION\"/" "$CARGO_TOML"
    echo "Version bumped: $CURRENT_VERSION → $VERSION"
fi

echo ""

# ── Step 2: Build Rust binaries ─────────────────────────────
if [ "$SKIP_CARGO" = false ]; then
    echo "╔══════════════════════════════════════════════════════════╗"
    echo "║  Building Rust binaries (cargo build --release)         ║"
    echo "╚══════════════════════════════════════════════════════════╝"
    if [ -n "$SERVICE" ]; then
        echo "Building: $SERVICE"
        cargo build --release --bin "$SERVICE"
    else
        cargo build --release
    fi
    echo ""
fi

# ── Step 3: Build Docker images ─────────────────────────────
echo "╔══════════════════════════════════════════════════════════╗"
echo "║  Building Docker images (tag: $VERSION)                 ║"
echo "╚══════════════════════════════════════════════════════════╝"

# Service definitions: service_name -> (dockerfile, binary)
declare -A APIS=(
    [api-auth]="docker/apis/auth/Dockerfile"
    [api-bakery]="docker/apis/bakery/Dockerfile"
    [api-email-template]="docker/apis/email-template/Dockerfile"
    [api-notification]="docker/apis/notification/Dockerfile"
    [api-profile]="docker/apis/profile/Dockerfile"
    [api-translation]="docker/apis/translation/Dockerfile"
    [api-event]="docker/apis/event/Dockerfile"
    [api-inventory]="docker/apis/inventory/Dockerfile"
    [api-booking]="docker/apis/booking/Dockerfile"
    [api-payment-core]="docker/apis/payment-core/Dockerfile"
    [api-stripe]="docker/apis/stripe/Dockerfile"
    [api-merchant]="docker/apis/merchant/Dockerfile"
    [api-fee]="docker/apis/fee/Dockerfile"
    [api-wallet]="docker/apis/wallet/Dockerfile"
    [api-lookup]="docker/apis/lookup/Dockerfile"
)

declare -A APPS=(
    [app-gateway]="docker/apps/gateway/Dockerfile"
    [app-notification]="docker/apps/notification/Dockerfile"
    [auth-notification]="docker/apps/auth-notification/Dockerfile"
)

declare -A MIGRATIONS=(
    [migrations-auth]="docker/migrations/auth/Dockerfile"
    [migrations-bakery]="docker/migrations/bakery/Dockerfile"
    [migrations-booking]="docker/migrations/booking/Dockerfile"
    [migrations-email-template]="docker/migrations/email-template/Dockerfile"
    [migrations-event]="docker/migrations/event/Dockerfile"
    [migrations-fee]="docker/migrations/fee/Dockerfile"
    [migrations-inventory]="docker/migrations/inventory/Dockerfile"
    [migrations-merchant]="docker/migrations/merchant/Dockerfile"
    [migrations-payments-core]="docker/migrations/payments-core/Dockerfile"
    [migrations-payments-stripe]="docker/migrations/payments-stripe/Dockerfile"
    [migrations-profiles]="docker/migrations/profiles/Dockerfile"
    [migrations-translation]="docker/migrations/translation/Dockerfile"
    [migrations-wallet]="docker/migrations/wallet/Dockerfile"
    [migrations-lookup]="docker/migrations/lookup/Dockerfile"
)

build_image() {
    local name="$1"
    local dockerfile="$2"
    local image="dn-ms/$name:$VERSION"
    echo "  → $image"
    docker build -t "$image" -t "dn-ms/$name:latest" -f "$dockerfile" .
}

if [ -n "$SERVICE" ]; then
    # Find which category the service belongs to
    if [[ -v "APIS[$SERVICE]" ]]; then
        build_image "$SERVICE" "${APIS[$SERVICE]}"
    elif [[ -v "APPS[$SERVICE]" ]]; then
        build_image "$SERVICE" "${APPS[$SERVICE]}"
    elif [[ -v "MIGRATIONS[$SERVICE]" ]]; then
        build_image "$SERVICE" "${MIGRATIONS[$SERVICE]}"
    else
        echo "Error: Unknown service '$SERVICE'"
        echo "Available services:"
        for key in "${!APIS[@]}"; do echo "  $key"; done
        for key in "${!APPS[@]}"; do echo "  $key"; done
        for key in "${!MIGRATIONS[@]}"; do echo "  $key"; done
        exit 1
    fi
else
    echo "Building API images..."
    for name in "${!APIS[@]}"; do
        build_image "$name" "${APIS[$name]}"
    done

    echo "Building App images..."
    for name in "${!APPS[@]}"; do
        build_image "$name" "${APPS[$name]}"
    done

    echo "Building Migration images..."
    for name in "${!MIGRATIONS[@]}"; do
        build_image "$name" "${MIGRATIONS[$name]}"
    done
fi

echo ""
echo "╔══════════════════════════════════════════════════════════╗"
echo "║  Done! All images tagged: $VERSION                      ║"
echo "╚══════════════════════════════════════════════════════════╝"
