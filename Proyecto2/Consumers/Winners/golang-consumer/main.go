package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"

	"golang-consumer/redis"

	"github.com/segmentio/kafka-go"
)

func processEvent(event []byte) {

	// Unmarshal the data
	var student redis.Student
	err := json.Unmarshal(event, &student)
	if err != nil {
		fmt.Printf("Failed to unmarshal message: %s", err)
		return
	}

	// Save in Redis directly using the student object
	go redis.InsertWinner(student)
}

func main() {
	topic := "winners"

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
		var student redis.Student
		if err := json.Unmarshal(m.Value, &student); err != nil {
			log.Printf("Error al deserializar el mensaje: %v\n", err)
			continue
		}

		fmt.Printf("Mensaje recibido en el topic '%s' (offset %d): %v\n", topic, m.Offset, student)

		// Process the event
		processEvent(m.Value)

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
