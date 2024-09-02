#!/bin/bash

# Lista de imagenes
images=(
    "low_image1"
    "high_image1"
    "low_image2"
    "high_image2"
)

# Crear una lista de 10 imagenes seleccionadas aleatoriamente
selected_images=()
for ((i=0; i<10; i++)); do
    selected_images+=("${images[RANDOM % ${#images[@]}]}")
done

# Crear 10 contenedores
for image in "${selected_images[@]}"; do
    docker run -d "$image"
done