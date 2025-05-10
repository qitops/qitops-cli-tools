#!/bin/bash

# Test script for QitOps AI features with local LLM
echo "Testing QitOps AI features with local LLM..."

# Set up environment variables for offline mode
export QITOPS_OFFLINE=true
export QITOPS_MODEL_PATH="/path/to/local/model.gguf"

# Create a directory for test outputs
mkdir -p test_outputs
cd test_outputs

echo "1. Testing API test generation..."
../target/debug/qitops generate --test-type api --description "Test the Twitter API to fetch user timeline" --output twitter_api_test.json --model custom --model-path "$QITOPS_MODEL_PATH"

echo "2. Testing performance test generation..."
../target/debug/qitops generate --test-type performance --description "Load test for an e-commerce checkout API with 100 concurrent users" --output ecommerce_perf_test.json --model custom --model-path "$QITOPS_MODEL_PATH"

echo "3. Testing security test generation..."
../target/debug/qitops generate --test-type security --description "Security scan for a banking API" --output banking_security_test.json --model custom --model-path "$QITOPS_MODEL_PATH"

echo "4. Testing web test generation..."
../target/debug/qitops generate --test-type web --description "Test the checkout flow of an e-commerce website" --output ecommerce_web_test.json --model custom --model-path "$QITOPS_MODEL_PATH"

# Create a sample test result file
cat > sample_test_results.json << EOF
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
      },
      {
        "type": "json",
        "path": "$.type",
        "expected": "User",
        "actual": "User",
        "result": "pass"
      }
    ]
  },
  {
    "test_id": "api-test-2",
    "name": "GitHub Repos API Test",
    "description": "Test the GitHub API to fetch repositories",
    "timestamp": "2025-05-10T21:15:01Z",
    "duration_ms": 250,
    "status": "success",
    "url": "https://api.github.com/users/octocat/repos",
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
        "path": "$[0].owner.login",
        "expected": "octocat",
        "actual": "octocat",
        "result": "pass"
      }
    ]
  },
  {
    "test_id": "api-test-3",
    "name": "GitHub Non-existent User Test",
    "description": "Test the GitHub API with a non-existent user",
    "timestamp": "2025-05-10T21:15:02Z",
    "duration_ms": 180,
    "status": "failure",
    "url": "https://api.github.com/users/non-existent-user-12345",
    "method": "GET",
    "request_headers": {
      "Accept": "application/vnd.github.v3+json",
      "User-Agent": "QitOps-Test"
    },
    "response_status": 404,
    "response_headers": {
      "content-type": "application/json; charset=utf-8",
      "cache-control": "public, max-age=60, s-maxage=60"
    },
    "assertions": [
      {
        "type": "status",
        "expected": 200,
        "actual": 404,
        "result": "fail"
      }
    ],
    "error": "Expected status 200 but got 404"
  }
]
EOF

echo "5. Testing test results analysis..."
../target/debug/qitops analyze --results sample_test_results.json --output test_analysis.md --model custom --model-path "$QITOPS_MODEL_PATH"

echo "6. Testing improvement suggestions..."
../target/debug/qitops improve --results sample_test_results.json --output test_improvements.md --model custom --model-path "$QITOPS_MODEL_PATH"

echo "All tests completed. Results are in the test_outputs directory."
echo "Generated files:"
ls -la

cd ..
