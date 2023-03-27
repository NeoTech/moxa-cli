#!/bin/env bash
echo "Building moxa CLI - just to strip the binary."
RUSTFLAGS='-C link-args=-s' cargo build --bin moxa-cli --release
mv target/release/moxa-cli.exe ./moxa-cli.exe