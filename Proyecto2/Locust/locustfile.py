from locust import HttpUser, task, between
import random
from faker import Faker

# Se inicializa faker
fake = Faker()

class WebsiteUser(HttpUser):
    wait_time = between(1, 5)  # Espera entre 1 y 5 segundos entre solicitudes

    @task
    def send_student_agronomia(self):
        disciplines = [1, 2, 3]
        
        # Generar estudiante de Agronomia
        student = {
            "name": fake.name(),
            "age": random.randint(18, 65),
            "faculty": "Agronomia",
            "discipline": random.choice(disciplines)
        }
        
        # Enviar la solicitud al endpoint de agronomia
        self.client.post("/agronomia", json=student)

    @task
    def send_student_ingenieria(self):
        disciplines = [1, 2, 3]
        
        # Generar estudiante de Ingenieria
        student = {
            "name": fake.name(),
            "age": random.randint(18, 65),
            "faculty": "Ingenieria",
            "discipline": random.choice(disciplines)
        }
        
        # Enviar la solicitud al endpoint de ingenieria
        self.client.post("/ingenieria", json=student)
