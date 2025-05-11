# Supported LLM Models

QitOps supports a variety of local LLM models for its AI features. These models run entirely on your local machine, ensuring privacy and offline operation.

## LLaMA Models

- **LLaMA 1**: The original Meta AI model (7B, 13B, 33B, 65B parameters)
- **LLaMA 2**: Improved version with longer context (7B, 13B, 70B parameters)
- **LLaMA 3**: Latest version with enhanced capabilities (8B, 70B parameters)
- **Code LLaMA**: Specialized for code generation and analysis

## Mistral Models

- **Mistral 7B**: Efficient base model with strong performance
- **Mixtral 8x7B**: Mixture-of-experts model with enhanced capabilities
- **Mistral Instruct**: Fine-tuned for instruction following
- **Mistral Small**: Smaller, faster models for resource-constrained environments

## Phi Models

- **Phi-1**: Microsoft's small but capable model (1.3B parameters)
- **Phi-2**: Improved version with enhanced reasoning (2.7B parameters)
- **Phi-3**: Latest version with advanced capabilities (3.8B, 14B parameters)

## Other Supported Models

- **GPT-J**: EleutherAI's open-source GPT model (6B parameters)
- **GPT4All**: Locally running assistant models
- **Falcon**: Technology Innovation Institute's models
- **MPT**: MosaicML's Pretrained Transformer models
- **RWKV**: RNN with transformer-like capabilities
- **Qwen**: Alibaba's series of multilingual models

## Model Format Support

- **GGUF**: Primary supported format for efficient inference
- **GGML**: Legacy format (automatically converted to GGUF)
- **ONNX**: Support via external runtime

## Recommended Models for Different Use Cases

| Use Case | Recommended Model | Size | Performance |
|----------|------------------|------|-------------|
| Test Generation | Phi-2 | 2.7GB | Good balance of size and quality |
| Results Analysis | Mistral 7B | 4.1GB | Strong reasoning capabilities |
| Improvement Suggestions | LLaMA 2 13B | 8.2GB | Detailed, high-quality suggestions |
| Resource-Constrained | Phi-1 | 1.5GB | Works on machines with limited RAM |
| Best Quality | Mixtral 8x7B | 26GB | Highest quality results (requires 32GB+ RAM) |

## Where to Download Models

Models can be downloaded from:
- [Hugging Face](https://huggingface.co/models) - Search for GGUF versions
- [TheBloke's repositories](https://huggingface.co/TheBloke) - Pre-converted GGUF models
- [Ollama Library](https://ollama.com/library) - Easy model management

## Model Quantization Options

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

## Model Configuration

QitOps allows you to configure various model parameters:

```bash
# Set context size
qitops generate --test-type api --description "Test the login API" --context-size 8192

# Set temperature
qitops analyze --results test_results.json --output analysis.md --temperature 0.7

# Set maximum tokens
qitops improve --results test_results.json --output improvements.md --max-tokens 4096
```

## Using Ollama Models

QitOps can use models managed by Ollama:

```bash
# Use an Ollama model
qitops generate --test-type api --description "Test the login API" --model ollama:phi

# Use a specific Ollama model version
qitops analyze --results test_results.json --output analysis.md --model ollama:mistral:7b-instruct
```

## Using Local Model Files

QitOps can use model files stored locally:

```bash
# Use a local model file
qitops generate --test-type api --description "Test the login API" --model-path /path/to/models/phi-2.gguf

# Use a local model with specific parameters
qitops analyze --results test_results.json --output analysis.md --model-path /path/to/models/mistral-7b.gguf --context-size 4096 --temperature 0.5
```

## Model Performance Considerations

- **Memory Usage**: Larger models require more RAM
- **Disk Space**: Model files can be large (1-30GB)
- **CPU vs. GPU**: GPU acceleration significantly improves performance
- **Quantization**: Lower quantization reduces quality but improves speed and reduces memory usage
- **Context Size**: Larger context sizes require more memory but can improve results for complex tasks

## Troubleshooting

### Common Issues

- **Out of Memory**: Try a smaller model or lower quantization
- **Slow Generation**: Use GPU acceleration or a smaller model
- **Poor Quality Results**: Try a larger model or higher quantization
- **Model Not Found**: Check the model path and ensure the file exists
- **Unsupported Format**: Ensure the model is in GGUF format

### Performance Optimization

- **GPU Acceleration**: Enable GPU acceleration for faster inference
- **Batch Processing**: Process multiple requests in batch for better throughput
- **Quantization**: Use appropriate quantization for your hardware
- **Context Size**: Use the smallest context size that works for your use case
- **Model Selection**: Choose the smallest model that provides acceptable quality
