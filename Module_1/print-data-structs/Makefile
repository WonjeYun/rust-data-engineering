SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project using cargo
	cargo clean

run: ## Run the project using cargo
	cargo run

build: ## Build the project using cargo
	cargo build

lint: ## Lint the project using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy --quiet

fmt: ## Format the project using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --quiet

test: ## Test the project using cargo
  	cargo test --quiet
