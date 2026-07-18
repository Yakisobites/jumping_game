# ============================
# Macroquad WASM ビルド用 Justfile
# ============================

# Cargo test
# コード整形
fmt:
    cargo fmt --all
# 静的解析（警告をエラー扱い）
clippy:
    cargo clippy --all-targets --all-features -- -D warnings
# Cargo check
check:
  cargo check

# fmt, clippy, checkをまとめて実行
test: fmt clippy check

# WASMターゲット追加（初回のみ）
add-target:
    rustup target add wasm32-unknown-unknown

# Releaseビルド（古い方式）
build:
    cargo build --release --target wasm32-unknown-unknown

# dist フォルダ作成 & 必要ファイルコピー
dist:
    mkdir -p dist
    cp index.html dist/
    cp target/wasm32-unknown-unknown/release/ferris_jumping_game.wasm dist/
    cp -r assets dist/ || true

# ローカルで簡易サーバー起動（WASMはHTTP必須）
serve:
    python3 -m http.server --directory dist

# 一括実行（ビルド → dist作成 → ローカルサーバー）
run: build dist serve

