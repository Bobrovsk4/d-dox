FROM rust:slim-trixie AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY migration/Cargo.toml ./migration/Cargo.toml

RUN mkdir -p src/bin migration/src \
    && touch src/bin/main.rs src/bin/tool.rs migration/src/lib.rs \
    && cargo fetch

COPY src/ src/
COPY migration/src/ migration/src/

RUN cargo build --release --bin server-cli

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    openssl \
    curl \
    minio-client \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/server-cli /usr/local/bin/server
COPY config/ config/

EXPOSE 3000
CMD ["server", "start"]

