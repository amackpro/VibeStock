# =============================================================================
# Stage 1: Build the Rust API
# =============================================================================
FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache openssl-dev pkgconfig postgresql-dev

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY shared/ ./shared/
COPY api/ ./api/

# Build dependencies (cached)
RUN cargo build --release --package api

# =============================================================================
# Stage 2: Production runtime
# =============================================================================
FROM debian:bookworm-slim AS production

# Install OpenSSL and ca-certificates for HTTPS
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from builder
COPY --from=builder /app/target/release/api /app/api

# Copy .env.example as template (actual env vars provided by Railway)
COPY api/migrations ./migrations

# Expose the API port
EXPOSE 3000

# Run the API
CMD ["/app/api"]
