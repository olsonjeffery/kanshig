#!/bin/bash

#export LLAMA_ARG_MODEL=~/src/models/Qwen3-Coder-Next-UD-Q2_K_XL.gguf
#export LLAMA_ARG_MODEL=~/src/models/Devstral-Small-2-24B-Instruct-2512-UD-Q4_K_XL.gguf 
export LLAMA_ARG_MODEL=~/src/models/Qwen3.5-27B-UD-Q8_K_XL.gguf

export LLAMA_ARG_N_GPU_LAYERS=99
export LLAMA_ARG_CTX_SIZE=131072

# temperature=0.6, top_p=0.95, top_k=20, min_p=0.0, presence_penalty=0.0, repetition_penalty=1.0
llama-server --temp 0.6 --min-p 0.0 --top-p 0.95 --top-k 20 --presence-penalty 0.0 --repeat-penalty 1.0 --alias model
