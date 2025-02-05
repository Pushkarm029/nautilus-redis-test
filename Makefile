# Makefile
.PHONY: setup run clean populate test

# Variables
REDIS_CONTAINER=nautilus-redis-test-redis-1
# later 1000000
CURRENCY_COUNT=100000

setup:
	@echo "Setting up Redis and building Rust project..."
	docker compose up -d
	cargo build --release
	@echo "Waiting for Redis to be ready..."
	@sleep 2

populate:
	@echo "Populating Redis with $(CURRENCY_COUNT) currency keys..."
	docker exec -i $(REDIS_CONTAINER) redis-cli FLUSHALL
	@for i in $$(seq 1 $(CURRENCY_COUNT)); do \
		if [ $$((i % 10000)) -eq 0 ]; then \
			echo "Populated $$i keys..."; \
		fi; \
		echo "SET currency:SYM$$i \"Currency $$i\""; \
	done | docker exec -i $(REDIS_CONTAINER) redis-cli --pipe

run: setup populate
	@echo "Running performance test..."
	cargo run --release

test: setup populate
	@echo "Running tests..."
	cargo test --release -- --nocapture

clean:
	@echo "Cleaning up..."
	docker-compose down -v
	cargo clean

monitor:
	@echo "Monitoring Redis memory usage..."
	docker stats $(REDIS_CONTAINER)

info:
	@echo "Redis Info:"
	docker exec -i $(REDIS_CONTAINER) redis-cli info | grep -E "used_memory_human|db0|connected"

benchmark:
	@echo "Running Redis benchmark..."
	docker exec -i $(REDIS_CONTAINER) redis-benchmark -t get -n 100000 -q