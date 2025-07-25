#!/bin/bash
echo 'cargo sqlx prepare(using local)'
DATABASE_URL=postgres://devuser:devuser@localhost/vpn cargo sqlx prepare
git add .

# use "SQLX_OFFLINE=TRUE cargo build --release" for prd.
