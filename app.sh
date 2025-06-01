#!/bin/sh -e

cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/nyan app/
app/nyan