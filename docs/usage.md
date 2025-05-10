# Usage Guide

This page provides detailed information on using QitOps for different testing scenarios.

## Command Line Interface

QitOps provides a unified command-line interface for all testing types:

```bash
qitops [global options] <command> [command options]
```

### Global Options

| Option | Description |
|--------|-------------|
| -r, --report <FORMAT> | Report format (json, html, xml, csv) |
| -o, --output <FILE> | Output file for the report |
| --ci-mode | Run in CI mode (reduced output, exit code based on test results) |
| -e, --environment <ENV> | Environment to use (default: production) |
| -v, --verbose | Enable verbose output |
| -h, --help | Show help |
| -V, --version | Show version |

### Commands

| Command | Description |
|---------|-------------|
| api | Run API tests |
| collection | Run API collection tests |
| performance | Run performance tests |
| security | Run security tests |
| web | Run web tests |
| data-driven | Run data-driven tests |
| generate | Generate test configurations using AI |
| analyze | Analyze test results using AI |
| improve | Generate improvement suggestions using AI |

## API Testing

```bash
# Run a single API test
qitops api -c tests/configs/api_test.json

# Run tests in a specific environment
qitops api -c tests/configs/api_test.json -e production

# Run with custom variables
qitops api -c tests/configs/api_test.json -v base_url=https://api.example.com -v api_key=12345
```

## API Collection Testing

```bash
# Run an API collection
qitops collection -c tests/configs/api_collection.json

# Run with environment variables
qitops collection -c tests/configs/api_collection.json -v API_KEY=your-api-key

# Run with a specific environment
qitops collection -c tests/configs/api_collection.json -e staging
```

## Performance Testing

```bash
# Run a basic performance test
qitops performance -c tests/configs/performance_test.json -u 10 -d 30

# Run with custom users and duration
qitops performance -c tests/configs/performance_test.json -u 20 -d 60

# Run with a specific ramp-up time
qitops performance -c tests/configs/performance_test.json -u 10 -d 30 -r 5
```

## Enhanced Performance Testing

```bash
# Run an enhanced performance test with multiple scenarios
qitops performance -c tests/configs/enhanced_performance_test.json

# Run with streaming metrics
qitops performance -c tests/configs/enhanced_performance_test.json --stream-metrics

# Run with custom metrics interval
qitops performance -c tests/configs/enhanced_performance_test.json --metrics-interval 10
```

## Security Testing

```bash
# Run a security test
qitops security -c tests/configs/security_test.json

# Run with a specific scan depth
qitops security -c tests/configs/security_test.json -d 3

# Run with specific scan types
qitops security -c tests/configs/security_test.json --scan-types headers,ssl
```

## Web Testing

```bash
# Run a web test
qitops web -c tests/configs/web_test.json

# Run in headless mode
qitops web -c tests/configs/web_test.json --headless

# Run with custom viewport
qitops web -c tests/configs/web_test.json --width 1920 --height 1080
```

## Data-Driven Testing

```bash
# Run data-driven tests with CSV data
qitops data-driven -c tests/configs/data_driven_api_test.json -d tests/data/users.csv -t csv

# Run data-driven tests with JSON data
qitops data-driven -c tests/configs/data_driven_collection.json -d tests/data/products.json -t json

# Run with a limit on iterations
qitops data-driven -c tests/configs/data_driven_api_test.json -d tests/data/users.csv -t csv --limit 10

# Run with stop-on-failure
qitops data-driven -c tests/configs/data_driven_api_test.json -d tests/data/users.csv -t csv --stop-on-failure
```

## AI Features

```bash
# Generate an API test configuration
qitops generate --test-type api --description "Test the GitHub API to fetch user information" --output github_test.json

# Generate a performance test configuration
qitops generate --test-type performance --description "Load test for a web service" --output performance_test.json

# Analyze test results
qitops analyze --results test_result.json --output analysis.md

# Generate improvement suggestions
qitops improve --results test_result.json --output improvements.md

# Use a custom AI model
qitops generate --test-type api --description "Test description" --model custom --model-path /path/to/model.gguf
```

## CI/CD Integration

```bash
# Run in CI mode with JSON report
qitops --ci-mode -r json -o results.json api -c tests/configs/api_test.json

# Run in CI mode with XML report (JUnit format)
qitops --ci-mode -r xml -o test-results.xml api -c tests/configs/api_test.json

# Run in CI mode with HTML report
qitops --ci-mode -r html -o report.html api -c tests/configs/api_test.json
```

## Environment Variables

QitOps supports the following environment variables:

| Variable | Description |
|----------|-------------|
| QITOPS_ENV | Default environment to use |
| QITOPS_REPORT_FORMAT | Default report format |
| QITOPS_OUTPUT_FILE | Default output file |
| QITOPS_CI_MODE | Run in CI mode if set to "true" |
| QITOPS_VERBOSE | Enable verbose output if set to "true" |
| QITOPS_CONFIG_DIR | Directory containing configuration files |
| QITOPS_DATA_DIR | Directory containing data files |
| QITOPS_MODEL_PATH | Path to AI model weights |
