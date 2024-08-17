#!/bin/bash

# NOMBRE: JULIO ALEJANDRO ZALDAÑA RÍOS
# CARNET: 202110206
# TAREA 2 - LAB SISTEMAS OPERATIVOS 1

# Nombre de la imagen base
IMAGE="alpine"

# Número de contenedores a crear
NUM_CONTAINERS=10

# Crear los contenedores
for i in $(seq 1 $NUM_CONTAINERS); do
    # Generar un nombre aleatorio para el contenedor
    #CONTAINER_NAME=$(cat /dev/urandom | tr -dc 'a-z0-9' | fold -w 10 | head -n 1)
    
    # Se crea un contenedor utilizando la imagen Alpine
    docker run -d $IMAGE
    
    # Se verifica que el contenedor se creó correctamente
    if [ $? -eq 0 ]; then
        echo "Contenedor creado con éxito."
    else
        echo "Error al crear el contenedor."
    fi
done
