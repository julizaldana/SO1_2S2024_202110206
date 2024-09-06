from fastapi import FastAPI  # type: ignore
import os
import json
from typing import List
from models.models import Logs

app = FastAPI()

@app.get("/")
def read_root():
    return {"Hello": "World"}

@app.post("/logs")
def get_logs(logs: Logs):
    logs_file = 'logs/logs.json'

    # Checar si existe el archivo logs.json
    if os.path.exists(logs_file):
        # Leer el archivo logs.json
        with open(logs_file, 'r') as file:
            existing_logs = json.load(file)
    else:
        # Si no existe, crear una lista vacía
        existing_logs = []

    # Agregar los nuevos logs a la lista existente
    new_logs = logs.dict()
    
    # Guardar en el archivo de logs
    existing_logs.append(new_logs)

    # Escribir los logs actualizados en el archivo
    with open(logs_file, 'w') as file:
        json.dump(existing_logs, file, indent=4)

    return {"received": True}