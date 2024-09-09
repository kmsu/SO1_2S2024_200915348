## Iniciar con modo env (eso es similar a crear un json de instancias)

* en ubuntu primero intalar en un cmd de sistema:
    sudo apt install python3.10-venv
* luego inicializar el "proyecto" python desde la bash (ruta del proyecto, en este caso /ServicioLogs)
    python3 -m venv env
    source env/bin/activate

* dentro de env instalar fastapi (esto para que solo se instale localmente en el proyecto y no de forma global)
    pip install "fastapi[standard]"

* Ejecutar (siempre en entorno env)
    fastapi dev main.py

## Para llenar los requirements
    pip freeze > requirements.txt

## para levantar el servidor de docker
    sudo docker run -d --name py_container -p 8000:8000 py_image
    sudo docker exec -it py_container bash

## levantar el servidor con docker compose
    docker compose up -d

    Ver:
    docker compose ps
    docker compose down
