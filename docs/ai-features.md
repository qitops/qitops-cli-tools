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

QitOps supports a variety of local LLM models:

- **LLaMA**: Versions 1, 2, and 3
- **Mistral**: 7B and 8x7B models
- **GPT-J**: All versions
- **Phi**: Versions 1, 2, and 3
- **Any GGUF-compatible model**: Models in GGUF format

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

Generate test configurations from natural language descriptions:

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

## Test Results Analysis {#test-results-analysis}

Analyze test results to identify patterns and issues:

```bash
# Analyze test results
qitops analyze --results test_results.json --output analysis.md

# Analyze with a specific model
qitops analyze --results test_results.json --output analysis.md --model llama --model-path /path/to/model.gguf
```

The analysis includes:
- Overview of test results
- Detailed breakdown of successes and failures
- Performance metrics
- Recommendations for improvement

## Improvement Suggestions {#improvement-suggestions}

Get actionable suggestions to improve your tests:

```bash
# Generate improvement suggestions
qitops improve --results test_results.json --output improvements.md

# Generate suggestions with a specific model
qitops improve --results test_results.json --output improvements.md --model llama --model-path /path/to/model.gguf
```

Improvement suggestions cover:
- Performance optimizations
- Reliability enhancements
- Coverage improvements
- Best practices

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
