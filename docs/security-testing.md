# Security Testing

QitOps provides comprehensive security testing capabilities that allow you to identify vulnerabilities, ensure compliance with security standards, and protect your applications from common attacks.

## Overview

Security testing in QitOps allows you to:

- Scan for common vulnerabilities (OWASP Top 10)
- Check for secure headers and SSL/TLS configuration
- Detect sensitive data exposure
- Validate authentication and authorization mechanisms
- Perform basic penetration testing
- Generate detailed security reports

## Getting Started

### Basic Usage

```bash
# Run a security test
qitops security -c tests/configs/security_test.json

# Run with a specific scan depth
qitops security -c tests/configs/security_test.json -d 3

# Run with specific scan types
qitops security -c tests/configs/security_test.json --scan-types headers,ssl
```

### Command-Line Options

| Option | Description |
|--------|-------------|
| `-c, --config <FILE>` | Security test configuration file |
| `-d, --depth <NUMBER>` | Scan depth (1-5, default: 2) |
| `--scan-types <TYPES>` | Comma-separated list of scan types |
| `--max-findings <NUMBER>` | Maximum number of findings to report |
| `--severity <LEVEL>` | Minimum severity level to report (low, medium, high, critical) |
| `-e, --environment <ENV>` | Environment to use (default: production) |
| `-r, --report <FORMAT>` | Report format (json, html, xml, csv) |
| `-o, --output <FILE>` | Output file for the report |

## Configuration

Security tests are defined in JSON configuration files that specify the target, scan types, and severity thresholds.

### Basic Configuration Structure

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
  "scan_depth": 2,
  "max_high_severity_findings": 0,
  "max_medium_severity_findings": 5,
  "severity_threshold": "low"
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
| headers | object | No | HTTP headers to send |
| auth | object | No | Authentication configuration |
| scan_types | array | No | Types of scans to perform (default: all) |
| scan_depth | number | No | Depth of the scan (1-5, default: 2) |
| max_high_severity_findings | number | No | Maximum allowed high severity findings (default: 0) |
| max_medium_severity_findings | number | No | Maximum allowed medium severity findings (default: 5) |
| severity_threshold | string | No | Minimum severity level to report (default: "low") |

### Authentication

QitOps supports various authentication methods for security testing:

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

## Scan Types

QitOps supports various types of security scans:

### Headers Security

Checks for secure HTTP headers:

- Content-Security-Policy
- X-Content-Type-Options
- X-Frame-Options
- X-XSS-Protection
- Strict-Transport-Security
- Referrer-Policy
- Permissions-Policy

### SSL/TLS Security

Checks for secure SSL/TLS configuration:

- Protocol versions (TLS 1.2+)
- Cipher suites
- Certificate validity
- Certificate chain
- Key strength
- OCSP stapling
- Perfect forward secrecy

### Vulnerabilities

Scans for common vulnerabilities:

- SQL Injection
- Cross-Site Scripting (XSS)
- Cross-Site Request Forgery (CSRF)
- Server-Side Request Forgery (SSRF)
- XML External Entity (XXE)
- Command Injection
- Path Traversal
- Insecure Deserialization

### Sensitive Data Exposure

Checks for sensitive data in responses:

- Credit card numbers
- Social security numbers
- API keys and tokens
- Passwords
- Email addresses
- Private keys
- Internal IP addresses

### Authentication and Authorization

Tests authentication and authorization mechanisms:

- Brute force protection
- Session management
- Password policies
- Multi-factor authentication
- Role-based access control

## Examples

### Basic Security Scan

```json
{
  "name": "Basic Security Scan",
  "description": "Basic security scan of a public API",
  "target_url": "https://api.example.com",
  "scan_types": [
    "headers",
    "ssl"
  ],
  "scan_depth": 1,
  "severity_threshold": "medium"
}
```

### Comprehensive Security Scan

```json
{
  "name": "Comprehensive Security Scan",
  "description": "Detailed security scan with authentication",
  "target_url": "https://api.example.com",
  "headers": {
    "Accept": "application/json",
    "User-Agent": "QitOps-SecurityTest"
  },
  "auth": {
    "type": "bearer",
    "token": "your-token"
  },
  "scan_types": [
    "headers",
    "ssl",
    "vulnerabilities",
    "sensitive-data",
    "authentication"
  ],
  "scan_depth": 3,
  "max_high_severity_findings": 0,
  "max_medium_severity_findings": 3,
  "severity_threshold": "low"
}
```

### API Security Scan

```json
{
  "name": "API Security Scan",
  "description": "Security scan focused on API vulnerabilities",
  "target_url": "https://api.example.com",
  "headers": {
    "Content-Type": "application/json",
    "Accept": "application/json"
  },
  "auth": {
    "type": "apikey",
    "key": "X-API-Key",
    "value": "your-api-key",
    "in": "header"
  },
  "scan_types": [
    "vulnerabilities",
    "sensitive-data",
    "authentication"
  ],
  "scan_depth": 2,
  "endpoints": [
    {
      "path": "/users",
      "method": "GET",
      "params": {
        "limit": "10"
      }
    },
    {
      "path": "/users",
      "method": "POST",
      "body": {
        "name": "Test User",
        "email": "test@example.com"
      }
    }
  ]
}
```

## Best Practices

### Test Design

- **Define Security Requirements**: Define clear security requirements for your tests
- **Test Different Scan Types**: Test different scan types to identify different types of vulnerabilities
- **Set Severity Thresholds**: Set severity thresholds for pass/fail criteria
- **Regular Testing**: Perform security tests regularly to identify new vulnerabilities

### Authentication

- **Test Authentication**: Test authentication mechanisms to ensure they are secure
- **Test Authorization**: Test authorization to ensure users can only access resources they are authorized to access
- **Test Input Validation**: Test input validation to prevent injection attacks
- **Test Error Handling**: Test error handling to ensure sensitive information is not leaked

### Test Execution

- **Run Tests in Isolation**: Run security tests in isolation to avoid interference
- **Run Tests Regularly**: Run security tests regularly to identify new vulnerabilities
- **Scan Different Environments**: Scan different environments (development, staging, production)
- **Follow Up on Findings**: Follow up on security findings to ensure they are addressed

## Integration with CI/CD

Security tests can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow step
- name: Run Security Tests
  run: |
    qitops security -c tests/configs/security_test.json --ci-mode --report json --output security-results.json
    
- name: Upload Security Results
  uses: actions/upload-artifact@v2
  with:
    name: security-results
    path: security-results.json
```

## Troubleshooting

### Common Issues

- **False Positives**: Security scans may report false positives
- **Scan Depth**: Higher scan depths may take longer to complete
- **Authentication Issues**: Ensure authentication credentials are valid
- **Rate Limiting**: Some targets may rate limit security scans
- **Firewall Blocking**: Firewalls may block security scans

### Security Analysis

- **Prioritize Findings**: Prioritize findings based on severity and impact
- **Verify Findings**: Verify findings to ensure they are not false positives
- **Document Findings**: Document findings and remediation steps
- **Track Progress**: Track progress in addressing security findings
