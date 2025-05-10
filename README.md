# QitOps CLI Tools

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/qitops/qitops-cli-tools/actions/workflows/ci.yml/badge.svg)](https://github.com/qitops/qitops-cli-tools/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/qitops)](https://crates.io/crates/qitops)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)
[![Documentation](https://img.shields.io/badge/docs-latest-brightgreen.svg)](https://qitops.github.io/qitops-cli-tools/index.html)

QitOps is a comprehensive Software Quality Assurance CLI tool for API, Performance, Security, and Web Testing. It provides a unified command-line interface with minimal dependencies and maximum flexibility.

## Quick Start

```bash
# Install QitOps
cargo install --path .

# Run a basic API test
qitops api -c tests/configs/api_test.json

# Run an API collection (multiple requests with dependencies)
qitops collection -c tests/configs/api_collection.json

# Run a performance test
qitops performance -c tests/configs/performance_test.json -u 10 -d 30

# Run a security scan
qitops security -c tests/configs/security_test.json -d 3

# Run a web test
qitops web -c tests/configs/web_test.json

# Generate a report in HTML format
qitops -r html -o report.html api -c tests/configs/api_test.json

# Run in CI mode (reduced output, exit code based on test results)
qitops --ci-mode -r json -o results.json api -c tests/configs/api_test.json

# Run data-driven tests with CSV data
qitops data-driven -c tests/configs/data_driven_api_test.json -d tests/data/users.csv -t csv

# Run data-driven tests with JSON data
qitops data-driven -c tests/configs/data_driven_collection.json -d tests/data/products.json -t json
```

## Features

### API Testing
- HTTP method support (GET, POST, PUT, DELETE, etc.)
- URL configuration with environment-specific settings
- Custom headers and request body support
- Response validation (status codes, body, headers)
- Response time monitoring
- Configurable timeouts and retries
- Retry mechanism with exponential backoff and jitter
  - Configurable retry attempts
  - Customizable retry delay
  - Status code-based retry conditions
  - Connection error handling
  - Timeout handling

### API Collections
- Group related API requests in a single configuration
- Define dependencies between requests
- Capture and use data from previous responses using JSONPath
- Variable interpolation with {{variable}} syntax
- Environment variables and environment-specific configurations
- Sequential request execution with dependency management
- Shared authentication across requests (Basic, Bearer, API Key)
- Default request configuration (headers, timeout, retries)
- Detailed collection reporting with captured variables
- Request chaining for complex workflows

### Performance Testing
- Load testing with configurable concurrent users
- Response time analysis
- Success rate monitoring
- Ramp-up time configuration
- Detailed performance metrics
  - Average response time
  - Minimum response time
  - Maximum response time
  - Total requests
  - Success/error counts
  - Success rate percentage

### Enhanced Performance Testing
- Multiple load profiles (constant, ramping, spike)
- Multi-stage test execution
- Multiple scenarios in a single test
- Weighted scenario distribution
- Detailed metrics collection and reporting
- Percentile calculations (p50, p90, p95, p99)
- Custom thresholds with pass/fail criteria
- Real-time metrics streaming
- Tagged metrics for detailed analysis
- Scenario-based reporting

### CI/CD Integration
- GitHub Actions workflow templates
- CI mode with reduced output
- Exit codes based on test results
- JSON report generation for CI pipelines
- Parallel test execution
- Environment-specific configurations
- Scheduled test runs
- Artifact storage for test results and reports

### Data-Driven Testing
- Parameterize tests with CSV and JSON datasets
- Variable interpolation with {{placeholder}} syntax
- Support for multiple iterations of the same test
- Configurable iteration limits
- Stop-on-failure option
- Detailed iteration reporting
- Support for all test types (API, Performance, Security, Web)
- CSV header row support
- JSON path extraction
- Inline data definition
- Customizable success thresholds

### Security Testing
- Comprehensive security scanning
- Multiple scan types (headers, SSL, vulnerabilities, sensitive data)
- Severity-based reporting
- Authentication testing
- Common vulnerability checks
- Security header validation
- CSRF and XSS detection
- SQL injection testing
- JWT security analysis
- Access control verification

### Web Testing
- Headless browser automation
- Viewport configuration
- Screenshot capture
- Element assertions
- Text content validation
- URL and title validation
- Action simulation (click, type, wait, navigate)
- Custom user agent configuration

### AI-Powered Features
- Test configuration generation from natural language descriptions
- Test results analysis with insights and patterns
- Improvement suggestions based on test results
- Support for local LLM models (LLaMA, Mistral, GPT-J, Phi)
- Customizable model parameters (temperature, context size, etc.)
- Offline operation with no data sent to external services

## Installation

### From crates.io (Recommended)

```bash
# Install directly from crates.io
cargo install qitops

# Run QitOps
qitops --help
```

### From GitHub Releases

1. Download the latest binary for your platform from the [GitHub Releases page](https://github.com/qitops/qitops-cli-tools/releases)
2. Make the file executable (Linux/macOS): `chmod +x qitops-*`
3. Move it to a directory in your PATH:
   - Linux/macOS: `sudo mv qitops-* /usr/local/bin/qitops`
   - Windows: Add the directory containing the executable to your PATH

### Using Docker

```bash
# Pull the Docker image
docker pull qitops/qitops:latest

# Run QitOps
docker run --rm qitops/qitops:latest --help

# Run with mounted volumes for configs and results
docker run --rm -v $(pwd)/configs:/workspace/configs -v $(pwd)/results:/workspace/results qitops/qitops:latest api -c /workspace/configs/api_test.json
```

### From Source

```bash
# Clone the repository
git clone https://github.com/qitops/qitops-cli-tools.git
cd qitops-cli-tools

# Build the project
cargo build --release

# Install the binary (optional)
cargo install --path .
```

### With AI Features (Optional)

```bash
# Install with AI features enabled
cargo install qitops --features ai
```

## Configuration

### API Test Configuration
```json
{
    "name": "Example API Test",
    "description": "Test description",
    "timeout": 30,
    "retries": 3,
    "environment": "production",
    "url": "https://api.example.com",
    "method": "GET",
    "headers": {
        "Accept": "application/json",
        "User-Agent": "QitOps-Test"
    },
    "expected_status": 200,
    "expected_body": {
        "field1": "value1",
        "field2": "value2"
    },
    "max_response_time": 2,
    "expected_headers": {
        "content-type": "application/json",
        "cache-control": "no-cache"
    },
    "retry": {
        "max_retries": 3,
        "initial_delay_ms": 100,
        "max_delay_ms": 1000,
        "retry_status_codes": [408, 429, 500, 502, 503, 504],
        "retry_on_timeout": true,
        "retry_on_connection_error": true
    }
}
```

### API Collection Configuration
```json
{
    "name": "GitHub API Collection",
    "description": "A collection of GitHub API tests",
    "version": "1.0.0",
    "variables": {
        "base_url": "https://api.github.com",
        "username": "octocat",
        "repo": "Hello-World"
    },
    "auth": {
        "type": "bearer",
        "token": "{{GITHUB_TOKEN}}"
    },
    "defaults": {
        "headers": {
            "Accept": "application/vnd.github.v3+json",
            "User-Agent": "QitOps-Test"
        },
        "timeout": 30,
        "retries": 3
    },
    "requests": [
        {
            "name": "Get User",
            "description": "Get a GitHub user",
            "id": "get-user",
            "url": "{{base_url}}/users/{{username}}",
            "method": "GET",
            "expected_status": 200,
            "expected_body": {
                "login": "{{username}}",
                "type": "User"
            },
            "capture": {
                "user_id": "$.id",
                "user_url": "$.url"
            }
        },
        {
            "name": "Get User Repos",
            "description": "Get repositories for a user",
            "id": "get-user-repos",
            "url": "{{user_url}}/repos",
            "method": "GET",
            "depends_on": ["get-user"],
            "expected_status": 200
        }
    ],
    "environments": {
        "production": {
            "base_url": "https://api.github.com"
        },
        "staging": {
            "base_url": "https://api.staging.github.com"
        }
    },
    "run_options": {
        "sequential": true,
        "stop_on_failure": true,
        "delay_between_requests_ms": 500
    }
}
```

### Performance Test Configuration
```json
{
    "name": "Sample Performance Test",
    "description": "Load testing a public API endpoint",
    "timeout": 30,
    "retries": 3,
    "environment": "production",
    "target_url": "https://api.example.com/endpoint",
    "method": "GET",
    "headers": {
        "Accept": "application/json"
    },
    "success_threshold": 95.0,
    "ramp_up_time_secs": 5
}
```

### Enhanced Performance Test Configuration
```json
{
    "name": "Enhanced Performance Test",
    "description": "Testing API performance with multiple scenarios and load profiles",
    "timeout": 60,
    "retries": 0,
    "environment": "production",
    "load_profile": {
        "type": "ramping_vus",
        "initial": 1,
        "stages": [
            {
                "duration_secs": 30,
                "target": 10
            },
            {
                "duration_secs": 60,
                "target": 20
            },
            {
                "duration_secs": 30,
                "target": 0
            }
        ]
    },
    "scenarios": [
        {
            "name": "Get Request",
            "target_url": "https://httpbin.org/get",
            "method": "GET",
            "headers": {
                "Accept": "application/json",
                "User-Agent": "QitOps-Test/1.0"
            },
            "weight": 3,
            "tags": {
                "endpoint": "get",
                "category": "read"
            }
        },
        {
            "name": "Post Request",
            "target_url": "https://httpbin.org/post",
            "method": "POST",
            "headers": {
                "Content-Type": "application/json",
                "Accept": "application/json"
            },
            "body": {
                "test": "data",
                "number": 123
            },
            "weight": 1,
            "tags": {
                "endpoint": "post",
                "category": "write"
            }
        }
    ],
    "thresholds": [
        {
            "metric": "response_time.avg",
            "expression": "< 0.5",
            "abort_on_fail": false
        },
        {
            "metric": "response_time.p95",
            "expression": "< 1.0",
            "abort_on_fail": false
        },
        {
            "metric": "success.avg",
            "expression": "> 0.95",
            "abort_on_fail": true
        }
    ],
    "success_threshold": 95.0,
    "stream_metrics": true,
    "metrics_interval_secs": 5
}
```

### Security Test Configuration
```json
{
    "name": "Security Scan",
    "description": "Comprehensive security scan of the API",
    "timeout": 30,
    "retries": 3,
    "environment": "production",
    "target_url": "https://api.example.com",
    "headers": {
        "Accept": "application/json"
    },
    "auth": {
        "type": "bearer",
        "token": "your-token"
    },
    "scan_types": [
        "headers",
        "ssl",
        "vulnerabilities",
        "sensitive-data"
    ],
    "max_high_severity_findings": 0
}
```

### Web Test Configuration
```json
{
    "name": "Sample Web Test",
    "description": "Testing a public website",
    "timeout": 30,
    "retries": 3,
    "environment": "production",
    "target_url": "https://example.com",
    "viewport": {
        "width": 1280,
        "height": 800,
        "device_scale_factor": 1.0,
        "is_mobile": false
    },
    "wait_for_selector": "body",
    "wait_timeout_secs": 10,
    "screenshots": true,
    "user_agent": "QitOps-WebTester/1.0",
    "assertions": [
        {
            "assertion_type": "title",
            "expected_value": "Example Domain",
            "comparison": "contains"
        },
        {
            "assertion_type": "element",
            "selector": "h1",
            "expected_value": "true"
        }
    ],
    "actions": [
        {
            "action_type": "wait",
            "wait_time_ms": 1000
        },
        {
            "action_type": "click",
            "selector": "a"
        }
    ]
}
```

### AI Configuration
```json
{
    "model_type": "llama",
    "model_path": "/usr/local/share/models/llama-2-7b-chat.gguf",
    "context_size": 4096,
    "temperature": 0.7,
    "max_tokens": 2048,
    "system_prompt": "You are an AI assistant specialized in software testing. Your task is to help generate test configurations, analyze test results, and suggest improvements."
}
```

## Usage

### API Testing
```bash
# Run a single API test
qitops api -c tests/configs/api_test.json

# Run tests in a specific environment
qitops api -c tests/configs/api_test.json -e production
```

Example output:
```
Test Results:
Name: Sample API Test
Status: passed
Duration: 0.90s
Details: {
  "headers": {
    "content-type": "application/json",
    "cache-control": "no-cache",
    ...
  },
  "response_time": 0.903123787,
  "status_code": 200
}
Timestamp: 2025-05-09T21:06:33.923402733+00:00
```

### API Collections
```bash
# Run an API collection
qitops collection -c tests/configs/api_collection.json

# Run in a specific environment
qitops collection -c tests/configs/api_collection.json -e staging

# Output in JSON format
qitops collection -c tests/configs/api_collection.json -f json
```

Example output:
```
Collection Results:
Name: HTTPBin API Collection
Status: passed
Duration: 2.35s
Timestamp: 2025-05-09T21:07:05.165804274+00:00

Request Results:
  1. Get IP Address - passed
  2. Post with JSON - passed
  3. Get with Headers - passed
  4. Get with Query Parameters - passed

Captured Variables:
  client_ip: 203.0.113.1
  request_id: 97e5b974-e2c3-4073-b45e-5bf5a7f3f0b2
  posted_data: {"test_value":"qitops_test_value","client_ip":"203.0.113.1"}
```

The API collections feature supports:
- Variable interpolation using `{{variable}}` syntax
- Capturing data from responses using JSONPath expressions
- Request dependencies to ensure proper execution order
- Environment-specific configurations
- Shared authentication and default headers

### Performance Testing
```bash
# Run performance test with default settings
qitops performance -c tests/configs/performance_test.json

# Run with custom concurrent users and duration
qitops performance -c tests/configs/performance_test.json -u 50 -d 120
```

Example output:
```
Performance Test Results:
Name: Sample Performance Test
Status: passed
Duration: 10.04s
Details: {
  "average_response_time": 0.11007347446511631,
  "error_count": 0,
  "max_response_time": 0.521383212,
  "min_response_time": 0.057222978,
  "success_count": 215,
  "success_rate": 100.0,
  "total_requests": 215
}
Timestamp: 2025-05-09T21:06:50.438713024+00:00
```

### Enhanced Performance Testing
```bash
# Run enhanced performance test with load profiles and scenarios
qitops performance-enhanced -c tests/configs/enhanced_performance_test.json

# Run in a specific environment
qitops performance-enhanced -c tests/configs/enhanced_performance_test.json -e staging
```

Example output:
```
Enhanced Performance Test Results:
Name: Enhanced Performance Test
Status: passed
Duration: 120.35s

Metrics Summary:
  Total Requests: 1250
  Success Count: 1245
  Error Count: 5
  Success Rate: 99.60%

Response Time:
  Average: 125.32ms
  Min: 57.89ms
  Max: 521.45ms
  p50: 115.67ms
  p90: 198.34ms
  p95: 245.78ms
  p99: 378.91ms

Scenario Results:
  Get Request: 937/940 requests successful (99.68%)
  Post Request: 308/310 requests successful (99.35%)

Thresholds:
  response_time.avg: < 0.5 - PASSED
  response_time.p95: < 1.0 - PASSED
  success.avg: > 0.95 - PASSED

For full details, use the --report option to generate a JSON report.
Timestamp: 2025-05-09T21:08:15.723654912+00:00
```

The enhanced performance testing feature supports:
- Multiple load profiles (constant, ramping, spike)
- Multi-stage test execution
- Multiple scenarios with weighted distribution
- Custom thresholds with pass/fail criteria
- Real-time metrics streaming
- Detailed metrics with percentiles

### Data-Driven Testing
```bash
# Run data-driven API tests with CSV data
qitops data-driven -c tests/configs/data_driven_api_test.json -d tests/data/users.csv -t csv

# Run with JSON data
qitops data-driven -c tests/configs/data_driven_collection.json -d tests/data/products.json -t json

# Limit the number of iterations
qitops data-driven -c tests/configs/data_driven_api_test.json -d tests/data/users.csv -m 3

# Stop on first failure
qitops data-driven -c tests/configs/data_driven_api_test.json -d tests/data/users.csv -s
```

Example output:
```
Iteration 1 Results:
Name: User API Test for johndoe
Status: passed
Duration: 0.52s
Details: {
  "headers": {
    "content-type": "application/json",
    ...
  },
  "response_time": 0.523456,
  "status_code": 200
}
Timestamp: 2025-05-09T21:07:05.165804274+00:00

Data Row:
  username: johndoe
  email: john.doe@example.com
  user_id: 1001
  role: admin

...

Data-Driven Test Summary:
Total Iterations: 5
Successful: 5
Failed: 0
Success Rate: 100.00%
```

The data-driven testing feature supports:
- Parameterizing tests with CSV and JSON datasets
- Variable interpolation using `{{placeholder}}` syntax
- Running multiple iterations of the same test with different data
- Configurable iteration limits and stop-on-failure options
- Detailed reporting for each iteration

### Security Testing
```bash
# Run security scan with default settings
qitops security -c tests/configs/security_test.json

# Run with custom scan depth and passive scanning
qitops security -c tests/configs/security_test.json -d 4 -p
```

Example output:
```
Security Test Results:
Name: API Security Test
Status: passed
Duration: 0.00s
Details: {
  "findings": [],
  "summary": {
    "critical_findings": 0,
    "high_findings": 0,
    "low_findings": 0,
    "medium_findings": 0,
    "total_findings": 0
  }
}
Timestamp: 2025-05-09T21:07:05.165804274+00:00
```

### Web Testing
```bash
# Run web test with default settings
qitops web -c tests/configs/web_test.json

# Run with headless mode disabled and custom screenshot directory
qitops web -c tests/configs/web_test.json -h false -s ./screenshots
```

Example output:
```
Web Test Results:
Name: Sample Web Test
Status: passed
Duration: 1.25s
Details: {
  "action_results": [
    {
      "duration_ms": 1000,
      "success": true,
      "type": "wait"
    },
    {
      "selector": "a",
      "success": true,
      "type": "click"
    }
  ],
  "assertion_results": [
    {
      "details": "Title: Simulated Page Title",
      "passed": true,
      "type": "title"
    },
    {
      "details": "Element with selector 'h1' exists: true",
      "passed": true,
      "type": "element"
    }
  ],
  "content_length": 1256,
  "page_title": "Simulated Page Title",
  "screenshot": "./screenshots/screenshot_1715284025.png",
  "status_code": 200
}
Timestamp: 2025-05-09T21:07:05.165804274+00:00
```

### Output Formats
QitOps provides both human-readable and machine-readable output formats to support both interactive use and CI integration:

```bash
# Default human-readable output to stdout
qitops api -c tests/configs/api_test.json

# Generate machine-readable JSON report for CI integration
qitops -r json -o report.json api -c tests/configs/api_test.json

# Generate XML report (JUnit format for CI integration)
qitops -r xml -o report.xml security -c tests/configs/security_test.json

# Generate HTML report for visual inspection
qitops -r html -o report.html performance -c tests/configs/performance_test.json

# Generate CSV report for data analysis
qitops -r csv -o report.csv web -c tests/configs/web_test.json
```

All outputs include consistent timestamping for audit trails and traceability.

### AI-Powered Features
```bash
# Generate an API test configuration from a description
qitops generate -t api -d "Test the GitHub API to fetch user information" -o tests/configs/github_api_test.json

# Generate a web test configuration from a description
qitops generate -t web -d "Test the login form on example.com with valid credentials" -o tests/configs/login_test.json

# Analyze test results
qitops analyze -r results/api_test_result.json -o analysis.md

# Get improvement suggestions based on test results
qitops improve -r results/performance_test_result.json -o improvements.md

# Use a specific AI model
qitops generate -t security -d "Test for SQL injection vulnerabilities" -o tests/configs/sql_injection_test.json -m mistral

# Use a custom model
qitops analyze -r results/security_test_result.json -o analysis.md -m custom -p /path/to/custom/model.gguf
```

## Command Line Options

### Global Options
- `-r, --report`: Generate report in specified format (json, xml, html, csv)
- `-o, --output`: Output path for the report
- `--ci-mode`: Run in CI mode (reduced output, exit code based on test results)

### API Testing
- `-c, --config`: Path to the test configuration file
- `-e, --environment`: Environment to run tests in (default: "production")

### API Collections
- `-c, --config`: Path to the collection configuration file
- `-e, --environment`: Environment to run tests in (default: "production")
- `-f, --format`: Output format (human, json) (default: "human")

### Performance Testing
- `-c, --config`: Path to the test configuration file
- `-e, --environment`: Environment to run tests in (default: "production")
- `-u, --users`: Number of concurrent users (default: 10)
- `-d, --duration`: Test duration in seconds (default: 60)

### Enhanced Performance Testing
- `-c, --config`: Path to the test configuration file
- `-e, --environment`: Environment to run tests in (default: "production")

### Data-Driven Testing
- `-c, --config`: Path to the test configuration file
- `-d, --data`: Path to the data source file (CSV or JSON)
- `-t, --data-type`: Data source type (csv, json) (default: "csv")
- `-e, --environment`: Environment to run tests in (default: "production")
- `-m, --max-iterations`: Maximum number of iterations to run
- `-s, --stop-on-failure`: Stop on first failure

### Security Testing
- `-c, --config`: Path to the test configuration file
- `-e, --environment`: Environment to run tests in (default: "production")
- `-d, --depth`: Scan depth (1-5, default: 3)
  - Level 1: Basic security checks (headers, SSL)
  - Level 2: Common vulnerabilities
  - Level 3: Authentication and authorization
  - Level 4: Advanced vulnerability scanning
  - Level 5: Comprehensive security audit
- `-p, --passive`: Include passive scanning

### Web Testing
- `-c, --config`: Path to the test configuration file
- `-e, --environment`: Environment to run tests in (default: "production")
- `-h, --headless`: Run in headless mode (default: true)
- `-s, --screenshot_dir`: Directory to save screenshots

### AI Test Generation
- `-t, --test_type`: Type of test to generate (api, performance, security, web)
- `-d, --description`: Description of the test to generate
- `-o, --output`: Output file path for the generated configuration
- `-m, --model`: AI model to use (llama, mistral, gptj, phi, custom)
- `-p, --model_path`: Path to model weights (required for custom models)

### AI Test Analysis
- `-r, --results`: Path to test results file(s)
- `-o, --output`: Output file path for the analysis
- `-m, --model`: AI model to use (llama, mistral, gptj, phi, custom)
- `-p, --model_path`: Path to model weights (required for custom models)

### AI Improvement Suggestions
- `-r, --results`: Path to test results file(s)
- `-o, --output`: Output file path for the suggestions
- `-m, --model`: AI model to use (llama, mistral, gptj, phi, custom)
- `-p, --model_path`: Path to model weights (required for custom models)

## Test Results

### API Test Results
- Test name and status
- Duration
- Response details:
  - Status code
  - Response time
  - Headers
  - Retry attempts (if any)
- Timestamp
- Environment information

### API Collection Results
- Collection name and status
- Overall duration
- Individual request results:
  - Request name
  - Status
  - Duration
  - Response details
- Captured variables
- Timestamp
- Environment information

### Performance Test Results
- Test name and status
- Duration
- Performance metrics:
  - Total requests
  - Success/error counts
  - Success rate
  - Average response time
  - Minimum response time
  - Maximum response time
- Timestamp
- Environment information

### Enhanced Performance Test Results
- Test name and status
- Duration
- Detailed metrics:
  - Total requests
  - Success/error counts
  - Success rate
  - Response time statistics (avg, min, max)
  - Percentile measurements (p50, p90, p95, p99)
- Scenario-specific metrics:
  - Per-scenario success rates
  - Per-scenario request counts
- Threshold evaluations:
  - Metric name
  - Expression
  - Pass/fail status
- Tagged metrics for detailed analysis
- Timestamp
- Environment information

### Security Test Results
- Test name and status
- Duration
- Security findings:
  - Critical findings
  - High severity findings
  - Medium severity findings
  - Low severity findings
  - Total findings
- Detailed findings with:
  - Severity level
  - Category
  - Description
  - Recommendation
- Timestamp
- Environment information

### Web Test Results
- Test name and status
- Duration
- Page information:
  - Page title
  - Status code
  - Content length
- Assertion results:
  - Type (title, url, element, text)
  - Pass/fail status
  - Details
- Action results:
  - Type (click, type, wait, navigate)
  - Success status
  - Details
- Screenshot path (if enabled)
- Timestamp
- Environment information

## Best Practices

### API Testing
- Use environment-specific configurations for different deployment stages
- Set appropriate timeouts based on your API's expected response times
- Configure retry mechanisms for transient failures
- Validate both success and error responses
- Use JSON Schema validation for complex response structures
- Monitor response times to catch performance degradation

### API Collections
- Organize related requests into logical collections
- Use meaningful request IDs for better readability and dependencies
- Leverage JSONPath for precise data extraction from responses
- Use variable capture and reuse for efficient testing
- Define environment-specific variables for different deployment stages
- Use sequential execution for dependent requests
- Set appropriate delays between requests to avoid rate limiting
- Define shared authentication at the collection level
- Use defaults for common configuration across requests
- Structure collections to follow user journeys or business processes
- Use request dependencies to ensure proper execution order
- Keep collections focused on a specific testing goal

### Performance Testing
- Start with a small number of concurrent users and gradually increase
- Use appropriate ramp-up times to avoid overwhelming the system
- Set realistic success thresholds based on your requirements
- Monitor system resources during tests
- Run tests in a controlled environment
- Consider network latency in your test environment

### Enhanced Performance Testing
- Design multi-stage tests to simulate realistic user behavior
- Use different load profiles for different testing scenarios
- Define multiple scenarios to test different API endpoints
- Use weighted scenarios to simulate real-world usage patterns
- Set appropriate thresholds based on SLAs and performance requirements
- Use tags to categorize and analyze metrics
- Monitor real-time metrics during test execution
- Analyze percentile measurements for a better understanding of performance
- Use custom thresholds to catch performance regressions early
- Run tests in different environments to compare performance

### Data-Driven Testing
- Organize test data in CSV or JSON format based on complexity
- Use CSV for simple tabular data with a consistent structure
- Use JSON for complex nested data structures
- Include a header row in CSV files for better readability
- Use meaningful placeholder names that match data column names
- Keep test configurations generic with placeholders
- Use stop-on-failure for dependent test iterations
- Set appropriate max iterations for large datasets
- Validate data files before running tests
- Use environment-specific configurations with data-driven tests
- Combine data-driven testing with API collections for complex workflows

### Security Testing
- Start with passive scanning before active scanning
- Use appropriate scan depth based on your security requirements
- Review and address high-severity findings immediately
- Keep authentication tokens secure
- Run security tests in a controlled environment
- Regularly update security test configurations

### Web Testing
- Use headless mode for CI/CD pipelines
- Configure appropriate viewport sizes for different devices
- Keep assertions focused and specific
- Use explicit waits for dynamic content
- Capture screenshots for visual verification
- Test across different browsers in production
- Organize tests by user journey or feature
- Use custom user agents when needed

### AI-Powered Testing
- Use specific, detailed descriptions when generating tests
- Review and refine AI-generated test configurations before use
- Provide context in your descriptions (e.g., authentication requirements)
- Use lower temperature values (0.1-0.3) for more deterministic outputs
- Use higher temperature values (0.7-0.9) for more creative test scenarios
- Analyze test results regularly to identify patterns and issues
- Combine AI suggestions with human expertise for best results
- Keep model weights updated for best performance

### CI/CD Integration
- Use `--ci-mode` for reduced output and exit codes in CI pipelines
- Generate JSON reports for machine-readable results
- Use GitHub Actions templates for quick setup
- Run different test types in parallel for faster feedback
- Set up scheduled runs for regular testing
- Use environment-specific configurations for different stages
- Store test results as artifacts for historical analysis
- Set appropriate timeouts for CI environments
- Use exit codes to gate deployments
- Integrate with notification systems for test failures

## Configuration Reference

### API Test Configuration
| Field | Type | Description | Default |
|-------|------|-------------|---------|
| name | string | Test name | Required |
| description | string | Test description | Optional |
| timeout | number | Request timeout in seconds | 30 |
| retries | number | Number of retry attempts | 3 |
| environment | string | Environment name | "production" |
| url | string | Target URL | Required |
| method | string | HTTP method | Required |
| headers | object | Request headers | Optional |
| expected_status | number | Expected HTTP status code | Optional |
| expected_body | object | Expected response body | Optional |
| max_response_time | number | Maximum allowed response time in seconds | Optional |
| expected_headers | object | Expected response headers | Optional |
| retry | object | Retry configuration | See below |

#### Retry Configuration
| Field | Type | Description | Default |
|-------|------|-------------|---------|
| max_retries | number | Maximum number of retry attempts | 3 |
| initial_delay_ms | number | Initial delay between retries in milliseconds | 100 |
| max_delay_ms | number | Maximum delay between retries in milliseconds | 1000 |
| retry_status_codes | array | HTTP status codes that trigger retries | [408, 429, 500, 502, 503, 504] |
| retry_on_timeout | boolean | Whether to retry on timeout | true |
| retry_on_connection_error | boolean | Whether to retry on connection errors | true |

### API Collection Configuration
| Field | Type | Description | Default |
|-------|------|-------------|---------|
| name | string | Collection name | Required |
| description | string | Collection description | Optional |
| version | string | Collection version | Optional |
| variables | object | Collection variables | Optional |
| auth | object | Collection authentication | Optional |
| defaults | object | Default request configuration | Optional |
| requests | array | Collection requests | Required |
| environments | object | Environment-specific variables | Optional |
| run_options | object | Run options | Optional |

#### Collection Auth
| Field | Type | Description | Default |
|-------|------|-------------|---------|
| type | string | Authentication type (basic, bearer, api_key) | Required |
| username | string | Username for basic auth | Optional |
| password | string | Password for basic auth | Optional |
| token | string | Token for bearer auth | Optional |
| key_name | string | Key name for API key auth | Optional |
| key_value | string | Key value for API key auth | Optional |

#### Collection Defaults
| Field | Type | Description | Default |
|-------|------|-------------|---------|
| headers | object | Default headers for all requests | Optional |
| timeout | number | Default timeout in seconds | Optional |
| retries | number | Default number of retries | Optional |

#### Collection Request
| Field | Type | Description | Default |
|-------|------|-------------|---------|
| name | string | Request name | Required |
| description | string | Request description | Optional |
| id | string | Request ID (used for dependencies) | Optional |
| url | string | Request URL | Required |
| method | string | HTTP method | Required |
| headers | object | Request headers | Optional |
| body | object | Request body | Optional |
| expected_status | number | Expected HTTP status code | Optional |
| expected_body | object | Expected response body | Optional |
| expected_body_type | string | Expected response body type | Optional |
| depends_on | array | Request dependencies | Optional |
| capture | object | Variables to capture from response | Optional |

### Data-Driven Testing Configuration

#### CSV Data File
```csv
username,email,user_id,role
johndoe,john.doe@example.com,1001,admin
janedoe,jane.doe@example.com,1002,user
bobsmith,bob.smith@example.com,1003,user
```

#### JSON Data File
```json
[
  {
    "product_id": "P001",
    "name": "Smartphone",
    "price": 599.99,
    "category": "Electronics",
    "in_stock": true
  },
  {
    "product_id": "P002",
    "name": "Laptop",
    "price": 1299.99,
    "category": "Electronics",
    "in_stock": true
  }
]
```

#### Test Configuration with Placeholders
```json
{
  "name": "User API Test for {{username}}",
  "description": "Test the user API for {{username}}",
  "url": "https://api.example.com/users/{{user_id}}",
  "method": "GET",
  "headers": {
    "Accept": "application/json",
    "X-User-Email": "{{email}}"
  },
  "expected_status": 200,
  "expected_body": {
    "id": "{{user_id}}",
    "role": "{{role}}"
  }
}
```

#### Run Options
| Field | Type | Description | Default |
|-------|------|-------------|---------|
| sequential | boolean | Run requests sequentially | true |
| stop_on_failure | boolean | Stop on first failure | true |
| delay_between_requests_ms | number | Delay between requests in milliseconds | 0 |

### Performance Test Configuration
| Field | Type | Description | Default |
|-------|------|-------------|---------|
| name | string | Test name | Required |
| description | string | Test description | Optional |
| timeout | number | Request timeout in seconds | 30 |
| retries | number | Number of retry attempts | 3 |
| environment | string | Environment name | "production" |
| target_url | string | Target URL | Required |
| method | string | HTTP method | Required |
| headers | object | Request headers | Optional |
| success_threshold | number | Minimum success rate percentage | 95.0 |
| ramp_up_time_secs | number | Time to ramp up to full load in seconds | 5 |

### Security Test Configuration
| Field | Type | Description | Default |
|-------|------|-------------|---------|
| name | string | Test name | Required |
| description | string | Test description | Optional |
| timeout | number | Request timeout in seconds | 30 |
| retries | number | Number of retry attempts | 3 |
| environment | string | Environment name | "production" |
| target_url | string | Target URL | Required |
| headers | object | Request headers | Optional |
| auth | object | Authentication configuration | Optional |
| scan_types | array | Types of security scans to perform | ["headers", "ssl", "vulnerabilities", "sensitive-data"] |
| max_high_severity_findings | number | Maximum allowed high severity findings | 0 |

### Web Test Configuration
| Field | Type | Description | Default |
|-------|------|-------------|---------|
| name | string | Test name | Required |
| description | string | Test description | Optional |
| timeout | number | Request timeout in seconds | 30 |
| retries | number | Number of retry attempts | 3 |
| environment | string | Environment name | "production" |
| target_url | string | Target URL | Required |
| viewport | object | Browser viewport configuration | Optional |
| wait_for_selector | string | Selector to wait for before starting test | Optional |
| wait_timeout_secs | number | Timeout for waiting for selector | 30 |
| screenshots | boolean | Whether to capture screenshots | false |
| user_agent | string | Custom user agent string | "QitOps-WebTester/1.0" |
| assertions | array | List of assertions to perform | Optional |
| actions | array | List of actions to perform | Optional |

#### Viewport Configuration
| Field | Type | Description | Default |
|-------|------|-------------|---------|
| width | number | Viewport width in pixels | Required |
| height | number | Viewport height in pixels | Required |
| device_scale_factor | number | Device scale factor | 1.0 |
| is_mobile | boolean | Whether to emulate a mobile device | false |

#### Web Assertion
| Field | Type | Description | Default |
|-------|------|-------------|---------|
| assertion_type | string | Type of assertion (title, url, element, text, attribute) | Required |
| selector | string | CSS selector for element assertions | Optional |
| attribute | string | Attribute name for attribute assertions | Optional |
| expected_value | string | Expected value to compare against | Required |
| comparison | string | Comparison type (equals, contains, startsWith, endsWith, matches) | "equals" |

#### Web Action
| Field | Type | Description | Default |
|-------|------|-------------|---------|
| action_type | string | Type of action (click, type, wait, navigate) | Required |
| selector | string | CSS selector for element actions | Optional |
| value | string | Value for type actions or URL for navigate actions | Optional |
| wait_time_ms | number | Wait time in milliseconds for wait actions | 1000 |

### AI Configuration
| Field | Type | Description | Default |
|-------|------|-------------|---------|
| model_type | string | Type of AI model (llama, mistral, gptj, phi, custom) | Required |
| model_path | string | Path to the model weights | Optional |
| context_size | number | Context window size | 2048 |
| temperature | number | Temperature for generation (0.0-1.0) | 0.7 |
| max_tokens | number | Maximum tokens to generate | 1024 |
| system_prompt | string | System prompt to use | Optional |

## Troubleshooting

### Common Issues

#### API Testing
- **Timeout Errors**: Increase the timeout value in your configuration
- **Connection Errors**: Check network connectivity and retry settings
- **Validation Failures**: Verify expected response format and values
- **Retry Loop**: Check retry configuration and target system status

#### API Collections
- **Dependency Errors**: Ensure dependent requests are correctly defined and executed
- **Variable Capture Failures**: Check JSONPath expressions and response structure
- **Variable Interpolation Issues**: Verify variable names and syntax ({{variable}})
- **Authentication Failures**: Check auth configuration and token validity
- **Sequential Execution Problems**: Verify run_options and dependencies
- **JSONPath Errors**: Validate JSONPath expressions against actual response structure
- **Missing Variables**: Ensure all referenced variables are defined or captured
- **Request Chaining Issues**: Check that dependent requests are capturing the expected data
- **Environment Configuration**: Verify environment-specific variables are correctly set

#### Performance Testing
- **High Error Rate**: Reduce concurrent users or increase ramp-up time
- **Slow Response Times**: Check network latency and target system load
- **Resource Exhaustion**: Monitor system resources and adjust test parameters
- **Inconsistent Results**: Ensure test environment stability

#### Enhanced Performance Testing
- **Stage Transition Issues**: Check stage durations and target values
- **Threshold Failures**: Verify threshold expressions and metric names
- **Scenario Distribution Problems**: Check scenario weights and total count
- **Metric Collection Issues**: Ensure metrics are being properly captured
- **High Resource Usage**: Reduce the number of concurrent VUs or increase stage duration
- **Percentile Calculation Errors**: Ensure enough samples for accurate percentiles
- **Tagged Metrics Missing**: Verify tag names and values in scenario configuration

#### CI/CD Integration
- **Exit Code Issues**: Ensure test status is correctly reported
- **GitHub Actions Failures**: Check workflow YAML syntax and permissions
- **Report Generation Failures**: Verify output directory exists and is writable
- **Parallel Test Conflicts**: Ensure tests don't interfere with each other
- **Timeout Errors**: Increase CI job timeout or reduce test duration
- **Environment Variable Issues**: Check environment variable configuration
- **Artifact Storage Problems**: Verify artifact paths and retention settings

#### Data-Driven Testing
- **CSV Parsing Errors**: Check CSV format and delimiter settings
- **JSON Parsing Errors**: Validate JSON syntax and structure
- **Placeholder Not Found**: Ensure placeholder names match data column names
- **Missing Data Columns**: Verify data file contains all required columns
- **Iteration Failures**: Check if stop-on-failure is appropriate for your test
- **Performance Issues**: Reduce max iterations for large datasets
- **File Not Found Errors**: Verify data file paths
- **JSON Path Errors**: Check JSON path syntax for complex data structures
- **Type Conversion Issues**: Ensure data types match expected values
- **Empty Data Sets**: Verify data files contain valid test data

#### Security Testing
- **False Positives**: Review and adjust scan depth and types
- **Authentication Failures**: Verify auth configuration
- **Scan Timeouts**: Adjust timeout settings for complex scans
- **Missing Findings**: Check scan depth and types configuration

#### Web Testing
- **Element Not Found**: Check selectors and wait conditions
- **Timeout Errors**: Increase wait timeout for dynamic content
- **Screenshot Issues**: Verify screenshot directory permissions
- **Assertion Failures**: Check expected values and comparison types
- **Action Failures**: Verify element visibility and interactability

#### AI Features
- **Model Loading Errors**: Verify model path and format compatibility
- **Out of Memory**: Reduce context size or use a smaller model
- **Poor Quality Output**: Adjust temperature or provide more detailed prompts
- **Slow Generation**: Use a smaller model or reduce max tokens
- **Missing Dependencies**: Install AI dependencies with `cargo build --features ai`

## Development

### Prerequisites
- Rust 1.70 or higher
- Cargo
- Git

### Constraints
QitOps is designed with the following constraints in mind:
- **Minimal dependencies**: Uses only the Rust standard library and well-known crates
- **Static binary compilation**: Can be compiled to a static binary for Linux
- **CLI-only interface**: All functionality is accessible via CLI flags with no UI dependencies
- **Terminal-friendly output**: Human-readable output for direct terminal use
- **Machine-readable formats**: Structured output for CI/CD integration

### Building from Source
```bash
# Clone the repository
git clone https://github.com/yourusername/qitops.git
cd qitops

# Build the project
cargo build

# Run tests
cargo test

# Build documentation
cargo doc --no-deps

# Build static binary for Linux
cargo build --release --target x86_64-unknown-linux-musl
```

### Architecture & Project Structure

QitOps follows a modular architecture with clear boundaries between components:

```
qitops/
├── src/
│   ├── main.rs        # CLI parsing using clap
│   ├── api.rs         # API testing implementation
│   ├── performance.rs # Performance testing implementation
│   ├── security.rs    # Security testing implementation
│   ├── web.rs         # Web testing implementation (extension)
│   ├── ai.rs          # AI-powered test generation (extension)
│   ├── reporting.rs   # Report generation (extension)
│   ├── common.rs      # Shared functionality and interfaces
│   └── error.rs       # Error handling
├── tests/
│   └── configs/       # JSON test configuration files
│       ├── api_test.json
│       ├── performance_test.json
│       ├── security_test.json
│       ├── web_test.json
│       └── ai_config.json
├── .github/
│   └── workflows/     # CI configuration
├── Dockerfile         # Container definition
├── docker-compose.yml # Container orchestration
└── Cargo.toml         # Dependencies (minimal and native)
```

The architecture is designed with the following principles:
- **Clear module boundaries**: Each testing type has its own module
- **Common interfaces**: All test runners implement the `TestRunner` trait
- **JSON-based configuration**: Tests are defined using structured JSON files
- **Minimal dependencies**: Uses Rust standard library and well-known crates
- **CLI-first approach**: All functionality accessible via command-line flags
- **Extensibility**: Designed for future expansion (TUI, AI integration)

### CI/CD Integration

QitOps is designed to be CI/CD ready and can be easily integrated into your CI/CD pipeline:

#### GitHub Actions

The included GitHub Actions workflow (`ci.yml`) automatically:
- Builds and tests the project
- Runs linting and formatting checks
- Executes sample tests for each test type
- Creates release artifacts

#### Docker Integration

The included Dockerfile and docker-compose.yml allow you to:
- Build a containerized version of QitOps
- Run tests in isolated containers
- Mount configuration and result volumes
- Set environment variables for different environments

#### JUnit XML Reports

Generate JUnit XML reports for integration with CI/CD tools:
```bash
qitops -r xml -o test-results.xml api -c tests/configs/api_test.json
```

Most CI/CD platforms (Jenkins, GitHub Actions, GitLab CI, etc.) can automatically parse these reports to display test results.

### Adding New Features
1. Create a new module in `src/`
2. Implement the `TestRunner` trait
3. Add CLI commands in `main.rs`
4. Add configuration structures
5. Write tests
6. Update documentation

## QitOps OS Integration

QitOps CLI is designed to be bundled into QitOps OS, a custom bootable Linux distribution for QA professionals. When integrated into QitOps OS, the tool will be:

- Pre-installed in `/usr/local/bin` as a static binary
- Configured with default test configurations in `/etc/qitops/configs`
- Available directly from the terminal without additional setup
- Optimized for the QitOps OS environment

QA professionals can boot directly into QitOps OS and run tests from the terminal without any additional installation or configuration steps.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

When contributing, please keep in mind the core principles of the project:
- Maintain CLI-first approach with no UI dependencies
- Keep dependencies minimal and native
- Ensure compatibility with static binary compilation
- Preserve clear module boundaries
- Design for extensibility

## License

This project is licensed under the MIT License - see the LICENSE file for details.