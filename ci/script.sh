#!/usr/bin/env bash
set -eux

## README.md should be generated from src/lib.rs
f=`mktemp`
cargo readme > $f
diff $f README.md

cargo fmt --all -- --check
cargo build --release --verbose --all
cargo test --release --verbose --all
cargo doc
cargo bench --all

## Move criterion's HTML report into doc/ dir in order to be uploaded in github.io
rm -rf target/doc/criterion && mv target/criterion target/doc/
