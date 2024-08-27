# Dockerfile para un contenedor de bajo consumo de CPU

FROM python:3.9-slim

RUN pip install flask

COPY app.py /app.py

CMD ["python", "/app.py"]
