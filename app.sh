#!/bin/sh -e

ffmpeg -i src/sound/audio/nyan.mp3 src/sound/audio/nyan.wav

cargo build --release
mkdir -p app
cp target/release/nyan app/
app/nyan
