#!/bin/bash
DATABASE_URL=postgres://devuser:devuser@localhost/vpn cargo sqlx prepare
