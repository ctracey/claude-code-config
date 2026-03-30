# claude-code-config
# Top-level Makefile — build, install, and release.
#
# Quick start:   make setup
# Full install:  make install
# Update:        make update

.DEFAULT_GOAL := help
.PHONY: setup install uninstall update clean help ways test

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
	@$(WAYS_BIN) lint --check && echo "  lint: PASS"
	@$(WAYS_BIN) match "write a unit test" >/dev/null && echo "  match: PASS"
	@$(WAYS_BIN) graph --output /dev/null && echo "  graph: PASS"
	@echo "All smoke tests passed."

# --- Supporting ---

hooks-executable:
	@find hooks -name '*.sh' -exec chmod +x {} + 2>/dev/null || true
	@echo "Hooks marked executable."

clean:
	$(MAKE) -C tools/way-embed clean
	cargo clean --manifest-path tools/ways-cli/Cargo.toml 2>/dev/null || true
