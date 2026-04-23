#!/bin/bash
cwd=$PWD

# Build Rust source

nix develop --command bash -c "cargo clean && \
                               cargo build --release && \
                               maturin build --release"
