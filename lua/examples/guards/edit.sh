#!/bin/bash
cd "$(dirname "$0")"
cargo run --release -p editor-lua --manifest-path ../../../Cargo.toml