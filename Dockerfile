# Stage 1: Build the Rust project
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/keymastro

# Copy the source code
COPY . .

# Build the project
RUN cargo build --release

# Stage 2: Create a minimal image with the compiled binary
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/keymastro/target/release/keymastro .

# Set the entrypoint
ENTRYPOINT ["./keymastro"]
