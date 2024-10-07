#!/bin/sh

if [ "$1" == "android" ]; then
    cargo +nightly ndk -p 34 -t arm64-v8a build --target aarch64-linux-android -r
    return
fi

cargo build
