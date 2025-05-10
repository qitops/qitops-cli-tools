#!/bin/bash

# Build the project
echo "Building QitOps..."
cargo build

# Run the enhanced performance test
echo "Running enhanced performance test..."
./target/debug/qitops performance-enhanced -c tests/configs/enhanced_performance_test.json -e production

echo "Test completed!"
