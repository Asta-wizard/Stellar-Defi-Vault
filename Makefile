.PHONY: build test docs coverage coverage-lcov scout scout-sarif mutants deploy-testnet stake claim

build:
	cargo build --target wasm32-unknown-unknown --release

test:
	cargo test --features testutils

# Line coverage summary via cargo-llvm-cov (issue #663). See docs/COVERAGE.md.
coverage:
	cargo llvm-cov --features testutils --summary-only

# Write lcov.info for editor coverage gutters.
coverage-lcov:
	cargo llvm-cov --features testutils --lcov --output-path lcov.info

# Scout static analysis (issue #664). See docs/STATIC_ANALYSIS.md.
scout:
	SCOUT_OFFLINE=1 cargo scout-audit

# Write scout.sarif for the triage script / report upload.
scout-sarif:
	SCOUT_OFFLINE=1 cargo scout-audit --output-format sarif

# Mutation testing scoped to the accounting core (issue #662).
# See docs/MUTATION_TESTING.md.
mutants:
	cargo mutants --config mutants.toml

# Build rustdoc and list undocumented public items (#675).
docs:
	cargo doc --no-deps --lib 2>&1 | tee target/rustdoc.log
	@echo "Undocumented public items: $$(grep -c 'missing documentation' target/rustdoc.log || true)"

deploy-testnet:
	chmod +x scripts/deploy_testnet.sh
	./scripts/deploy_testnet.sh

stake:
	@chmod +x scripts/stake.sh
	@if [ -z "$(AMOUNT)" ]; then \
		echo "Error: AMOUNT is required. Usage: make stake AMOUNT=10000000"; \
		exit 1; \
	fi
	./scripts/stake.sh $(AMOUNT)

claim:
	chmod +x scripts/claim.sh
	./scripts/claim.sh
