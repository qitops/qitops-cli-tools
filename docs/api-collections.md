# API Collections

QitOps provides powerful API collection capabilities that allow you to group related API requests, chain them together, and create complex test workflows.

## Overview

API collections in QitOps allow you to:

- Group related API requests into a single collection
- Share variables and authentication across requests
- Capture data from responses to use in subsequent requests
- Define dependencies between requests
- Set up pre-request and post-request scripts
- Configure environment-specific variables
- Run collections sequentially or in parallel

## Getting Started

### Basic Usage

```bash
# Run an API collection
qitops collection -c tests/configs/api_collection.json

# Run with environment variables
qitops collection -c tests/configs/api_collection.json -v API_KEY=your-api-key

# Run with a specific environment
qitops collection -c tests/configs/api_collection.json -e staging
```

### Command-Line Options

| Option | Description |
|--------|-------------|
| `-c, --config <FILE>` | API collection configuration file |
| `-e, --environment <ENV>` | Environment to use (default: production) |
| `-v, --variable <KEY=VALUE>` | Set a variable for the collection |
| `--sequential` | Run requests sequentially (default) |
| `--parallel` | Run requests in parallel where possible |
| `--stop-on-failure` | Stop execution after the first failure |
| `--delay <MILLISECONDS>` | Delay between requests in milliseconds |
| `-r, --report <FORMAT>` | Report format (json, html, xml, csv) |
| `-o, --output <FILE>` | Output file for the report |

## Configuration

API collections are defined in JSON configuration files that specify the requests, variables, and execution options.

### Basic Configuration Structure

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

### Configuration Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| name | string | Yes | Name of the collection |
| description | string | No | Description of the collection |
| version | string | No | Version of the collection |
| variables | object | No | Variables to use in the collection |
| auth | object | No | Authentication configuration |
| defaults | object | No | Default values for all requests |
| requests | array | Yes | Array of request objects |
| environments | object | No | Environment-specific variables |
| run_options | object | No | Options for running the collection |

### Request Configuration

Each request in a collection has the following structure:

```json
{
  "name": "Get User",
  "description": "Get a GitHub user",
  "id": "get-user",
  "url": "{{base_url}}/users/{{username}}",
  "method": "GET",
  "headers": {
    "Accept": "application/json"
  },
  "body": {
    "param1": "value1",
    "param2": "value2"
  },
  "expected_status": 200,
  "expected_body": {
    "login": "{{username}}",
    "type": "User"
  },
  "expected_headers": {
    "content-type": "application/json"
  },
  "capture": {
    "user_id": "$.id",
    "user_url": "$.url"
  },
  "depends_on": ["another-request-id"],
  "skip_if": "{{condition}}",
  "pre_request_script": "// JavaScript code to run before the request",
  "post_request_script": "// JavaScript code to run after the request"
}
```

### Variable Interpolation

QitOps supports variable interpolation in collection configurations:

- **Collection Variables**: `{{variable_name}}`
- **Environment Variables**: `{{ENV_VARIABLE}}`
- **Captured Variables**: `{{captured_variable}}`
- **System Variables**: `{{$timestamp}}`, `{{$random}}`

### Data Capture

You can capture data from responses to use in subsequent requests:

```json
"capture": {
  "user_id": "$.id",
  "user_url": "$.url",
  "token": "$.headers.authorization",
  "status": "$.status"
}
```

QitOps uses JSONPath expressions to extract data from responses.

### Request Dependencies

You can define dependencies between requests:

```json
"depends_on": ["get-user", "get-auth-token"]
```

Dependent requests will only run if all their dependencies have succeeded.

## Examples

### GitHub API Collection

```json
{
  "name": "GitHub API Collection",
  "description": "A collection of GitHub API tests",
  "variables": {
    "base_url": "https://api.github.com",
    "username": "octocat"
  },
  "auth": {
    "type": "bearer",
    "token": "{{GITHUB_TOKEN}}"
  },
  "defaults": {
    "headers": {
      "Accept": "application/vnd.github.v3+json",
      "User-Agent": "QitOps-Test"
    }
  },
  "requests": [
    {
      "name": "Get User",
      "id": "get-user",
      "url": "{{base_url}}/users/{{username}}",
      "method": "GET",
      "expected_status": 200,
      "capture": {
        "user_id": "$.id",
        "repos_url": "$.repos_url"
      }
    },
    {
      "name": "Get User Repos",
      "id": "get-repos",
      "url": "{{repos_url}}",
      "method": "GET",
      "depends_on": ["get-user"],
      "expected_status": 200,
      "capture": {
        "first_repo_name": "$.0.name",
        "first_repo_url": "$.0.url"
      }
    },
    {
      "name": "Get First Repo Details",
      "id": "get-repo-details",
      "url": "{{first_repo_url}}",
      "method": "GET",
      "depends_on": ["get-repos"],
      "expected_status": 200
    }
  ]
}
```

### E-commerce API Collection

```json
{
  "name": "E-commerce API Collection",
  "description": "Test e-commerce API workflow",
  "variables": {
    "base_url": "https://api.example.com",
    "product_id": "12345",
    "user_id": "user123"
  },
  "auth": {
    "type": "apikey",
    "key": "X-API-Key",
    "value": "{{API_KEY}}",
    "in": "header"
  },
  "requests": [
    {
      "name": "Get Product",
      "id": "get-product",
      "url": "{{base_url}}/products/{{product_id}}",
      "method": "GET",
      "expected_status": 200,
      "capture": {
        "product_price": "$.price",
        "product_name": "$.name"
      }
    },
    {
      "name": "Add to Cart",
      "id": "add-to-cart",
      "url": "{{base_url}}/users/{{user_id}}/cart",
      "method": "POST",
      "body": {
        "product_id": "{{product_id}}",
        "quantity": 1
      },
      "depends_on": ["get-product"],
      "expected_status": 201,
      "capture": {
        "cart_id": "$.cart_id"
      }
    },
    {
      "name": "Get Cart",
      "id": "get-cart",
      "url": "{{base_url}}/users/{{user_id}}/cart/{{cart_id}}",
      "method": "GET",
      "depends_on": ["add-to-cart"],
      "expected_status": 200,
      "expected_body": {
        "items": [
          {
            "product_id": "{{product_id}}",
            "quantity": 1,
            "price": "{{product_price}}"
          }
        ]
      }
    },
    {
      "name": "Checkout",
      "id": "checkout",
      "url": "{{base_url}}/users/{{user_id}}/cart/{{cart_id}}/checkout",
      "method": "POST",
      "body": {
        "payment_method": "credit_card",
        "shipping_address": {
          "street": "123 Main St",
          "city": "Anytown",
          "zip": "12345"
        }
      },
      "depends_on": ["get-cart"],
      "expected_status": 201,
      "capture": {
        "order_id": "$.order_id"
      }
    },
    {
      "name": "Get Order",
      "id": "get-order",
      "url": "{{base_url}}/users/{{user_id}}/orders/{{order_id}}",
      "method": "GET",
      "depends_on": ["checkout"],
      "expected_status": 200,
      "expected_body": {
        "status": "processing",
        "total": "{{product_price}}"
      }
    }
  ]
}
```

## Best Practices

### Collection Organization

- **Group Related Requests**: Group related requests in a single collection
- **Use Descriptive Names**: Use descriptive names for collections and requests
- **Include Descriptions**: Include descriptions for collections and requests
- **Organize by Workflow**: Organize collections by business workflow or feature

### Variables and Environments

- **Use Variables**: Use variables for values that change between environments
- **Define Environments**: Define environment-specific configurations
- **Avoid Hardcoding**: Avoid hardcoding sensitive information
- **Use Variable Interpolation**: Use variable interpolation for dynamic values

### Dependencies and Data Flow

- **Define Dependencies**: Define clear dependencies between requests
- **Capture Data**: Capture data from responses to use in subsequent requests
- **Handle Errors**: Configure how to handle errors in dependent requests
- **Use Pre/Post Scripts**: Use pre-request and post-request scripts for complex logic

## Integration with CI/CD

API collections can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow step
- name: Run API Collection
  run: |
    qitops collection -c tests/configs/api_collection.json -e staging --report json --output collection-results.json
    
- name: Upload Collection Results
  uses: actions/upload-artifact@v2
  with:
    name: collection-results
    path: collection-results.json
```

## Troubleshooting

### Common Issues

- **Variable Not Found**: Check that all variables are defined
- **Dependency Failure**: Check that dependent requests are successful
- **Capture Failure**: Check that JSONPath expressions are correct
- **Authentication Issues**: Verify authentication credentials
- **Environment Mismatch**: Check that environment-specific variables are correct
