# Configuration Reference

This page provides detailed information on configuring QitOps for different test types.

## API Test Configuration

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

### API Test Configuration Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| name | string | Yes | Name of the test |
| description | string | No | Description of the test |
| timeout | number | No | Request timeout in seconds (default: 30) |
| retries | number | No | Number of retries (default: 3) |
| environment | string | No | Environment to use (default: production) |
| url | string | Yes | URL to test |
| method | string | Yes | HTTP method (GET, POST, PUT, DELETE, etc.) |
| headers | object | No | HTTP headers to send |
| body | object/string | No | Request body (JSON object or string) |
| expected_status | number | No | Expected HTTP status code (default: 200) |
| expected_body | object/string | No | Expected response body (JSON object or string) |
| max_response_time | number | No | Maximum acceptable response time in seconds |
| expected_headers | object | No | Expected response headers |
| retry | object | No | Retry configuration |

## API Collection Configuration

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

## Performance Test Configuration

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

## Security Test Configuration

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

## Web Test Configuration

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

## AI Configuration

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
