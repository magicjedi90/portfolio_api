# Use Rust slim image for smaller size and faster builds
FROM rust:slim as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /app
RUN cargo new --bin portfolio-api
WORKDIR /app/portfolio-api

# Copy only the manifests first
COPY Cargo.toml Cargo.lock ./

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Now copy your source code
COPY src ./src

# Build for release
RUN touch src/main.rs && cargo build --release

# Use Ubuntu 22.04 as the base image (has OpenSSL 3)
FROM ubuntu:22.04
WORKDIR /app

# Install SSL libraries and other runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/portfolio-api/target/release/portfolio-api .

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["./portfolio-api"]
