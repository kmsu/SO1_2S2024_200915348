package main

import (
	"context"
	"flag"
	"fmt"
	pb "grpc-example/proto"
	"log"
	"time"

	"github.com/gofiber/fiber/v2"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

var (
	addrNatacion  = flag.String("addrNatacion", "disciplinas-service:50051", "the address for swimming server")
	addrAtletismo = flag.String("addrAtletismo", "disciplinas-service:50052", "the address for athletics server")
	addrBoxeo     = flag.String("addrBoxeo", "disciplinas-service:50053", "the address for boxing server")
)

// Estructura para recibir los datos JSON
type Student struct {
	Name       string json:"student" //Viene student y se pone en el campo name
	Age        int    json:"age"
	Faculty    string json:"faculty"
	Discipline int    json:"discipline"
}

// Función para enviar datos al servidor gRPC
func sendData(student Student) error {
	var addr string
	switch student.Discipline {
	case 1:
		addr = *addrNatacion
	case 2:
		addr = *addrAtletismo
	case 3:
		addr = *addrBoxeo
	default:
		return fmt.Errorf("invalid discipline: %d", student.Discipline)
	}

	// Configurar conexión al servidor gRPC
	conn, err := grpc.Dial(addr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	c := pb.NewStudentClient(conn)

	// Crear un canal para recibir la respuesta y el error
	responseChan := make(chan *pb.StudentResponse)
	errorChan := make(chan error)

	go func() {
		// Contactar al servidor gRPC y obtener su respuesta
		ctx, cancel := context.WithTimeout(context.Background(), time.Second)
		defer cancel()

		r, err := c.GetStudent(ctx, &pb.StudentRequest{
			Name:       student.Name,
			Age:        int32(student.Age),
			Faculty:    student.Faculty,
			Discipline: pb.Discipline(student.Discipline),
		})

		if err != nil {
			errorChan <- err
			return
		}

		responseChan <- r
	}()

	select {
	case response := <-responseChan:
		log.Printf("Response from gRPC: %s", response.GetSuccess())
		return nil
	case err := <-errorChan:
		return err
	case <-time.After(5 * time.Second):
		return context.DeadlineExceeded
	}
}

// Nueva función para recibir JSON desde Locust
func receiveData(fiberCtx *fiber.Ctx) error {
	var body Student
	if err := fiberCtx.BodyParser(&body); err != nil {
		return fiberCtx.Status(400).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	// Imprimir los datos JSON que llegaron
	log.Printf("Datos JSON recibidos: %+v\n", body)

	// Enviar los datos al servidor gRPC
	if err := sendData(body); err != nil {
		log.Printf("Error sending data to gRPC: %v", err)
		return fiberCtx.Status(500).JSON(fiber.Map{
			//"error": err.Error(),
		})
	}

	return fiberCtx.JSON(fiber.Map{
		"message": "Data received and sent to gRPC successfully",
	})
}

func main() {
	app := fiber.New()

	// Endpoint para recibir datos de Locust
	app.Post("/golang", receiveData)

	err := app.Listen(":8080")
	if err != nil {
		log.Println(err)
		return
	}
}