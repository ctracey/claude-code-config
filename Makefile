# claude-code-config
# Top-level Makefile — build, install, and release.
#
# Quick start:   make setup
# Full install:  make install
# Update:        make update

.DEFAULT_GOAL := help
.PHONY: setup install uninstall update clean help ways test test-sim release

WAYS_BIN = bin/ways
XDG_BIN = $(or $(XDG_BIN_HOME),$(HOME)/.local/bin)

# --- Primary targets ---

help:
	@echo "claude-code-config"
	@echo ""
	@echo "  make setup      Build ways CLI + fetch embedding model"
	@echo "  make install    Full first-time setup (hooks + tools)"
	@echo "  make update     Pull latest changes and re-run setup"
	@echo "  make ways       Build the ways CLI binary (Rust)"
	@echo "  make test       Smoke test the ways binary"
	@echo "  make test-sim   Run session simulator (8 scenarios)"
	@echo "  make release    Build release tarball for current platform"
	@echo "  make uninstall   Remove ways from PATH"
	@echo "  make clean      Remove build artifacts"
	@echo ""

# Build ways CLI + set up embedding engine + generate initial corpus.
setup: ways
	@echo "Setting up embedding engine..."
	$(MAKE) -C tools/way-embed setup
	@echo ""
	@echo "Setting up mmaid diagram renderer..."
	@bash tools/mmaid/download-mmaid.sh || echo "  (mmaid optional — skipping)"
	@echo ""
	@echo "Generating corpus..."
	@$(WAYS_BIN) corpus --quiet

# Full install: build, setup, symlink to PATH.
install: hooks-executable setup
	@mkdir -p $(XDG_BIN)
	@ln -sf $(CURDIR)/$(WAYS_BIN) $(XDG_BIN)/ways
	@echo ""
	@echo "Install complete."
	@echo "  ways binary: $(XDG_BIN)/ways → $(CURDIR)/$(WAYS_BIN)"
	@echo "  Restart Claude Code for ways to take effect."

# Remove symlink from PATH.
uninstall:
	@rm -f $(XDG_BIN)/ways
	@echo "Removed $(XDG_BIN)/ways"

# Pull upstream and re-setup.
update:
	git pull --ff-only
	$(MAKE) install

# --- Build ---

ways:
	@if [ ! -x $(WAYS_BIN) ] || [ tools/ways-cli/src/main.rs -nt $(WAYS_BIN) ]; then \
		cargo build --release --manifest-path tools/ways-cli/Cargo.toml; \
		mkdir -p bin; \
		cp tools/ways-cli/target/release/ways $(WAYS_BIN); \
		echo "Built: $(WAYS_BIN)"; \
	fi

# --- Test ---

test: ways
	@echo "Smoke testing ways binary..."
	@$(WAYS_BIN) --version
	@$(WAYS_BIN) lint --check --global && echo "  lint: PASS"
	@$(WAYS_BIN) match "write a unit test" >/dev/null && echo "  match: PASS"
	@$(WAYS_BIN) graph --output /dev/null && echo "  graph: PASS"
	@echo "All smoke tests passed."

test-sim: ways
	@echo "Running session simulator (8 scenarios)..."
	@cargo test --manifest-path tools/ways-cli/Cargo.toml --test session_sim -- --test-threads=1
	@echo "All simulation scenarios passed."

# --- Supporting ---

hooks-executable:
	@find hooks -name '*.sh' -exec chmod +x {} + 2>/dev/null || true
	@echo "Hooks marked executable."

release: ways
	@echo "Building release tarball..."
	@mkdir -p dist
	@PLATFORM=$$(uname -s | tr '[:upper:]' '[:lower:]')-$$(uname -m | sed 's/arm64/aarch64/'); \
		cp $(WAYS_BIN) dist/ways-$$PLATFORM; \
		cd dist && sha256sum ways-$$PLATFORM > ways-$$PLATFORM.sha256; \
		echo "dist/ways-$$PLATFORM ($$(ls -lh ways-$$PLATFORM | awk '{print $$5}'))"; \
		cat ways-$$PLATFORM.sha256

clean:
	$(MAKE) -C tools/way-embed clean
	cargo clean --manifest-path tools/ways-cli/Cargo.toml 2>/dev/null || true
	rm -rf dist/
