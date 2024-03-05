#!/usr/bin/env bash

rm -rf _site || exit $?
cp -r ./web ./_site || exit $?
wasm-bindgen --target web --out-dir _site/ target/wasm32-unknown-unknown/debug/sgs.wasm || exit $?

cd _site

python3 -m http.server || exit $?
