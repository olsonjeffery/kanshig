server-qwen35:
	./qwen35-27B.sh

agent-qwen35-8081:
	MSWEA_COST_TRACKING=ignore_errors mini --yolo --cost-limit 0 -c mini-worker.yaml -c model.model_kwargs.api_base=http://localhost:8081

agent-glm47flash-8081:
	MSWEA_COST_TRACKING=ignore_errors mini --yolo --cost-limit 0 -c glm-4.7-flash-mini.yaml -c model.model_kwargs.api_base=http://localhost:8081
