FROM rust:1-slim-bookworm

# Install binaryen (provides wasm-opt)
RUN apt-get update && \
    apt-get install -y --no-install-recommends binaryen && \
    rm -rf /var/lib/apt/lists/*

# Install just and basic-http-server
RUN cargo install just basic-http-server

# Add WASM build target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

EXPOSE 4000
