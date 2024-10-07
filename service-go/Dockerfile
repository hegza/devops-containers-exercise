# Use an official Go runtime as a parent image
FROM docker.io/library/golang:1.23.2

# Set the working directory inside the container
WORKDIR /app

# Copy the source code to the container. We use .dockerignore to avoid unnecessary files.
COPY . .

# Download all Go modules and dependencies
RUN go mod download

# Build the application
RUN go build -o build/main .

# Run the program on container startup
CMD ["./build/main"]
