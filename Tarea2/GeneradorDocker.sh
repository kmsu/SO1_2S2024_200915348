#!/bin/bash

# Función para generar un nombre aleatorio
getRandName() {
    echo "Tarea2_Container-$(tr -dc 0-9 </dev/urandom | head -c 8)"
}

# Crear 10 contenedores con nombres aleatorios
for i in {1..10}; do
    container_name=$(getRandName)
    docker run -d --name "$container_name" alpine sleep 3600
    #el sleep es para que ejecuten algo por una hora y no se detengan inmediatamente despues de crearse
    echo "Contenedor $i creado con nombre: $container_name"
done
