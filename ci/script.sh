#!/usr/bin/env bash
set -eux

## Auto commit & push by CI
### README.md from src/lib.rs
cargo readme > README.md
git add README.md
git commit -m 'cargo readme > README.md' &&:
### cargo fmt
cargo fmt --all
git add -A
git commit -m 'cargo fmt --all' &&:
### git push
git fetch
git push https://${GITHUB_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git ${TRAVIS_PULL_REQUEST_BRANCH}

## cargo build ~ bench
cargo build --release --verbose --all
cargo test --release --verbose --all
cargo doc
cargo bench --all

## Move criterion's HTML report into doc/ dir in order to be uploaded in github.io
rm -rf target/doc/criterion && mv target/criterion target/doc/
