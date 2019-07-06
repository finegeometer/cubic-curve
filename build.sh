#!/bin/sh

set -ex

rustfmt src/lib.rs
wasm-pack build --target web
rm pkg/.gitignore   # I need the code for GitHub Pages
python3 -m http.server