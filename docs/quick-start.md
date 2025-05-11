# Quick Start

This guide will help you get started with QitOps quickly. You'll learn how to run your first tests and understand the basic workflow.

## Prerequisites

Before you begin, make sure you have:

- QitOps installed (see [Installation](installation.md))
- A target API or web application to test
- Basic understanding of HTTP requests and responses

## Your First API Test

Let's start with a simple API test to verify that a public API is working correctly.

### 1. Create a Test Configuration

Create a file named `github_api_test.json` with the following content:

```json
{
  "name": "GitHub API Test",
  "description": "Test the GitHub API to fetch user information",
  "url": "https://api.github.com/users/octocat",
  "method": "GET",
  "headers": {
    "Accept": "application/vnd.github.v3+json",
    "User-Agent": "QitOps-Test"
  },
  "expected_status": 200,
  "expected_body": {
    "login": "octocat",
    "type": "User"
  }
}
```

This configuration defines a test that:
- Makes a GET request to the GitHub API
- Expects a 200 status code
- Validates that the response contains specific fields

### 2. Run the Test

Run the test using the QitOps CLI:

```bash
qitops api -c github_api_test.json
```

You should see output similar to:

```
Running API test: GitHub API Test
✓ Status code is 200
✓ Response body contains expected fields
✓ Test completed successfully in 0.324s
```

### 3. Generate a Report

Generate a detailed report of the test results:

```bash
qitops api -c github_api_test.json -r html -o github_test_report.html
```

This command runs the test and generates an HTML report with detailed information about the request, response, and validation results.

## Testing an API Collection

Now let's create a collection of related API tests.

### 1. Create a Collection Configuration

Create a file named `github_api_collection.json` with the following content:

```json
{
  "name": "GitHub API Collection",
  "description": "A collection of GitHub API tests",
  "variables": {
    "base_url": "https://api.github.com",
    "username": "octocat"
  },
  "defaults": {
    "headers": {
      "Accept": "application/vnd.github.v3+json",
      "User-Agent": "QitOps-Test"
    }
  },
  "requests": [
    {
      "name": "Get User",
      "id": "get-user",
      "url": "{{base_url}}/users/{{username}}",
      "method": "GET",
      "expected_status": 200,
      "capture": {
        "repos_url": "$.repos_url"
      }
    },
    {
      "name": "Get User Repos",
      "id": "get-repos",
      "url": "{{repos_url}}",
      "method": "GET",
      "depends_on": ["get-user"],
      "expected_status": 200
    }
  ]
}
```

This collection defines:
- Variables that can be reused across requests
- Default headers for all requests
- Two related requests, where the second depends on the first
- Data capture from the first request to use in the second

### 2. Run the Collection

Run the collection using the QitOps CLI:

```bash
qitops collection -c github_api_collection.json
```

You should see output showing the execution of each request in the collection.

## Running a Performance Test

Let's run a simple performance test to measure the response time of an API.

### 1. Create a Performance Test Configuration

Create a file named `performance_test.json` with the following content:

```json
{
  "name": "GitHub API Performance Test",
  "description": "Test the performance of the GitHub API",
  "target_url": "https://api.github.com/users/octocat",
  "method": "GET",
  "headers": {
    "Accept": "application/vnd.github.v3+json",
    "User-Agent": "QitOps-Test"
  },
  "concurrent_users": 10,
  "duration_secs": 30,
  "ramp_up_time_secs": 5,
  "success_threshold": 95.0,
  "max_response_time_ms": 500
}
```

This configuration defines a performance test that:
- Simulates 10 concurrent users
- Runs for 30 seconds
- Gradually ramps up to full load over 5 seconds
- Expects a 95% success rate
- Expects a maximum response time of 500ms

### 2. Run the Performance Test

Run the performance test using the QitOps CLI:

```bash
qitops performance -c performance_test.json
```

You should see output showing real-time metrics during the test and a summary at the end.

## Using AI Features

QitOps includes AI features that can help you create and improve tests.

### 1. Generate a Test Configuration

Generate a test configuration from a natural language description:

```bash
qitops generate --test-type api --description "Test the GitHub API to fetch user information for 'octocat' and verify the response contains the correct login name and user type" --output generated_test.json
```

This command uses AI to generate a test configuration based on your description.

### 2. Analyze Test Results

Run a test and analyze the results:

```bash
# Run a test and save the results
qitops api -c github_api_test.json -r json -o test_results.json

# Analyze the results
qitops analyze --results test_results.json --output analysis.md
```

The analysis will provide insights into the test results and identify any patterns or issues.

### 3. Get Improvement Suggestions

Get suggestions for improving your tests:

```bash
qitops improve --results test_results.json --output improvements.md
```

The suggestions will help you enhance your tests for better coverage and reliability.

## Next Steps

Now that you've run your first tests with QitOps, you can explore more advanced features:

- [API Testing](api-testing.md): Learn more about API testing capabilities
- [API Collections](api-collections.md): Create complex API test workflows
- [Performance Testing](performance-testing.md): Measure and validate system performance
- [Security Testing](security-testing.md): Identify vulnerabilities and ensure compliance
- [Web Testing](web-testing.md): Automate browser interactions and validate web applications
- [Data-Driven Testing](data-driven-testing.md): Run tests with multiple data sets
- [AI Features](ai-features.md): Leverage AI to enhance your testing workflow

For a complete reference of all QitOps commands and options, see the [Usage Guide](usage.md).
