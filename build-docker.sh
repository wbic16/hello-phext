#!/bin/bash
# rustup target add x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl --release
docker build -t hello-phext:v1 .
docker images |grep hello-phext
