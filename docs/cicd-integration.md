# CI/CD Integration

QitOps is designed to integrate seamlessly with CI/CD pipelines, allowing you to automate testing as part of your development workflow.

## Overview

CI/CD integration with QitOps allows you to:

- Run tests automatically on code changes
- Validate API functionality before deployment
- Ensure performance meets requirements
- Identify security vulnerabilities early
- Generate test reports for review
- Fail builds when tests don't meet criteria

## Getting Started

### Basic Usage

```bash
# Run in CI mode with JSON report
qitops --ci-mode -r json -o results.json api -c tests/configs/api_test.json

# Run in CI mode with XML report (JUnit format)
qitops --ci-mode -r xml -o test-results.xml api -c tests/configs/api_test.json

# Run in CI mode with HTML report
qitops --ci-mode -r html -o report.html api -c tests/configs/api_test.json
```

### CI Mode Options

| Option | Description |
|--------|-------------|
| `--ci-mode` | Run in CI mode (reduced output, exit code based on test results) |
| `-r, --report <FORMAT>` | Report format (json, html, xml, csv) |
| `-o, --output <FILE>` | Output file for the report |
| `--fail-on-error` | Exit with non-zero code on any test failure |
| `--fail-threshold <PERCENT>` | Exit with non-zero code if success rate is below threshold |

## GitHub Actions Integration

QitOps can be easily integrated with GitHub Actions workflows.

### Basic Workflow

```yaml
name: QitOps Tests

on:
  push:
    branches: [ main, master ]
  pull_request:
    branches: [ main, master ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install QitOps
        run: |
          curl -sSL https://github.com/qitops/qitops-cli-tools/releases/latest/download/qitops-linux-x86_64 -o /usr/local/bin/qitops
          chmod +x /usr/local/bin/qitops
      
      - name: Run API Tests
        run: |
          qitops --ci-mode -r xml -o api-test-results.xml api -c tests/configs/api_test.json
      
      - name: Upload Test Results
        uses: actions/upload-artifact@v2
        with:
          name: api-test-results
          path: api-test-results.xml
```

### Complete Workflow

```yaml
name: QitOps Complete Test Suite

on:
  push:
    branches: [ main, master ]
  pull_request:
    branches: [ main, master ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install QitOps
        run: |
          curl -sSL https://github.com/qitops/qitops-cli-tools/releases/latest/download/qitops-linux-x86_64 -o /usr/local/bin/qitops
          chmod +x /usr/local/bin/qitops
      
      - name: Run API Tests
        run: |
          qitops --ci-mode -r xml -o api-test-results.xml api -c tests/configs/api_test.json
      
      - name: Run API Collection Tests
        run: |
          qitops --ci-mode -r xml -o collection-test-results.xml collection -c tests/configs/api_collection.json
      
      - name: Run Performance Tests
        run: |
          qitops --ci-mode -r json -o performance-test-results.json performance -c tests/configs/performance_test.json
      
      - name: Run Security Tests
        run: |
          qitops --ci-mode -r json -o security-test-results.json security -c tests/configs/security_test.json
      
      - name: Run Web Tests
        run: |
          qitops --ci-mode -r html -o web-test-results.html web -c tests/configs/web_test.json
      
      - name: Upload Test Results
        uses: actions/upload-artifact@v2
        with:
          name: test-results
          path: |
            api-test-results.xml
            collection-test-results.xml
            performance-test-results.json
            security-test-results.json
            web-test-results.html
```

## GitLab CI Integration

QitOps can also be integrated with GitLab CI/CD pipelines.

### Basic Pipeline

```yaml
stages:
  - test

api-tests:
  stage: test
  image: ubuntu:latest
  script:
    - apt-get update && apt-get install -y curl
    - curl -sSL https://github.com/qitops/qitops-cli-tools/releases/latest/download/qitops-linux-x86_64 -o /usr/local/bin/qitops
    - chmod +x /usr/local/bin/qitops
    - qitops --ci-mode -r json -o api-test-results.json api -c tests/configs/api_test.json
  artifacts:
    paths:
      - api-test-results.json
```

### Complete Pipeline

```yaml
stages:
  - test
  - performance
  - security

api-tests:
  stage: test
  image: ubuntu:latest
  script:
    - apt-get update && apt-get install -y curl
    - curl -sSL https://github.com/qitops/qitops-cli-tools/releases/latest/download/qitops-linux-x86_64 -o /usr/local/bin/qitops
    - chmod +x /usr/local/bin/qitops
    - qitops --ci-mode -r json -o api-test-results.json api -c tests/configs/api_test.json
  artifacts:
    paths:
      - api-test-results.json

collection-tests:
  stage: test
  image: ubuntu:latest
  script:
    - apt-get update && apt-get install -y curl
    - curl -sSL https://github.com/qitops/qitops-cli-tools/releases/latest/download/qitops-linux-x86_64 -o /usr/local/bin/qitops
    - chmod +x /usr/local/bin/qitops
    - qitops --ci-mode -r json -o collection-test-results.json collection -c tests/configs/api_collection.json
  artifacts:
    paths:
      - collection-test-results.json

performance-tests:
  stage: performance
  image: ubuntu:latest
  script:
    - apt-get update && apt-get install -y curl
    - curl -sSL https://github.com/qitops/qitops-cli-tools/releases/latest/download/qitops-linux-x86_64 -o /usr/local/bin/qitops
    - chmod +x /usr/local/bin/qitops
    - qitops --ci-mode -r json -o performance-test-results.json performance -c tests/configs/performance_test.json
  artifacts:
    paths:
      - performance-test-results.json

security-tests:
  stage: security
  image: ubuntu:latest
  script:
    - apt-get update && apt-get install -y curl
    - curl -sSL https://github.com/qitops/qitops-cli-tools/releases/latest/download/qitops-linux-x86_64 -o /usr/local/bin/qitops
    - chmod +x /usr/local/bin/qitops
    - qitops --ci-mode -r json -o security-test-results.json security -c tests/configs/security_test.json
  artifacts:
    paths:
      - security-test-results.json
```

## Jenkins Integration

QitOps can be integrated with Jenkins pipelines.

### Jenkinsfile Example

```groovy
pipeline {
    agent any
    
    stages {
        stage('Install QitOps') {
            steps {
                sh '''
                curl -sSL https://github.com/qitops/qitops-cli-tools/releases/latest/download/qitops-linux-x86_64 -o /usr/local/bin/qitops
                chmod +x /usr/local/bin/qitops
                '''
            }
        }
        
        stage('API Tests') {
            steps {
                sh 'qitops --ci-mode -r xml -o api-test-results.xml api -c tests/configs/api_test.json'
            }
            post {
                always {
                    junit 'api-test-results.xml'
                }
            }
        }
        
        stage('Performance Tests') {
            steps {
                sh 'qitops --ci-mode -r json -o performance-test-results.json performance -c tests/configs/performance_test.json'
            }
            post {
                always {
                    archiveArtifacts artifacts: 'performance-test-results.json', fingerprint: true
                }
            }
        }
        
        stage('Security Tests') {
            steps {
                sh 'qitops --ci-mode -r json -o security-test-results.json security -c tests/configs/security_test.json'
            }
            post {
                always {
                    archiveArtifacts artifacts: 'security-test-results.json', fingerprint: true
                }
            }
        }
    }
}
```

## CircleCI Integration

QitOps can be integrated with CircleCI workflows.

### config.yml Example

```yaml
version: 2.1

jobs:
  api-tests:
    docker:
      - image: cimg/base:2021.04
    steps:
      - checkout
      - run:
          name: Install QitOps
          command: |
            curl -sSL https://github.com/qitops/qitops-cli-tools/releases/latest/download/qitops-linux-x86_64 -o /usr/local/bin/qitops
            chmod +x /usr/local/bin/qitops
      - run:
          name: Run API Tests
          command: |
            qitops --ci-mode -r xml -o api-test-results.xml api -c tests/configs/api_test.json
      - store_artifacts:
          path: api-test-results.xml
      - store_test_results:
          path: api-test-results.xml

  performance-tests:
    docker:
      - image: cimg/base:2021.04
    steps:
      - checkout
      - run:
          name: Install QitOps
          command: |
            curl -sSL https://github.com/qitops/qitops-cli-tools/releases/latest/download/qitops-linux-x86_64 -o /usr/local/bin/qitops
            chmod +x /usr/local/bin/qitops
      - run:
          name: Run Performance Tests
          command: |
            qitops --ci-mode -r json -o performance-test-results.json performance -c tests/configs/performance_test.json
      - store_artifacts:
          path: performance-test-results.json

workflows:
  version: 2
  test:
    jobs:
      - api-tests
      - performance-tests
```

## Best Practices

### Test Organization

- **Group Related Tests**: Group related tests in separate configuration files
- **Use Environment Variables**: Use environment variables for sensitive information
- **Parameterize Configurations**: Use variables and environment variables to parameterize your configurations
- **Use CI-Specific Configurations**: Create CI-specific configurations for different environments

### CI/CD Integration

- **Use CI Mode**: Always use the `--ci-mode` flag when running tests in CI/CD pipelines
- **Generate Reports**: Generate reports in a format that can be consumed by your CI/CD system
- **Set Exit Codes**: Use exit codes to determine the success or failure of your tests
- **Store Artifacts**: Store test results and reports as artifacts for historical analysis

### Performance Considerations

- **Optimize Test Execution**: Optimize test execution to reduce CI/CD pipeline time
- **Run Tests in Parallel**: Run tests in parallel where possible
- **Use Caching**: Use caching to speed up test execution
- **Limit Test Scope**: Limit test scope based on changes to reduce execution time

## Troubleshooting

### Common Issues

- **Exit Codes**: QitOps returns non-zero exit codes on test failures when `--ci-mode` is used
- **Report Formats**: Different CI/CD systems support different report formats
- **Environment Variables**: Ensure environment variables are properly set in your CI/CD environment
- **Permissions**: Ensure QitOps has the necessary permissions to run tests

### CI/CD System-Specific Issues

- **GitHub Actions**: Use the `actions/upload-artifact` action to store test results
- **GitLab CI**: Use the `artifacts` section to store test results
- **Jenkins**: Use the `junit` step to process XML test results
- **CircleCI**: Use the `store_test_results` step to process test results
