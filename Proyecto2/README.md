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



#### 4. Consumidores Kafka - Redis




#### 5. Grafana - Prometheus