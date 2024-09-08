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

#for ((i=0; i<10; i++)); do
#    image="${selected_images[i]}"
#    container_name="${image}_Container_$((i+1))"
    
    # Ejecutar el contenedor y asignarle el nombre
#    docker run -d --name "$container_name" "$image"
    
    # Verificar si el contenedor se creó con éxito
    #if [ $? -eq 0 ]; then
    #    echo "Contenedor $container_name creado exitosamente."
    #else
    #    echo "Error al crear el contenedor $container_name con la imagen $image."
    #fi
#done
