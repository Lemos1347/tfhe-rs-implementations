.PHONY: start
start:
	@echo "Starting TFHE-rs development environment..."
	@echo "Activating Nix shell (use Ctrl+D or 'exit' to exit)"
	@nix develop

.PHONY: build-cli
build-cli:
	@echo "Building TFHE-rs CLI Docker image..."
	docker build -f build/Dockerfile.cli -t tfhe-rs-cli .
	@echo "Docker image 'tfhe-rs-cli' built successfully!"

.PHONY: cli
cli:
	@echo "Checking if TFHE-rs CLI Docker image exists..."
	@if ! docker image inspect tfhe-rs-cli >/dev/null 2>&1; then \
		echo "Image not found. Building it now..."; \
		$(MAKE) build-cli; \
	else \
		echo "Image found!"; \
	fi
	@echo "Starting TFHE-rs CLI environment (use 'exit' to leave)..."
	@echo "Type 'tfhe-cli help' once inside to see available commands."
	@echo ""
	docker run -it --rm tfhe-rs-cli

.PHONY: help
help:
	@echo "TFHE-rs Development Environment"
	@echo ""
	@echo "Available targets:"
	@echo "  start        - Start the Nix development environment"
	@echo "  docker-build - Build the TFHE-rs CLI Docker image"
	@echo "  docker-cli   - Run the TFHE-rs CLI Docker container (builds if needed)"
	@echo "  help         - Show this help message"
	@echo ""
	@echo "Development Environment (Nix):"
	@echo "  just run-fibonacci       - Run Fibonacci benchmark with default settings"
	@echo "  just run-fibonacci-n 5   - Run with 5 Fibonacci numbers"
	@echo "  just run-fibonacci-fast  - Run in fast/lower security mode"
	@echo ""
	@echo "Docker CLI Environment:"
	@echo "  tfhe-cli help            - Show CLI help (inside container)"
	@echo "  tfhe-cli fibonacci [n]   - Run Fibonacci example (inside container)"

# Default target
.DEFAULT_GOAL := help 