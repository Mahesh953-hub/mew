#!/usr/bin/env bash
set -euo pipefail

echo "== mew local check =="

rustc --version
cargo --version

echo
echo "== fmt =="
cargo fmt --all -- --check

echo
echo "== test =="
cargo test --workspace

echo
echo "== build =="
cargo build --workspace

echo
echo "== cli smoke =="
cargo run -p mew-cli -- --help >/dev/null
cargo run -p mew-cli -- doctor >/dev/null
cargo run -p mew-cli -- init --dry-run >/dev/null
cargo run -p mew-cli -- style preview >/dev/null

echo
echo "purrfect."
