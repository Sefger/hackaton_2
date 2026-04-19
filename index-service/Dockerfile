# syntax=docker/dockerfile:1.6

# ====================== Builder stage ======================
FROM rust:1.94-trixie AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY index-service ./index-service
COPY search-service ./search-service
COPY mocks ./mocks

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --package index-service --bin index-service --bin bake_sparse \
    && cp /app/target/release/index-service /usr/local/bin/index-service \
    && cp /app/target/release/bake_sparse /usr/local/bin/bake_sparse

RUN mkdir -p /opt/models && /usr/local/bin/bake_sparse /opt/models

# ====================== Runtime stage ======================
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/local/bin/index-service /app/index-service
COPY --from=builder /opt/models /app/models

ENV MODEL_DIR=/app/models
EXPOSE 8080

CMD ["./index-service"]
