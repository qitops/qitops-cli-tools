#!/bin/bash

# Test script to verify that QitOps AI features work offline

# Set up test environment
echo "Setting up test environment..."
export QITOPS_OFFLINE=true
export QITOPS_MODEL_PATH="/usr/local/share/models/llama-2-7b-chat.gguf"

# Build QitOps with AI features
echo "Building QitOps with AI features..."
cargo build --features ai

# Test API test generation
echo "Testing API test generation..."
./target/debug/qitops generate --test-type api --description "Test the GitHub API to fetch user information" --output offline_api_test.json

# Verify the output file exists
if [ ! -f offline_api_test.json ]; then
    echo "ERROR: Failed to generate API test configuration"
    exit 1
fi

echo "API test configuration generated successfully"

# Test performance test generation
echo "Testing performance test generation..."
./target/debug/qitops generate --test-type performance --description "Load test for an e-commerce checkout API" --output offline_perf_test.json

# Verify the output file exists
if [ ! -f offline_perf_test.json ]; then
    echo "ERROR: Failed to generate performance test configuration"
    exit 1
fi

echo "Performance test configuration generated successfully"

# Test security test generation
echo "Testing security test generation..."
./target/debug/qitops generate --test-type security --description "Security scan for a banking API" --output offline_security_test.json

# Verify the output file exists
if [ ! -f offline_security_test.json ]; then
    echo "ERROR: Failed to generate security test configuration"
    exit 1
fi

echo "Security test configuration generated successfully"

# Test web test generation
echo "Testing web test generation..."
./target/debug/qitops generate --test-type web --description "Test the checkout flow of an e-commerce website" --output offline_web_test.json

# Verify the output file exists
if [ ! -f offline_web_test.json ]; then
    echo "ERROR: Failed to generate web test configuration"
    exit 1
fi

echo "Web test configuration generated successfully"

# Create a sample test result file
cat > offline_test_results.json << EOF
[
  {
    "test_id": "api-test-1",
    "name": "GitHub User API Test",
    "description": "Test the GitHub API to fetch user information",
    "timestamp": "2025-05-10T21:15:00Z",
    "duration_ms": 190,
    "status": "success",
    "url": "https://api.github.com/users/octocat",
    "method": "GET",
    "request_headers": {
      "Accept": "application/vnd.github.v3+json",
      "User-Agent": "QitOps-Test"
    },
    "response_status": 200,
    "response_headers": {
      "content-type": "application/json; charset=utf-8",
      "cache-control": "public, max-age=60, s-maxage=60"
    },
    "assertions": [
      {
        "type": "status",
        "expected": 200,
        "actual": 200,
        "result": "pass"
      },
      {
        "type": "json",
        "path": "$.login",
        "expected": "octocat",
        "actual": "octocat",
        "result": "pass"
      }
    ]
  }
]
EOF

# Test test results analysis
echo "Testing test results analysis..."
./target/debug/qitops analyze --results offline_test_results.json --output offline_test_analysis.md

# Verify the output file exists
if [ ! -f offline_test_analysis.md ]; then
    echo "ERROR: Failed to generate test analysis"
    exit 1
fi

echo "Test analysis generated successfully"

# Test improvement suggestions
echo "Testing improvement suggestions..."
./target/debug/qitops improve --results offline_test_results.json --output offline_test_improvements.md

# Verify the output file exists
if [ ! -f offline_test_improvements.md ]; then
    echo "ERROR: Failed to generate improvement suggestions"
    exit 1
fi

echo "Improvement suggestions generated successfully"

# Clean up
echo "Cleaning up..."
rm offline_api_test.json offline_perf_test.json offline_security_test.json offline_web_test.json offline_test_results.json offline_test_analysis.md offline_test_improvements.md

echo "All tests passed successfully!"
exit 0
