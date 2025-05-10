#!/bin/bash

# Script to download a small language model for testing QitOps AI features

# Create models directory if it doesn't exist
mkdir -p models

# Download a small model (Phi-2 quantized)
echo "Downloading Phi-2 model (quantized version for faster inference)..."
wget -c https://huggingface.co/TheBloke/phi-2-GGUF/resolve/main/phi-2.Q4_K_M.gguf -O models/phi-2.Q4_K_M.gguf

echo "Model downloaded to models/phi-2.Q4_K_M.gguf"
echo "You can use this model with QitOps by specifying --model phi --model-path models/phi-2.Q4_K_M.gguf"
