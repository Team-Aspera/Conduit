BINARY   = conduit
PREFIX  ?= /usr/local
BUILD_DIR = target/release
UNAME_M  = $(shell uname -m)
ARCH     = $(if $(filter x86_64,$(UNAME_M)),amd64,$(UNAME_M))
UNAME_S  = $(shell uname -s | tr A-Z a-z)
DIST     = $(BINARY)-$(UNAME_S)-$(ARCH)

.PHONY: all build install uninstall clean test lint fmt dist

all: build

build:
	cargo build --release

dist: build
	cp $(BUILD_DIR)/$(BINARY) $(BUILD_DIR)/$(DIST)
	@echo "Binary: $(BUILD_DIR)/$(DIST)"

install: build
	sudo install -d $(DESTDIR)$(PREFIX)/bin
	sudo install -m 755 $(BUILD_DIR)/$(BINARY) $(DESTDIR)$(PREFIX)/bin/
	sudo install -d $(DESTDIR)$(PREFIX)/share/icons
	sudo install -m 644 assets/images/Conduit-logoonly.png $(DESTDIR)$(PREFIX)/share/icons/conduit-icon.png
	sudo install -d $(DESTDIR)$(PREFIX)/share/fonts
	sudo install -m 644 assets/fonts/NotoSansSymbols2-Regular.ttf $(DESTDIR)$(PREFIX)/share/fonts/

uninstall:
	sudo rm -f $(DESTDIR)$(PREFIX)/bin/$(BINARY)
	sudo rm -f $(DESTDIR)$(PREFIX)/share/icons/conduit-icon.png
	sudo rm -f $(DESTDIR)$(PREFIX)/share/fonts/NotoSansSymbols2-Regular.ttf

clean:
	cargo clean

test:
	cargo test

lint:
	cargo clippy -- -D warnings

fmt:
	cargo fmt
