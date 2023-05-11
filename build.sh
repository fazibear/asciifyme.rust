#/bin/bash

mkdir -p build
cp index.html build/
wasm-pack build -t no-modules -d build/
