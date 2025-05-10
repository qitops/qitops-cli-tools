# QitOps GitHub Actions Templates

This directory contains GitHub Actions workflow templates for running QitOps tests in your CI/CD pipeline.

## Available Templates

- **all-tests.yml**: Runs all test types (API, Performance, Security, Web) in a single workflow
- **api-test.yml**: Runs API tests and API collection tests
- **performance-test.yml**: Runs basic and enhanced performance tests
- **security-test.yml**: Runs security tests with configurable depth
- **web-test.yml**: Runs web tests with screenshot capture

## Usage

1. Copy the desired template to your repository's `.github/workflows/` directory
2. Customize the template as needed for your project
3. Commit and push the changes to trigger the workflow

## Configuration

Each template can be configured through workflow inputs when triggered manually:

### Common Inputs

- **environment**: The environment to run tests against (development, staging, production)
- **test_type**: The type of tests to run (all, api, performance, security, web)

### Performance Test Inputs

- **users**: Number of concurrent users for performance tests
- **duration**: Test duration in seconds

### Security Test Inputs

- **depth**: Scan depth (1-5)
- **passive**: Whether to run passive scan only

### Web Test Inputs

- **headless**: Whether to run in headless mode

## Example Usage

### Running API Tests

```yaml
name: API Tests

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  api-tests:
    uses: ./.github/workflows/api-test.yml
    with:
      environment: staging
```

### Running All Tests on Schedule

```yaml
name: Nightly Tests

on:
  schedule:
    - cron: '0 0 * * *'  # Run at midnight every day

jobs:
  all-tests:
    uses: ./.github/workflows/all-tests.yml
    with:
      environment: staging
      test_type: all
```

## Customization

You can customize these templates by:

1. Adding environment variables specific to your project
2. Modifying the test configuration paths
3. Adding additional steps for setup or teardown
4. Integrating with other GitHub Actions for notifications or reporting

## Requirements

- QitOps CLI installed or downloaded during workflow execution
- Test configuration files in the specified locations
- Appropriate permissions for the GitHub Actions workflow

## Best Practices

- Store sensitive information (API keys, passwords) in GitHub Secrets
- Use environment-specific configurations for different deployment stages
- Run comprehensive tests on schedule rather than on every push
- Use test results to gate deployments to production
- Archive test reports for historical analysis
