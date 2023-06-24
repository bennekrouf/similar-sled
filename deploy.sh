#!/bin/bash

# SSH connection details for your Debian machine
SSH_HOST="128.140.61.231"
SSH_USER="similar"
SSH_KEY="/Users/mb/.ssh/id_rsa"

# Remote directory where your repository is located
REMOTE_DIR="/home/similar/similar-sled"

# Connect to the Debian machine and execute commands
ssh -i "$SSH_KEY" "$SSH_USER@$SSH_HOST" << EOF
    cd "$REMOTE_DIR"
    git pull origin master
    cargo build --release
    rm -Rf ../dbs
    cd target/release
    ./similars-sled
EOF