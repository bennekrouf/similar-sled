#!/bin/bash

ENV=$1

APP_ENV=$ENV cargo build --release
mv target/release/similars-sled /home/similar/$ENV/similars-sled-$ENV
cp -r config.yml /home/similar/$ENV/
