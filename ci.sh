#!/bin/bash

set -euxo pipefail

cargo build
cargo test
cargo clippy -- --deny warnings
cargo fmt --check
