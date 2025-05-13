#!/bin/bash

# Test script for QitOps AI features with Ollama (direct approach)

echo "QitOps AI Features Test with Ollama (Direct Approach)"
echo "This script will test the AI features of QitOps using the Ollama model"
echo

# Check if Ollama is running
if ! curl -s http://localhost:11434/api/version > /dev/null; then
    echo "Error: Ollama is not running. Please start Ollama with 'ollama serve'"
    exit 1
fi

# Check if phi model is available
if ! ollama list | grep -q "phi"; then
    echo "Error: phi model is not available. Please pull it with 'ollama pull phi'"
    exit 1
fi

# Create directory for test outputs
echo "Creating directory for test outputs..."
mkdir -p ollama_direct_outputs
cd ollama_direct_outputs
echo "Directory created!"
echo

# Test 1: Generate API Test Configuration
echo "Test 1: Generate API Test Configuration"
echo "Running: cargo run --features ai -- generate --test-type api --description \"Test the Twitter API to fetch user timeline\" --model custom --model-path ollama:phi --output twitter_api_test_ollama.json"
cargo run --features ai -- generate --test-type api --description "Test the Twitter API to fetch user timeline" --model custom --model-path ollama:phi --output twitter_api_test_ollama.json
if [ $? -ne 0 ]; then
    echo "Error: Failed to generate API test configuration"
    exit 1
fi
echo "API test configuration generated successfully!"
echo "Generated file: twitter_api_test_ollama.json"
echo "Content:"
cat twitter_api_test_ollama.json
echo

# Test 2: Generate Performance Test Configuration
echo "Test 2: Generate Performance Test Configuration"
echo "Running: cargo run --features ai -- generate --test-type performance --description \"Load test for an e-commerce checkout API with 100 concurrent users\" --model custom --model-path ollama:phi --output ecommerce_perf_test_ollama.json"
cargo run --features ai -- generate --test-type performance --description "Load test for an e-commerce checkout API with 100 concurrent users" --model custom --model-path ollama:phi --output ecommerce_perf_test_ollama.json
if [ $? -ne 0 ]; then
    echo "Error: Failed to generate performance test configuration"
    exit 1
fi
echo "Performance test configuration generated successfully!"
echo "Generated file: ecommerce_perf_test_ollama.json"
echo "Content:"
cat ecommerce_perf_test_ollama.json
echo

# Create sample test results file for analysis
echo "Creating sample test results file for analysis..."
cat > sample_test_results_ollama.json << EOF
[
  {
    "name": "GitHub User API Test",
    "status": "success",
    "duration": 0.19,
    "details": {
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
    },
    "timestamp": "2025-05-10T21:15:00Z"
  },
  {
    "name": "GitHub Non-existent User Test",
    "status": "failure",
    "duration": 0.18,
    "details": {
      "test_id": "api-test-2",
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
    },
    "timestamp": "2025-05-10T21:15:02Z"
  }
]
EOF
echo "Sample test results file created!"
echo

# Test 3: Analyze Test Results
echo "Test 3: Analyze Test Results"
echo "Running: cargo run --features ai -- analyze --results sample_test_results_ollama.json --model custom --model-path ollama:phi --output test_analysis_ollama.md"
cargo run --features ai -- analyze --results sample_test_results_ollama.json --model custom --model-path ollama:phi --output test_analysis_ollama.md
if [ $? -ne 0 ]; then
    echo "Error: Failed to analyze test results"
    exit 1
fi
echo "Test results analysis completed successfully!"
echo "Generated file: test_analysis_ollama.md"
echo "Content:"
cat test_analysis_ollama.md
echo

# Test 4: Generate Improvement Suggestions
echo "Test 4: Generate Improvement Suggestions"
echo "Running: cargo run --features ai -- improve --results sample_test_results_ollama.json --model custom --model-path ollama:phi --output test_improvements_ollama.md"
cargo run --features ai -- improve --results sample_test_results_ollama.json --model custom --model-path ollama:phi --output test_improvements_ollama.md
if [ $? -ne 0 ]; then
    echo "Error: Failed to generate improvement suggestions"
    exit 1
fi
echo "Improvement suggestions generated successfully!"
echo "Generated file: test_improvements_ollama.md"
echo "Content:"
cat test_improvements_ollama.md
echo

echo "All tests completed successfully!"
echo "Generated files:"
ls -la
echo

echo "AI features are working correctly with Ollama!"
