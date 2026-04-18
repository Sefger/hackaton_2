# ====================== Builder stage ======================
FROM rust:1.94-slim-bookworm AS builder

# Устанавливаем системные зависимости для сборки (openssl + pkg-config)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Копируем workspace файлы
COPY Cargo.toml Cargo.lock ./

# Копируем Cargo.toml всех членов workspace
COPY shared/Cargo.toml ./shared/
COPY index-service/Cargo.toml ./index-service/
COPY search-service/Cargo.toml ./search-service/

# Создаём заглушки для всех пакетов (чтобы Cargo не ругался)
RUN mkdir -p shared/src index-service/src search-service/src \
    && echo "pub fn dummy() {}" > shared/src/lib.rs \
    && echo "fn main() { println!(\"dummy\"); }" > index-service/src/main.rs \
    && echo "fn main() { println!(\"dummy\"); }" > search-service/src/main.rs

# Кэшируем зависимости — собираем только index-service
RUN cargo build --release --package index-service

# Копируем реальный исходный код
COPY shared ./shared
COPY index-service ./index-service

# Финальная пересборка с реальным кодом
RUN cargo build --release --package index-service

# ====================== Runtime stage ======================
FROM ubuntu:24.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Копируем собранный бинарник
COPY --from=builder /app/target/release/index-service /app/index-service

# Папка для моделей fastembed (sparse embedding)
RUN mkdir -p /root/.cache/huggingface/hub

EXPOSE 8080

CMD ["./index-service"]