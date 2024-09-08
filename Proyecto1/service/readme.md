En la carpeta del proyecto crear el servicio rust con
cargo new [nombre] -> para este caso el nombre es service

en cargo.toml copia en las dependencias 

    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"

Para ejecutar el servicio, desde el directorio src usar:
    cargo run