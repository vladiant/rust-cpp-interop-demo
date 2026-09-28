# Convenience wrapper around the canonical CMake + Corrosion workflow.
#
# Every recipe below invokes exactly the commands documented in
# docs/tech-stack.md so this Makefile, CI (.github/workflows/ci.yml), and the
# README stay in sync. It adds no build logic of its own -- CMake + CTest remain
# the real entry point; this is just a one-obvious-door shortcut for reviewers.
#
# Run `make` (or `make help`) to list targets.

BUILD_DIR := build
ASAN_BUILD_DIR := build-asan
BUILD_TYPE := Debug
CPP_SOURCES := $(shell find cpp -name '*.cpp' -o -name '*.hpp' -o -name '*.h')

.DEFAULT_GOAL := help

.PHONY: help configure build run test rust-test fmt-check clippy clang-format-check lint asan valgrind check clean

help: ## Show this help message
	@echo "Usage:"
	@echo "  make <target>"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| sort \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

configure: ## Configure the CMake build (fetches Corrosion on first run)
	cmake -S . -B $(BUILD_DIR) -DCMAKE_BUILD_TYPE=$(BUILD_TYPE)

build: configure ## Build everything (libs + both demo binaries + test)
	cmake --build $(BUILD_DIR)

run: build ## Run the full demo, both directions (Rust -> C++, C++ -> Rust)
	cmake --build $(BUILD_DIR) --target run_demo

test: build ## Run the full test suite via CTest (3 tests)
	ctest --test-dir $(BUILD_DIR) --output-on-failure

rust-test: ## Run the pure-Rust unit tests only
	cargo test -p interop_rs

fmt-check: ## Check Rust formatting (rustfmt, no changes made)
	cargo fmt --all -- --check

clippy: ## Lint Rust with clippy (warnings are errors)
	cargo clippy --all-targets --all-features -- -D warnings

clang-format-check: ## Check C++ formatting (clang-format dry-run)
	clang-format --dry-run --Werror $(CPP_SOURCES)

lint: fmt-check clippy clang-format-check ## Run all format + lint gates

asan: ## Build & run ASan/UBSan on the C++-driven targets (scoped)
	cmake -S . -B $(ASAN_BUILD_DIR) -DCMAKE_BUILD_TYPE=$(BUILD_TYPE) \
		-DCMAKE_CXX_FLAGS="-fsanitize=address,undefined -fno-omit-frame-pointer" \
		-DCMAKE_EXE_LINKER_FLAGS="-fsanitize=address,undefined"
	cmake --build $(ASAN_BUILD_DIR) --target cpp_demo --target test_cpp_to_rust
	./$(ASAN_BUILD_DIR)/cpp_demo
	./$(ASAN_BUILD_DIR)/test_cpp_to_rust

valgrind: build ## Run valgrind on both demo binaries (leak/error check)
	valgrind --leak-check=full --error-exitcode=1 ./$(BUILD_DIR)/rust_demo
	valgrind --leak-check=full --error-exitcode=1 ./$(BUILD_DIR)/cpp_demo

check: build test lint ## Local mirror of CI's fast gate: build + test + lint

clean: ## Remove CMake build directories
	rm -rf $(BUILD_DIR) $(ASAN_BUILD_DIR)
