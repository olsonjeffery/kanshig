server-worker:
	./qwen3-coder-next.sh

server-small-worker:
	./devstral-2-small.sh

agent-qwen3:
	MSWEA_COST_TRACKING=ignore_errors mini -c mini-worker.yaml --yolo --cost-limit 0  -c agent.model.model_name=llamacpp/qwen3-coder-next -c agent.model.model_kwargs.api_base=${API_BASE}

agent-devstral2small:
	MSWEA_COST_TRACKING=ignore_errors mini -c mini-worker.yaml --yolo --cost-limit 0 -c agent.model.model_name=llamacpp/devstral-2-small -c agent.model.model_kwargs.api_base=${API_BASE}
