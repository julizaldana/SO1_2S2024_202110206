from locust import HttpUser, task, between
import random
from faker import Faker

# se inicializa faker
fake = Faker()

class WebsiteUser(HttpUser):
    wait_time = between(1, 5)  # Espera entre 1 y 5 segundos entre solicitudes
    host = "http://go-app.local"  # Configura el host apuntando al Ingress

    @task
    def send_students(self):
        faculties = ["Ingeniería", "Agronomía"]
        disciplines = [1, 2, 3]

        # Generar una lista de estudiantes aleatorios
        students = []
        for _ in range(3):  # Generar 3 estudiantes aleatorios
            student = {
                "name": fake.name(),  # Nombre aleatorio
                "age": random.randint(18, 65),  # Edad aleatoria entre 18 y 65
                "faculty": random.choice(faculties),  # Facultad aleatoria
                "discipline": random.choice(disciplines),  # Disciplina aleatoria
            }
            students.append(student)

        # Enviar la solicitud POST con los datos de los estudiantes aleatorios
        self.client.post("/students", json=students)
