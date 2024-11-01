### <div align="center">CONTENEDORES Golang CONSUMERS</div>

#### - Consumers de Kafka

<div align="center">
<img src=consumers.png width=450>
</div>

* Consumen mensajes provenientes de los tópicos de kafka, ya sean winners o losers.

* Insertan los estudiantes respectivos de winners y losers, en Redis.


```bash
# Construir las imagenes de consumidores golang
docker build -t juliozaldana/winners-go-consumer:v4 .
docker build -t juliozaldana/losers-go-consumer:v3 .

# Iniciar sesión en Docker Hub
docker login

# Subir la imagen con la etiqueta respectiva
docker push juliozaldana/winners-go-consumer:v4
docker push juliozaldana/losers-go-consumer:v3
```

