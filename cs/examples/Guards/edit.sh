#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e
cd ../../..
cargo build -p fyroxed-c
cp target/debug/libfyroxed_c.dylib cs/examples/Guards/bin/Debug/net8.0
cd cs/FyroxLite
dotnet build
cd ../examples/Guards
RUST_BACKTRACE=1 dotnet run -p ../../FyroxLite/Editor
