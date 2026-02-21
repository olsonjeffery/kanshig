#!/bin/bash

export LLAMA_ARG_MODEL=~/src/models/Qwen3-Coder-30B-A3B-Instruct-UD-Q8_K_XL.gguf
export LLAMA_ARG_N_GPU_LAYERS=99
export LLAMA_ARG_CTX_SIZE=65536

llama-server --temp 0.7 --min-p 0.0 --top-p 0.80 --top-k 20 --repeat-penalty 1.05
