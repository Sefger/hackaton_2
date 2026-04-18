# syntax=docker/dockerfile:1.6

# ====================== Builder stage ======================
FROM rust:1.94-trixie AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Копируем весь workspace (manifest'ы + реальные исходники)
COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY index-service ./index-service
COPY search-service ./search-service

# Собираем только search-service.
# Cache mount'ы кэшируют registry и target — дают инкрементальные сборки без stub'ов.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --package search-service \
    && cp /app/target/release/search-service /usr/local/bin/search-service

# ====================== Runtime stage ======================
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/local/bin/search-service /app/search-service

RUN mkdir -p /root/.cache/huggingface/hub

EXPOSE 8080

CMD ["./search-service"]
