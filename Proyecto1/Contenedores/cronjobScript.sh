#!/bin/bash

# Ruta al script1
script_path="./script.sh"

# Bucle infinito para ejecutar script1 cada 60 segundos
while true; do
    echo "Ejecutando script..."
    
    # Ejecutar script1
    bash "$script_path"
    
    # Esperar 60 segundos antes de la próxima ejecución
    sleep 120
done
