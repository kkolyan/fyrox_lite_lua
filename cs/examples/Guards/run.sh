#!/usr/bin/env bash
set -e
cd ../../..
cargo build -p fyrox-c
cp target/debug/libfyrox_c.dylib cs/examples/Guards/bin/Debug/net8.0
cd cs/FyroxLite
dotnet build
cd ../examples/Guards
dotnet run
