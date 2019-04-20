#!/usr/bin/env bash
set -eux

rustup component add rustfmt
cargo install --force cargo-readme
