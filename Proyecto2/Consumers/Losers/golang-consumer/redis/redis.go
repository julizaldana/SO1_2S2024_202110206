package redis

import (
	"fmt"
	"sync"

	"github.com/redis/go-redis/v9"
)

var redisLock = &sync.Mutex{}

var redisClient *redis.Client

func Connect2Redis() *redis.Client {

	host := "redis-db-master.sopes1.svc.cluster.local" // Service name de Redis en Kubernetes
	port := "6379"
	client := redis.NewClient(&redis.Options{
		Addr:     host + ":" + port,
		Password: "XKcuMbTOcn", // Reemplaza con la contraseña de Redis decodificada
	})

	return client
}

func GetRedisInstance() *redis.Client {
	if redisClient == nil {
		redisLock.Lock()
		defer redisLock.Unlock()
		if redisClient == nil {
			fmt.Println("Creating single redis instance now.")
			redisClient = Connect2Redis()
		} else {
			fmt.Println("Single instance already created.")
		}
	} else {
		fmt.Println("Single instance already created.")
	}
	return redisClient
}
