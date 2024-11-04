set -e
# cargo run --bin luagen
cd lua/examples/guards
cargo run -p executor-lua --manifest-path ../../../Cargo.toml
