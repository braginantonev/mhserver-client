#!/bin/bash

if [[ !(-e Cargo.toml) ]]; then
    echo zxc
    cd ..
fi

if [[ -e build/bin ]]; then
    rm -rf build/bin
fi

mkdir build/bin/

# linux build
echo "start linux build"
cargo build --release
cp target/release/mhserver-client build/bin

echo

# windows build
echo "start windows build"
cargo build --target x86_64-pc-windows-gnu --release
cp target/x86_64-pc-windows-gnu/release/mhserver-client.exe build/bin
