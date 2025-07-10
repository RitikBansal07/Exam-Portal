# Image name
IMAGE_NAME=exam-portal

# Build the Docker image
build: set-env
	@$(MAKE) ENV=docker set-env
	@echo "🧼 Cleaning up old containers and volumes before build..."
	@docker rm -f exam-portal exam-db > NUL 2>&1 || exit 0
	@docker volume rm examdb-data > NUL 2>&1 || exit 0
	@echo "🐳 Building Docker image..."
	docker build -t $(IMAGE_NAME) .

# Build the Docker image
build-deploy: set-env
	@$(MAKE) ENV=deploy set-env
	@echo "🧼 Cleaning up old containers and volumes before build..."
	@docker rm -f exam-portal exam-db > NUL 2>&1 || exit 0
	@docker volume rm examdb-data > NUL 2>&1 || exit 0
	@echo "🐳 Building Docker image..."
	docker build -t $(IMAGE_NAME) .


set-env:
ifeq ($(OS),Windows_NT)
	bash ./set-env.sh $(ENV)
else
	chmod +x set-env.sh
	./set-env.sh $(ENV)
endif


run-docker:
	@echo "🧼 Removing old container (if exists and running)..."
	@if docker ps -a --format '{{.Names}}' | grep -q "^$(IMAGE_NAME)$$"; then \
		docker rm -f $(IMAGE_NAME); \
	else \
		echo "ℹ️  No existing container named $(IMAGE_NAME) found."; \
	fi
	@echo "🐳 Running $(IMAGE_NAME) on port 8080..."
	docker run -p 8080:8080 --name $(IMAGE_NAME) $(IMAGE_NAME)



run-local:
	@$(MAKE) ENV=local set-env
	@echo "🚀 Running with cargo (ENV=local)..."
	cargo run

stop:
	docker-compose down -v


stop-docker:
	docker-compose down -v


# Build and then run
up: build run

# Clean dangling images (optional)
clean:
	docker image prune -f
