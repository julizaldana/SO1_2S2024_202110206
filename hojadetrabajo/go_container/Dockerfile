FROM golang:1.18-alpine

WORKDIR /app

COPY go.mod ./
RUN go mod download

COPY . .

RUN go build -o app .

# Exponer el puerto 8080
EXPOSE 8080

# Ejecutar la aplicación
CMD ["./app"]
