FROM rust:1-slim-bookworm

# Use Debian's default mirror configuration. deb.debian.org already resolves through Debian's CDN.
RUN apt-get update && \
    apt-get install -y --no-install-recommends binaryen && \
    rm -rf /var/lib/apt/lists/*

# Avoid cargo install here because compiling helper tools significantly slows image builds.
# Downloading release binaries keeps the image setup fast and deterministic.
RUN cargo install just basic-http-server

# Add the WebAssembly target used by the build pipeline.
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

EXPOSE 4000
