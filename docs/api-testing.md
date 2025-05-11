# API Testing

QitOps provides comprehensive API testing capabilities that allow you to validate REST APIs, ensure correct functionality, and verify performance and reliability.

## Overview

API testing in QitOps allows you to:

- Test individual API endpoints with various HTTP methods
- Validate response status codes, headers, and body content
- Set up authentication and authorization
- Measure and validate response times
- Configure timeouts and retries for reliability
- Chain requests with data from previous responses

## Getting Started

### Basic Usage

```bash
# Run a single API test
qitops api -c tests/configs/api_test.json

# Run tests in a specific environment
qitops api -c tests/configs/api_test.json -e production

# Run with custom variables
qitops api -c tests/configs/api_test.json -v base_url=https://api.example.com -v api_key=12345
```

### Command-Line Options

| Option | Description |
|--------|-------------|
| `-c, --config <FILE>` | API test configuration file |
| `-e, --environment <ENV>` | Environment to use (default: production) |
| `-v, --variable <KEY=VALUE>` | Set a variable for the test |
| `--timeout <SECONDS>` | Override timeout in seconds |
| `--retries <NUMBER>` | Override number of retries |
| `-r, --report <FORMAT>` | Report format (json, html, xml, csv) |
| `-o, --output <FILE>` | Output file for the report |
| `--verbose` | Enable verbose output |

## Configuration

API tests are defined in JSON configuration files that specify the endpoint, method, headers, and expected responses.

### Basic Configuration Structure

```json
{
  "name": "Example API Test",
  "description": "Test description",
  "timeout": 30,
  "retries": 3,
  "environment": "production",
  "url": "https://api.example.com/users/123",
  "method": "GET",
  "headers": {
    "Accept": "application/json",
    "User-Agent": "QitOps-Test"
  },
  "expected_status": 200,
  "expected_body": {
    "id": 123,
    "name": "John Doe"
  },
  "max_response_time": 2,
  "expected_headers": {
    "content-type": "application/json",
    "cache-control": "no-cache"
  }
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
| url | string | Yes | URL to test |
| method | string | Yes | HTTP method (GET, POST, PUT, DELETE, etc.) |
| headers | object | No | HTTP headers to send |
| body | object/string | No | Request body (JSON object or string) |
| expected_status | number | No | Expected HTTP status code (default: 200) |
| expected_body | object/string | No | Expected response body (JSON object or string) |
| max_response_time | number | No | Maximum acceptable response time in seconds |
| expected_headers | object | No | Expected response headers |
| retry | object | No | Retry configuration |

### Authentication

QitOps supports various authentication methods:

#### Basic Authentication

```json
"auth": {
  "type": "basic",
  "username": "user",
  "password": "pass"
}
```

#### Bearer Token

```json
"auth": {
  "type": "bearer",
  "token": "your-token"
}
```

#### API Key

```json
"auth": {
  "type": "apikey",
  "key": "X-API-Key",
  "value": "your-api-key",
  "in": "header"  // or "query"
}
```

#### OAuth2

```json
"auth": {
  "type": "oauth2",
  "token_url": "https://auth.example.com/token",
  "client_id": "your-client-id",
  "client_secret": "your-client-secret",
  "scope": "read write"
}
```

### Request Body

You can specify the request body in different formats:

#### JSON Body

```json
"body": {
  "name": "John Doe",
  "email": "john@example.com",
  "age": 30
}
```

#### String Body

```json
"body": "name=John%20Doe&email=john%40example.com",
"headers": {
  "Content-Type": "application/x-www-form-urlencoded"
}
```

#### Binary Body

```json
"body": {
  "_file": "path/to/file.pdf"
},
"headers": {
  "Content-Type": "application/pdf"
}
```

### Response Validation

QitOps provides several ways to validate API responses:

#### Status Code

```json
"expected_status": 200
```

#### Headers

```json
"expected_headers": {
  "content-type": "application/json",
  "cache-control": "no-cache"
}
```

#### Body (Exact Match)

```json
"expected_body": {
  "id": 123,
  "name": "John Doe"
}
```

#### Body (Partial Match)

```json
"expected_body_contains": {
  "name": "John Doe"
}
```

#### JSON Schema Validation

```json
"validate_json_schema": {
  "type": "object",
  "required": ["id", "name"],
  "properties": {
    "id": { "type": "integer" },
    "name": { "type": "string" }
  }
}
```

#### JSONPath Assertions

```json
"assertions": [
  {
    "jsonpath": "$.name",
    "expected": "John Doe",
    "comparison": "equals"
  },
  {
    "jsonpath": "$.age",
    "expected": 18,
    "comparison": "greater_than"
  }
]
```

### Retry Configuration

Configure retry behavior for transient failures:

```json
"retry": {
  "max_retries": 3,
  "initial_delay_ms": 100,
  "max_delay_ms": 1000,
  "retry_status_codes": [408, 429, 500, 502, 503, 504],
  "retry_on_timeout": true,
  "retry_on_connection_error": true
}
```

## Examples

### GET Request

```json
{
  "name": "Get User",
  "description": "Get a user by ID",
  "url": "https://api.example.com/users/123",
  "method": "GET",
  "headers": {
    "Accept": "application/json"
  },
  "expected_status": 200,
  "expected_body": {
    "id": 123,
    "name": "John Doe",
    "email": "john@example.com"
  }
}
```

### POST Request

```json
{
  "name": "Create User",
  "description": "Create a new user",
  "url": "https://api.example.com/users",
  "method": "POST",
  "headers": {
    "Content-Type": "application/json",
    "Accept": "application/json"
  },
  "body": {
    "name": "Jane Doe",
    "email": "jane@example.com",
    "age": 28
  },
  "expected_status": 201,
  "expected_body_contains": {
    "id": true,
    "name": "Jane Doe"
  }
}
```

### PUT Request

```json
{
  "name": "Update User",
  "description": "Update an existing user",
  "url": "https://api.example.com/users/123",
  "method": "PUT",
  "headers": {
    "Content-Type": "application/json",
    "Accept": "application/json"
  },
  "body": {
    "name": "John Smith",
    "email": "john.smith@example.com"
  },
  "expected_status": 200,
  "expected_body": {
    "id": 123,
    "name": "John Smith",
    "email": "john.smith@example.com"
  }
}
```

### DELETE Request

```json
{
  "name": "Delete User",
  "description": "Delete a user",
  "url": "https://api.example.com/users/123",
  "method": "DELETE",
  "expected_status": 204
}
```

### Authentication Example

```json
{
  "name": "Authenticated Request",
  "description": "Make an authenticated API request",
  "url": "https://api.example.com/protected-resource",
  "method": "GET",
  "auth": {
    "type": "bearer",
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  },
  "expected_status": 200
}
```

### File Upload

```json
{
  "name": "Upload File",
  "description": "Upload a file to the API",
  "url": "https://api.example.com/upload",
  "method": "POST",
  "headers": {
    "Content-Type": "multipart/form-data"
  },
  "form_data": {
    "file": {
      "_file": "path/to/file.jpg"
    },
    "description": "Profile picture"
  },
  "expected_status": 200,
  "expected_body_contains": {
    "success": true
  }
}
```

## Best Practices

### Test Organization

- Group related tests in separate configuration files
- Use descriptive names for tests
- Include detailed descriptions
- Organize tests by functionality or endpoint

### Reliability

- Set appropriate timeouts based on expected response times
- Configure retries for transient failures
- Use environment-specific configurations
- Validate both success and error scenarios

### Validation

- Validate status codes, headers, and body content
- Use JSON schema validation for complex responses
- Include performance assertions (max_response_time)
- Test edge cases and boundary conditions

### Variables and Environments

- Use variables for values that change between environments
- Create environment-specific configurations
- Avoid hardcoding sensitive information
- Use variable interpolation for dynamic values

## Integration with CI/CD

API tests can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow step
- name: Run API Tests
  run: |
    qitops api -c tests/configs/api_test.json -e staging --report json --output api-test-results.json
    
- name: Upload Test Results
  uses: actions/upload-artifact@v2
  with:
    name: api-test-results
    path: api-test-results.json
```

## Troubleshooting

### Common Issues

- **Connection Refused**: Check if the API server is running and accessible
- **Timeout Errors**: Increase the timeout value or check server performance
- **Authentication Failures**: Verify credentials and token expiration
- **Unexpected Responses**: Check if the API contract has changed
- **SSL/TLS Errors**: Verify certificate validity and trust chain

### Debugging Tips

- Use the `--verbose` flag for detailed output
- Check the request and response headers
- Verify the request body is correctly formatted
- Test the API endpoint with a tool like curl or Postman
- Check for environment-specific issues
