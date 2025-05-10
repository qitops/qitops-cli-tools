#!/bin/bash

# QitOps AI Features Test Script
# This script tests the AI features of QitOps using the mock implementation

# Set up colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}QitOps AI Features Test Script${NC}"
echo -e "${YELLOW}This script will test the AI features of QitOps using the mock implementation${NC}"
echo

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo is not installed. Please install Rust and Cargo first.${NC}"
    exit 1
fi

# Build QitOps with AI features
echo -e "${YELLOW}Building QitOps with AI features...${NC}"
cargo build --features ai
if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Failed to build QitOps with AI features.${NC}"
    exit 1
fi
echo -e "${GREEN}Build successful!${NC}"
echo

# Create a directory for test outputs
echo -e "${YELLOW}Creating directory for test outputs...${NC}"
mkdir -p ai_test_outputs
cd ai_test_outputs
echo -e "${GREEN}Directory created!${NC}"
echo

# Test 1: Generate API Test Configuration
echo -e "${BLUE}Test 1: Generate API Test Configuration${NC}"
echo -e "${YELLOW}Running: cargo run --features ai -- generate --test-type api --description \"Test the Twitter API to fetch user timeline\" --output twitter_api_test.json${NC}"
../target/debug/qitops generate --test-type api --description "Test the Twitter API to fetch user timeline" --output twitter_api_test.json
if [ $? -ne 0 ] || [ ! -f twitter_api_test.json ]; then
    echo -e "${RED}Error: Failed to generate API test configuration.${NC}"
    exit 1
fi
echo -e "${GREEN}API test configuration generated successfully!${NC}"
echo -e "${YELLOW}Generated file: twitter_api_test.json${NC}"
echo

# Test 2: Generate Performance Test Configuration
echo -e "${BLUE}Test 2: Generate Performance Test Configuration${NC}"
echo -e "${YELLOW}Running: cargo run --features ai -- generate --test-type performance --description \"Load test for an e-commerce checkout API with 100 concurrent users\" --output ecommerce_perf_test.json${NC}"
../target/debug/qitops generate --test-type performance --description "Load test for an e-commerce checkout API with 100 concurrent users" --output ecommerce_perf_test.json
if [ $? -ne 0 ] || [ ! -f ecommerce_perf_test.json ]; then
    echo -e "${RED}Error: Failed to generate performance test configuration.${NC}"
    exit 1
fi
echo -e "${GREEN}Performance test configuration generated successfully!${NC}"
echo -e "${YELLOW}Generated file: ecommerce_perf_test.json${NC}"
echo

# Create a sample test result file
echo -e "${BLUE}Creating sample test results file for analysis...${NC}"
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
      }
    ]
  },
  {
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
  }
]
EOF
echo -e "${GREEN}Sample test results file created!${NC}"
echo

# Test 3: Analyze Test Results
echo -e "${BLUE}Test 3: Analyze Test Results${NC}"
echo -e "${YELLOW}Running: cargo run --features ai -- analyze --results sample_test_results.json --output test_analysis.md${NC}"
../target/debug/qitops analyze --results sample_test_results.json --output test_analysis.md
if [ $? -ne 0 ] || [ ! -f test_analysis.md ]; then
    echo -e "${RED}Error: Failed to analyze test results.${NC}"
    exit 1
fi
echo -e "${GREEN}Test results analysis completed successfully!${NC}"
echo -e "${YELLOW}Generated file: test_analysis.md${NC}"
echo

# Test 4: Generate Improvement Suggestions
echo -e "${BLUE}Test 4: Generate Improvement Suggestions${NC}"
echo -e "${YELLOW}Running: cargo run --features ai -- improve --results sample_test_results.json --output test_improvements.md${NC}"
../target/debug/qitops improve --results sample_test_results.json --output test_improvements.md
if [ $? -ne 0 ] || [ ! -f test_improvements.md ]; then
    echo -e "${RED}Error: Failed to generate improvement suggestions.${NC}"
    exit 1
fi
echo -e "${GREEN}Improvement suggestions generated successfully!${NC}"
echo -e "${YELLOW}Generated file: test_improvements.md${NC}"
echo

# Summary
echo -e "${BLUE}All tests completed successfully!${NC}"
echo -e "${YELLOW}Generated files:${NC}"
ls -la
echo

# Return to the original directory
cd ..

echo -e "${GREEN}AI features are working correctly!${NC}"
echo -e "${YELLOW}To test with a real local LLM, see the documentation at docs/testing-ai-features.md${NC}"
