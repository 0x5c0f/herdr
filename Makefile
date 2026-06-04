# herdr Makefile (feature/i18n branch)
# Usage: make [target]

# Zig path for libghostty-vt build
ZIG_PATH ?= /opt/zig/0.15.2
export PATH := $(ZIG_PATH):$(PATH)

# Install prefix
PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin

# Binary name
BIN = herdr

.PHONY: build install uninstall test lint check clean release help

## Build release binary
build:
	cargo build --release --locked

## Build and install to $(PREFIX)/bin
install: build
	install -d $(DESTDIR)$(BINDIR)
	install -m 755 target/release/$(BIN) $(DESTDIR)$(BINDIR)/$(BIN)
	@echo "installed $(BIN) to $(DESTDIR)$(BINDIR)/$(BIN)"

## Uninstall binary
uninstall:
	rm -f $(DESTDIR)$(BINDIR)/$(BIN)
	@echo "removed $(DESTDIR)$(BINDIR)/$(BIN)"

## Run tests
test:
	cargo nextest run --locked --status-level fail --final-status-level fail --failure-output final --success-output never

## Run lint checks
lint:
	cargo fmt --check
	cargo clippy --all-targets --locked -- -D warnings

## Run full checks (lint + tests)
check: lint test

## Clean build artifacts
clean:
	cargo clean

## Build and create release tarball
release: build
	@VERSION=$$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/'); \
	echo "packaging herdr v$$VERSION..."; \
	mkdir -p dist; \
	tar -czf dist/herdr-v$$VERSION-linux-x86_64.tar.gz -C target/release $(BIN); \
	echo "created dist/herdr-v$$VERSION-linux-x86_64.tar.gz"

## Show available targets
help:
	@echo "herdr build targets:"
	@echo ""
	@echo "  make build      - Build release binary"
	@echo "  make install    - Build and install to PREFIX/bin (default: /usr/local/bin)"
	@echo "  make uninstall  - Remove installed binary"
	@echo "  make test       - Run tests"
	@echo "  make lint       - Run lint checks"
	@echo "  make check      - Run full checks (lint + tests)"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make release    - Build and create release tarball in dist/"
	@echo "  make help       - Show this help"
	@echo ""
	@echo "Options:"
	@echo "  ZIG_PATH=/opt/zig/0.15.2  - Path to zig binary"
	@echo "  PREFIX=/usr/local         - Install prefix"
