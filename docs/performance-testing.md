# Performance Testing

QitOps provides powerful performance testing capabilities that allow you to measure the speed, scalability, and stability of your APIs and web services under various load conditions.

## Overview

Performance testing in QitOps allows you to:

- Simulate concurrent users and requests
- Measure response times, throughput, and error rates
- Test different load profiles (constant, ramping, spike)
- Define performance thresholds and success criteria
- Generate detailed performance reports
- Identify bottlenecks and performance issues

## Getting Started

### Basic Usage

```bash
# Run a basic performance test
qitops performance -c tests/configs/performance_test.json

# Run with custom users and duration
qitops performance -c tests/configs/performance_test.json -u 20 -d 60

# Run with a specific ramp-up time
qitops performance -c tests/configs/performance_test.json -u 10 -d 30 -r 5
```

### Command-Line Options

| Option | Description |
|--------|-------------|
| `-c, --config <FILE>` | Performance test configuration file |
| `-u, --users <NUMBER>` | Number of concurrent users (overrides config) |
| `-d, --duration <SECONDS>` | Test duration in seconds (overrides config) |
| `-r, --ramp-up <SECONDS>` | Ramp-up time in seconds (overrides config) |
| `--stream-metrics` | Stream metrics to console during test |
| `--metrics-interval <SECONDS>` | Interval for metrics collection (default: 5) |
| `-e, --environment <ENV>` | Environment to use (default: production) |
| `-r, --report <FORMAT>` | Report format (json, html, xml, csv) |
| `-o, --output <FILE>` | Output file for the report |

## Configuration

Performance tests are defined in JSON configuration files that specify the target, load profile, and success criteria.

### Basic Configuration Structure

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
  "body": {
    "param1": "value1",
    "param2": "value2"
  },
  "concurrent_users": 10,
  "duration_secs": 60,
  "ramp_up_time_secs": 5,
  "success_threshold": 95.0,
  "max_response_time_ms": 500,
  "requests_per_second_threshold": 50
}
```

### Configuration Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| name | string | Yes | Name of the test |
| description | string | No | Description of the test |
| timeout | number | No | Request timeout in seconds (default: 30) |
| retries | number | No | Number of retries (default: 3) |
| environment | string | No | Environment to use (default: production) |
| target_url | string | Yes | URL to test |
| method | string | Yes | HTTP method (GET, POST, PUT, DELETE, etc.) |
| headers | object | No | HTTP headers to send |
| body | object/string | No | Request body (JSON object or string) |
| concurrent_users | number | Yes | Number of concurrent users to simulate |
| duration_secs | number | Yes | Test duration in seconds |
| ramp_up_time_secs | number | No | Time to ramp up to full user count (default: 0) |
| success_threshold | number | No | Minimum success rate percentage (default: 95.0) |
| max_response_time_ms | number | No | Maximum acceptable response time in milliseconds |
| requests_per_second_threshold | number | No | Minimum requests per second |

## Enhanced Performance Testing

QitOps also supports enhanced performance testing with multiple scenarios, custom load profiles, and detailed metrics.

### Enhanced Configuration Structure

```json
{
  "name": "Enhanced Performance Test",
  "description": "Advanced load testing with multiple scenarios",
  "version": "1.0.0",
  "variables": {
    "base_url": "https://api.example.com",
    "api_key": "{{API_KEY}}"
  },
  "defaults": {
    "headers": {
      "Accept": "application/json",
      "Authorization": "Bearer {{api_key}}"
    },
    "timeout": 30,
    "retries": 3
  },
  "scenarios": [
    {
      "name": "Light Load",
      "description": "Light load test with 10 users",
      "target_url": "{{base_url}}/endpoint1",
      "method": "GET",
      "concurrent_users": 10,
      "duration_secs": 60,
      "ramp_up_time_secs": 5,
      "success_threshold": 99.0,
      "max_response_time_ms": 200
    },
    {
      "name": "Medium Load",
      "description": "Medium load test with 50 users",
      "target_url": "{{base_url}}/endpoint2",
      "method": "POST",
      "body": {
        "param1": "value1",
        "param2": "value2"
      },
      "concurrent_users": 50,
      "duration_secs": 120,
      "ramp_up_time_secs": 15,
      "success_threshold": 95.0,
      "max_response_time_ms": 500
    },
    {
      "name": "Heavy Load",
      "description": "Heavy load test with 100 users",
      "target_url": "{{base_url}}/endpoint3",
      "method": "GET",
      "concurrent_users": 100,
      "duration_secs": 180,
      "ramp_up_time_secs": 30,
      "success_threshold": 90.0,
      "max_response_time_ms": 1000
    }
  ],
  "load_profile": {
    "type": "ramp-up-down",
    "stages": [
      { "duration_secs": 30, "target_users": 0 },
      { "duration_secs": 60, "target_users": 100 },
      { "duration_secs": 120, "target_users": 100 },
      { "duration_secs": 60, "target_users": 0 }
    ]
  },
  "metrics": {
    "collection_interval_secs": 5,
    "percentiles": [50, 90, 95, 99],
    "tags": ["api", "performance", "load-test"]
  }
}
```

### Load Profiles

QitOps supports different load profiles to simulate various real-world scenarios:

#### Constant Load

```json
"load_profile": {
  "type": "constant",
  "users": 50,
  "duration_secs": 300
}
```

#### Ramp-Up

```json
"load_profile": {
  "type": "ramp-up",
  "start_users": 0,
  "end_users": 100,
  "duration_secs": 300
}
```

#### Ramp-Up-Down

```json
"load_profile": {
  "type": "ramp-up-down",
  "stages": [
    { "duration_secs": 60, "target_users": 0 },
    { "duration_secs": 120, "target_users": 100 },
    { "duration_secs": 60, "target_users": 0 }
  ]
}
```

#### Step

```json
"load_profile": {
  "type": "step",
  "stages": [
    { "duration_secs": 60, "target_users": 10 },
    { "duration_secs": 60, "target_users": 20 },
    { "duration_secs": 60, "target_users": 50 },
    { "duration_secs": 60, "target_users": 100 }
  ]
}
```

#### Spike

```json
"load_profile": {
  "type": "spike",
  "base_users": 10,
  "spike_users": 100,
  "base_duration_secs": 60,
  "spike_duration_secs": 30,
  "cycles": 3
}
```

## Examples

### Basic API Performance Test

```json
{
  "name": "Basic API Performance Test",
  "description": "Testing a public API endpoint",
  "target_url": "https://api.example.com/users",
  "method": "GET",
  "headers": {
    "Accept": "application/json"
  },
  "concurrent_users": 10,
  "duration_secs": 60,
  "ramp_up_time_secs": 5,
  "success_threshold": 95.0,
  "max_response_time_ms": 500
}
```

### POST Request Performance Test

```json
{
  "name": "POST Request Performance Test",
  "description": "Testing a POST endpoint",
  "target_url": "https://api.example.com/users",
  "method": "POST",
  "headers": {
    "Content-Type": "application/json",
    "Accept": "application/json"
  },
  "body": {
    "name": "John Doe",
    "email": "john@example.com",
    "age": 30
  },
  "concurrent_users": 20,
  "duration_secs": 120,
  "ramp_up_time_secs": 10,
  "success_threshold": 95.0,
  "max_response_time_ms": 800
}
```

### Multi-Scenario Performance Test

```json
{
  "name": "Multi-Scenario Performance Test",
  "description": "Testing multiple endpoints with different loads",
  "variables": {
    "base_url": "https://api.example.com"
  },
  "defaults": {
    "headers": {
      "Accept": "application/json"
    },
    "timeout": 30
  },
  "scenarios": [
    {
      "name": "Get Users",
      "target_url": "{{base_url}}/users",
      "method": "GET",
      "concurrent_users": 20,
      "duration_secs": 60,
      "success_threshold": 95.0
    },
    {
      "name": "Create User",
      "target_url": "{{base_url}}/users",
      "method": "POST",
      "headers": {
        "Content-Type": "application/json"
      },
      "body": {
        "name": "John Doe",
        "email": "john@example.com"
      },
      "concurrent_users": 10,
      "duration_secs": 60,
      "success_threshold": 90.0
    },
    {
      "name": "Search Users",
      "target_url": "{{base_url}}/users/search?q=john",
      "method": "GET",
      "concurrent_users": 30,
      "duration_secs": 60,
      "success_threshold": 85.0
    }
  ]
}
```

## Best Practices

### Test Design

- **Define Clear Objectives**: Define clear performance objectives for your tests
- **Start Small**: Start with a small number of users and gradually increase
- **Test Different Scenarios**: Test different scenarios to understand the performance characteristics of your system
- **Monitor System Resources**: Monitor system resources during performance tests

### Load Profiles

- **Use Appropriate Load Profiles**: Use appropriate load profiles for your tests (constant, ramping, spike)
- **Set Realistic Ramp-Up Times**: Set realistic ramp-up times to avoid overwhelming your system
- **Define Thresholds**: Define thresholds for pass/fail criteria
- **Use Tags**: Use tags to categorize metrics for detailed analysis

### Test Execution

- **Run Tests in Isolation**: Run performance tests in isolation to avoid interference
- **Run Tests Regularly**: Run performance tests regularly to track changes over time
- **Warm Up the System**: Allow the system to warm up before measuring performance
- **Consider Time of Day**: Consider the time of day when running tests

## Integration with CI/CD

Performance tests can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow step
- name: Run Performance Tests
  run: |
    qitops performance -c tests/configs/performance_test.json --ci-mode --report json --output perf-results.json
    
- name: Upload Performance Results
  uses: actions/upload-artifact@v2
  with:
    name: performance-results
    path: perf-results.json
```

## Troubleshooting

### Common Issues

- **Connection Refused**: Check if the target server is running and accessible
- **Timeout Errors**: Increase the timeout value or check server performance
- **Memory Issues**: Reduce the number of concurrent users or optimize memory usage
- **CPU Bottlenecks**: Check if the client machine has enough CPU resources
- **Network Limitations**: Consider running tests from multiple locations

### Performance Analysis

- **Identify Bottlenecks**: Use the performance report to identify bottlenecks
- **Analyze Response Time Distribution**: Look at the distribution of response times
- **Check Error Patterns**: Analyze error patterns to identify issues
- **Compare with Baselines**: Compare results with baseline measurements
