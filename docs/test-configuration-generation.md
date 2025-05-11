# Test Configuration Generation

QitOps can automatically generate test configurations from natural language descriptions, saving you time and effort in creating test files manually. This feature leverages local LLMs to understand your testing requirements and produce appropriate JSON configurations.

## How It Works

1. You provide a natural language description of the test you want to create
2. QitOps processes this description using a local LLM
3. The LLM generates a complete JSON configuration file based on your description
4. The configuration is saved to the specified output file

## Basic Usage

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

## Command-Line Options

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

## Writing Effective Descriptions

The quality of the generated configuration depends on the clarity and specificity of your description. Here are some tips for writing effective descriptions:

### For API Tests:
- Specify the endpoint URL and HTTP method
- Mention any required headers or authentication
- Describe the expected response status and content
- Include any specific validation requirements

Example: "Test the GitHub API to fetch user information for 'octocat' using GET request to /users/octocat endpoint. Verify that the response status is 200 and the response contains the correct login name and user type."

### For Performance Tests:
- Specify the target URL and HTTP method
- Mention the number of concurrent users
- Describe the test duration and ramp-up time
- Include any performance thresholds

Example: "Load test the checkout API at https://api.example.com/checkout with 100 concurrent users for 5 minutes. Use a 30-second ramp-up time. The API should handle at least 50 requests per second with a response time under 200ms."

### For Security Tests:
- Specify the target URL or application
- Mention the types of security checks to perform
- Describe any authentication requirements
- Include any specific vulnerability concerns

Example: "Security scan the banking API at https://api.bank.com/v1 focusing on SQL injection, XSS, and authentication vulnerabilities. Use Bearer token authentication and check for sensitive data exposure in responses."

### For Web Tests:
- Specify the target website URL
- Describe the user journey or actions to test
- Mention any specific elements to interact with
- Include validation criteria

Example: "Test the checkout flow of an e-commerce website at https://shop.example.com. Add a product to the cart, proceed to checkout, fill in shipping and payment information, and complete the purchase. Verify that the order confirmation page shows the correct order details."

## Example Generated Configurations

### API Test Configuration

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

### Performance Test Configuration

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

## Customizing Generation

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

## Post-Generation Editing

Generated configurations are meant to be starting points. You may want to:

1. Review and edit the generated configuration
2. Add or modify specific fields
3. Adjust validation criteria
4. Add environment-specific variables

## Best Practices

- Start with detailed, specific descriptions
- Review generated configurations before using them
- Use lower temperature (0.3-0.5) for more deterministic results
- Use higher temperature (0.7-0.9) for more creative variations
- Keep descriptions focused on one test scenario at a time
- Specify concrete details like URLs, methods, and expected responses
- Use the generated configurations as starting points, not final products
