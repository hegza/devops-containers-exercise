# Use the official Rust image
FROM docker.io/library/rust:1.81

# Set the working directory inside the container
WORKDIR /app

# Copy the source code to the container. We use .dockerignore to avoid unnecessary files.
COPY . .

# Build the application
RUN cargo build --release -Fsim-delay

# Run the program on container startup
CMD ["./target/release/service-rs"]
