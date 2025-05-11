# Data-Driven Testing

Data-driven testing allows you to run the same test with multiple sets of data, making it easy to test different scenarios without duplicating test configurations. QitOps provides robust support for data-driven testing with CSV and JSON data sources.

## Overview

Data-driven testing in QitOps works by:

1. Creating a test configuration template with placeholders
2. Providing a data source (CSV or JSON) with values for the placeholders
3. Running the test for each row/entry in the data source
4. Generating a report with results for each iteration

This approach is particularly useful for:
- Testing APIs with different input parameters
- Validating business logic with multiple test cases
- Performance testing with various payload sizes
- Security testing with different authentication credentials
- Web testing with different user journeys

## Getting Started

### Basic Usage

```bash
# Run data-driven tests with CSV data
qitops data-driven -c tests/configs/data_driven_api_test.json -d tests/data/users.csv -t csv

# Run data-driven tests with JSON data
qitops data-driven -c tests/configs/data_driven_collection.json -d tests/data/products.json -t json
```

### Command-Line Options

| Option | Description |
|--------|-------------|
| `-c, --config <FILE>` | Test configuration file |
| `-d, --data <FILE>` | Data source file (CSV or JSON) |
| `-t, --type <TYPE>` | Data source type (csv or json) |
| `--limit <NUMBER>` | Maximum number of iterations to run |
| `--stop-on-failure` | Stop execution after the first failure |
| `--delay <MILLISECONDS>` | Delay between iterations in milliseconds |
| `-r, --report <FORMAT>` | Report format (json, html, xml, csv) |
| `-o, --output <FILE>` | Output file for the report |

## Configuration Templates

Test configuration templates use the `{{variable}}` syntax for placeholders that will be replaced with values from the data source.

### Example API Test Template

```json
{
  "name": "User API Test - {{username}}",
  "description": "Test the user API for {{username}}",
  "url": "https://api.example.com/users/{{username}}",
  "method": "GET",
  "headers": {
    "Authorization": "Bearer {{token}}",
    "Accept": "application/json"
  },
  "expected_status": {{expected_status}},
  "expected_body": {
    "username": "{{username}}",
    "email": "{{email}}",
    "role": "{{role}}"
  }
}
```

### Example API Collection Template

```json
{
  "name": "Product API Collection - {{product_id}}",
  "description": "Test product API flows for {{product_name}}",
  "variables": {
    "base_url": "https://api.example.com",
    "product_id": "{{product_id}}",
    "product_name": "{{product_name}}",
    "price": {{price}}
  },
  "requests": [
    {
      "name": "Get Product",
      "id": "get-product",
      "url": "{{base_url}}/products/{{product_id}}",
      "method": "GET",
      "expected_status": 200,
      "expected_body": {
        "id": "{{product_id}}",
        "name": "{{product_name}}",
        "price": {{price}}
      },
      "capture": {
        "product_url": "$.url"
      }
    },
    {
      "name": "Get Product Reviews",
      "id": "get-product-reviews",
      "url": "{{product_url}}/reviews",
      "method": "GET",
      "depends_on": ["get-product"],
      "expected_status": 200
    }
  ]
}
```

## Data Sources

### CSV Data Source

CSV files should have a header row with column names that match the placeholder names in the template.

Example `users.csv`:
```csv
username,email,role,token,expected_status
user1,user1@example.com,user,token123,200
user2,user2@example.com,admin,token456,200
nonexistent,,,token789,404
```

### JSON Data Source

JSON data sources can be either an array of objects or a single object with arrays as values.

Example `products.json` (array of objects):
```json
[
  {
    "product_id": "prod-001",
    "product_name": "Smartphone",
    "price": 999.99,
    "category": "electronics"
  },
  {
    "product_id": "prod-002",
    "product_name": "Laptop",
    "price": 1499.99,
    "category": "electronics"
  },
  {
    "product_id": "prod-003",
    "product_name": "Headphones",
    "price": 199.99,
    "category": "accessories"
  }
]
```

Example `test_matrix.json` (object with arrays):
```json
{
  "product_id": ["prod-001", "prod-002", "prod-003"],
  "product_name": ["Smartphone", "Laptop", "Headphones"],
  "price": [999.99, 1499.99, 199.99],
  "category": ["electronics", "electronics", "accessories"]
}
```

## Advanced Features

### Iteration Control

Control the number of iterations and behavior on failure:

```bash
# Limit to first 10 iterations
qitops data-driven -c config.json -d data.csv -t csv --limit 10

# Stop on first failure
qitops data-driven -c config.json -d data.csv -t csv --stop-on-failure

# Add delay between iterations (500ms)
qitops data-driven -c config.json -d data.csv -t csv --delay 500
```

### Data Transformation

QitOps supports basic data transformation using template functions:

```json
{
  "url": "https://api.example.com/users/{{lowercase:username}}",
  "headers": {
    "Authorization": "Bearer {{uppercase:token}}"
  },
  "body": {
    "timestamp": "{{now}}",
    "random_id": "{{uuid}}",
    "formatted_price": "{{format_number:price}}"
  }
}
```

Available functions:
- `lowercase`: Convert to lowercase
- `uppercase`: Convert to uppercase
- `now`: Current timestamp
- `uuid`: Generate a UUID
- `format_number`: Format a number with commas
- `base64_encode`: Encode to Base64
- `base64_decode`: Decode from Base64
- `url_encode`: URL encode a string
- `url_decode`: URL decode a string

### Environment Variables

You can use environment variables in your data sources:

```bash
# Set environment variables
export API_TOKEN="your-secret-token"
export API_URL="https://api.example.com"

# Run with environment variables
qitops data-driven -c config.json -d data.csv -t csv
```

In your CSV file:
```csv
username,token,api_url
user1,${API_TOKEN},${API_URL}/users
user2,${API_TOKEN},${API_URL}/admin
```

## Reporting

Generate detailed reports for data-driven tests:

```bash
# Generate JSON report
qitops data-driven -c config.json -d data.csv -t csv -r json -o report.json

# Generate HTML report
qitops data-driven -c config.json -d data.csv -t csv -r html -o report.html

# Generate CSV report
qitops data-driven -c config.json -d data.csv -t csv -r csv -o results.csv
```

The reports include:
- Summary of all iterations
- Individual results for each iteration
- Execution time for each iteration
- Success/failure status
- Error messages for failed iterations
- Captured values

## Best Practices

### Organizing Data Files

- Keep data files in a dedicated directory (e.g., `tests/data/`)
- Use descriptive filenames that indicate the test scenario
- Version control your data files alongside your test configurations
- Document the structure and purpose of your data files

### Test Design

- Start with a small data set during development
- Gradually add more test cases as you refine your tests
- Include edge cases and error scenarios
- Use meaningful placeholder names that reflect their purpose
- Group related tests into collections

### Performance Considerations

- For large data sets, consider using the `--limit` option during development
- Monitor memory usage when testing with very large data sets
- Use the `--delay` option to prevent overwhelming the system under test
- Consider splitting very large data sets into multiple files

## Examples

### Testing User Registration

Configuration (`user_registration.json`):
```json
{
  "name": "User Registration - {{username}}",
  "description": "Test user registration with {{email}}",
  "url": "https://api.example.com/register",
  "method": "POST",
  "headers": {
    "Content-Type": "application/json"
  },
  "body": {
    "username": "{{username}}",
    "email": "{{email}}",
    "password": "{{password}}",
    "age": {{age}}
  },
  "expected_status": {{expected_status}},
  "expected_body": {
    "success": {{expected_success}},
    "message": "{{expected_message}}"
  }
}
```

Data (`registration_data.csv`):
```csv
username,email,password,age,expected_status,expected_success,expected_message
validuser,valid@example.com,Password123,30,201,true,User registered successfully
,invalid@example.com,Password123,30,400,false,Username is required
validuser,,Password123,30,400,false,Email is required
validuser,valid@example.com,,30,400,false,Password is required
validuser,valid@example.com,Password123,17,400,false,Age must be 18 or older
```

Command:
```bash
qitops data-driven -c user_registration.json -d registration_data.csv -t csv -r html -o registration_report.html
```

### Testing Product Search

Configuration (`product_search.json`):
```json
{
  "name": "Product Search - {{query}}",
  "description": "Test product search with query '{{query}}'",
  "url": "https://api.example.com/products/search?q={{url_encode:query}}&category={{category}}&min_price={{min_price}}&max_price={{max_price}}",
  "method": "GET",
  "headers": {
    "Accept": "application/json"
  },
  "expected_status": 200,
  "assertions": [
    {
      "type": "json",
      "path": "$.total_results",
      "comparison": ">=",
      "value": {{min_expected_results}}
    },
    {
      "type": "json",
      "path": "$.products[0].category",
      "comparison": "equals",
      "value": "{{category}}"
    }
  ]
}
```

Data (`search_queries.json`):
```json
[
  {
    "query": "smartphone",
    "category": "electronics",
    "min_price": 500,
    "max_price": 2000,
    "min_expected_results": 5
  },
  {
    "query": "laptop",
    "category": "electronics",
    "min_price": 1000,
    "max_price": 3000,
    "min_expected_results": 3
  },
  {
    "query": "headphones",
    "category": "accessories",
    "min_price": 50,
    "max_price": 500,
    "min_expected_results": 10
  }
]
```

Command:
```bash
qitops data-driven -c product_search.json -d search_queries.json -t json -r json -o search_results.json
```
