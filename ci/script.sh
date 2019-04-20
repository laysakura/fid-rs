#!/usr/bin/env bash
set -eux

cargo build --release --verbose --all
cargo test --release --verbose --all
cargo fmt --all -- --check
cargo doc
cargo bench --all
rm -rf target/doc/criterion && mv target/criterion target/doc/
