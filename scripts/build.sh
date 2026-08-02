#!/bin/bash

if [[ !(-e Cargo.toml) ]]; then
    echo zxc
    cd ..
fi

tag_name=""
echo -n "Tag name: "
read tag_name

git tag -a $tag_name -m "release $tag_name"
git push --tags

mkdir build/bin/

# linux build
echo "start linux build"
cargo build --release

echo

# windows build
echo "start windows build"
cargo build --target x86_64-pc-windows-gnu --release
