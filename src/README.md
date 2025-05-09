# QitOps - Quality Assurance Testing CLI

[![CI Status](https://github.com/yourusername/qitops/workflows/QitOps%20CI/badge.svg)](https://github.com/yourusername/qitops/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

QitOps is a professional-grade CLI tool for comprehensive software testing, designed to be the core component of QitOps OS - a custom Linux distribution for QA professionals. It provides a unified command-line interface for API, performance, security, and web testing with minimal dependencies and maximum flexibility.

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

QitOps is designed to be compiled as a static binary for Linux systems, making it ideal for inclusion in the QitOps OS distribution or installation in `/usr/local/bin`.

```bash
# Clone the repository
git clone https://github.com/yourusername/qitops.git
cd qitops

# Build the project with default features
cargo build --release

# Build with static linking for distribution
cargo build --release --target x86_64-unknown-linux-musl

# Install globally (optional)
cargo install --path .

# Install the static binary to system location (requires sudo)
sudo cp target/x86_64-unknown-linux-musl/release/qitops /usr/local/bin/
```

### Optional Features

```bash
# Build with AI features
cargo build --release --features ai
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