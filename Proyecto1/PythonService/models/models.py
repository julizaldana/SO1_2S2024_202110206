from pydantic import BaseModel  # type: ignore
from typing import List


#Se crea estructura para recibir información de procesos desde el servicio de Rust

class LogProcess(BaseModel):
    pid: int
    container_id: str
    name: str
    vsz: int
    rss: int
    memory_usage: float
    cpu_usage: float
    action: str  #esta no es tan importante
    timestamp: str


#Se crea estructura para recibir información de memoria RAM desde el servicio de Rust

class LogMemory(BaseModel):
    totalram: int
    freeram: int
    usedram: int
    timestamp: str
