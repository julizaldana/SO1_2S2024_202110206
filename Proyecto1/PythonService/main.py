from fastapi import FastAPI  # type: ignore
import os
import json
from typing import List
from models.models import Logs
from fastapi.responses import StreamingResponse # type: ignore
import matplotlib.pyplot as plt  # type: ignore
from io import BytesIO

app = FastAPI()

@app.get("/")
def read_root():
    return {"Hello": "World"}

#RECIBIR LOGS DESDE RUST, Y ALMACENARLOS

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


#REALIZAR LAS GRAFICAS Y GENERAR IMAGENES CON MATPLOTLIB:

@app.get("/graph")
def generate_graph():
    logs_file = 'logs/logs.json'
    graphs_dir = 'graphs'  # Carpeta donde se guardarán las gráficas

    # Crear la carpeta si no existe
    if not os.path.exists(graphs_dir):
        os.makedirs(graphs_dir)

    # Leer el archivo logs.json
    if os.path.exists(logs_file):
        with open(logs_file, 'r') as file:
            logs = json.load(file)
    else:
        return {"error": "No logs found"}
    
    # Preparar los datos para las gráficas
    timestamps = []
    ram_used = []
    killed_processes_count = []

    for entry in logs:
        ram_info = entry.get('RAMmemory', [])
        if ram_info:
            timestamps.append(ram_info[0]['timestamp'])
            ram_used.append(ram_info[0]['usedram'])

        killed_processes_count.append(len(entry.get('KilledProcesses', [])))

    # Crear la gráfica de uso de RAM
    fig, ax1 = plt.subplots()

    color = 'tab:blue'
    ax1.set_xlabel('Timestamp')
    ax1.set_ylabel('Used RAM', color=color)
    ax1.plot(timestamps, ram_used, color=color)
    ax1.tick_params(axis='y', labelcolor=color)
    ax1.set_xticks(range(0, len(timestamps), max(1, len(timestamps)//10)))  # Reduce the number of x-ticks
    ax1.set_xticklabels(timestamps[::max(1, len(timestamps)//10)], rotation=45, ha="right")

    ax2 = ax1.twinx()
    color = 'tab:red'
    ax2.set_ylabel('Killed Processes', color=color)
    ax2.plot(timestamps, killed_processes_count, color=color)
    ax2.tick_params(axis='y', labelcolor=color)

    fig.tight_layout()

    # Guardar la gráfica en la carpeta 'graphs'
    graph_path = os.path.join(graphs_dir, 'ram_usage_graph.png')
    plt.savefig(graph_path)
    plt.close(fig)

    return {"message": f"Graph saved at {graph_path}"}


@app.get("/scatter_vsz_rss")
def generate_scatter_vsz_rss():
    logs_file = 'logs/logs.json'
    graphs_dir = 'graphs'  # Carpeta donde se guardarán las gráficas

    # Leer el archivo logs.json
    if os.path.exists(logs_file):
        with open(logs_file, 'r') as file:
            logs = json.load(file)
    else:
        return {"error": "No logs found"}

    # Extraer los valores de VSZ y RSS
    vsz_values = []
    rss_values = []

    for entry in logs:
        killed_processes = entry.get('KilledProcesses', [])
        for process in killed_processes:
            vsz_values.append(process.get('vsz', 0))
            rss_values.append(process.get('rss', 0))

    # Crear el gráfico de dispersión
    fig, ax = plt.subplots()
    ax.scatter(vsz_values, rss_values, color='blue')
    ax.set_xlabel('VSZ (KB)')
    ax.set_ylabel('RSS (KB)')
    ax.set_title('VSZ vs RSS of Killed Containers')

    # Guardar la gráfica en la carpeta 'graphs'
    graph_path = os.path.join(graphs_dir, 'scatter_vsz_vs_rss.png')
    plt.savefig(graph_path)
    plt.close(fig)

    return {"message": f"Graph saved at {graph_path}"}