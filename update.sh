#!/usr/bin/env bash

set -ex

cargo install --version 0.25.1 svd2rust
cargo install --version 0.10.0 form
cargo install --version 0.2.6 svdtools
rustup component add rustfmt

git rm -r -f src
rm -rf src
mkdir src

svdtools patch svd/bl702.yaml

svd2rust -i svd/bl702.svd.patched --target riscv

form -i lib.rs -o src
rm lib.rs

cargo fmt
git add src
