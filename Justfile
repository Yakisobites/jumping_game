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
run: build dist serve
