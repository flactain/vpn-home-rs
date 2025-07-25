#!/bin/bash
echo 'cargo fmt'
cargo fmt
echo 'cargo clippy(unused imports)'
cargo clippy --fix --allow-dirty --allow-staged -- -W unused-imports

git add .
