#!/bin/bash

# Build the project
echo "Building QitOps..."
cargo build

# Run the HTTPBin collection test
echo "Running HTTPBin collection test..."
./target/debug/qitops collection -c tests/configs/httpbin_collection.json -e production

# Run with JSON output
echo "Running with JSON output..."
./target/debug/qitops collection -c tests/configs/httpbin_collection.json -e production -f json

echo "Test completed!"
