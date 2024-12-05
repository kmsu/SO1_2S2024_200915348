package main

import (
	"context"
	"flag"
	"fmt"
	pb "go-client/proto"
	"log"
	"time"

	"github.com/gofiber/fiber/v2"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

var (
	addrSwimming = flag.String("addrSwimming", "go-server-service:50051", "the address to connect to swimming")
	addrBoxing   = flag.String("addrBoxing", "go-server-service:50052", "the address to connect to boxing")
	addrAtletist = flag.String("addrAtletist", "go-server-service:50053", "the address to connect to atletist")
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
		return fiberCtx.Status(400).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	// Imprimir los datos JSON que llegaron
	log.Printf("Datos JSON recibidos: %+v\n", body)

	// Seleccionar la dirección del servidor en función de la disciplina
	var serverAddr string
	switch body.Discipline {
	case 0:
		serverAddr = *addrSwimming
	case 1:
		serverAddr = *addrBoxing
	case 2:
		serverAddr = *addrAtletist
	default:
		return fiberCtx.Status(400).JSON(fiber.Map{
			"error": "Invalid discipline value",
		})
	}

	// Intentar establecer conexión con el servidor gRPC correspondiente
	conn, err := grpc.Dial(serverAddr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		customError := fmt.Sprintf("Error connecting to discipline %d (%s): %v", body.Discipline, serverAddr, err)
		log.Println(customError)
		return fiberCtx.Status(500).JSON(fiber.Map{
			"error": customError,
		})
	}
	defer conn.Close()
	c := pb.NewStudentClient(conn)

	// Canal para recibir la respuesta y error
	responseChan := make(chan *pb.StudentResponse)
	errorChan := make(chan error)
	go func() {
		// Contactar al servidor y obtener respuesta
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
		return fiberCtx.JSON(fiber.Map{
			"message": response.Success,
		})
	case err := <-errorChan:
		return fiberCtx.Status(500).JSON(fiber.Map{
			"error":      err.Error(),
			"discipline": body.Discipline,
		})
	case <-time.After(5 * time.Second):
		return fiberCtx.Status(500).JSON(fiber.Map{
			"error":      "timeout",
			"discipline": body.Discipline,
		})
	}
}

func main() {
	flag.Parse()

	app := fiber.New()
	app.Post("/faculty", sendData)

	err := app.Listen(":8080")
	if err != nil {
		log.Println(err)
		return
	}
}
