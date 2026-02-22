server:
	./lcsa.sh

start-worker:
	MSWEA_COST_TRACKING=ignore_errors mini -c mini-worker.yaml --yolo --cost-limit 1
