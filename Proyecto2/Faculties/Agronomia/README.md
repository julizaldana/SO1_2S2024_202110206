### <div align="center">CONTENEDOR Golang</div>

#### - Cliente gRPC

<div align="center">
<img src=golang.png width=450>
</div>


* Recibe información vía http desde Locust

* Envía información por gRPC a contenedores de disciplinas.

#### Aplicar compilación para protos en ./proto

```bash
protoc --go_out=. --go-grpc_out=. client.proto
```

#### Subir imagen a Docker Hub

```bash
# Construir la imagen 
docker build -t juliozaldana/agronomia-go-container:v2 .

# Iniciar sesión en Docker Hub
docker login

# Subir la imagen con la etiqueta v1
docker push juliozaldana/agronomia-go-container:v2
```

