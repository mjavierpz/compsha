# Nombre del binario
BIN_NAME = compsha

# Ruta del binario compilado
TARGET_DIR = target/release
BIN_PATH = $(TARGET_DIR)/$(BIN_NAME)

# Ruta de instalación
INSTALL_DIR = /usr/local/bin

# Comandos básicos
.PHONY: all build install uninstall clean

all: build

# Compilar en modo release
build:
	cargo build --release
	@echo "Compilación completa: $(BIN_PATH)"

# Instalar el binario en /usr/local/bin
install: build
	@sudo install -Dm755 $(BIN_PATH) $(INSTALL_DIR)/$(BIN_NAME)
	@echo "Instalado en $(INSTALL_DIR)/$(BIN_NAME)"

# Desinstalar el binario
uninstall:
	@sudo rm -f $(INSTALL_DIR)/$(BIN_NAME)
	@echo "Eliminado: $(INSTALL_DIR)/$(BIN_NAME)"

# Limpiar archivos de compilación
clean:
	cargo clean
	@echo "Archivos de compilación eliminados."
