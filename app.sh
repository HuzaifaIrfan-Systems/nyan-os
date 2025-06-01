#!/bin/sh -e

ffmpeg -i src/audio/nyan.mp3 src/audio/nyan.wav

cargo build --release
mkdir -p app
cp target/release/nyan app/
app/nyan
