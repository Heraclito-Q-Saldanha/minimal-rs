#!/bin/sh
if ! [ -x "$(command -v upx)" ]; then
    cargo build --release
else
    cargo build --release && upx target/release/minimal >> /dev/null
fi
