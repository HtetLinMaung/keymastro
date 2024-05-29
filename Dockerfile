# Stage 1: Build the Rust project
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/keymastro

# Copy the source code
COPY . .

# Build the project
RUN cargo build --release

# Stage 2: Create a minimal image with the compiled binary
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libpq5 libssl1.1 && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/keymastro/target/release/keymastro .

# Set the entrypoint
ENTRYPOINT ["./keymastro"]
