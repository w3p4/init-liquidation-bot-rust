#!/usr/bin/make -f

# Default target is build
default: build

# Define variables
ALLOY_VERSION=0.12.0
CARGO=cargo
CRATES_FOLDER=crates
CONTRACTS_PATH=contracts
BINDINGS_FOLDER=bindings
BINDINGS_CRATES_FOLDER=$(CRATES_FOLDER)/$(BINDINGS_FOLDER)

# Target for generating bindings
bindings:
	rm -rf $(BINDINGS_CRATES_FOLDER)
# Generate new bindings
	@forge bind --alloy-version $(ALLOY_VERSION) --root $(CONTRACTS_PATH) -b $(BINDINGS_CRATES_FOLDER) --force

# Target for building the project
build: bindings
	@$(CARGO) build

# Target for building the project in release mode
build-release: bindings
	@$(CARGO) build --release

# Target for cleaning the project
clean:
	@forge clean --root $(CONTRACTS_PATH)
	@$(CARGO) clean

# Target for formatting the code
fmt:
	@forge fmt --check --root $(CONTRACTS_PATH)
	@$(CARGO) fmt

# Target for running tests
test:
	@forge test --root $(CONTRACTS_PATH)
	@$(CARGO) test

# Target for installing forge dependencies
setup:
	@forge install --root $(CONTRACTS_PATH)


# Declare phony targets
.PHONY: build build-release clean fmt bindings
