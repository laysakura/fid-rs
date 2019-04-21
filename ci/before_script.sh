#!/usr/bin/env bash
set -eux

rustup component add rustfmt
cargo readme || cargo install cargo-readme  # skip if already available

## Auto commit & push by CI
(
    cd `mktemp -d`
    git clone git@github.com:laysakura/fid-rs.git
    cd fid-rs
    git checkout ${TRAVIS_PULL_REQUEST_BRANCH}

    committed=0

    ### README.md from src/lib.rs
    cargo readme > README.md
    git add README.md
    git commit -m 'cargo readme > README.md' && committed=1

    ### cargo fmt
    cargo fmt --all
    git add -A
    git commit -m 'cargo fmt --all' && committed=1

    ### git push
    git push origin ${TRAVIS_PULL_REQUEST_BRANCH}

    ### Stop build if anything updated in remote
    [ $committed -eq 1 ] && exit 1
)
