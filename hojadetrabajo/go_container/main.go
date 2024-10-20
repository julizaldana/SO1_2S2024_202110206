package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

type Student struct {
	Name       string `json:"name"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int    `json:"discipline"`
}

// Handler para recibir la información de los estudiantes
func receiveStudents(w http.ResponseWriter, r *http.Request) {
	var students []Student
	if err := json.NewDecoder(r.Body).Decode(&students); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// Mostrar la información recibida en consola
	for _, student := range students {
		fmt.Printf("Nombre: %s, Edad: %d, Facultad: %s, Disciplina: %d\n",
			student.Name, student.Age, student.Faculty, student.Discipline)
	}

	// Responder con un mensaje de éxito
	w.WriteHeader(http.StatusOK)
	w.Write([]byte("Datos recibidos correctamente"))
}

func main() {
	http.HandleFunc("/students", receiveStudents)
	fmt.Println("Servidor ejecutándose en http://localhost:8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
