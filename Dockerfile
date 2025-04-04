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

# Create start script with improved debugging and PORT handling
RUN echo '#!/bin/bash\n\
echo "DEBUG: Environment variables:"\n\
printenv\n\
echo "DEBUG: Starting server with PORT=${PORT:-8081}"\n\
export PORT="${PORT:-8081}"\n\
ldd ./portfolio-api\n\
./portfolio-api' > /app/start.sh && \
    chmod +x /app/start.sh

# Expose the default port (this is just documentation, not functional)
EXPOSE 8081

# Run the application using our wrapper script
CMD ["/app/start.sh"]
