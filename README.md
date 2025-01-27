# compsha

`compsha` es una herramienta de línea de comandos escrita en Rust para calcular y verificar hashes de archivos utilizando algoritmos como **SHA-256** y **MD5**. Es útil para validar la integridad de archivos comparando su hash calculado con un hash esperado.

## Características

- **Compatibilidad con múltiples algoritmos de hash:**
  - SHA-256
  - MD5
- **Validación de hashes:** Muestra un indicador visual (**\u2713** o **\u2717**) dependiendo de si los hashes coinciden o no.
- **Interfaz sencilla y animación durante el cálculo del hash.**
- Comandos adicionales:
  - `--version`: Muestra la versión de la aplicación.
  - `help`: Explica cómo usar el programa.

## Instalación

1. **Clonar el repositorio:**
   ```bash
   git clone git@github.com:tuusuario/compsha.git
   cd compsha
   ```

2. **Compilar el proyecto:**
   ```bash
   cargo build --release
   ```

3. **Instalar mediante `make` (opcional):**
   ```bash
   sudo make install
   ```

Esto copiará el binario al directorio `/usr/local/bin`.

## Uso

### Comando básico
```bash
compsha <tipo_hash> <ruta_archivo> <hash_esperado>
```

- **`tipo_hash`**: Tipo de algoritmo de hash a utilizar (`sha256` o `md5`).
- **`ruta_archivo`**: Ruta del archivo cuyo hash deseas calcular.
- **`hash_esperado`**: El hash esperado contra el cual se realizará la validación.

### Ejemplos

1. Verificar un archivo usando SHA-256:
   ```bash
   compsha sha256 archivo.txt d2d2d0...abc123
   ```

2. Verificar un archivo usando MD5:
   ```bash
   compsha md5 archivo.txt 098f6b...e7fe3
   ```

### Opciones adicionales

- **Ver la versión del programa:**
  ```bash
  compsha --version
  ```

- **Ver ayuda:**
  ```bash
  compsha help
  ```

## Dependencias

El programa utiliza las siguientes bibliotecas externas en Rust:

- [sha2](https://crates.io/crates/sha2): Para cálculos con SHA-256.
- [md5](https://crates.io/crates/md5): Para cálculos con MD5.

Estas dependencias se gestionan automáticamente mediante Cargo.

## Licencia

Este proyecto está bajo la licencia MIT. Consulta el archivo `LICENSE` para más detalles.

