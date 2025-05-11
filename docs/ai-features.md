# AI Features

QitOps includes powerful AI features that can help you generate test configurations, analyze test results, and suggest improvements to your tests. These features are designed to work completely offline with local LLM models, ensuring your data never leaves your machine.

## Installation

To use the AI features, you need to install QitOps with the `ai` feature flag:

```bash
# Install from crates.io with AI features
cargo install qitops --features ai

# Or build from source with AI features
cargo build --features ai
```

## Supported LLM Models {#supported-llm-models}

QitOps supports a variety of local LLM models for its AI features. These models run entirely on your local machine, ensuring privacy and offline operation.

### LLaMA Models
- **LLaMA 1**: The original Meta AI model (7B, 13B, 33B, 65B parameters)
- **LLaMA 2**: Improved version with longer context (7B, 13B, 70B parameters)
- **LLaMA 3**: Latest version with enhanced capabilities (8B, 70B parameters)
- **Code LLaMA**: Specialized for code generation and analysis

### Mistral Models
- **Mistral 7B**: Efficient base model with strong performance
- **Mixtral 8x7B**: Mixture-of-experts model with enhanced capabilities
- **Mistral Instruct**: Fine-tuned for instruction following
- **Mistral Small**: Smaller, faster models for resource-constrained environments

### Phi Models
- **Phi-1**: Microsoft's small but capable model (1.3B parameters)
- **Phi-2**: Improved version with enhanced reasoning (2.7B parameters)
- **Phi-3**: Latest version with advanced capabilities (3.8B, 14B parameters)

### Other Supported Models
- **GPT-J**: EleutherAI's open-source GPT model (6B parameters)
- **GPT4All**: Locally running assistant models
- **Falcon**: Technology Innovation Institute's models
- **MPT**: MosaicML's Pretrained Transformer models
- **RWKV**: RNN with transformer-like capabilities
- **Qwen**: Alibaba's series of multilingual models

### Model Format Support
- **GGUF**: Primary supported format for efficient inference
- **GGML**: Legacy format (automatically converted to GGUF)
- **ONNX**: Support via external runtime

### Recommended Models for Different Use Cases

| Use Case | Recommended Model | Size | Performance |
|----------|------------------|------|-------------|
| Test Generation | Phi-2 | 2.7GB | Good balance of size and quality |
| Results Analysis | Mistral 7B | 4.1GB | Strong reasoning capabilities |
| Improvement Suggestions | LLaMA 2 13B | 8.2GB | Detailed, high-quality suggestions |
| Resource-Constrained | Phi-1 | 1.5GB | Works on machines with limited RAM |
| Best Quality | Mixtral 8x7B | 26GB | Highest quality results (requires 32GB+ RAM) |

### Where to Download Models

Models can be downloaded from:
- [Hugging Face](https://huggingface.co/models) - Search for GGUF versions
- [TheBloke's repositories](https://huggingface.co/TheBloke) - Pre-converted GGUF models
- [Ollama Library](https://ollama.com/library) - Easy model management

### Model Quantization Options

QitOps supports various quantization levels to balance quality and resource usage:

| Quantization | File Size | RAM Usage | Quality | Speed |
|--------------|-----------|-----------|---------|-------|
| Q4_K_M | Smallest | Lowest | Good | Fastest |
| Q5_K_M | Small | Low | Better | Fast |
| Q6_K | Medium | Medium | Very Good | Medium |
| Q8_0 | Large | High | Excellent | Slower |

Example model selection with quantization:
```bash
# Use a smaller, faster model
qitops generate --test-type api --description "Test the login API" --model phi --model-path models/phi-2.Q4_K_M.gguf

# Use a higher quality model
qitops analyze --results test_results.json --output analysis.md --model mistral --model-path models/mistral-7b.Q8_0.gguf
```

## Local LLM Integration Options

QitOps provides several ways to integrate with local LLMs:

### 1. Direct Model Loading

Load models directly from local files:

```bash
qitops generate --test-type api --description "Test description" --model llama --model-path /path/to/model.gguf
```

### 2. Ollama Integration

Connect to Ollama for local model inference:

```bash
# Start Ollama server
ollama serve

# Pull a model
ollama pull llama2

# Use Ollama with QitOps
export QITOPS_OLLAMA_URL="http://localhost:11434"
qitops generate --test-type api --description "Test description" --model ollama:llama2
```

### 3. Custom Model Path

Specify a custom path to your model files:

```bash
qitops generate --test-type api --description "Test description" --model custom --model-path /path/to/custom/model.gguf
```

## Offline Operation

QitOps AI features can work completely offline:

```bash
# Set environment variables for offline mode
export QITOPS_OFFLINE=true
export QITOPS_MODEL_PATH="/path/to/model.gguf"

# Run AI features offline
qitops analyze --results test_results.json --output analysis.md
```

## AI Features

## Test Configuration Generation {#test-configuration-generation}

QitOps can automatically generate test configurations from natural language descriptions, saving you time and effort in creating test files manually. This feature leverages local LLMs to understand your testing requirements and produce appropriate JSON configurations.

### How It Works

1. You provide a natural language description of the test you want to create
2. QitOps processes this description using a local LLM
3. The LLM generates a complete JSON configuration file based on your description
4. The configuration is saved to the specified output file

### Basic Usage

```bash
# Generate an API test configuration
qitops generate --test-type api --description "Test the GitHub API to fetch user information" --output github_test.json

# Generate a performance test configuration
qitops generate --test-type performance --description "Load test for an e-commerce checkout API with 100 concurrent users" --output perf_test.json

# Generate a security test configuration
qitops generate --test-type security --description "Security scan for a banking API" --output security_test.json

# Generate a web test configuration
qitops generate --test-type web --description "Test the checkout flow of an e-commerce website" --output web_test.json
```

### Command-Line Options

| Option | Description |
|--------|-------------|
| `--test-type <TYPE>` | Type of test to generate (api, performance, security, web) |
| `--description <TEXT>` | Natural language description of the test |
| `--output <FILE>` | Output file for the generated configuration |
| `--model <MODEL>` | LLM model to use (default: auto) |
| `--model-path <PATH>` | Path to the model file |
| `--temperature <FLOAT>` | Temperature for generation (0.0-1.0) |
| `--context-size <INT>` | Context size in tokens |
| `--max-tokens <INT>` | Maximum tokens for generation |
| `--system-prompt <TEXT>` | Custom system prompt for the model |

### Writing Effective Descriptions

The quality of the generated configuration depends on the clarity and specificity of your description. Here are some tips for writing effective descriptions:

#### For API Tests:
- Specify the endpoint URL and HTTP method
- Mention any required headers or authentication
- Describe the expected response status and content
- Include any specific validation requirements

Example: "Test the GitHub API to fetch user information for 'octocat' using GET request to /users/octocat endpoint. Verify that the response status is 200 and the response contains the correct login name and user type."

#### For Performance Tests:
- Specify the target URL and HTTP method
- Mention the number of concurrent users
- Describe the test duration and ramp-up time
- Include any performance thresholds

Example: "Load test the checkout API at https://api.example.com/checkout with 100 concurrent users for 5 minutes. Use a 30-second ramp-up time. The API should handle at least 50 requests per second with a response time under 200ms."

#### For Security Tests:
- Specify the target URL or application
- Mention the types of security checks to perform
- Describe any authentication requirements
- Include any specific vulnerability concerns

Example: "Security scan the banking API at https://api.bank.com/v1 focusing on SQL injection, XSS, and authentication vulnerabilities. Use Bearer token authentication and check for sensitive data exposure in responses."

#### For Web Tests:
- Specify the target website URL
- Describe the user journey or actions to test
- Mention any specific elements to interact with
- Include validation criteria

Example: "Test the checkout flow of an e-commerce website at https://shop.example.com. Add a product to the cart, proceed to checkout, fill in shipping and payment information, and complete the purchase. Verify that the order confirmation page shows the correct order details."

### Example Generated Configurations

#### API Test Configuration

```json
{
  "name": "GitHub User API Test",
  "description": "Test the GitHub API to fetch user information",
  "url": "https://api.github.com/users/octocat",
  "method": "GET",
  "headers": {
    "Accept": "application/vnd.github.v3+json",
    "User-Agent": "QitOps-Test"
  },
  "expected_status": 200,
  "expected_body": {
    "login": "octocat",
    "type": "User"
  },
  "timeout": 30,
  "retries": 3
}
```

#### Performance Test Configuration

```json
{
  "name": "E-commerce Checkout API Load Test",
  "description": "Load test for an e-commerce checkout API with 100 concurrent users",
  "target_url": "https://api.example.com/checkout",
  "method": "POST",
  "headers": {
    "Content-Type": "application/json",
    "Accept": "application/json"
  },
  "body": {
    "cart_id": "{{cart_id}}",
    "payment_method": "credit_card",
    "shipping_method": "standard"
  },
  "concurrent_users": 100,
  "duration_secs": 300,
  "ramp_up_time_secs": 30,
  "success_threshold": 95.0,
  "max_response_time_ms": 200,
  "requests_per_second_threshold": 50
}
```

### Customizing Generation

You can customize the generation process by adjusting the model parameters:

```bash
# Use a specific model with custom parameters
qitops generate --test-type api \
  --description "Test the login API with different credentials" \
  --output login_test.json \
  --model llama \
  --model-path models/llama-2-7b.gguf \
  --temperature 0.8 \
  --context-size 4096 \
  --max-tokens 2048
```

### Post-Generation Editing

Generated configurations are meant to be starting points. You may want to:

1. Review and edit the generated configuration
2. Add or modify specific fields
3. Adjust validation criteria
4. Add environment-specific variables

### Best Practices

- Start with detailed, specific descriptions
- Review generated configurations before using them
- Use lower temperature (0.3-0.5) for more deterministic results
- Use higher temperature (0.7-0.9) for more creative variations
- Keep descriptions focused on one test scenario at a time
- Specify concrete details like URLs, methods, and expected responses
- Use the generated configurations as starting points, not final products

## Test Results Analysis {#test-results-analysis}

QitOps can analyze your test results using AI to identify patterns, issues, and insights that might not be immediately obvious. This feature helps you understand test outcomes and make data-driven decisions about your testing strategy.

### How It Works

1. You provide a JSON file containing test results
2. QitOps processes these results using a local LLM
3. The LLM generates a comprehensive analysis report
4. The analysis is saved to the specified output file (typically in Markdown format)

### Basic Usage

```bash
# Analyze test results
qitops analyze --results test_results.json --output analysis.md

# Analyze with a specific model
qitops analyze --results test_results.json --output analysis.md --model llama --model-path /path/to/model.gguf
```

### Command-Line Options

| Option | Description |
|--------|-------------|
| `--results <FILE>` | JSON file containing test results |
| `--output <FILE>` | Output file for the analysis report |
| `--model <MODEL>` | LLM model to use (default: auto) |
| `--model-path <PATH>` | Path to the model file |
| `--format <FORMAT>` | Output format (markdown, html, json) |
| `--temperature <FLOAT>` | Temperature for generation (0.0-1.0) |
| `--context-size <INT>` | Context size in tokens |
| `--max-tokens <INT>` | Maximum tokens for generation |
| `--system-prompt <TEXT>` | Custom system prompt for the model |

### Analysis Content

The generated analysis report typically includes:

#### 1. Executive Summary
- Overall success rate
- Number of tests run
- Key findings and insights
- Critical issues identified

#### 2. Test Results Overview
- Distribution of test statuses (success, failure, error)
- Average response times
- Success rates by test type
- Temporal patterns (if timestamps are available)

#### 3. Detailed Breakdown
- Analysis of each failed test
- Common failure patterns
- Root cause analysis
- Error categorization

#### 4. Performance Analysis
- Response time statistics
- Performance bottlenecks
- Throughput analysis
- Resource utilization patterns

#### 5. Recommendations
- Suggestions for fixing failed tests
- Performance optimization opportunities
- Test coverage improvements
- Testing strategy recommendations

### Example Analysis Report

```markdown
# Test Results Analysis

## Executive Summary

Analyzed 25 test results with an overall success rate of 84% (21/25 tests passed).
The average response time was 187ms, with 3 tests exceeding the 500ms threshold.
The main issues were related to authentication failures and timeout errors.

## Test Results Overview

| Status | Count | Percentage |
|--------|-------|------------|
| Success | 21 | 84% |
| Failure | 3 | 12% |
| Error | 1 | 4% |

### Success Rate by Test Type
- API Tests: 90% (18/20)
- Performance Tests: 75% (3/4)
- Security Tests: 0% (0/1)

## Detailed Breakdown

### Failed Tests Analysis

1. **API-Test-003**: Authentication failure
   - Error: "Unauthorized (401)"
   - Possible cause: Expired API token or incorrect credentials
   - Recommendation: Verify and update the authentication token

2. **API-Test-012**: Unexpected response format
   - Error: "Expected field 'user_id' not found in response"
   - Possible cause: API response structure has changed
   - Recommendation: Update the expected response structure in the test configuration

3. **Perf-Test-002**: Timeout error
   - Error: "Request timed out after 30 seconds"
   - Possible cause: Server under heavy load or network issues
   - Recommendation: Increase the timeout threshold or investigate server performance

## Performance Analysis

- 90th percentile response time: 342ms
- Slowest endpoint: /api/users/search (avg: 412ms)
- Fastest endpoint: /api/health (avg: 42ms)
- 3 tests exceeded the 500ms threshold

## Recommendations

1. **Fix Authentication Issues**
   - Implement token refresh mechanism
   - Add pre-test validation of authentication credentials

2. **Improve Error Handling**
   - Add more specific assertions for response structure
   - Implement retry logic for transient failures

3. **Performance Optimization**
   - Review the /api/users/search endpoint for optimization opportunities
   - Consider caching frequently accessed resources
   - Implement pagination for large result sets

4. **Test Coverage Improvements**
   - Add more security tests (currently only 1)
   - Increase test coverage for error conditions
   - Add tests for edge cases identified in this analysis
```

### Input Format Requirements

The analysis feature expects test results in a specific JSON format:

```json
[
  {
    "test_id": "api-test-001",
    "name": "User API Test",
    "description": "Test the user API endpoint",
    "timestamp": "2025-05-10T12:34:56Z",
    "duration_ms": 156,
    "status": "success",
    "url": "https://api.example.com/users/123",
    "method": "GET",
    "request_headers": {
      "Accept": "application/json",
      "Authorization": "Bearer token123"
    },
    "response_status": 200,
    "response_headers": {
      "content-type": "application/json"
    },
    "assertions": [
      {
        "type": "status",
        "expected": 200,
        "actual": 200,
        "result": "pass"
      },
      {
        "type": "json",
        "path": "$.id",
        "expected": 123,
        "actual": 123,
        "result": "pass"
      }
    ]
  },
  {
    "test_id": "api-test-002",
    "name": "Invalid User API Test",
    "description": "Test the user API with invalid ID",
    "timestamp": "2025-05-10T12:35:12Z",
    "duration_ms": 134,
    "status": "failure",
    "url": "https://api.example.com/users/999",
    "method": "GET",
    "request_headers": {
      "Accept": "application/json",
      "Authorization": "Bearer token123"
    },
    "response_status": 404,
    "response_headers": {
      "content-type": "application/json"
    },
    "assertions": [
      {
        "type": "status",
        "expected": 200,
        "actual": 404,
        "result": "fail"
      }
    ],
    "error": "Expected status 200 but got 404"
  }
]
```

### Best Practices

- Include as much detail as possible in your test results
- Run analysis after each test suite execution
- Compare analyses over time to identify trends
- Use the analysis to prioritize test improvements
- Share analysis reports with your team for collaborative debugging
- Customize the analysis by adjusting the temperature parameter
- Use lower temperatures (0.3-0.5) for more factual, concise analyses
- Use higher temperatures (0.7-0.9) for more creative, detailed analyses

## Improvement Suggestions {#improvement-suggestions}

QitOps can generate actionable suggestions to improve your tests based on test results and industry best practices. This feature helps you continuously enhance your testing strategy and address issues before they become critical problems.

### How It Works

1. You provide a JSON file containing test results
2. QitOps analyzes these results using a local LLM
3. The LLM generates specific, actionable improvement suggestions
4. The suggestions are saved to the specified output file

### Basic Usage

```bash
# Generate improvement suggestions
qitops improve --results test_results.json --output improvements.md

# Generate suggestions with a specific model
qitops improve --results test_results.json --output improvements.md --model llama --model-path /path/to/model.gguf
```

### Command-Line Options

| Option | Description |
|--------|-------------|
| `--results <FILE>` | JSON file containing test results |
| `--output <FILE>` | Output file for the improvement suggestions |
| `--model <MODEL>` | LLM model to use (default: auto) |
| `--model-path <PATH>` | Path to the model file |
| `--format <FORMAT>` | Output format (markdown, html, json) |
| `--focus <AREA>` | Focus area for suggestions (performance, reliability, coverage, all) |
| `--temperature <FLOAT>` | Temperature for generation (0.0-1.0) |
| `--context-size <INT>` | Context size in tokens |
| `--max-tokens <INT>` | Maximum tokens for generation |
| `--system-prompt <TEXT>` | Custom system prompt for the model |

### Improvement Categories

The generated suggestions typically cover the following areas:

#### 1. Performance Optimizations
- Response time improvements
- Resource utilization efficiency
- Concurrency and parallelism recommendations
- Caching strategies
- Database query optimizations
- Network latency reduction

#### 2. Reliability Enhancements
- Error handling improvements
- Retry mechanisms
- Timeout configurations
- Fault tolerance strategies
- Resilience patterns
- Circuit breaker implementations

#### 3. Coverage Improvements
- Missing test scenarios
- Edge case coverage
- Error condition testing
- Boundary value testing
- Security vulnerability testing
- Cross-functional testing

#### 4. Best Practices
- Test organization and structure
- Naming conventions
- Documentation improvements
- Assertion strategies
- Test data management
- CI/CD integration

### Example Improvement Suggestions

```markdown
# Test Improvement Suggestions

## Executive Summary

Based on the analysis of 25 test results, we've identified several opportunities for improvement across performance, reliability, coverage, and best practices. Implementing these suggestions will help enhance the effectiveness and efficiency of your testing suite.

## Performance Optimizations

1. **Optimize Slow Endpoints**
   - The `/api/users/search` endpoint has an average response time of 412ms, which is significantly higher than other endpoints.
   - Suggestion: Review the implementation of this endpoint, focusing on database query optimization and potential caching.
   - Implementation: Add query indexing for frequently searched fields and implement a Redis cache for common search queries.

2. **Implement Connection Pooling**
   - Multiple tests show connection establishment overhead.
   - Suggestion: Implement connection pooling to reuse connections across tests.
   - Implementation: Configure a connection pool with appropriate min/max settings based on your concurrency requirements.

3. **Reduce Payload Sizes**
   - Several API responses exceed 100KB in size.
   - Suggestion: Implement pagination and field filtering to reduce payload sizes.
   - Implementation: Add `limit`, `offset`, and `fields` query parameters to your API endpoints.

## Reliability Enhancements

1. **Improve Authentication Handling**
   - 3 tests failed due to authentication issues.
   - Suggestion: Implement token refresh and pre-test authentication validation.
   - Implementation: Add a token refresh mechanism that automatically renews expired tokens before test execution.

2. **Add Retry Logic for Transient Failures**
   - Several timeout errors appear to be transient.
   - Suggestion: Implement exponential backoff retry logic for network-related failures.
   - Implementation: Add a retry decorator/wrapper that retries failed requests with increasing delays.

3. **Enhance Error Handling**
   - Error messages are inconsistent and sometimes not actionable.
   - Suggestion: Standardize error handling and improve error messages.
   - Implementation: Create a centralized error handler that provides consistent, detailed error information.

## Coverage Improvements

1. **Add Security Testing**
   - Only 1 security test was found in the results.
   - Suggestion: Expand security testing to cover authentication, authorization, input validation, and data protection.
   - Implementation: Add tests for OWASP Top 10 vulnerabilities relevant to your application.

2. **Increase Edge Case Coverage**
   - Most tests focus on happy paths with valid inputs.
   - Suggestion: Add tests for boundary conditions, invalid inputs, and error scenarios.
   - Implementation: For each API endpoint, add tests with empty values, extremely large values, special characters, and malformed requests.

3. **Add Load and Stress Testing**
   - Current performance tests use relatively low concurrency (max 100 users).
   - Suggestion: Add high-concurrency load tests and stress tests to identify breaking points.
   - Implementation: Create load test scenarios with gradually increasing concurrency until performance degrades.

## Best Practices

1. **Improve Test Organization**
   - Tests are not consistently organized by feature or functionality.
   - Suggestion: Reorganize tests into logical groups based on functionality.
   - Implementation: Create a directory structure that mirrors your application's architecture.

2. **Enhance Test Data Management**
   - Test data is hardcoded in many tests.
   - Suggestion: Implement a test data management strategy with fixtures or factories.
   - Implementation: Create a test data generator that produces consistent, realistic test data.

3. **Add Documentation**
   - Many tests lack clear documentation about their purpose and expected behavior.
   - Suggestion: Add descriptive comments and documentation to all tests.
   - Implementation: Standardize on a documentation format that includes purpose, prerequisites, and expected outcomes.

## Implementation Roadmap

We recommend implementing these improvements in the following order:

1. **Immediate (1-2 weeks)**
   - Fix authentication handling issues
   - Add retry logic for transient failures
   - Improve error messages

2. **Short-term (2-4 weeks)**
   - Optimize slow endpoints
   - Enhance test organization
   - Add basic security tests

3. **Medium-term (1-3 months)**
   - Implement connection pooling
   - Add edge case coverage
   - Improve test data management

4. **Long-term (3+ months)**
   - Reduce payload sizes
   - Add comprehensive load and stress testing
   - Enhance documentation
```

### Integration with CI/CD

You can integrate the improvement suggestions feature into your CI/CD pipeline to automatically generate suggestions after test runs:

```yaml
# Example GitHub Actions workflow step
- name: Generate Test Improvement Suggestions
  run: |
    qitops improve --results test-results.json --output improvements.md

- name: Upload Improvement Suggestions
  uses: actions/upload-artifact@v2
  with:
    name: test-improvements
    path: improvements.md
```

### Best Practices

- Run the improvement suggestions tool regularly (weekly or after major test runs)
- Prioritize suggestions based on their impact and implementation effort
- Create tickets or tasks for implementing high-priority suggestions
- Track the implementation of suggestions over time
- Re-run the tool after implementing suggestions to measure progress
- Customize the focus area based on your current priorities
- Use the suggestions as input for sprint planning and technical debt discussions
- Share the suggestions with your team to foster a culture of continuous improvement

## Model Parameter Customization

Customize model parameters to fine-tune the AI behavior:

```bash
# Set temperature (controls randomness, 0.0-1.0)
qitops generate --test-type api --description "Test description" --temperature 0.7

# Set context size (in tokens)
qitops generate --test-type api --description "Test description" --context-size 4096

# Set maximum tokens for generation
qitops generate --test-type api --description "Test description" --max-tokens 2048

# Set system prompt
qitops generate --test-type api --description "Test description" --system-prompt "You are a testing expert."
```

## Testing AI Features

You can test the AI features using the provided test script:

```bash
# Clone the repository
git clone https://github.com/qitops/qitops-cli-tools.git
cd qitops-cli-tools

# Build with AI features
cargo build --features ai

# Run the test script
./test_local_ai.sh
```

The test script will:
1. Set up environment variables for offline mode
2. Create a directory for test outputs
3. Test all AI features (generation, analysis, improvement)
4. Show the generated files

## Troubleshooting

### Model Loading Issues

If you encounter issues loading a model:

```bash
# Check if the model file exists
ls -la /path/to/model.gguf

# Try a different model format
qitops generate --test-type api --description "Test description" --model llama --model-path /path/to/different/model.gguf
```

### Ollama Connection Issues

If you have trouble connecting to Ollama:

```bash
# Check if Ollama is running
curl http://localhost:11434/api/version

# Check available models
ollama list

# Pull the model if it's not available
ollama pull llama2
```

### Memory Issues

If you encounter memory issues with large models:

```bash
# Use a smaller model
qitops generate --test-type api --description "Test description" --model phi --model-path /path/to/phi-2.gguf

# Reduce context size
qitops generate --test-type api --description "Test description" --context-size 2048
```

## Environment Variables

QitOps supports the following environment variables for AI features:

| Variable | Description | Default |
|----------|-------------|---------|
| `QITOPS_OFFLINE` | Run in offline mode | `false` |
| `QITOPS_MODEL_PATH` | Path to the model file | None |
| `QITOPS_OLLAMA_URL` | URL for Ollama server | `http://localhost:11434` |
| `QITOPS_TEMPERATURE` | Temperature for generation | `0.7` |
| `QITOPS_CONTEXT_SIZE` | Context size in tokens | `4096` |
| `QITOPS_MAX_TOKENS` | Maximum tokens for generation | `2048` |
| `QITOPS_SYSTEM_PROMPT` | System prompt for the model | Default system prompt |
