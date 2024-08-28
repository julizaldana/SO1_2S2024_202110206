#SCRIPT DE ALTO CONSUMO CON PYTHON
#PROGRAMA CON UN CICLO WHILE INFINITO

import time
from datetime import datetime

while True:
    for _ in range(1000):
        list = [i ** 5 for i in range(100)] 
    print(f"¡Hola que tengas un buen día! La fecha de hoy es {datetime.now()}")
    time.sleep(0.1)
