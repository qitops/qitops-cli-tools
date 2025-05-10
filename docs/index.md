# QitOps Documentation

QitOps is a comprehensive Software Quality Assurance CLI tool for API, Performance, Security, and Web Testing. It provides a unified command-line interface with minimal dependencies and maximum flexibility.

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
- Test configuration generation from natural language descriptions
- Test results analysis with insights and patterns
- Improvement suggestions based on test results
- Support for local LLM models (LLaMA, Mistral, GPT-J, Phi)
- Customizable model parameters (temperature, context size, etc.)
- Offline operation with no data sent to external services

## Configuration

See the [Configuration Reference](configuration.md) for detailed information on configuring QitOps for different test types.

## Usage

See the [Usage Guide](usage.md) for detailed information on using QitOps for different testing scenarios.

## Best Practices

See the [Best Practices Guide](best-practices.md) for recommendations on how to use QitOps effectively.

## Contributing

Contributions are welcome! Please see our [Contributing Guide](../CONTRIBUTING.md) for details on how to contribute to QitOps.

## License

QitOps is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.
