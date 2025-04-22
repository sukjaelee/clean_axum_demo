FROM rust:1.86-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y libssl-dev pkg-config curl

# Copy sources
COPY . .

# offline mode
ENV SQLX_OFFLINE=true

# Build in release mode
RUN cargo build --release

# Create runtime image
FROM debian:stable-slim

WORKDIR /app

RUN apt-get update && apt-get install -y libssl3 ca-certificates curl file && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/clean_axum_demo .

# Copy and rename environment file
COPY --from=builder /app/.env.test .env

# Copy assets if needed
COPY assets ./assets

ENV RUST_LOG=info

ENTRYPOINT ["/app/clean_axum_demo"]