# DTU Notes - Makefile for Windows Development
# Use with: make <target>

# Variables
BINARY_NAME = noter.exe
CARGO = cargo
RELEASE_DIR = target/release
DEBUG_DIR = target/debug

# Default target
.DEFAULT_GOAL := help

# Colors for output (Windows compatible)
BLUE = @echo.
GREEN = @echo.
YELLOW = @echo.
RED = @echo.

help: ## Show this help message
	$(BLUE) DTU Notes CLI - Available Commands:
	$(BLUE) ================================
	@powershell -Command "& {Get-Content Makefile | Select-String '##' | ForEach-Object {$$_.Line -replace '^([^:]*):.*?## (.*)$$', '  $$1 - $$2'}}"

build: ## Build the project in debug mode
	$(GREEN) Building debug version...
	$(CARGO) build

build-release: ## Build the project in release mode
	$(GREEN) Building release version...
	$(CARGO) build --release

run: ## Run the project (use: make run ARGS="status")
	$(GREEN) Running application...
	$(CARGO) run -- $(ARGS)

test: ## Run tests
	$(GREEN) Running tests...
	$(CARGO) test

test-verbose: ## Run tests with verbose output
	$(GREEN) Running tests (verbose)...
	$(CARGO) test -- --nocapture

lint: ## Run clippy for linting
	$(GREEN) Running clippy...
	$(CARGO) clippy -- -D warnings

fmt: ## Format code
	$(GREEN) Formatting code...
	$(CARGO) fmt

check-fmt: ## Check code formatting
	$(GREEN) Checking code format...
	$(CARGO) fmt -- --check

clean: ## Clean build artifacts
	$(GREEN) Cleaning build artifacts...
	$(CARGO) clean

install: ## Install binary locally
	$(GREEN) Installing locally...
	$(CARGO) install --path .

docs: ## Generate and open documentation
	$(GREEN) Generating documentation...
	$(CARGO) doc --open

check: check-fmt lint test ## Run all checks (format, lint, test)
	$(GREEN) All checks completed!

info: ## Show project information
	$(BLUE) DTU Notes Project Information:
	$(BLUE) ============================
	@echo Binary Name: $(BINARY_NAME)
	@powershell -Command "& {(Select-String -Path Cargo.toml -Pattern '^version\s*=\s*\"(.+)\"').Matches[0].Groups[1].Value}" 2>nul || echo Version info unavailable
	@echo Debug Binary: $(DEBUG_DIR)/$(BINARY_NAME)
	@echo Release Binary: $(RELEASE_DIR)/$(BINARY_NAME)

size: build-release ## Show binary size
	$(GREEN) Binary size information:
	@powershell -Command "& {if (Test-Path '$(RELEASE_DIR)/$(BINARY_NAME)') {Get-Item '$(RELEASE_DIR)/$(BINARY_NAME)' | Select-Object Name, @{Name='Size';Expression={'{0:N0} bytes' -f $_.Length}}} else {Write-Host 'Release binary not found. Run make build-release first.'}}"

profile: build-release size ## Profile the release build
	$(GREEN) Release build profiling complete!

# Development workflow targets
dev-quick: fmt build test ## Quick development cycle
	$(GREEN) Quick development cycle complete!

dev-full: fmt lint test build ## Full development cycle
	$(GREEN) Full development cycle complete!

release: check build-release profile ## Prepare release build
	$(GREEN) Release preparation complete!
	@echo Release binary: $(RELEASE_DIR)/$(BINARY_NAME)

# Testing specific commands
test-setup: ## Test setup command
	$(CARGO) run -- setup

test-status: ## Test status command
	$(CARGO) run -- status

test-courses: ## Test courses command
	$(CARGO) run -- courses list

test-config: ## Test config command
	$(CARGO) run -- config show

# Maintenance targets
update: ## Update dependencies
	$(GREEN) Updating dependencies...
	$(CARGO) update

audit: ## Security audit
	$(GREEN) Running security audit...
	$(CARGO) audit

deps: ## Show dependency tree
	$(GREEN) Dependency tree:
	$(CARGO) tree

# Package management
package: ## Package for distribution
	$(GREEN) Creating package...
	$(CARGO) package --allow-dirty

# Git workflow helpers
git-status: ## Show git status
	@git status --porcelain

git-clean: ## Check if git is clean
	@git diff-index --quiet HEAD -- || (echo "Working directory is not clean" && exit 1)

tag: git-clean ## Create git tag (use: make tag VERSION=1.0.0)
	@if "$(VERSION)"=="" (echo Please specify VERSION: make tag VERSION=1.0.0 && exit 1)
	git tag -a v$(VERSION) -m "Release version $(VERSION)"
	git push origin v$(VERSION)
	$(GREEN) Tagged version v$(VERSION)

# Utility targets
watch: ## Watch files and rebuild on changes (requires cargo-watch)
	$(CARGO) watch -x check

watch-test: ## Watch files and run tests on changes
	$(CARGO) watch -x test

setup-dev: ## Setup development environment
	$(GREEN) Setting up development environment...
	$(CARGO) install cargo-watch cargo-edit cargo-audit cargo-tarpaulin 2>nul || echo Some tools may already be installed
	$(GREEN) Development environment setup complete!

# Windows-specific helpers
open-target: ## Open target directory in Explorer
	@if exist "target" (explorer target) else (echo Target directory does not exist)

open-release: build-release ## Open release directory in Explorer
	@if exist "$(RELEASE_DIR)" (explorer $(RELEASE_DIR)) else (echo Release directory does not exist)

# All-in-one targets
setup-and-build: setup-dev build ## Setup dev environment and build
	$(GREEN) Setup and build complete!

ci: check build-release ## Continuous integration target
	$(GREEN) CI pipeline complete!

.PHONY: help build build-release run test test-verbose lint fmt check-fmt clean install docs check info size profile dev-quick dev-full release test-setup test-status test-courses test-config update audit deps package git-status git-clean tag watch watch-test setup-dev open-target open-release setup-and-build ci
