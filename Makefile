# Makefile for setup
# Define the name of the executable and the installation directory
TARGET = commands
INSTALL_DIR = ~/.commands
LINUX_FILE = linux

# Default target
all: build

# Build the project
build:
	cargo build --release

# Install the executable and the linux file
install: build
	mkdir -p $(INSTALL_DIR)
	cp target/release/$(TARGET) $(INSTALL_DIR)/
	cp $(LINUX_FILE) $(INSTALL_DIR)/  # Copy the linux file to the installation directory
	@echo "Installed to $(INSTALL_DIR)/$(TARGET) and $(INSTALL_DIR)/$(LINUX_FILE)"
	@echo "Please add the following line to your shell configuration file (.bashrc, .bash_profile, or .zshrc):"
	@echo "export PATH=\"\$$PATH:$(INSTALL_DIR)\""
	@echo "After adding the line, run 'source <your-config-file>' or restart your terminal for the changes to take effect."

# Uninstall the project
uninstall:
	rm -f $(INSTALL_DIR)/$(TARGET)
	rm -f $(INSTALL_DIR)/$(LINUX_FILE)
	@echo "Uninstalled $(TARGET) and $(LINUX_FILE) from $(INSTALL_DIR)"
	@echo "You may also want to remove 'export PATH=\"\$$PATH:$(INSTALL_DIR)\"' from your shell configuration file."

# Clean up build files
clean:
	cargo clean

# Help message
help:
	@echo "Makefile commands:"
	@echo "  make build      - Compile the project"
	@echo "  make install    - Install the project to $(INSTALL_DIR) and provide instructions for updating PATH"
	@echo "  make uninstall  - Uninstall the project from $(INSTALL_DIR)"
	@echo "  make clean      - Remove build files"
	@echo "  make help       - Show this help message"

.PHONY: all build install clean help
