### <div align="center">CONTENEDORES Golang Disciplinas</div>

#### - Servidores gRPC

<div align="center">
<img src=disciplines.png width=450>
</div>


* Recibe información por gRPC de contenedores de facultades.

* Envía información por gRPC a kafka.

* Son publishers/producers a kafka.

#### Aplicar compilación para protos en ./proto

```bash
protoc --go_out=. --go-grpc_out=. client.proto
```

```bash
# Construir la imagen disciplinas
docker build -t juliozaldana/natacion-go-container:v1 .
docker build -t juliozaldana/boxeo-go-container:v1 .
docker build -t juliozaldana/atletismo-go-container:v1 .

# Iniciar sesión en Docker Hub
docker login

# Subir la imagen con la etiqueta v1
docker push juliozaldana/natacion-go-container:v1 
docker push juliozaldana/boxeo-go-container:v1 
docker push juliozaldana/atletismo-go-container:v1 
```

