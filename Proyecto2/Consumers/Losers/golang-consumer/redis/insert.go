package redis

import (
	"context"
	"log"
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

// InsertLoser agrega a la lista de perdedores en Redis
func InsertLoser(value Log) {
	client := GetRedisInstance()
	err := client.RPush(context.Background(), "losers", value.Data.Name).Err()
	if err != nil {
		log.Println("Error saving loser in Redis: ", err)
	}
	log.Println("Loser saved on Redis -> ", value)
}
