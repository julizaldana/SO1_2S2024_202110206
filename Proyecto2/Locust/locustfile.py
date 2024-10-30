from locust import HttpUser, task, between
import random
from faker import Faker

# Se inicializa faker
fake = Faker()

class WebsiteUser(HttpUser):
    wait_time = between(1, 5)  # Espera entre 1 y 5 segundos entre solicitudes

    @task
    def send_student(self):
        faculties = ["Ingeniería", "Agronomía"]
        disciplines = [1, 2, 3]

        # Generar un solo estudiante aleatorio
        student = {
            "name": fake.name(),  # Nombre aleatorio
            "age": random.randint(18, 65),  # Edad aleatoria entre 18 y 65
            "faculty": random.choice(faculties),  # Facultad aleatoria
            "discipline": random.choice(disciplines),  # Disciplina aleatoria
        }

        # Enviar la solicitud POST con los datos del estudiante aleatorio
        self.client.post("/faculty", json=student)
