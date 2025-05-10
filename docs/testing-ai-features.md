# Testing AI Features

This guide provides detailed instructions on how to test the AI features of QitOps locally. These features include test configuration generation, test results analysis, and improvement suggestions.

## Prerequisites

Before testing the AI features, make sure you have:

1. **QitOps built with AI features**:
   ```bash
   cargo build --features ai
   ```

2. **A local LLM setup** (one of the following):
   - **Ollama**: For easy local LLM management
   - **Direct model files**: GGUF format models
   - **Mock implementation**: For testing without a real model

## Option 1: Testing with Ollama

[Ollama](https://ollama.ai/) provides an easy way to run local LLMs. It's recommended for testing as it handles model management and provides a simple API.

### Step 1: Install Ollama

```bash
# Linux
curl -fsSL https://ollama.ai/install.sh | sh

# macOS
brew install ollama

# Windows
# Download from https://ollama.ai/download
```

### Step 2: Start Ollama Server

```bash
ollama serve
```

### Step 3: Pull a Model

Pull a small model for testing:

```bash
# Pull a small model (recommended for testing)
ollama pull phi

# Or pull a larger model for better results
ollama pull llama2
```

### Step 4: Test AI Features with Ollama

```bash
# Test configuration generation
cargo run --features ai -- generate --test-type api --description "Test the Twitter API to fetch user timeline" --output twitter_api_test.json --model ollama:phi

# Test results analysis
cargo run --features ai -- analyze --results sample_test_results.json --output test_analysis.md --model ollama:phi

# Test improvement suggestions
cargo run --features ai -- improve --results sample_test_results.json --output test_improvements.md --model ollama:phi
```

## Option 2: Testing with Direct Model Files

If you prefer to use model files directly, you can download GGUF format models and use them with QitOps.

### Step 1: Download a Model

Download a GGUF model from Hugging Face or other sources:

```bash
# Create a models directory
mkdir -p models

# Download a model (example using wget)
wget https://huggingface.co/TheBloke/phi-2-GGUF/resolve/main/phi-2.Q4_K_M.gguf -O models/phi-2.gguf
```

### Step 2: Test AI Features with Direct Model Files

```bash
# Test configuration generation
cargo run --features ai -- generate --test-type api --description "Test the Twitter API to fetch user timeline" --output twitter_api_test.json --model custom --model-path models/phi-2.gguf

# Test results analysis
cargo run --features ai -- analyze --results sample_test_results.json --output test_analysis.md --model custom --model-path models/phi-2.gguf

# Test improvement suggestions
cargo run --features ai -- improve --results sample_test_results.json --output test_improvements.md --model custom --model-path models/phi-2.gguf
```

## Option 3: Testing with Mock Implementation

For quick testing without a real model, you can use the built-in mock implementation.

### Step 1: Create Sample Test Results

Create a sample test results file for testing analysis and improvement features:

```bash
cat > sample_test_results.json << EOF
[
  {
    "test_id": "api-test-1",
    "name": "GitHub User API Test",
    "description": "Test the GitHub API to fetch user information",
    "timestamp": "2025-05-10T21:15:00Z",
    "duration_ms": 190,
    "status": "success",
    "url": "https://api.github.com/users/octocat",
    "method": "GET",
    "request_headers": {
      "Accept": "application/vnd.github.v3+json",
      "User-Agent": "QitOps-Test"
    },
    "response_status": 200,
    "response_headers": {
      "content-type": "application/json; charset=utf-8",
      "cache-control": "public, max-age=60, s-maxage=60"
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
        "path": "$.login",
        "expected": "octocat",
        "actual": "octocat",
        "result": "pass"
      }
    ]
  },
  {
    "test_id": "api-test-2",
    "name": "GitHub Non-existent User Test",
    "description": "Test the GitHub API with a non-existent user",
    "timestamp": "2025-05-10T21:15:02Z",
    "duration_ms": 180,
    "status": "failure",
    "url": "https://api.github.com/users/non-existent-user-12345",
    "method": "GET",
    "request_headers": {
      "Accept": "application/vnd.github.v3+json",
      "User-Agent": "QitOps-Test"
    },
    "response_status": 404,
    "response_headers": {
      "content-type": "application/json; charset=utf-8",
      "cache-control": "public, max-age=60, s-maxage=60"
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
EOF
```

### Step 2: Test AI Features with Mock Implementation

```bash
# Test configuration generation
cargo run --features ai -- generate --test-type api --description "Test the Twitter API to fetch user timeline" --output twitter_api_test.json

# Test results analysis
cargo run --features ai -- analyze --results sample_test_results.json --output test_analysis.md

# Test improvement suggestions
cargo run --features ai -- improve --results sample_test_results.json --output test_improvements.md
```

## Automated Testing Script

For convenience, you can use the provided test script to test all AI features at once:

```bash
# Download the test script
curl -O https://raw.githubusercontent.com/qitops/qitops-cli-tools/master/test_local_ai.sh
chmod +x test_local_ai.sh

# Run the test script
./test_local_ai.sh
```

The test script will:
1. Set up environment variables for offline mode
2. Create a directory for test outputs
3. Test all AI features (generation, analysis, improvement)
4. Show the generated files

## Verifying Test Results

After running the tests, you should check the generated files:

```bash
# Check the generated API test configuration
cat twitter_api_test.json

# Check the test analysis
cat test_analysis.md

# Check the improvement suggestions
cat test_improvements.md
```

The generated files should contain:

1. **API Test Configuration**:
   - URL, method, headers, and assertions
   - Properly formatted JSON

2. **Test Analysis**:
   - Overview of test results
   - Details about performance and status codes
   - Recommendations for improvement

3. **Improvement Suggestions**:
   - Performance optimizations
   - Reliability enhancements
   - Coverage improvements

## Troubleshooting

### Model Loading Issues

If you encounter issues loading a model:

```bash
# Check if the model file exists
ls -la models/phi-2.gguf

# Try with verbose logging
RUST_LOG=debug cargo run --features ai -- generate --test-type api --description "Test description" --model custom --model-path models/phi-2.gguf
```

### Ollama Connection Issues

If you have trouble connecting to Ollama:

```bash
# Check if Ollama is running
curl http://localhost:11434/api/version

# Check available models
ollama list

# Pull the model if it's not available
ollama pull phi
```

### Memory Issues

If you encounter memory issues with large models:

```bash
# Use a smaller model
ollama pull tinyllama
cargo run --features ai -- generate --test-type api --description "Test description" --model ollama:tinyllama

# Or reduce context size
cargo run --features ai -- generate --test-type api --description "Test description" --context-size 2048
```
