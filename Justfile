# ============================
# Macroquad WASM ビルド用 Justfile
# ============================

# コード整形
fmt:
    cargo fmt --all

# 静的解析（警告をエラー扱い）
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Cargo check
check:
    cargo check

# ユニットテスト
cargo-test:
    cargo test

# fmt, clippy, check, testをまとめて実行
test: fmt clippy check cargo-test

# WASMターゲット追加（初回のみ）
add-target:
    rustup target add wasm32-unknown-unknown

# Releaseビルド（CIと同じRUSTFLAGSを使用）
build:
    RUSTFLAGS='-C link-arg=--allow-undefined' cargo build --release --target wasm32-unknown-unknown

# dist フォルダ作成 & wasm-opt最適化 & 必要ファイルコピー
dist:
    mkdir -p dist
    wasm-opt -O3 target/wasm32-unknown-unknown/release/ferris_jumping_game.wasm -o dist/ferris_jumping_game.wasm
    cp index.html dist/
    cp -r assets dist/ || true

# ローカルで簡易サーバー起動（WASMはHTTP必須）
serve:
    basic-http-server dist

# 一括実行（ビルド → dist作成 → ローカルサーバー）
run: build dist serve

