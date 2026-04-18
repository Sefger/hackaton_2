FROM rust:1.85-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared/Cargo.toml ./shared/
COPY index-service/Cargo.toml ./index-service/
RUN mkdir -p shared/src index-service/src \
    && echo "fn main() {}" > shared/src/lib.rs \
    && echo "fn main() {}" > index-service/src/main.rs
RUN cargo build --release --package index-service
COPY shared ./shared
COPY index-service ./index-service
RUN cargo build --release --package index-service
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/index-service /app/index-service
RUN mkdir -p /root/.cache/huggingface/hub
EXPOSE 8080
CMD ["./index-service"]