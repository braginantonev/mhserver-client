#!/bin/bash

if [[ !(-e Cargo.toml) ]]; then
    echo zxc
    cd ..
fi

if [[ !(-e build/bin) ]]; then
    mkdir build/bin/
else
    rm -rf build/bin/
fi

# linux build
cargo build --release
cp target/release/mhserver-client build/bin/mhserver-client

# windows build
cargo build --target x86_64-pc-windows-gnu --release
cp target/x86_64-pc-windows-gnu/release/mhserver-client.exe build/bin/mhserver-client.exe

