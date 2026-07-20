# ============================
# Justfile for Macroquad WASM build (Cross-Platform)
# ============================

[windows]
set shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

# Format code
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

# Release build (handling environment variable differences per OS)
build:
    {{ if os() == "windows" { "$env:RUSTFLAGS='-C link-arg=--allow-undefined'; cargo build --release --target wasm32-unknown-unknown" } else { "RUSTFLAGS='-C link-arg=--allow-undefined' cargo build --release --target wasm32-unknown-unknown" } }}

# Create dist directory and copy required files (handling cross-platform command differences)
dist:
    {{ if os() == "windows" { "if (-not (Test-Path dist)) { New-Item -ItemType Directory -Path dist -Force }" } else { "mkdir -p dist" } }}
    # Uncomment to optimize with wasm-opt if needed
    # wasm-opt -O3 target/wasm32-unknown-unknown/release/ferris_jumping_game.wasm -o dist/ferris_jumping_game.wasm
    {{ if os() == "windows" { "Copy-Item target/wasm32-unknown-unknown/release/ferris_jumping_game.wasm dist/" } else { "cp target/wasm32-unknown-unknown/release/ferris_jumping_game.wasm dist/" } }}
    {{ if os() == "windows" { "Copy-Item index.html dist/" } else { "cp index.html dist/" } }}
    {{ if os() == "windows" { "if (Test-Path assets) { Copy-Item -Recurse -Force assets dist/ }" } else { "cp -r assets dist/ 2>/dev/null || true" } }}

# Start a simple local server (WASM requires HTTP)
serve:
    basic-http-server dist

# Run all steps (uses Docker if available; falls back to local toolchain)
run:
    {{ if os() == "windows" { "if (Get-Command docker -ErrorAction SilentlyContinue) { just docker-run } else { just build; just dist; just serve }" } else { "if command -v docker >/dev/null 2>&1; then just docker-run; else just build && just dist && just serve; fi" } }}

# ============================
# Docker targets (Linux/Mac, or Windows with Docker Desktop)
# ============================

# Build the Docker image
docker-image:
    docker build -t jumping_game .

# Run the full pipeline inside Docker: build -> dist -> serve
docker-run: docker-image
    docker run --rm -p 4000:4000 --mount type=bind,source={{justfile_directory()}},target=/app --mount type=volume,source=jumping_game_cargo_cache,target=/root/.cargo/registry jumping_game sh -c "just build && just dist && basic-http-server --addr 0.0.0.0:4000 dist"