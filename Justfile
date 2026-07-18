# ============================
# Justfile for Macroquad WASM build
# ============================

# Code formatting
fmt:
    cargo fmt --all

# Static analysis (treat warnings as errors)
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Cargo check
check:
    cargo check

# Unit tests
cargo-test:
    cargo test

# Run fmt, clippy, check, and tests together
test: fmt clippy check cargo-test

# Add WASM target (first time only)
add-target:
    rustup target add wasm32-unknown-unknown

# Release build (using the same RUSTFLAGS as CI)
build:
    RUSTFLAGS='-C link-arg=--allow-undefined' cargo build --release --target wasm32-unknown-unknown

# Create dist, optimize wasm with wasm-opt, and copy required files
dist:
    mkdir -p dist
    wasm-opt -O3 target/wasm32-unknown-unknown/release/ferris_jumping_game.wasm -o dist/ferris_jumping_game.wasm
    cp index.html dist/
    cp -r assets dist/ || true

# Start a simple local server (WASM requires HTTP)
serve:
    basic-http-server dist

# Run all steps (build -> dist -> local server)
# Uses Docker automatically if available; falls back to local toolchain
run:
    #!/usr/bin/env sh
    if command -v docker >/dev/null 2>&1; then
        just docker-run
    else
        just build && just dist && just serve
    fi

# ============================
# Docker targets
# ============================

# Build the Docker image (installs all required tools)
docker-image:
    docker build -t jumping_game .

# Run the full pipeline inside Docker: build -> dist -> serve on http://localhost:4000
# Mounts the project directory so no source copy is needed
docker-run: docker-image
    docker run --rm -p 4000:4000 \
        -v "{{justfile_directory()}}:/app" \
        -v "jumping_game_cargo_cache:/root/.cargo/registry" \
        jumping_game \
        sh -c "just build && just dist && basic-http-server --addr 0.0.0.0:4000 dist"
