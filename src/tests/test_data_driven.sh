#!/bin/bash

# Build the project
echo "Building QitOps..."
cargo build

# Run data-driven API test with CSV data
echo "Running data-driven API test with CSV data..."
./target/debug/qitops data-driven -c tests/configs/data_driven_api_test.json -d tests/data/users.csv -t csv -e production

# Run data-driven API collection with JSON data
echo "Running data-driven API collection with JSON data..."
./target/debug/qitops data-driven -c tests/configs/data_driven_collection.json -d tests/data/products.json -t json -e production

echo "Test completed!"
