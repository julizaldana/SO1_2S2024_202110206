package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"

	"github.com/segmentio/kafka-go"
)

// Estructura de los datos que recibirás, aquí podrías ajustarlo si sabes el formato exacto de los mensajes
type Student struct {
	Name       string `json:"name"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int    `json:"discipline"`
}

func main() {
	topic := "losers"

	// Configuración del lector de Kafka
	r := kafka.NewReader(kafka.ReaderConfig{
		Brokers:     []string{"my-cluster-kafka-bootstrap:9092"},
		Topic:       topic,
		Partition:   0,
		MinBytes:    10e3, // 10KB
		MaxBytes:    10e6, // 10MB
		StartOffset: kafka.LastOffset,
		GroupID:     "consumer-group-" + topic,
	})

	fmt.Printf("Consumer iniciado para el topic: %s\n", topic)

	for {
		m, err := r.ReadMessage(context.Background())
		if err != nil {
			log.Println("Error al leer el mensaje:", err)
			break
		}

		// Imprime el mensaje recibido en Kafka
		var student Student
		if err := json.Unmarshal(m.Value, &student); err != nil {
			log.Printf("Error al deserializar el mensaje: %v\n", err)
			continue
		}

		fmt.Printf("Mensaje recibido en el topic '%s' (offset %d): %v\n", topic, m.Offset, student)

		// Confirmación de que el mensaje fue leído
		if err := r.CommitMessages(context.Background(), m); err != nil {
			log.Println("Error al confirmar el mensaje:", err)
		}
	}

	// Cerrar el lector de Kafka
	if err := r.Close(); err != nil {
		log.Fatal("Error al cerrar el lector:", err)
	}
}
