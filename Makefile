# Conduit Makefile for Cross-Compilation
APP_NAME := conduit
VERSION := $(shell grep '^version' Cargo.toml | head -n 1 | cut -d '"' -f 2)
DIST_DIR := dist

# Targets
LINUX_X64 := x86_64-unknown-linux-gnu
LINUX_ARM64 := aarch64-unknown-linux-gnu
WIN_X64 := x86_64-pc-windows-gnu

.PHONY: all help clean build-linux-x64 build-linux-arm64 build-windows-x64 dist

help:
	@echo "Conduit $(VERSION) Cross-Compilation Makefile"
	@echo "Usage:"
	@echo "  make build-linux-x64    - Build for Linux x64"
	@echo "  make build-linux-arm64  - Build for Linux ARM64 (RK3588 etc.)"
	@echo "  make build-windows-x64  - Build for Windows x64 (requires mingw-w64)"
	@echo "  make dist               - Build all platforms and package"
	@echo "  make clean              - Remove build artifacts"

clean:
	cargo clean
	rm -rf $(DIST_DIR)

build-linux-x64:
	@echo "Building for $(LINUX_X64)..."
	cargo build --release --target $(LINUX_X64)
	mkdir -p $(DIST_DIR)/linux-x64
	cp target/$(LINUX_X64)/release/$(APP_NAME) $(DIST_DIR)/linux-x64/

build-linux-arm64:
	@echo "Building for $(LINUX_ARM64)..."
	# Ensure you have the cross-linker installed (e.g., aarch64-linux-gnu-gcc)
	cargo build --release --target $(LINUX_ARM64)
	mkdir -p $(DIST_DIR)/linux-arm64
	cp target/$(LINUX_ARM64)/release/$(APP_NAME) $(DIST_DIR)/linux-arm64/

build-windows-x64:
	@echo "Building for $(WIN_X64)..."
	# Requires x86_64-w64-mingw32-gcc
	cargo build --release --target $(WIN_X64)
	mkdir -p $(DIST_DIR)/windows-x64
	cp target/$(WIN_X64)/release/$(APP_NAME).exe $(DIST_DIR)/windows-x64/

dist: clean build-linux-x64 build-linux-arm64 build-windows-x64
	@echo "Packaging all builds..."
	cd $(DIST_DIR) && tar -czvf $(APP_NAME)-$(VERSION)-linux-x64.tar.gz linux-x64/
	cd $(DIST_DIR) && tar -czvf $(APP_NAME)-$(VERSION)-linux-arm64.tar.gz linux-arm64/
	cd $(DIST_DIR) && zip -r $(APP_NAME)-$(VERSION)-windows-x64.zip windows-x64/
	@echo "Distribution files ready in $(DIST_DIR)/"
