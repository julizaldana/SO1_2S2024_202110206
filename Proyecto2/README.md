## <div align="center">Proyecto #2</div>
### <div align="center">Kubernetes y Servicios</div>
#### <div align="center">Olimpiadas USAC</div>
##### <div align="center">Sistemas Operativos 1 - Sección A</div>
##### <div align="center">1 de noviembre de 2024</div>
___

#### **<div align="center">Introducción</div>**




____


#### **<div align="center">Comandos de Kubernetes</div>**

Creación de reglas de firewall de entrada y salida.

Instalación de Helm y NGINX-Ingress-Controller para poder facilitar la entrada de tráfico en la arquitectura.

#### 1. Ingress

Permite la entrada de tráfico, mediante Locust


```bash
# Aplicar ingress

kubectl apply -f ingress.yaml
```


#### 2. Facultades

- Aplicación de Deployment y Service 

```bash
# AGRONOMIA
kubectl apply -f goclient.yaml

# INGENIERIA
kubectl apply -f rustclient.yaml
```


#### 2. Disciplinas

- Aplicación de Deployment y Service

```bash
# NATACION
kubectl apply -f gonatacion.yaml

# BOXEO
kubectl apply -f goboxeo.yaml

# ATLETISMO
kubectl apply -f goatletismo.yaml
```



#### 3. Kafka

Se utiliza kafka, para el envío y recepción de datos/mensajes en forma de colas.

Se instala Strimzi en el namespace creado del proyecto:

```bash
kubectl create -f 'https://strimzi.io/install/latest?namespace=sopes1' -n sopes1

# Desplegar el cluster de kafka 

kubectl apply -f https://strimzi.io/examples/latest/kafka/kafka-persistent-single.yaml -n sopes1

# Aplicar topicos definidos en el archivo kafka-topics.yaml
kubectl apply -f kafka-topics.yaml -n sopes1
```

```bash
# Se puede utilizar para verificar el pod de kafka, para ver mensajes
kubectl exec -it my-cluster-kafka-0 -n sopes1 -- /bin/bash

# Se puede verificar los mensajes recibidos en los topicos, winners o losers.

/opt/kafka/bin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic winners --from-beginning
/opt/kafka/bin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic losers --from-beginning
```

#### 4. Consumidores Kafka 

Los consumidores recibirán los mensajes de los tópicos de kafka, para insertarlos en redis.

```bash
# TOPIC: WINNERS
kubectl apply -f consumer-winners.yaml

# TOPIC: LOSERS
kubectl apply -f consumer-losers.yaml
```


#### 5. Redis


Agregar el repositorio de Helm para Redis:

```bash
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update
```
Instalar redis con helm.

```bash
helm install redis-db bitnami/redis --namespace sopes1 --create-namespace
```
Verificar contraseña de redis.

```bash
export REDIS_PASSWORD=$(kubectl get secret --namespace sopes1 redis-db -o jsonpath="{.data.redis-password}" | base64 -d)
echo $REDIS_PASSWORD
```

**Verficar en pods de redis, si se guarda info**

```bash
kubectl exec -it redis-db-master-0 --namespace sopes1 -- redis-cli

AUTH PAJRLlnnPn

# Verificar todas las claves de ganadores
KEYS "winner:*"

# Acceder a los datos de un estudiante específico
HGETALL "winner:nombre"
```

#### 6. Grafana - Prometheus


