#!/bin/bash

cd /data/lipsum_best

echo "Waiting for database to be up..."
sleep 10

if [[ ! -f /.initial_setup_done ]]; then
    echo "Setting up initial databases..."
    cat << EOF > /data/lipsum_best/Rocket.toml
[global.databases]
lipsum_best = { url = "$POSTGRES_URL" }
EOF
    echo "$POSTGRES_URL"
    diesel setup --database-url "$POSTGRES_URL"
    diesel migration run --database-url "$POSTGRES_URL"

    touch /.initial_setup_done
else 
    echo "Already set up..."
fi

echo "Running cargo binary..."
git pull
cargo run --release
