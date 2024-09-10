En la carpeta del proyecto crear el servicio rust con
cargo new [nombre] -> para este caso el nombre es service

en cargo.toml copia en las dependencias 

    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"

Para ejecutar el servicio, desde el directorio src usar:
    cargo run

Si hubiera necesidad de instalar OpenSSL al usar reqwest para tokio
    en cmd del sistema instalar (Ubuntu):
        sudo apt-get update
        sudo apt-get install libssl-dev

    Configurar variables de entorno
        export OPENSSL_DIR=/usr/local/opt/openssl
        export PKG_CONFIG_PATH=$OPENSSL_DIR/lib/pkgconfig

    Verificar
        pkg-config --libs --cflags openssl
        --nota: si esto no devuelve nada, esta mal instalado

