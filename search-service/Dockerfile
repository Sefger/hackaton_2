# ====================== Builder stage ======================
FROM rust:1.94-slim-bookworm AS builder

# Устанавливаем необходимые системные пакеты
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Копируем workspace
COPY Cargo.toml Cargo.lock ./

COPY shared/Cargo.toml ./shared/
COPY index-service/Cargo.toml ./index-service/
COPY search-service/Cargo.toml ./search-service/

# Заглушки для всех членов workspace
RUN mkdir -p shared/src index-service/src search-service/src \
    && echo "pub fn dummy() {}" > shared/src/lib.rs \
    && echo "fn main() { println!(\"dummy\"); }" > index-service/src/main.rs \
    && echo "fn main() { println!(\"dummy\"); }" > search-service/src/main.rs

# Кэшируем зависимости
RUN cargo build --release --package search-service

# Копируем реальный код
COPY shared ./shared
COPY search-service ./search-service

# Финальная сборка
RUN cargo build --release --package search-service

# ====================== Runtime stage ======================
FROM ubuntu:24.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/search-service /app/search-service

RUN mkdir -p /root/.cache/huggingface/hub

EXPOSE 8080

CMD ["./search-service"]