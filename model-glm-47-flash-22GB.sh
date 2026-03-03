#!/bin/bash

export LLAMA_ARG_MODEL=~/src/models/GLM-4.7-Flash-UD-Q5_K_XL.gguf

export LLAMA_ARG_N_GPU_LAYERS=99
export LLAMA_ARG_CTX_SIZE=131072

llama-server --temp 0.7 --top-p 1.0 --min-p 0.01 --presence-penalty 0.0 --repeat-penalty 1.0 --alias model
