# Dockerfile
FROM golang:1.21-bullseye AS builder

WORKDIR /app

# Instalar dependencias de compilación
RUN apt-get update && apt-get install -y \
    build-essential \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY go.mod go.sum ./
RUN go mod download

COPY . .

# Compilar con compatibilidad específica de GLIBC
RUN CGO_ENABLED=1 \
    GOOS=linux \
    GOARCH=amd64 \
    go build -a \
    -o main

# Usar debian slim para el runtime para asegurar la compatibilidad con GLIBC
FROM debian:bullseye-slim

# Instalar dependencias necesarias para el runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/main .

EXPOSE 50051
CMD ["./main"]