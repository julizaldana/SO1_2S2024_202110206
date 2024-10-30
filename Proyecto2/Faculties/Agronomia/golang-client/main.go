package main

import (
	"context"
	"flag"
	pb "grpc-example/proto"
	"log"
	"time"

	"github.com/gofiber/fiber/v2"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

var (
	addrBoxeo     = flag.String("addrBoxeo", "go-boxeo:50051", "the address for Boxeo server")
	addrAtletismo = flag.String("addrAtletismo", "go-atletismo:50051", "the address for Atletismo server")
	addrNatacion  = flag.String("addrNatacion", "go-natacion:50051", "the address for Natacion server")
)

type Student struct {
	Name       string `json:"name"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int    `json:"discipline"`
}

func sendData(fiberCtx *fiber.Ctx) error {
	var body Student
	if err := fiberCtx.BodyParser(&body); err != nil {
		log.Println("Error parsing request body:", err)
		return fiberCtx.Status(400).JSON(fiber.Map{"error": err.Error()})
	}

	// Log to confirm receipt of data
	log.Printf("Received student data: %+v\n", body)

	// Select gRPC server based on discipline
	var addr string
	switch body.Discipline {
	case 1:
		addr = *addrNatacion
	case 2:
		addr = *addrAtletismo
	case 3:
		addr = *addrBoxeo
	default:
		return fiberCtx.Status(400).JSON(fiber.Map{"error": "Invalid discipline"})
	}

	// Set up a connection to the gRPC server.
	conn, err := grpc.Dial(addr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("Failed to connect to gRPC server: %v", err)
	}
	defer conn.Close()
	c := pb.NewStudentClient(conn)

	// Create channels to receive the response and error
	responseChan := make(chan *pb.StudentResponse)
	errorChan := make(chan error)

	go func() {
		ctx, cancel := context.WithTimeout(context.Background(), time.Second)
		defer cancel()

		r, err := c.GetStudent(ctx, &pb.StudentRequest{
			Name:       body.Name,
			Age:        int32(body.Age),
			Faculty:    body.Faculty,
			Discipline: pb.Discipline(body.Discipline),
		})

		if err != nil {
			errorChan <- err
			return
		}

		responseChan <- r
	}()

	select {
	case response := <-responseChan:
		log.Printf("gRPC response: %v\n", response.GetSuccess())
		return fiberCtx.JSON(fiber.Map{"message": response.GetSuccess()})
	case err := <-errorChan:
		log.Println("Error from gRPC server:", err)
		return fiberCtx.Status(500).JSON(fiber.Map{"error": err.Error()})
	case <-time.After(5 * time.Second):
		log.Println("Request to gRPC server timed out")
		return fiberCtx.Status(500).JSON(fiber.Map{"error": "timeout"})
	}
}

func main() {
	app := fiber.New()
	app.Post("/agronomia", sendData)

	log.Println("Starting server on port 8080...")
	if err := app.Listen(":8080"); err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}
