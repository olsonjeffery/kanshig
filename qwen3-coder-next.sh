#!/bin/bash

export LLAMA_ARG_MODEL=~/src/models/Qwen3-Coder-Next-UD-Q2_K_XL.gguf
#export LLAMA_ARG_MODEL=~/src/models/Devstral-Small-2-24B-Instruct-2512-UD-Q4_K_XL.gguf 

export LLAMA_ARG_N_GPU_LAYERS=99
export LLAMA_ARG_CTX_SIZE=131072

llama-server --temp 0.7 --min-p 0.0 --top-p 0.80 --top-k 20 --repeat-penalty 1.05 --alias qwen3-coder-next
