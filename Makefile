#!/usr/bin/make -f

# # Default target is build
# default: build

# Define variables
ALLOY_VERSION=0.13.0
CARGO=cargo
CRATES_FOLDER=crates
CONTRACTS_PATH=contracts
BINDINGS_FOLDER=bindings
BINDINGS_CRATES_FOLDER=$(CRATES_FOLDER)/$(BINDINGS_FOLDER)

# Default target (if no target is specified)
.DEFAULT_GOAL := help

# Help command (lists all available commands)
help:
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

bindings: # Generate new bindings
	rm -rf $(BINDINGS_CRATES_FOLDER)
	@forge bind --alloy-version $(ALLOY_VERSION) --root $(CONTRACTS_PATH) -b $(BINDINGS_CRATES_FOLDER) --force

build: bindings ## Build the Docker image
	@$(CARGO) build

build-release: bindings ## Build the Docker image, release mode
	@$(CARGO) build --release

clean: ## Clean the project
	@forge clean --root $(CONTRACTS_PATH)
	@$(CARGO) clean

fmt: ## Format the code
	@forge fmt --check --root $(CONTRACTS_PATH)
	@$(CARGO) fmt

test: ## Run tests
	@forge test --root $(CONTRACTS_PATH)
	@$(CARGO) test

setup: ## Install forge dependencies
	@forge install --root $(CONTRACTS_PATH)


# Declare phony targets
.PHONY: build build-release clean fmt bindings
