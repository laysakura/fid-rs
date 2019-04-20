#!/usr/bin/env bash
set -eux

rustup component add rustfmt
cargo readme || cargo install cargo-readme  # skip if already available
