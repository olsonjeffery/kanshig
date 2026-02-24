#!/bin/bash

#export LLAMA_ARG_MODEL=~/src/models/Qwen3-Coder-Next-UD-Q2_K_XL.gguf
#export LLAMA_ARG_MODEL=~/src/models/Devstral-Small-2-24B-Instruct-2512-UD-Q4_K_XL.gguf 
export LLAMA_ARG_MODEL=~/src/models/Qwen3.5-27B-UD-Q8_K_XL.gguf

export LLAMA_ARG_N_GPU_LAYERS=99
export LLAMA_ARG_CTX_SIZE=131072

llama-server --temp 0.6 --min-p 0.0 --top-p 0.95 --top-k 20 --alias qwen35-27B
