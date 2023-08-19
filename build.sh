#!/bin/bash
set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <environment>"
    exit 1
fi

ENV=$1

APP_ENV=${ENV} cargo build --release
mv target/release/similars-sled /home/similar/${ENV}/similars-sled-${ENV}
cp -r config.${ENV}.yml /home/similar/${ENV}/
