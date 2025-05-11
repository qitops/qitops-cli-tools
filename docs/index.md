# QitOps: The Ultimate QA CLI Tool

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/qitops/qitops-cli-tools/actions/workflows/ci.yml/badge.svg)](https://github.com/qitops/qitops-cli-tools/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/qitops)](https://crates.io/crates/qitops)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)
[![Documentation](https://img.shields.io/badge/docs-latest-brightgreen.svg)](https://qitops.github.io/qitops-cli-tools/)

QitOps is a comprehensive Software Quality Assurance CLI tool for API, Performance, Security, and Web Testing. It provides a unified command-line interface with minimal dependencies and maximum flexibility.

## Why QitOps?

QitOps stands out from other testing tools with its unique combination of features:

- **All-in-One Testing Solution**: Combines API, performance, security, and web testing in a single tool
- **CLI-First Approach**: Designed for automation and CI/CD integration with a powerful command-line interface
- **Local AI-Powered Features**: Generate tests, analyze results, and get improvement suggestions using local LLMs
- **Minimal Dependencies**: Built with Rust for speed, reliability, and minimal resource usage
- **Maximum Flexibility**: Works with any API, web service, or application
- **Data-Driven Testing**: Parameterize tests with CSV and JSON datasets
- **Comprehensive Reporting**: Generate reports in multiple formats (JSON, XML, HTML, CSV)
- **Git Integration**: Track test configurations and results alongside your code
- **Offline Operation**: Run tests without internet connectivity, including AI features
- **Cross-Platform**: Works on Linux, macOS, and Windows

Whether you're a developer, QA engineer, or DevOps professional, QitOps provides the tools you need to ensure software quality throughout the development lifecycle.

### QitOps vs. Other Testing Tools

| Feature | QitOps | Postman | k6 | JMeter | Cypress |
|---------|--------|---------|-------|--------|---------|
| API Testing | ✅ | ✅ | ✅ | ✅ | ✅ |
| Performance Testing | ✅ | ❌ | ✅ | ✅ | ❌ |
| Security Testing | ✅ | ❌ | ❌ | ❌ | ❌ |
| Web Testing | ✅ | ❌ | ❌ | ❌ | ✅ |
| CLI-First | ✅ | ❌ | ✅ | ❌ | ❌ |
| Local AI Features | ✅ | ❌ | ❌ | ❌ | ❌ |
| Offline Operation | ✅ | ❌ | ✅ | ✅ | ❌ |
| Resource Usage | Low | High | Medium | High | High |
| CI/CD Integration | ✅ | ✅ | ✅ | ✅ | ✅ |
| Data-Driven Testing | ✅ | ✅ | ✅ | ✅ | ✅ |
| Open Source | ✅ | ❌ | ✅ | ✅ | ✅ |

QitOps combines the best features of multiple tools into a single, unified CLI tool with the added power of local AI assistance.

### Key Use Cases

- **API Development & Testing**: Validate APIs during development with comprehensive test suites
- **Microservices Testing**: Test interactions between microservices with API collections
- **Performance Benchmarking**: Measure and optimize API performance under various load conditions
- **Security Compliance**: Scan APIs and web applications for security vulnerabilities
- **Continuous Integration**: Automate testing in CI/CD pipelines with CLI-first approach
- **Regression Testing**: Ensure new code changes don't break existing functionality
- **End-to-End Testing**: Combine API and web testing for complete user journey validation
- **Test Generation**: Use AI to quickly generate test configurations from descriptions
- **Test Analysis**: Get AI-powered insights and improvement suggestions for your tests

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

The AI features require a local LLM to be available. QitOps supports various local LLM models and can work completely offline with no data sent to external services.

#### Supported LLM Models

- LLaMA (versions 1, 2, 3)
- Mistral (versions 7B, 8x7B)
- GPT-J
- Phi (versions 1, 2, 3)
- Any GGUF-compatible model

#### Local LLM Integration Options

1. **Direct Model Loading**: Load models directly from local files
2. **Ollama Integration**: Connect to Ollama for local model inference
3. **Custom Model Path**: Specify a custom path to your model files

```bash
# Run with a specific model
qitops generate --test-type api --description "Test description" --model llama --model-path /path/to/model.gguf

# Run with Ollama
export QITOPS_OLLAMA_URL="http://localhost:11434"
qitops generate --test-type api --description "Test description" --model ollama:llama2

# Run in offline mode
export QITOPS_OFFLINE=true
export QITOPS_MODEL_PATH="/path/to/model.gguf"
qitops analyze --results results.json --output analysis.md
```

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

### API Collections
- Group related API requests in a single configuration
- Define dependencies between requests
- Capture and use data from previous responses using JSONPath
- Variable interpolation with {{variable}} syntax
- Environment variables and environment-specific configurations

### Performance Testing
- Load testing with configurable concurrent users
- Response time analysis
- Success rate monitoring
- Ramp-up time configuration
- Detailed performance metrics

### Security Testing
- Comprehensive security scanning
- Multiple scan types (headers, SSL, vulnerabilities, sensitive data)
- Severity-based reporting
- Authentication testing
- Common vulnerability checks

### Web Testing
- Headless browser automation
- Viewport configuration
- Screenshot capture
- Element assertions
- Text content validation
- URL and title validation

### AI-Powered Features
- **Test Configuration Generation**: Create test configurations from natural language descriptions
  ```bash
  qitops generate --test-type api --description "Test the GitHub API to fetch user information" --output github_test.json
  ```

- **Test Results Analysis**: Analyze test results to identify patterns and issues
  ```bash
  qitops analyze --results test_results.json --output analysis.md
  ```

- **Improvement Suggestions**: Get actionable suggestions to improve your tests
  ```bash
  qitops improve --results test_results.json --output improvements.md
  ```

- **Local LLM Support**: Works with various local models (LLaMA, Mistral, GPT-J, Phi)
  ```bash
  qitops generate --test-type api --description "Test description" --model llama --model-path /path/to/model.gguf
  ```

- **Model Parameter Customization**: Configure temperature, context size, and other parameters
  ```bash
  qitops generate --test-type api --description "Test description" --temperature 0.7 --context-size 4096
  ```

- **Offline Operation**: Run completely offline with no data sent to external services
  ```bash
  export QITOPS_OFFLINE=true
  export QITOPS_MODEL_PATH="/path/to/model.gguf"
  qitops analyze --results test_results.json --output analysis.md
  ```

## Configuration

See the [Configuration Reference](configuration.md) for detailed information on configuring QitOps for different test types.

## Usage

See the [Usage Guide](usage.md) for detailed information on using QitOps for different testing scenarios.

## Best Practices

See the [Best Practices Guide](best-practices.md) for recommendations on how to use QitOps effectively.

## Contributing

Contributions are welcome! Please see our [Contributing Guide](contributing.md) for details on how to contribute to QitOps.

## License

QitOps is licensed under the MIT License. See the [LICENSE](LICENSE.md) file for details.
