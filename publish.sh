#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "🔍 Formatting check..."
cargo fmt -- --check

echo "🔍 Running clippy..."
cargo clippy --workspace --all-targets --all-features

echo "🧪 Running tests..."
cargo test --workspace

echo "📦 Publishing dioxus-i18n-json-macro..."
cd "$SCRIPT_DIR/dioxus-i18n-json-macro"
cargo publish --dry-run
cargo publish

echo "⏳ Waiting for crates.io to index the macro crate..."
sleep 45

echo "📦 Publishing dioxus-i18n-json..."
cd "$SCRIPT_DIR/dioxus-i18n-json"
cargo publish --dry-run
cargo publish

echo "✅ Both crates published successfully!"
