package redis

import (
	"context"
	"fmt"
	"log"
	"time"
)

type Log struct {
	Data      Student
	CreatedAt string
}

type Student struct {
	Name       string `json:"name"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int    `json:"discipline"`
}

// InsertLoser agrega los datos del estudiante como un hash en Redis
func InsertLoser(student Student) {
	client := GetRedisInstance()
	createdAt := time.Now().Format(time.RFC3339)

	// Crear una clave única para cada estudiante ganador
	studentKey := fmt.Sprintf("loser:%s", student.Name)

	// Insertar los datos como un hash
	err := client.HSet(context.Background(), studentKey, map[string]interface{}{
		"name":       student.Name,
		"age":        student.Age,
		"faculty":    student.Faculty,
		"discipline": student.Discipline,
		"created_at": createdAt,
	}).Err()

	if err != nil {
		log.Println("Error saving loser in Redis:", err)
	} else {
		log.Println("Loser saved on Redis ->", studentKey)
	}

	//CONTADORES FINALES

	// Incrementar un contador para el conteo por facultad
	client.HIncrBy(context.Background(), "new_faculty_count", student.Faculty, 1)

	// Incrementar un contador específico para el conteo de perdedores por disciplina
	client.HIncrBy(context.Background(), "new_loser_discipline_count", fmt.Sprint(student.Discipline), 1)

	// Incrementar un contador para el conteo por facultad
	client.HIncrBy(context.Background(), "new_loser_faculty_count", student.Faculty, 1)

}
