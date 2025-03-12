# Use Rust official image with required dependencies
FROM rust:latest as builder

# Set working directory
WORKDIR /app

# Copy source code and build
COPY . .
RUN cargo build --release

# Use a smaller Debian-based image for final runtime
FROM debian:bullseye-slim
WORKDIR /app

# Copy the built Rust binary from the builder stage
COPY --from=builder /app/target/release/portfolio-api .

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["./portfolio-api"]
