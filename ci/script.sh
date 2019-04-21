#!/usr/bin/env bash
set -eux

## cargo build ~ bench
cargo build --release --verbose --all
cargo test --release --verbose --all
cargo doc
cargo bench --all

## Move criterion's HTML report into doc/ dir in order to be uploaded in github.io
rm -rf target/doc/criterion && mv target/criterion target/doc/
