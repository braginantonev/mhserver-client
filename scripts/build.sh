#!/bin/bash

if [[ !(-e Cargo.toml) ]]; then
    echo zxc
    cd ..
fi

# linux build
echo "start linux build"
cargo build --release

if [ $? -ne 0 ]; then
    exit 1
fi

echo

# windows build
echo "start windows build"
cargo build --target x86_64-pc-windows-gnu --release

if [ $? -ne 0 ]; then
    exit 1
fi

tag_name=""
echo -n "Tag name: "
read tag_name

git tag -a $tag_name -m "release $tag_name"
git push --tags