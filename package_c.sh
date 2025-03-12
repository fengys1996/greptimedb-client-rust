#!/bin/bash

cargo build --release

mkdir -p ./examples/c/lib

cp ./target/release/libgreptimedb_ingester.so ./examples/c/lib/

rm -f greptimedb_ingester.zip

7z a greptimedb_ingester.zip ./examples/c/ '-xr!build/'
