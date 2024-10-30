package main

import (
	"context"
	"encoding/json"
	"flag"
	"fmt"
	pb "grpc-server/proto"
	"log"
	"math/rand"
	"net"
	"time"

	"github.com/segmentio/kafka-go"
	"google.golang.org/grpc"
)

// Definir el puerto con flag
var (
	port = flag.Int("port", 50051, "The server port")
)

// Estructura del servidor
type server struct {
	pb.UnimplementedStudentServer
}

// Estructura del estudiante
type Student struct {
	Name       string `json:"name"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int    `json:"discipline"`
}

// Método gRPC que procesa al estudiante y lo envía a Kafka
func (s *server) GetStudent(_ context.Context, in *pb.StudentRequest) (*pb.StudentResponse, error) {
	log.Printf("Received student: %s, Faculty: %s", in.GetName(), in.GetFaculty())

	// Convertir el mensaje entrante a la estructura de estudiante
	student := Student{
		Name:       in.GetName(),
		Age:        int(in.GetAge()),
		Faculty:    in.GetFaculty(),
		Discipline: int(in.GetDiscipline()),
	}

	// Algoritmo de "lanzamiento de moneda" para determinar si el estudiante es ganador o perdedor
	rand.Seed(time.Now().UnixNano())
	topic := "losers" // Por defecto: perdedor
	if rand.Intn(2) == 0 {
		topic = "winners" // Si sale 0, el estudiante es ganador
	}

	// Enviar el estudiante al tópico adecuado de Kafka
	err := ProduceToKafka(student, topic)
	if err != nil {
		log.Printf("Failed to send to Kafka topic %s: %v", topic, err)
		return &pb.StudentResponse{Success: false}, err
	}

	return &pb.StudentResponse{Success: true}, nil
}

// Función para enviar el mensaje a Kafka
func ProduceToKafka(value Student, topic string) error {
	// Conectar con Kafka en el tópico adecuado
	conn, err := kafka.DialLeader(context.Background(), "tcp", "my-cluster-kafka-bootstrap:9092", topic, 0)
	if err != nil {
		return fmt.Errorf("failed to connect to Kafka: %w", err)
	}
	defer conn.Close()

	// Serializar la estructura de estudiante en JSON
	valueBytes, err := json.Marshal(value)
	if err != nil {
		return fmt.Errorf("failed to serialize message: %w", err)
	}

	// Escribir el mensaje en Kafka
	conn.SetWriteDeadline(time.Now().Add(10 * time.Second))
	_, err = conn.WriteMessages(kafka.Message{Value: valueBytes})
	if err != nil {
		return fmt.Errorf("failed to write message: %w", err)
	}

	log.Printf("Message sent to topic %s", topic)
	return nil
}

func main() {
	// Parsear el puerto desde la línea de comandos
	flag.Parse()

	// Iniciar el listener
	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", *port))
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}

	// Crear el servidor gRPC
	s := grpc.NewServer()
	pb.RegisterStudentServer(s, &server{})

	log.Printf("Server started on port %d", *port)
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Failed to serve: %v", err)
	}
}
