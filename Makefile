.PHONY: start
start:
	@echo "Starting TFHE-rs development environment..."
	@echo "Activating Nix shell (use Ctrl+D or 'exit' to exit)"
	@nix develop

.PHONY: help
help:
	@echo "TFHE-rs Development Environment"
	@echo ""
	@echo "Available targets:"
	@echo "  start   - Start the Nix development environment"
	@echo "  help    - Show this help message"
	@echo ""
	@echo "Once in the development shell, you can use Just commands:"
	@echo "  just run-fibonacci       - Run Fibonacci benchmark with default settings"
	@echo "  just run-fibonacci-n 5   - Run with 5 Fibonacci numbers"
	@echo "  just run-fibonacci-fast  - Run in fast/lower security mode"

# Default target
.DEFAULT_GOAL := help 