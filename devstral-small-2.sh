#!/bin/bash

#export LLAMA_ARG_MODEL=~/src/models/Qwen3-Coder-Next-UD-Q2_K_XL.gguf
export LLAMA_ARG_MODEL=~/src/models/Devstral-Small-2-24B-Instruct-2512-UD-Q4_K_XL.gguf 

export LLAMA_ARG_N_GPU_LAYERS=99
export LLAMA_ARG_CTX_SIZE=32268

llama-server --temp 0.15 --min-p 0.01 --alias devstral-small-1
