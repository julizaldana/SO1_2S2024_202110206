### <div align="center">CONTENEDOR Rust</div>

#### - Cliente gRPC

<div align="center">
<img src=rust.png width=450>
</div>


* Recibe información vía http desde Locust

* Envía información por gRPC a contenedores de disciplinas.

```bash
# Construir la imagen 
docker build -t juliozaldana/ingenieria-rust-container:v6 .

# Iniciar sesión en Docker Hub
docker login

# Subir la imagen con la etiqueta v1
docker push juliozaldana/ingenieria-rust-container:v6
```