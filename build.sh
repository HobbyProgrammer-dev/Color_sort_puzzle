#!/usr/bin/env bash

cd sorter
cargo build --release
cd ..
cp ./sorter/target/release/sorter ./bin/
