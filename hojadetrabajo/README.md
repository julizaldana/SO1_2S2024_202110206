## <div align="center">Hoja de Trabajo - Envío de tráfico con Locust y Recepción con contenedor de Golang</div>
##### <div align="center">Sistemas Operativos 1 - Sección A</div>
##### <div align="center">19 de octubre de 2024</div>

___

Para la hoja de trabajo del laboratorio del curso de Sistemas Operativos 1, se realizó el envío de tráfico con Locust (libería de Python) para poder generar información y enviarla a un contenedor donde contiene una aplicación elaborada con el lenguaje de programación de Golang. 

Se realizará la prueba de comunicación subiendo la imagen del programa de Golang a Docker Hub y poder acceder a ella, mediante un deployment en un cluster de Kubernetes en Google Cloud. 

### <div align="center">INSTALACIÓN LOCUST</div>

Se utiliza el comando para instalar de forma local:
```bash
pip install locust

locust --version #Sirve para ver la versión de Locust instalado
```

Se crea un archivo locustfile.py, donde se configura Locust, para que pueda enviar las peticiones de tráfico de información a la API de Golang. (Se envía información random, considerando la estructura tipo Estudiante del Proyecto2)



### <div align="center">CONTENEDOR GOLANG</div>

Se configura un servidor HTTP en Golang que recibe peticiones POST en la ruta /students con un JSON que contiene información de varios estudiantes. 

Se usa la estructura tipo Estudiante, que se utilizará en el Proyecto 2.

![alt text](./images/golang.png)

#### Uso de Docker para contenerizar la aplicación de Golang

Se crea un archivo Dockerfile para crear la imagen y luego el contenedor con el servidor sencillo de Golang.

```bash
docker build -t juliozaldana/go-container:latest .

#Subir imagen a DockerHub.

docker login

docker push juliozaldana/go-container:latest
```

![alt text](./images/dockerhub.png)


### <div align="center">RESUMEN DE FLUJO EN GKE</div>

1. **Deployment:** Define el pod con el contenedor de Golang.
2. **Service:** Se expone el contenedor Golang en el clúster de Kubernetes.
3. **Ingress:** Proporciona acceso externo al servicio a través de un controlador Ingress.
4. **Locust:** Se ejecuta Locust localmente para enviar peticiones hacia la aplicación Golang en Kubernetes a través del Ingress.

Comandos para desplegar en Kubernetes:


```bash
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
kubectl apply -f ingress.yaml
```


Verificar los logs en tiempo real, del contenedor golang en un pod.

```bash
kubectl logs -f <name_pod>
```
