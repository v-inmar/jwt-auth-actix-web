# --- Build Stage ---
FROM rust:1.92.0-slim-trixie AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

COPY . .
ENV SQLX_OFFLINE=true
RUN touch src/main.rs && cargo build --release

# --- Runtime Stage ---
FROM ubuntu:24.04
WORKDIR /app
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*


COPY --from=builder /app/target/release/jwt-auth ./server
COPY --from=builder /app/migrations ./migrations


EXPOSE 5000
CMD ["./server"]