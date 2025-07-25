#!/bin/bash
echo 'cargo sqlx prepare'
cargo sqlx prepare
git add .

# use "SQLX_OFFLINE=TRUE cargo build --release" for prd.
