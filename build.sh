#!/bin/bash

VERSION=$(cat Cargo.toml | grep 'version = ' | sed 's/version = \"\([0-9\.]*\)\"/\1/')
COMMIT=$(git rev-parse HEAD)

docker build -t hpcsc/pair-matching-bot:${VERSION}-${COMMIT} .
