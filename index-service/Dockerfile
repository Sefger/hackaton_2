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

# Собираем только нужный пакет.
# Используем cache mount'ы для registry и target — так зависимости переиспользуются
# между сборками без трюков с dummy main.rs.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --package index-service \
    && cp /app/target/release/index-service /usr/local/bin/index-service

# ====================== Runtime stage ======================
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/local/bin/index-service /app/index-service

# Папка для моделей fastembed (sparse embedding)
RUN mkdir -p /root/.cache/huggingface/hub

EXPOSE 8080

CMD ["./index-service"]
