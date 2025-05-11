# Test Results Analysis

QitOps can analyze your test results using AI to identify patterns, issues, and insights that might not be immediately obvious. This feature helps you understand test outcomes and make data-driven decisions about your testing strategy.

## How It Works

1. You provide a JSON file containing test results
2. QitOps processes these results using a local LLM
3. The LLM generates a comprehensive analysis report
4. The analysis is saved to the specified output file (typically in Markdown format)

## Basic Usage

```bash
# Analyze test results
qitops analyze --results test_results.json --output analysis.md

# Analyze with a specific model
qitops analyze --results test_results.json --output analysis.md --model llama --model-path /path/to/model.gguf
```

## Command-Line Options

| Option | Description |
|--------|-------------|
| `--results <FILE>` | JSON file containing test results |
| `--output <FILE>` | Output file for the analysis report |
| `--model <MODEL>` | LLM model to use (default: auto) |
| `--model-path <PATH>` | Path to the model file |
| `--format <FORMAT>` | Output format (markdown, html, json) |
| `--temperature <FLOAT>` | Temperature for generation (0.0-1.0) |
| `--context-size <INT>` | Context size in tokens |
| `--max-tokens <INT>` | Maximum tokens for generation |
| `--system-prompt <TEXT>` | Custom system prompt for the model |

## Analysis Content

The generated analysis report typically includes:

### 1. Executive Summary
- Overall success rate
- Number of tests run
- Key findings and insights
- Critical issues identified

### 2. Test Results Overview
- Distribution of test statuses (success, failure, error)
- Average response times
- Success rates by test type
- Temporal patterns (if timestamps are available)

### 3. Detailed Breakdown
- Analysis of each failed test
- Common failure patterns
- Root cause analysis
- Error categorization

### 4. Performance Analysis
- Response time statistics
- Performance bottlenecks
- Throughput analysis
- Resource utilization patterns

### 5. Recommendations
- Suggestions for fixing failed tests
- Performance optimization opportunities
- Test coverage improvements
- Testing strategy recommendations

## Example Analysis Report

```markdown
# Test Results Analysis

## Executive Summary

Analyzed 25 test results with an overall success rate of 84% (21/25 tests passed).
The average response time was 187ms, with 3 tests exceeding the 500ms threshold.
The main issues were related to authentication failures and timeout errors.

## Test Results Overview

| Status | Count | Percentage |
|--------|-------|------------|
| Success | 21 | 84% |
| Failure | 3 | 12% |
| Error | 1 | 4% |

### Success Rate by Test Type
- API Tests: 90% (18/20)
- Performance Tests: 75% (3/4)
- Security Tests: 0% (0/1)

## Detailed Breakdown

### Failed Tests Analysis

1. **API-Test-003**: Authentication failure
   - Error: "Unauthorized (401)"
   - Possible cause: Expired API token or incorrect credentials
   - Recommendation: Verify and update the authentication token

2. **API-Test-012**: Unexpected response format
   - Error: "Expected field 'user_id' not found in response"
   - Possible cause: API response structure has changed
   - Recommendation: Update the expected response structure in the test configuration

3. **Perf-Test-002**: Timeout error
   - Error: "Request timed out after 30 seconds"
   - Possible cause: Server under heavy load or network issues
   - Recommendation: Increase the timeout threshold or investigate server performance

## Performance Analysis

- 90th percentile response time: 342ms
- Slowest endpoint: /api/users/search (avg: 412ms)
- Fastest endpoint: /api/health (avg: 42ms)
- 3 tests exceeded the 500ms threshold

## Recommendations

1. **Fix Authentication Issues**
   - Implement token refresh mechanism
   - Add pre-test validation of authentication credentials

2. **Improve Error Handling**
   - Add more specific assertions for response structure
   - Implement retry logic for transient failures

3. **Performance Optimization**
   - Review the /api/users/search endpoint for optimization opportunities
   - Consider caching frequently accessed resources
   - Implement pagination for large result sets

4. **Test Coverage Improvements**
   - Add more security tests (currently only 1)
   - Increase test coverage for error conditions
   - Add tests for edge cases identified in this analysis
```

## Input Format Requirements

The analysis feature expects test results in a specific JSON format:

```json
[
  {
    "test_id": "api-test-001",
    "name": "User API Test",
    "description": "Test the user API endpoint",
    "timestamp": "2025-05-10T12:34:56Z",
    "duration_ms": 156,
    "status": "success",
    "url": "https://api.example.com/users/123",
    "method": "GET",
    "request_headers": {
      "Accept": "application/json",
      "Authorization": "Bearer token123"
    },
    "response_status": 200,
    "response_headers": {
      "content-type": "application/json"
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
        "path": "$.id",
        "expected": 123,
        "actual": 123,
        "result": "pass"
      }
    ]
  },
  {
    "test_id": "api-test-002",
    "name": "Invalid User API Test",
    "description": "Test the user API with invalid ID",
    "timestamp": "2025-05-10T12:35:12Z",
    "duration_ms": 134,
    "status": "failure",
    "url": "https://api.example.com/users/999",
    "method": "GET",
    "request_headers": {
      "Accept": "application/json",
      "Authorization": "Bearer token123"
    },
    "response_status": 404,
    "response_headers": {
      "content-type": "application/json"
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
```

## Best Practices

- Include as much detail as possible in your test results
- Run analysis after each test suite execution
- Compare analyses over time to identify trends
- Use the analysis to prioritize test improvements
- Share analysis reports with your team for collaborative debugging
- Customize the analysis by adjusting the temperature parameter
- Use lower temperatures (0.3-0.5) for more factual, concise analyses
- Use higher temperatures (0.7-0.9) for more creative, detailed analyses
