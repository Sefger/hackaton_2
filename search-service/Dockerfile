FROM rust:1.85-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared/Cargo.toml ./shared/
COPY search-service/Cargo.toml ./search-service/
RUN mkdir -p shared/src search-service/src \
    && echo "fn main() {}" > shared/src/lib.rs \
    && echo "fn main() {}" > search-service/src/main.rs
RUN cargo build --release --package search-service
COPY shared ./shared
COPY search-service ./search-service
RUN cargo build --release --package search-service
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/search-service /app/search-service
RUN mkdir -p /root/.cache/huggingface/hub

EXPOSE 8080

CMD ["./search-service"]