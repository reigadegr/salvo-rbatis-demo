#!/bin/bash
cargo +nightly ndk -p 34 -t arm64-v8a build --target aarch64-linux-android -r
dd if=./target/aarch64-linux-android/release/salvo-rbatis-demo of=./module/salvo-rbatis-demo-android
