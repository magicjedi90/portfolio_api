# Use Rust official image with required dependencies
FROM rust:latest as builder

# Set working directory
WORKDIR /app

# Copy only the files needed for building
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build dependencies first (this layer will be cached)
RUN cargo build --release

# Use Ubuntu 22.04 as the base image (has OpenSSL 3)
FROM ubuntu:22.04
WORKDIR /app

# Install SSL libraries and other runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the built Rust binary from the builder stage
COPY --from=builder /app/target/release/portfolio-api .

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["./portfolio-api"]
