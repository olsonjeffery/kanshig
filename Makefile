server-worker:
	./qwen3-coder-next.sh

server-small-worker:
	./devstral-small-2.sh

agent-qwen3-8081:
	MSWEA_COST_TRACKING=ignore_errors mini --yolo --cost-limit 0 -c qwen3-worker.yaml -c model.model_name=worker1/qwen3-coder-next -c model.model_kwargs.api_base=http://localhost:8081

agent-devstral2small:
	MSWEA_COST_TRACKING=ignore_errors mini -c devstral-small-2-worker.yaml --yolo --cost-limit 0 -c model.model_kwargs.api_base=${API_BASE}
