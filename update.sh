#!/usr/bin/env bash

set -ex

cargo install --version 0.21.0 svd2rust
cargo install --version 0.8.0 form
cargo install --version 0.2.1 svdtools
rustup component add rustfmt

rm -rf src
mkdir src

svd patch svd/bl702.yaml

svd2rust -i svd/bl702.svd.patched --target riscv

form -i lib.rs -o src
rm lib.rs

cargo fmt
